//! Driven adapter: load ColourTheme from ./colours.toml.
//! Missing file -> default theme. Malformed file -> stderr warning + default theme.

use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::application::ports::ThemeRepository;
use crate::domain::colour_theme::{ColourTheme, NamedColor};

/// Deserialises colours.toml; all fields are optional strings.
#[derive(Deserialize, Default)]
struct ColourThemeDto {
    active_panel_border: Option<String>,
    inactive_panel_border: Option<String>,
    selected_item_fg: Option<String>,
    selected_item_bg: Option<String>,
    normal_item_fg: Option<String>,
    completed_task_fg: Option<String>,
}

pub struct TomlThemeRepository {
    path: PathBuf,
}

impl TomlThemeRepository {
    /// Reads from `path` (e.g. `"./colours.toml"`).
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl ThemeRepository for TomlThemeRepository {
    fn load(&self) -> ColourTheme {
        load_theme(&self.path)
    }
}

fn load_theme(path: &Path) -> ColourTheme {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return ColourTheme::default(),
        Err(e) => {
            eprintln!("colours.toml: read error — {e}; using defaults");
            return ColourTheme::default();
        }
    };

    let dto: ColourThemeDto = match toml::from_str(&text) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("colours.toml: parse error — {e}; using defaults");
            return ColourTheme::default();
        }
    };

    let defaults = ColourTheme::default();
    // Pre-check before moving fields: explicit fg or bg disables reverse.
    let has_explicit_selected = dto.selected_item_fg.is_some() || dto.selected_item_bg.is_some();

    ColourTheme {
        active_panel_border: dto
            .active_panel_border
            .and_then(|s| parse_color(&s))
            .unwrap_or(defaults.active_panel_border),
        inactive_panel_border: dto
            .inactive_panel_border
            .and_then(|s| parse_color(&s))
            .unwrap_or(defaults.inactive_panel_border),
        selected_item_fg: dto
            .selected_item_fg
            .and_then(|s| parse_color(&s))
            .unwrap_or(defaults.selected_item_fg),
        selected_item_bg: dto
            .selected_item_bg
            .and_then(|s| parse_color(&s))
            .unwrap_or(defaults.selected_item_bg),
        selected_item_reverse: !has_explicit_selected,
        normal_item_fg: dto
            .normal_item_fg
            .and_then(|s| parse_color(&s))
            .unwrap_or(defaults.normal_item_fg),
        completed_task_fg: dto
            .completed_task_fg
            .and_then(|s| parse_color(&s))
            .unwrap_or(defaults.completed_task_fg),
    }
}

fn parse_color(s: &str) -> Option<NamedColor> {
    match s.to_lowercase().as_str() {
        "black" => Some(NamedColor::Black),
        "red" => Some(NamedColor::Red),
        "green" => Some(NamedColor::Green),
        "yellow" => Some(NamedColor::Yellow),
        "blue" => Some(NamedColor::Blue),
        "magenta" => Some(NamedColor::Magenta),
        "cyan" => Some(NamedColor::Cyan),
        "white" => Some(NamedColor::White),
        "lightblack" | "darkgray" => Some(NamedColor::LightBlack),
        "lightred" => Some(NamedColor::LightRed),
        "lightgreen" => Some(NamedColor::LightGreen),
        "lightyellow" => Some(NamedColor::LightYellow),
        "lightblue" => Some(NamedColor::LightBlue),
        "lightmagenta" => Some(NamedColor::LightMagenta),
        "lightcyan" => Some(NamedColor::LightCyan),
        "lightwhite" | "gray" => Some(NamedColor::LightWhite),
        "reset" => Some(NamedColor::Reset),
        _ => {
            eprintln!("colours.toml: unrecognised colour {s:?}; using default");
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    fn write_temp(name: &str, content: &str) -> PathBuf {
        let path = std::env::temp_dir().join(format!("{}_{}", std::process::id(), name));
        fs::write(&path, content).unwrap();
        path
    }

    // AT-2 covers REQ-2, REQ-3: missing colours.toml returns default theme
    #[test]
    fn missing_file_returns_default_theme() {
        let repo = TomlThemeRepository::new("/nonexistent/colours.toml");
        assert_eq!(repo.load(), ColourTheme::default());
    }

    // AT-3 covers REQ-3: valid colours.toml with one override applies it
    #[test]
    fn valid_toml_applies_override() {
        let path = write_temp("at3_colours.toml", r#"active_panel_border = "Green""#);
        let theme = TomlThemeRepository::new(&path).load();
        assert_eq!(theme.active_panel_border, NamedColor::Green);
        assert_eq!(
            theme.inactive_panel_border,
            ColourTheme::default().inactive_panel_border
        );
    }

    // AT-3 (green-completed-tasks) covers REQ-3: completed_task_fg overridden by TOML
    #[test]
    fn completed_task_fg_overridden_by_toml() {
        let path = write_temp("at_green_completed.toml", r#"completed_task_fg = "cyan""#);
        let theme = TomlThemeRepository::new(&path).load();
        assert_eq!(theme.completed_task_fg, NamedColor::Cyan);
    }

    // AT-4 covers REQ-3: malformed TOML returns default (no panic)
    #[test]
    fn malformed_toml_returns_default_no_panic() {
        let path = write_temp("at4_colours.toml", "this is not valid toml ===");
        assert_eq!(
            TomlThemeRepository::new(&path).load(),
            ColourTheme::default()
        );
    }
}
