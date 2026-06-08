//! Driving adapter: ratatui application shell.

use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};
use ratatui::{Frame, Terminal};

use crate::application::create_task_list::{CreateError, CreateTaskList};
use crate::application::delete_task_list::{DeleteError, DeleteTaskList};
use crate::application::list_task_lists::ListTaskLists;
use crate::application::ports::{IdGenerator, TaskListRepository};
use crate::application::rename_task_list::{RenameError, RenameTaskList};
use crate::application::task_commands::{
    AddTask, CompleteTask, DeleteTask, ListTasks, RenameTask, TaskCommandError,
};
use crate::domain::task::Task;
use crate::domain::task_list::{TaskList, TaskListId};

pub struct App<R, I> {
    repo: R,
    ids: I,
    state: AppState,
}

#[derive(Default)]
struct AppState {
    lists: Vec<TaskList>,
    tasks: Vec<Task>,
    selected_list: usize,
    selected_task: usize,
    mode: Mode,
    interaction: Interaction,
    input: String,
    status: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum Mode {
    #[default]
    Lists,
    Tasks(usize),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
enum Interaction {
    #[default]
    None,
    Help,
    Editing(EditAction),
    Confirming(ConfirmAction),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum EditAction {
    CreateList,
    RenameList(usize),
    CreateTask(usize),
    RenameTask {
        list_index: usize,
        task_index: usize,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ConfirmAction {
    DeleteList(usize),
    DeleteTask {
        list_index: usize,
        task_index: usize,
    },
}

impl<R: TaskListRepository, I: IdGenerator> App<R, I> {
    pub fn new(repo: R, ids: I) -> Self {
        Self {
            repo,
            ids,
            state: AppState::default(),
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.refresh().map_err(io_error)?;
        let mut terminal = TerminalSession::start()?;

        loop {
            terminal.terminal.draw(|frame| self.render(frame))?;

            if event::poll(Duration::from_millis(250))? {
                match event::read()? {
                    Event::Key(key) if self.handle_key(key.code).map_err(io_error)? => break,
                    _ => {}
                }
            }
        }

        Ok(())
    }

    pub fn load_once(&mut self) -> Result<(), String> {
        self.refresh()
    }

    fn refresh(&mut self) -> Result<(), String> {
        self.state.lists = ListTaskLists::new(&self.repo).execute().map_err(|e| e.0)?;
        self.clamp_selection();
        if matches!(self.state.mode, Mode::Tasks(_)) {
            self.reload_tasks()?;
        }
        Ok(())
    }

    fn handle_key(&mut self, code: KeyCode) -> Result<bool, String> {
        match self.state.interaction.clone() {
            Interaction::None => self.handle_normal_key(code),
            Interaction::Help => {
                // q quits (true); Esc/? closes help (false) — different returns, so two separate ifs
                if code == KeyCode::Char('q') {
                    return Ok(true);
                }
                if matches!(code, KeyCode::Esc | KeyCode::Char('?')) {
                    self.state.interaction = Interaction::None;
                }
                Ok(false)
            }
            Interaction::Editing(action) => {
                self.handle_edit_key(action, code)?;
                Ok(false)
            }
            Interaction::Confirming(action) => {
                self.handle_confirm_key(action, code)?;
                Ok(false)
            }
        }
    }

    fn handle_normal_key(&mut self, code: KeyCode) -> Result<bool, String> {
        match code {
            KeyCode::Char('q') => Ok(true),
            KeyCode::Char('?') => {
                self.state.interaction = Interaction::Help;
                Ok(false)
            }
            KeyCode::Char('n') => {
                self.start_create();
                Ok(false)
            }
            KeyCode::Char('r') => {
                self.start_rename();
                Ok(false)
            }
            KeyCode::Char('d') => {
                self.start_delete();
                Ok(false)
            }
            KeyCode::Char(' ') => {
                self.complete_selected_task()?;
                Ok(false)
            }
            KeyCode::Down => {
                self.select_next();
                Ok(false)
            }
            KeyCode::Up => {
                self.select_previous();
                Ok(false)
            }
            KeyCode::Enter | KeyCode::Right => {
                // Right is a no-op in Tasks mode — guard lives inside open_tasks
                if self.state.mode == Mode::Lists {
                    self.open_tasks()?;
                }
                Ok(false)
            }
            KeyCode::Esc | KeyCode::Left => {
                if matches!(self.state.mode, Mode::Tasks(_)) {
                    self.state.mode = Mode::Lists;
                    self.state.tasks.clear();
                    self.state.selected_task = 0;
                }
                Ok(false)
            }
            _ => Ok(false),
        }
    }

    fn handle_edit_key(&mut self, action: EditAction, code: KeyCode) -> Result<(), String> {
        match code {
            KeyCode::Enter => self.submit_edit(action),
            KeyCode::Esc => {
                self.state.interaction = Interaction::None;
                self.state.input.clear();
                self.state.status = "Cancelled".to_string();
                Ok(())
            }
            KeyCode::Backspace => {
                self.state.input.pop();
                Ok(())
            }
            KeyCode::Char(c) if !c.is_control() => {
                self.state.input.push(c);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_confirm_key(&mut self, action: ConfirmAction, code: KeyCode) -> Result<(), String> {
        match code {
            KeyCode::Char('y') | KeyCode::Char('Y') => self.submit_confirm(action),
            KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                self.state.interaction = Interaction::None;
                self.state.status = "Cancelled".to_string();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn start_create(&mut self) {
        self.state.input.clear();
        self.state.interaction = match self.state.mode {
            Mode::Lists => Interaction::Editing(EditAction::CreateList),
            Mode::Tasks(list_index) => Interaction::Editing(EditAction::CreateTask(list_index)),
        };
        self.state.status.clear();
    }

    fn start_rename(&mut self) {
        self.state.input.clear();
        self.state.interaction = match self.state.mode {
            Mode::Lists if !self.state.lists.is_empty() => {
                Interaction::Editing(EditAction::RenameList(self.state.selected_list))
            }
            Mode::Tasks(list_index) if !self.state.tasks.is_empty() => {
                Interaction::Editing(EditAction::RenameTask {
                    list_index,
                    task_index: self.state.selected_task,
                })
            }
            _ => {
                self.state.status = "Nothing to rename".to_string();
                Interaction::None
            }
        };
    }

    fn start_delete(&mut self) {
        self.state.interaction = match self.state.mode {
            Mode::Lists if !self.state.lists.is_empty() => {
                Interaction::Confirming(ConfirmAction::DeleteList(self.state.selected_list))
            }
            Mode::Tasks(list_index) if !self.state.tasks.is_empty() => {
                Interaction::Confirming(ConfirmAction::DeleteTask {
                    list_index,
                    task_index: self.state.selected_task,
                })
            }
            _ => {
                self.state.status = "Nothing to delete".to_string();
                Interaction::None
            }
        };
    }

    fn submit_edit(&mut self, action: EditAction) -> Result<(), String> {
        let value = self.state.input.trim().to_string();
        self.state.input.clear();
        self.state.interaction = Interaction::None;

        match action {
            EditAction::CreateList => {
                match CreateTaskList::new(&mut self.repo, &self.ids).execute(&value) {
                    Ok(id) => {
                        self.refresh()?;
                        self.select_list_id(&id);
                        self.state.mode = Mode::Lists;
                        self.state.status = format!("Created list {value}");
                        Ok(())
                    }
                    Err(e) => {
                        self.state.status = create_error_message(e);
                        Ok(())
                    }
                }
            }
            EditAction::RenameList(index) => {
                let Some(id) = self.list_id_at(index) else {
                    self.state.status = "List not found".to_string();
                    return Ok(());
                };
                match RenameTaskList::new(&mut self.repo).execute(&id, &value) {
                    Ok(()) => {
                        self.refresh()?;
                        self.state.selected_list =
                            index.min(self.state.lists.len().saturating_sub(1));
                        self.state.status = format!("Renamed list to {value}");
                        Ok(())
                    }
                    Err(e) => {
                        self.state.status = rename_error_message(e);
                        Ok(())
                    }
                }
            }
            EditAction::CreateTask(list_index) => {
                let Some(id) = self.list_id_at(list_index) else {
                    self.state.status = "List not found".to_string();
                    return Ok(());
                };
                match AddTask::new(&mut self.repo).execute(&id, &value) {
                    Ok(()) => {
                        self.state.selected_list = list_index;
                        self.state.mode = Mode::Tasks(list_index);
                        self.refresh()?;
                        if !self.state.tasks.is_empty() {
                            self.state.selected_task = self.state.tasks.len() - 1;
                        }
                        self.state.status = format!("Created task {value}");
                        Ok(())
                    }
                    Err(e) => {
                        self.state.status = task_error_message(e);
                        Ok(())
                    }
                }
            }
            EditAction::RenameTask {
                list_index,
                task_index,
            } => {
                let Some(id) = self.list_id_at(list_index) else {
                    self.state.status = "List not found".to_string();
                    return Ok(());
                };
                let Some(old_title) = self.task_title_at(task_index) else {
                    self.state.status = "Task not found".to_string();
                    return Ok(());
                };
                match RenameTask::new(&mut self.repo).execute(&id, &old_title, &value) {
                    Ok(()) => {
                        self.state.selected_list = list_index;
                        self.state.mode = Mode::Tasks(list_index);
                        self.refresh()?;
                        self.state.selected_task =
                            task_index.min(self.state.tasks.len().saturating_sub(1));
                        self.state.status = format!("Renamed task to {value}");
                        Ok(())
                    }
                    Err(e) => {
                        self.state.status = task_error_message(e);
                        Ok(())
                    }
                }
            }
        }
    }

    fn submit_confirm(&mut self, action: ConfirmAction) -> Result<(), String> {
        self.state.interaction = Interaction::None;
        match action {
            ConfirmAction::DeleteList(index) => {
                let Some(id) = self.list_id_at(index) else {
                    self.state.status = "List not found".to_string();
                    return Ok(());
                };
                match DeleteTaskList::new(&mut self.repo).execute(&id) {
                    Ok(()) => {
                        self.state.mode = Mode::Lists;
                        self.state.tasks.clear();
                        self.state.selected_list = index;
                        self.refresh()?;
                        self.state.status = "Deleted list".to_string();
                        Ok(())
                    }
                    Err(e) => {
                        self.state.status = delete_error_message(e);
                        Ok(())
                    }
                }
            }
            ConfirmAction::DeleteTask {
                list_index,
                task_index,
            } => {
                let Some(id) = self.list_id_at(list_index) else {
                    self.state.status = "List not found".to_string();
                    return Ok(());
                };
                let Some(title) = self.task_title_at(task_index) else {
                    self.state.status = "Task not found".to_string();
                    return Ok(());
                };
                match DeleteTask::new(&mut self.repo).execute(&id, &title) {
                    Ok(()) => {
                        self.state.selected_list = list_index;
                        self.state.selected_task = task_index;
                        self.state.mode = Mode::Tasks(list_index);
                        self.refresh()?;
                        self.state.status = "Deleted task".to_string();
                        Ok(())
                    }
                    Err(e) => {
                        self.state.status = task_error_message(e);
                        Ok(())
                    }
                }
            }
        }
    }

    fn complete_selected_task(&mut self) -> Result<(), String> {
        let Mode::Tasks(list_index) = self.state.mode else {
            return Ok(());
        };
        let Some(id) = self.list_id_at(list_index) else {
            self.state.status = "List not found".to_string();
            return Ok(());
        };
        let Some(title) = self.task_title_at(self.state.selected_task) else {
            self.state.status = "Task not found".to_string();
            return Ok(());
        };
        match CompleteTask::new(&mut self.repo).execute(&id, &title) {
            Ok(()) => {
                self.state.selected_list = list_index;
                self.state.mode = Mode::Tasks(list_index);
                self.refresh()?;
                self.state.status = "Completed task".to_string();
                Ok(())
            }
            Err(e) => {
                self.state.status = task_error_message(e);
                Ok(())
            }
        }
    }

    fn select_next(&mut self) {
        if self.state.interaction != Interaction::None {
            return;
        }
        match self.state.mode {
            Mode::Lists if !self.state.lists.is_empty() => {
                self.state.selected_list = (self.state.selected_list + 1) % self.state.lists.len();
                self.state.status = self.selection_status();
            }
            Mode::Tasks(_) if !self.state.tasks.is_empty() => {
                self.state.selected_task = (self.state.selected_task + 1) % self.state.tasks.len();
                self.state.status = self.selection_status();
            }
            _ => {
                self.state.status = self.selection_status();
            }
        }
    }

    fn select_previous(&mut self) {
        if self.state.interaction != Interaction::None {
            return;
        }
        match self.state.mode {
            Mode::Lists if !self.state.lists.is_empty() => {
                self.state.selected_list = if self.state.selected_list == 0 {
                    self.state.lists.len() - 1
                } else {
                    self.state.selected_list - 1
                };
                self.state.status = self.selection_status();
            }
            Mode::Tasks(_) if !self.state.tasks.is_empty() => {
                self.state.selected_task = if self.state.selected_task == 0 {
                    self.state.tasks.len() - 1
                } else {
                    self.state.selected_task - 1
                };
                self.state.status = self.selection_status();
            }
            _ => {
                self.state.status = self.selection_status();
            }
        }
    }

    fn clamp_selection(&mut self) {
        if self.state.lists.is_empty() {
            self.state.selected_list = 0;
            self.state.selected_task = 0;
            self.state.tasks.clear();
            self.state.mode = Mode::Lists;
        } else {
            self.state.selected_list = self.state.selected_list.min(self.state.lists.len() - 1);
            if let Mode::Tasks(index) = self.state.mode
                && index >= self.state.lists.len()
            {
                self.state.mode = Mode::Tasks(self.state.selected_list);
            }
        }
        self.clamp_task_selection();
    }

    fn clamp_task_selection(&mut self) {
        if self.state.tasks.is_empty() {
            self.state.selected_task = 0;
        } else {
            self.state.selected_task = self.state.selected_task.min(self.state.tasks.len() - 1);
        }
    }

    fn open_tasks(&mut self) -> Result<(), String> {
        let Some(list) = self.state.lists.get(self.state.selected_list) else {
            return Ok(());
        };
        self.state.selected_task = 0;
        self.state.mode = Mode::Tasks(self.state.selected_list);
        self.state.tasks = ListTasks::new(&self.repo)
            .execute(list.id())
            .map_err(task_error_message)?;
        self.clamp_task_selection();
        Ok(())
    }

    fn reload_tasks(&mut self) -> Result<(), String> {
        let Mode::Tasks(list_index) = self.state.mode else {
            return Ok(());
        };
        let Some(id) = self.list_id_at(list_index) else {
            self.state.tasks.clear();
            self.state.mode = Mode::Lists;
            self.state.selected_task = 0;
            return Ok(());
        };
        self.state.tasks = ListTasks::new(&self.repo)
            .execute(&id)
            .map_err(task_error_message)?;
        self.clamp_task_selection();
        Ok(())
    }

    fn select_list_id(&mut self, id: &TaskListId) {
        if let Some(index) = self.state.lists.iter().position(|list| list.id() == id) {
            self.state.selected_list = index;
        }
    }

    fn list_id_at(&self, index: usize) -> Option<TaskListId> {
        self.state.lists.get(index).map(|list| list.id().clone())
    }

    fn task_title_at(&self, index: usize) -> Option<String> {
        self.state
            .tasks
            .get(index)
            .map(|task| task.title().to_string())
    }

    fn selection_status(&self) -> String {
        match self.state.mode {
            Mode::Lists if self.state.lists.is_empty() => {
                "No task lists | n: new list | ?: help | q: quit".to_string()
            }
            Mode::Lists => {
                let list = &self.state.lists[self.state.selected_list];
                format!(
                    "List {}/{}: {} | Enter: tasks | n/r/d: list actions | ?: help | q: quit",
                    self.state.selected_list + 1,
                    self.state.lists.len(),
                    list.name()
                )
            }
            Mode::Tasks(_) if self.state.tasks.is_empty() => {
                "No tasks | n: new task | Esc: lists | ?: help | q: quit".to_string()
            }
            Mode::Tasks(_) => {
                let task = &self.state.tasks[self.state.selected_task];
                format!(
                    "Task {}/{}: {} | Space: complete | n/r/d: task actions | Esc: lists | ?: help | q: quit",
                    self.state.selected_task + 1,
                    self.state.tasks.len(),
                    task.title()
                )
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let [main_area, status_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(1), Constraint::Length(3)])
            .areas(frame.area());
        let [lists_area, tasks_area] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(45), Constraint::Percentage(55)])
            .areas(main_area);

        self.render_lists(frame, lists_area);
        self.render_tasks(frame, tasks_area);
        self.render_status(frame, status_area);
    }

    fn render_lists(&self, frame: &mut Frame, area: Rect) {
        let items = if self.state.lists.is_empty() {
            vec![ListItem::new("No task lists")]
        } else {
            self.state
                .lists
                .iter()
                .enumerate()
                .map(|(index, list)| {
                    let marker = if index == self.state.selected_list {
                        ">>"
                    } else {
                        "  "
                    };
                    let status = if list.is_completed() {
                        "✓"
                    } else {
                        "pending"
                    };
                    ListItem::new(format!("{marker} {status} {}", list.name()))
                })
                .collect()
        };

        let mut list_state = ListState::default();
        if !self.state.lists.is_empty() {
            list_state.select(Some(self.state.selected_list));
        }

        let active = matches!(self.state.mode, Mode::Lists);
        let list = List::new(items)
            .block(
                Block::default()
                    .title("Task Lists")
                    .borders(Borders::ALL)
                    .border_style(panel_style(active))
                    .title_style(panel_style(active)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut list_state);
    }

    fn render_tasks(&self, frame: &mut Frame, area: Rect) {
        match &self.state.interaction {
            Interaction::Help => self.render_help(frame, area),
            Interaction::Editing(action) => self.render_editor(frame, area, action),
            Interaction::Confirming(action) => self.render_confirmation(frame, area, action),
            Interaction::None => self.render_task_list(frame, area),
        }
    }

    fn render_task_list(&self, frame: &mut Frame, area: Rect) {
        let items = match self.state.mode {
            Mode::Lists => vec![ListItem::new("No task list open")],
            Mode::Tasks(_) if self.state.tasks.is_empty() => vec![ListItem::new("No tasks")],
            Mode::Tasks(_) => self.task_items(),
        };

        let mut task_state = ListState::default();
        if matches!(self.state.mode, Mode::Tasks(_)) && !self.state.tasks.is_empty() {
            task_state.select(Some(self.state.selected_task));
        }

        let active = matches!(self.state.mode, Mode::Tasks(_));
        let list = List::new(items)
            .block(
                Block::default()
                    .title("Tasks")
                    .borders(Borders::ALL)
                    .border_style(panel_style(active))
                    .title_style(panel_style(active)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, area, &mut task_state);
    }

    fn task_items(&self) -> Vec<ListItem<'static>> {
        self.state
            .tasks
            .iter()
            .enumerate()
            .map(|(index, task)| {
                let marker = if index == self.state.selected_task {
                    ">>"
                } else {
                    "  "
                };
                let status = if task.is_completed() { "✓" } else { "☐" };
                ListItem::new(format!("{marker} {status} {}", task.title()))
            })
            .collect()
    }

    fn render_help(&self, frame: &mut Frame, area: Rect) {
        let items = match self.state.mode {
            Mode::Lists => vec![
                ListItem::new("List mode"),
                ListItem::new("n new list"),
                ListItem::new("r rename list"),
                ListItem::new("d delete list"),
                ListItem::new("Enter/Right open tasks"),
                ListItem::new("Up/Down select list"),
                ListItem::new("Esc close help"),
                ListItem::new("q quit"),
            ],
            Mode::Tasks(_) => vec![
                ListItem::new("Task mode"),
                ListItem::new("n new task"),
                ListItem::new("r rename task"),
                ListItem::new("d delete task"),
                ListItem::new("Space complete task"),
                ListItem::new("Up/Down select task"),
                ListItem::new("Esc/Left return to lists"),
                ListItem::new("q quit"),
            ],
        };

        let list = List::new(items).block(Block::default().title("Help").borders(Borders::ALL));
        frame.render_widget(list, area);
    }

    fn render_editor(&self, frame: &mut Frame, area: Rect, action: &EditAction) {
        let prompt = match action {
            EditAction::CreateList => "New list",
            EditAction::RenameList(_) => "Rename list",
            EditAction::CreateTask(_) => "New task",
            EditAction::RenameTask { .. } => "Rename task",
        };
        let items = vec![
            ListItem::new(format!("{prompt}: {}", self.state.input)),
            ListItem::new("Enter submit"),
            ListItem::new("Esc cancel"),
        ];
        let list = List::new(items).block(Block::default().title("Edit").borders(Borders::ALL));
        frame.render_widget(list, area);
    }

    fn render_confirmation(&self, frame: &mut Frame, area: Rect, action: &ConfirmAction) {
        let prompt = match action {
            ConfirmAction::DeleteList(_) => "Delete list? y/n",
            ConfirmAction::DeleteTask { .. } => "Delete task? y/n",
        };
        let list = List::new(vec![ListItem::new(prompt), ListItem::new("Esc cancel")])
            .block(Block::default().title("Confirm").borders(Borders::ALL));
        frame.render_widget(list, area);
    }

    fn render_status(&self, frame: &mut Frame, area: Rect) {
        let text = if self.state.status.is_empty() {
            self.selection_status()
        } else {
            self.state.status.clone()
        };
        let paragraph =
            Paragraph::new(text).block(Block::default().title("Status").borders(Borders::ALL));
        frame.render_widget(paragraph, area);
    }
}

fn panel_style(active: bool) -> Style {
    if active {
        Style::default().fg(Color::LightCyan)
    } else {
        Style::default().fg(Color::Reset)
    }
}

struct TerminalSession {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalSession {
    fn start() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        Ok(Self { terminal })
    }
}

impl Drop for TerminalSession {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(self.terminal.backend_mut(), LeaveAlternateScreen);
        let _ = self.terminal.show_cursor();
    }
}

fn io_error(message: String) -> io::Error {
    io::Error::other(message)
}

fn create_error_message(error: CreateError) -> String {
    match error {
        CreateError::EmptyName => "List name is required".to_string(),
        CreateError::DuplicateName => "List name already exists".to_string(),
        CreateError::Repo(message) => format!("Storage error: {message}"),
    }
}

fn rename_error_message(error: RenameError) -> String {
    match error {
        RenameError::NotFound => "List not found".to_string(),
        RenameError::EmptyName => "List name is required".to_string(),
        RenameError::DuplicateName => "List name already exists".to_string(),
        RenameError::Repo(message) => format!("Storage error: {message}"),
    }
}

fn delete_error_message(error: DeleteError) -> String {
    match error {
        DeleteError::NotFound => "List not found".to_string(),
        DeleteError::Repo(message) => format!("Storage error: {message}"),
    }
}

fn task_error_message(error: TaskCommandError) -> String {
    match error {
        TaskCommandError::ListNotFound => "List not found".to_string(),
        TaskCommandError::EmptyTitle => "Task title is required".to_string(),
        TaskCommandError::DuplicateTitle => "Task title already exists".to_string(),
        TaskCommandError::TaskNotFound => "Task not found".to_string(),
        TaskCommandError::Repo(message) => format!("Storage error: {message}"),
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;
    use ratatui::backend::TestBackend;

    use super::*;
    use crate::adapters::in_memory::{InMemoryTaskListRepository, SeqIdGenerator};
    use crate::application::create_task_list::CreateTaskList;
    use crate::application::ports::TaskListRepository;
    use crate::application::task_commands::{AddTask, CompleteTask, ListTasks};

    #[test]
    fn help_view_lists_keys_and_esc_closes_it() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Char('?')).unwrap();
        let text = render_text(&mut app);

        assert!(text.contains("List mode"));
        assert!(text.contains("n new list"));
        assert!(text.contains("r rename list"));
        assert!(text.contains("d delete list"));
        assert!(text.contains("Enter/Right open tasks"));
        assert!(text.contains("q quit"));

        app.handle_key(KeyCode::Esc).unwrap();
        let text = render_text(&mut app);

        assert!(!text.contains("List mode"));
        assert!(text.contains("No task list open"));
    }

    #[test]
    fn creates_task_list_from_list_mode() {
        let mut app = empty_app();

        press_text(&mut app, KeyCode::Char('n'), "work");
        app.handle_key(KeyCode::Enter).unwrap();

        assert_eq!(list_names(&app), vec!["work"]);
        assert_eq!(app.state.selected_list, 0);
        assert!(render_text(&mut app).contains("work"));
    }

    #[test]
    fn renames_selected_task_list_and_preserves_tasks() {
        let mut app = seeded_app();

        press_text(&mut app, KeyCode::Char('r'), "office");
        app.handle_key(KeyCode::Enter).unwrap();

        assert_eq!(list_names(&app), vec!["office", "home"]);
        assert_eq!(task_titles(&app, 0), vec!["milk", "bread"]);
    }

    #[test]
    fn deletes_selected_task_list_after_confirmation() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Char('d')).unwrap();
        assert!(render_text(&mut app).contains("Delete list"));
        app.handle_key(KeyCode::Char('y')).unwrap();

        assert_eq!(list_names(&app), vec!["home"]);
        assert_eq!(app.state.selected_list, 0);
    }

    #[test]
    fn navigates_tasks_independently_from_lists() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Enter).unwrap();
        assert_eq!(app.state.selected_task, 0);
        app.handle_key(KeyCode::Down).unwrap();
        assert_eq!(app.state.selected_task, 1);
        app.handle_key(KeyCode::Up).unwrap();
        assert_eq!(app.state.selected_task, 0);

        let text = render_text(&mut app);
        assert!(text.contains("Task Lists"));
        assert!(text.contains("Tasks"));
        assert!(text.contains("✓ milk"));
        assert!(text.contains("☐ bread"));
    }

    #[test]
    fn arrow_keys_render_explicit_selection_feedback() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Down).unwrap();
        let text = render_text(&mut app);
        assert!(text.contains("List 2/2: home"));

        app.handle_key(KeyCode::Up).unwrap();
        let text = render_text(&mut app);
        assert!(text.contains("List 1/2: work"));

        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Down).unwrap();
        let text = render_text(&mut app);
        assert!(text.contains("Task 2/2: bread"));
    }

    #[test]
    fn arrow_keys_wrap_at_list_and_task_boundaries() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Up).unwrap();
        assert_eq!(app.state.selected_list, 1);
        assert!(render_text(&mut app).contains("List 2/2: home"));

        app.handle_key(KeyCode::Down).unwrap();
        assert_eq!(app.state.selected_list, 0);
        assert!(render_text(&mut app).contains("List 1/2: work"));

        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Up).unwrap();
        assert_eq!(app.state.selected_task, 1);
        assert!(render_text(&mut app).contains("Task 2/2: bread"));

