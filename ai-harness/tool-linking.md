# Tool Linking

This harness is **provider-agnostic**. Nothing inside `ai-harness/` depends on a
specific AI tool, model, or vendor. Any assistant that can read repository files
can use it by reading the entry point:

```text
ai-harness/START-HERE.md
```

This document explains how to *optionally* connect specific tools to the harness
**without** making the harness depend on them.

---

## The core idea: thin adapters, single source of truth

Most AI coding tools look for their own well-known instruction file at the
repository root. Rather than duplicating guidance into each of those files (which
drifts and rots), you create a **thin adapter**: a tiny root file whose only job
is to point the tool at `ai-harness/START-HERE.md`.

```text
          AGENTS.md ─┐
          CLAUDE.md ─┤
 copilot-instructions ┤──►  "Read ai-harness/START-HERE.md and follow it."  ──►  ai-harness/
          PI.md ─────┤
        (others) ────┘
```

- The harness stays the **single source of truth**.
- Each tool file is **3–6 lines** and contains no real content of its own.
- Adding or removing a tool never touches the harness.

> **These adapter files are NOT created by default.** This harness ships as
> folders and Markdown inside `ai-harness/` only, to avoid scattering
> tool-specific files at the repository root. A human creates an adapter **only**
> for the tool(s) they actually use, by copying a snippet below.

---

## How to add an adapter (general pattern)

1. Find out which instruction file your tool reads at startup (check its docs).
2. Create that file at the location the tool expects.
3. Put a short pointer in it — nothing more. Example body:

   ```markdown
   # Project AI Instructions

   This repository uses a shared, tool-agnostic AI harness.

   **Read `ai-harness/START-HERE.md` first and follow its boot sequence.**
   It defines the workflow, the architecture rules, where the active task is,
   and what to update before ending a session. Do not duplicate guidance here;
   keep this file thin and let the harness be the single source of truth.
   ```

That is the entire pattern. Everything below is convenience snippets and notes.

---

## Known tool entry points (verify against current docs)

Tool conventions change. **Confirm the exact filename/path in your tool's
current documentation before relying on it.** This table reflects common
conventions at the time of writing and may be out of date — do not assume exact
support for any specific tool.

| Tool | Common root file it reads | Notes |
|------|---------------------------|-------|
| Many agents (shared convention) | `AGENTS.md` | A growing cross-tool convention; good default |
| Claude Code | `CLAUDE.md` | Also supports nested `CLAUDE.md` files |
| GitHub Copilot | `.github/copilot-instructions.md` | Repo-wide custom instructions |
| Codex | `AGENTS.md` (or a tool-specific path) | Prefers the shared `AGENTS.md` convention |
| Gemini CLI | `GEMINI.md` | Project context file |
| Pi | `PI.md` (or tool-specific) | Check the tool's docs |
| OpenCode | tool-specific | Check the tool's docs |
| Aider | `CONVENTIONS.md` (added as read-only context) | You point Aider at it explicitly |
| Cursor | `.cursorrules` (or project rules) | Check the tool's docs |

If you only create **one** adapter, make it `AGENTS.md` — it is the most widely
shared convention, and several tools read it.

---

## Ready-to-copy adapter snippets

Each snippet is intentionally minimal. Replace nothing inside `ai-harness/`.

### `AGENTS.md` (recommended default)

```markdown
# Agent Instructions

This repository uses a tool-agnostic AI harness.
**Start by reading `ai-harness/START-HERE.md` and follow its boot sequence.**
You work one feature per branch; current work lives in
`ai-harness/specs/<your-feature>/state.md`.
Keep this file thin; the harness is the single source of truth.
```

### `CLAUDE.md`

```markdown
# Claude Code Instructions

Read `ai-harness/START-HERE.md` first and follow it.
Find current work in `ai-harness/specs/<your-feature>/state.md`.
Update that feature's state.md before ending the session (see START-HERE.md).
```

### `.github/copilot-instructions.md`

```markdown
# Copilot Instructions

This project is driven by a shared AI harness.
Read `ai-harness/START-HERE.md` and follow its rules and boot sequence.
Current work and next steps: `ai-harness/specs/<your-feature>/state.md`.
Keep the domain decoupled per `ai-harness/context/architecture.md`.
```

### `GEMINI.md`

```markdown
# Gemini Project Context

Read `ai-harness/START-HERE.md` first and follow it.
The active feature/task is in `ai-harness/specs/<your-feature>/state.md`.
```

### `PI.md`

```markdown
# Pi Agent Rules

Initialize every session by reading `ai-harness/START-HERE.md`.
Adopt the role for the current phase (`ai-harness/roles.md`), and keep task state
in your feature's `ai-harness/specs/<your-feature>/state.md`, updated before stopping.
```

### `CONVENTIONS.md` (Aider — point Aider at it)

```markdown
# Conventions

Follow the shared AI harness. Read `ai-harness/START-HERE.md` first.
Architecture rules: `ai-harness/context/architecture.md`.
Current work: `ai-harness/specs/<your-feature>/state.md`.
```

---

## Multiple tools, one harness

You can keep several adapter files at once (e.g. `AGENTS.md` **and** `CLAUDE.md`).
Because each only points to `ai-harness/START-HERE.md`, they cannot drift apart
in substance. When you change how you work, you change the harness, and every
tool follows automatically.

---

## What a tool still needs from the human

The harness tells an agent *how to work*, but a few things remain the human's job
to provide per tool, in that tool's own config (not in the harness):

- **Permissions / sandbox settings** — what commands the tool may run.
- **Model selection** — the harness is indifferent; pick any capable model.
- **Where the app source lives** — record this in
  [`context/project.md`](context/project.md) so every tool learns it from the
  harness rather than from tool-specific config.

The boundary is simple: **how we work → harness; how this tool runs → tool config.**
