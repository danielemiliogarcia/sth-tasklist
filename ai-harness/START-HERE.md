# START HERE

**You are an AI coding agent (or a human) beginning a session in this repository.**
Read this file first. It is tiny on purpose.

This repository uses a **model- and provider-agnostic AI development harness**
built only from folders and Markdown. It works with any assistant that can read
files (Claude Code, Codex, Copilot, Pi, OpenCode, Gemini CLI, Aider, or anything
future). Nothing here depends on a specific model or vendor.

The harness is **parallel-safe by design**: each feature owns all of its mutable
state inside its own folder, so multiple agents can work on different features in
separate branches / git worktrees without merge conflicts. See
[`parallel-work.md`](parallel-work.md).

---

## The boot sequence (do this every fresh session)

1. **Find your feature.** You work on one feature per branch / worktree. Identify
   it, in this order:
   - the branch / worktree name (usually matches a folder in `specs/`);
   - what the human told you to work on;
   - `ls specs/` and match the folder to your task. If still unclear, ask.
2. **Read your feature's live anchor:** `specs/<feature>/state.md`. It names the
   phase, the single next action, the read budget (which files to open, which to
   skip), and this feature's task table. This is the heart of resumability.
3. **Open only what the read budget lists** — typically the section of
   `specs/<feature>/spec.md` for the current phase. Do not read other features.
   Do not read the whole repo.
4. **Adopt the role** for the current phase (see [`roles.md`](roles.md)).
5. **Do the work.**
6. **Before you stop, update `specs/<feature>/state.md`** (phase, last step, next
   action, task states) so the next session resumes cleanly. If you update only
   one file before stopping, make it that one.

That is the whole protocol. Everything else is detail you pull on demand.

---

## If no feature exists yet (fresh project)

There is no work in flight. Adopt the **Planner** role ([`roles.md`](roles.md)),
then:

1. Customize [`context/project.md`](context/project.md): what this project is,
   where the application source will live, and how to build / run / test it.
2. Start the first feature: **copy the whole `specs/_template/` folder** to
   `specs/<feature-name>/` (short, kebab-case). Ideally do this on a feature
   branch.
3. As **Spec Author**, fill `specs/<feature>/spec.md` requirements and set
   `specs/<feature>/state.md` (phase = `requirements`, next action). Both are
   bare freeform files — see
   [`specs/global-spec-info.md`](specs/global-spec-info.md) and
   [`specs/global-state-info.md`](specs/global-state-info.md) for their structure
   and legal values.

---

## If you want to understand the system (optional, read once)

- [`README.md`](README.md) — what the harness is and how it works.
- [`parallel-work.md`](parallel-work.md) — file ownership + worktree workflow +
  merge rules. **Read this before running multiple agents in parallel.**
- [`roles.md`](roles.md) — the "hats" you wear for each phase.
- [`specs/global-spec-info.md`](specs/global-spec-info.md) /
  [`specs/global-state-info.md`](specs/global-state-info.md) — how to fill a
  feature's bare `spec.md` / `state.md` (structure + legal vocabulary).
- [`context/architecture.md`](context/architecture.md) — the hexagonal rulebook.
- [`context/testing.md`](context/testing.md) — TDD when practical, deferred
  validation when not.
- [`tool-linking.md`](tool-linking.md) — how a human connects a specific AI tool
  to this harness with a thin adapter file.