        app.handle_key(KeyCode::Down).unwrap();
        assert_eq!(app.state.selected_task, 0);
        assert!(render_text(&mut app).contains("Task 1/2: milk"));
    }

    #[test]
    fn creates_task_in_open_task_list() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Down).unwrap();
        app.handle_key(KeyCode::Enter).unwrap();
        press_text(&mut app, KeyCode::Char('n'), "dishes");
        app.handle_key(KeyCode::Enter).unwrap();

        assert_eq!(task_titles(&app, 1), vec!["laundry", "dishes"]);
        assert!(render_text(&mut app).contains("☐ dishes"));
    }

    #[test]
    fn renames_selected_task() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Enter).unwrap();
        press_text(&mut app, KeyCode::Char('r'), "buy milk");
        app.handle_key(KeyCode::Enter).unwrap();

        assert_eq!(task_titles(&app, 0), vec!["buy milk", "bread"]);
        assert!(render_text(&mut app).contains("buy milk"));
        assert!(!task_titles(&app, 0).contains(&"milk".to_string()));
    }

    #[test]
    fn deletes_selected_task_after_confirmation() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Char('d')).unwrap();
        assert!(render_text(&mut app).contains("Delete task"));
        app.handle_key(KeyCode::Char('y')).unwrap();

        assert_eq!(task_titles(&app, 0), vec!["bread"]);
        assert_eq!(app.state.selected_task, 0);
    }

    #[test]
    fn completes_selected_task_and_refreshes_list_badge() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Down).unwrap();
        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Char(' ')).unwrap();

        assert!(app.state.lists[1].is_completed());
        assert_eq!(task_titles(&app, 1), vec!["laundry"]);
        assert!(render_text(&mut app).contains("✓ home"));
    }

    #[test]
    fn invalid_duplicate_and_cancelled_edits_do_not_mutate_repository() {
        let mut app = seeded_app();

        press_text(&mut app, KeyCode::Char('n'), "work");
        app.handle_key(KeyCode::Enter).unwrap();
        assert_eq!(list_names(&app), vec!["work", "home"]);
        assert!(render_text(&mut app).contains("List name already exists"));

        press_text(&mut app, KeyCode::Char('n'), "draft");
        app.handle_key(KeyCode::Esc).unwrap();
        assert_eq!(list_names(&app), vec!["work", "home"]);
        assert!(render_text(&mut app).contains("Cancelled"));

        app.handle_key(KeyCode::Char('d')).unwrap();
        app.handle_key(KeyCode::Esc).unwrap();
        assert_eq!(list_names(&app), vec!["work", "home"]);
    }

    #[test]
    fn validation_errors_render_as_user_facing_status_text() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Char('n')).unwrap();
        app.handle_key(KeyCode::Enter).unwrap();
        let text = render_text(&mut app);
        assert!(text.contains("List name is required"));
        assert!(!text.contains("EmptyName"));

        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Char('n')).unwrap();
        app.handle_key(KeyCode::Enter).unwrap();
        let text = render_text(&mut app);
        assert!(text.contains("Task title is required"));
        assert!(!text.contains("EmptyTitle"));

        press_text(&mut app, KeyCode::Char('n'), "milk");
        app.handle_key(KeyCode::Enter).unwrap();
        let text = render_text(&mut app);
        assert!(text.contains("Task title already exists"));
        assert!(!text.contains("DuplicateTitle"));
    }

    fn empty_app() -> App<InMemoryTaskListRepository, SeqIdGenerator> {
        let repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        let mut app = App::new(repo, ids);
        app.load_once().unwrap();
        app
    }

    fn seeded_app() -> App<InMemoryTaskListRepository, SeqIdGenerator> {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();

        let work = {
            let mut create = CreateTaskList::new(&mut repo, &ids);
            create.execute("work").unwrap()
        };
        AddTask::new(&mut repo).execute(&work, "milk").unwrap();
        CompleteTask::new(&mut repo).execute(&work, "milk").unwrap();
        AddTask::new(&mut repo).execute(&work, "bread").unwrap();

        let home = {
            let mut create = CreateTaskList::new(&mut repo, &ids);
            create.execute("home").unwrap()
        };
        AddTask::new(&mut repo).execute(&home, "laundry").unwrap();

        let mut app = App::new(repo, ids);
        app.load_once().unwrap();
        app
    }

    fn press_text(
        app: &mut App<InMemoryTaskListRepository, SeqIdGenerator>,
        start: KeyCode,
        text: &str,
    ) {
        app.handle_key(start).unwrap();
        for c in text.chars() {
            app.handle_key(KeyCode::Char(c)).unwrap();
        }
    }

    fn list_names(app: &App<InMemoryTaskListRepository, SeqIdGenerator>) -> Vec<String> {
        app.repo
            .all()
            .unwrap()
            .iter()
            .map(|l| l.name().to_string())
            .collect()
    }

    fn task_titles(
        app: &App<InMemoryTaskListRepository, SeqIdGenerator>,
        list_index: usize,
    ) -> Vec<String> {
        let list = &app.state.lists[list_index];
        ListTasks::new(&app.repo)
            .execute(list.id())
            .unwrap()
            .iter()
            .map(|t| t.title().to_string())
            .collect()
    }

    fn render_text(app: &mut App<InMemoryTaskListRepository, SeqIdGenerator>) -> String {
        buffer_text(&render_terminal(app))
    }

    fn buffer_text(terminal: &Terminal<TestBackend>) -> String {
        let buffer = terminal.backend().buffer();
        let area = buffer.area;
        let mut text = String::new();
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                text.push_str(buffer[(x, y)].symbol());
            }
            text.push('\n');
        }
        text
    }

    fn render_terminal(app: &mut App<InMemoryTaskListRepository, SeqIdGenerator>) -> Terminal<TestBackend> {
        let backend = TestBackend::new(100, 28);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| app.render(frame)).unwrap();
        terminal
    }

    fn area_has_cyan(terminal: &Terminal<TestBackend>, x_start: u16, x_end: u16, y_start: u16, y_end: u16) -> bool {
        let buffer = terminal.backend().buffer();
        for y in y_start..y_end {
            for x in x_start..x_end {
                if buffer[(x, y)].fg == Color::LightCyan {
                    return true;
                }
            }
        }
        false
    }

    // --- panel-navigation tests (AT-1..AT-5) ---

    // AT-1 covers REQ-1: Right opens tasks panel (same as Enter)
    #[test]
    fn right_arrow_opens_tasks_panel() {
        let mut app = seeded_app();
        assert_eq!(app.state.mode, Mode::Lists);

        app.handle_key(KeyCode::Right).unwrap();

        assert_eq!(app.state.mode, Mode::Tasks(0));
        let text = render_text(&mut app);
        assert!(text.contains("milk") && text.contains("bread"));
    }

    // AT-2 covers REQ-2: Left returns to lists panel
    #[test]
    fn left_arrow_returns_to_lists() {
        let mut app = seeded_app();
        app.handle_key(KeyCode::Enter).unwrap();
        assert!(matches!(app.state.mode, Mode::Tasks(_)));

        app.handle_key(KeyCode::Left).unwrap();

        assert_eq!(app.state.mode, Mode::Lists);
        assert!(render_text(&mut app).contains("No task list open"));
    }

    // AT-3 covers REQ-1, REQ-3: Right and Enter produce identical result
    #[test]
    fn right_and_enter_are_equivalent() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Enter).unwrap();
        let mode_after_enter = app.state.mode;

        app.handle_key(KeyCode::Esc).unwrap();
        app.handle_key(KeyCode::Right).unwrap();
        let mode_after_right = app.state.mode;

        assert_eq!(mode_after_enter, mode_after_right);
    }

    // AT-4 covers REQ-2, REQ-3: Left and Esc are equivalent
    #[test]
    fn left_and_esc_are_equivalent() {
        let mut app = seeded_app();

        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Esc).unwrap();
        let mode_after_esc = app.state.mode;

        app.handle_key(KeyCode::Enter).unwrap();
        app.handle_key(KeyCode::Left).unwrap();
        let mode_after_left = app.state.mode;

        assert_eq!(mode_after_esc, mode_after_left);
        assert_eq!(mode_after_left, Mode::Lists);
    }

    // AT-5 covers REQ-1: Right is no-op when no lists exist
    #[test]
    fn right_is_noop_when_no_lists() {
        let mut app = empty_app();
        assert_eq!(app.state.mode, Mode::Lists);

        app.handle_key(KeyCode::Right).unwrap();

        assert_eq!(app.state.mode, Mode::Lists);
    }

    // --- modal-input tests (AT-1..AT-5) ---

    // AT-1 covers REQ-1, REQ-3: q in Editing does not quit; goes into input buffer
    #[test]
    fn q_in_editing_mode_appends_to_input_not_quit() {
        let mut app = seeded_app();
        app.handle_key(KeyCode::Char('n')).unwrap();
        assert!(matches!(app.state.interaction, Interaction::Editing(_)));

        let quit = app.handle_key(KeyCode::Char('q')).unwrap();

        assert!(!quit, "expected Ok(false) — app should not quit while editing");
        assert_eq!(app.state.input, "q");
    }

    // AT-2 covers REQ-1: full word containing q types correctly
    #[test]
    fn termotanque_in_editing_mode_fills_input_buffer() {
        let mut app = seeded_app();
        app.handle_key(KeyCode::Char('n')).unwrap();

        for c in "termotanque".chars() {
            let quit = app.handle_key(KeyCode::Char(c)).unwrap();
            assert!(!quit, "char '{c}' should not quit while editing");
        }

        assert_eq!(app.state.input, "termotanque");
    }

    // AT-3 covers REQ-2, REQ-3: q in Confirming does not quit; interaction stays Confirming
    #[test]
    fn q_in_confirming_mode_does_not_quit() {
        let mut app = seeded_app();
        app.handle_key(KeyCode::Char('d')).unwrap();
        assert!(matches!(app.state.interaction, Interaction::Confirming(_)));

        let quit = app.handle_key(KeyCode::Char('q')).unwrap();

        assert!(!quit, "expected Ok(false) — app should not quit while confirming");
        assert!(
            matches!(app.state.interaction, Interaction::Confirming(_)),
            "interaction should stay Confirming"
        );
    }

    // AT-4 covers REQ-3: q in None interaction quits normally
    #[test]
    fn q_in_normal_mode_quits() {
        let mut app = seeded_app();
        assert_eq!(app.state.interaction, Interaction::None);

        let quit = app.handle_key(KeyCode::Char('q')).unwrap();

        assert!(quit, "expected Ok(true) — q should quit in normal mode");
    }

    // AT-5 covers REQ-4: navigation keys inactive during Editing
    #[test]
    fn navigation_keys_inactive_during_editing() {
        let mut app = seeded_app();
        let mode_before = app.state.mode;
        let selected_before = app.state.selected_list;
        app.handle_key(KeyCode::Char('n')).unwrap();

        let r1 = app.handle_key(KeyCode::Up).unwrap();
        let r2 = app.handle_key(KeyCode::Down).unwrap();
        // Enter with empty input: submit_edit fails with EmptyName, does not change mode/selection
        let r3 = app.handle_key(KeyCode::Enter).unwrap();

        assert!(!r1 && !r2 && !r3, "Up/Down/Enter should return Ok(false) while editing");
        assert_eq!(app.state.selected_list, selected_before);
        assert_eq!(app.state.mode, mode_before);
    }

    // --- panel-focus-colors tests (AT-1..AT-3) ---
    // Layout fixed to 100x28 TestBackend. Constraint::Percentage(45) over 100 cols = x 0..45 for lists,
    // x 45..100 for tasks. If terminal width changes, update these bounds.

    // AT-1 covers REQ-1, REQ-3: lists border is LightCyan in Lists mode; tasks border is not
    #[test]
    fn lists_panel_border_is_cyan_in_lists_mode() {
        let mut app = seeded_app();
        assert_eq!(app.state.mode, Mode::Lists);
        let terminal = render_terminal(&mut app);

        assert!(area_has_cyan(&terminal, 0, 45, 0, 25), "lists border should be LightCyan in Lists mode");
        assert!(!area_has_cyan(&terminal, 45, 100, 0, 25), "tasks border should NOT be LightCyan in Lists mode");
    }

    // AT-2 covers REQ-2, REQ-3: tasks border is LightCyan in Tasks mode; lists border is not
    #[test]
    fn tasks_panel_border_is_cyan_in_tasks_mode() {
        let mut app = seeded_app();
        app.handle_key(KeyCode::Enter).unwrap();
        assert!(matches!(app.state.mode, Mode::Tasks(_)));
        let terminal = render_terminal(&mut app);

        assert!(area_has_cyan(&terminal, 45, 100, 0, 25), "tasks border should be LightCyan in Tasks mode");
        assert!(!area_has_cyan(&terminal, 0, 45, 0, 25), "lists border should NOT be LightCyan in Tasks mode");
    }

    // AT-3 covers REQ-1, REQ-2: cyan tracks mode toggle
    #[test]
    fn cyan_tracks_mode_toggle() {
        let mut app = seeded_app();

        // Lists mode: lists panel cyan
        let t1 = render_terminal(&mut app);
        assert!(area_has_cyan(&t1, 0, 45, 0, 25));
        assert!(!area_has_cyan(&t1, 45, 100, 0, 25));

        // Tasks mode: tasks panel cyan
        app.handle_key(KeyCode::Enter).unwrap();
        let t2 = render_terminal(&mut app);
        assert!(area_has_cyan(&t2, 45, 100, 0, 25));
        assert!(!area_has_cyan(&t2, 0, 45, 0, 25));

        // Back to Lists: lists panel cyan again
        app.handle_key(KeyCode::Esc).unwrap();
        let t3 = render_terminal(&mut app);
        assert!(area_has_cyan(&t3, 0, 45, 0, 25));
        assert!(!area_has_cyan(&t3, 45, 100, 0, 25));
    }
}
