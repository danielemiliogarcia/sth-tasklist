# Adopt the AI Harness (parallel-safe, feature-owned state)

- **Status:** accepted
- **Date:** 2026-06-07
- **Deciders:** project owner (with AI agent)
- **Related:** the whole `ai-harness/` folder; [`../README.md`](../README.md);
  [`../parallel-work.md`](../parallel-work.md)

---

## Context

This project will be developed largely with AI coding assistants, possibly
several at once and several different ones over time (Claude Code, Codex, Copilot,
Pi, OpenCode, Gemini CLI, Aider, and future tools). We need a way of working that:

- writes specifications and validation before implementation;
- keeps the domain independent of infrastructure (hexagonal architecture);
- tracks task state durably so progress is not lost between sessions;
- lets a brand-new AI session resume after reading only a few small files;
- **lets multiple agents work on different features in parallel (separate
  branches / git worktrees) and merge back to main without conflicts;**
- does not bind us to any single model, provider, plugin, or runtime tool;
- stays simple — folders and Markdown, no scripts or CI to maintain.

## Decision

We will adopt a single, self-contained, Markdown-only harness under
`ai-harness/`. It encodes Specification-Driven Development, Test-Driven
Development (preferred but not mandatory), hexagonal architecture, explicit task
state, and a fresh-session resume protocol anchored by `START-HERE.md`.

The defining structural rule is **feature-owned state**: each feature is a folder
under `specs/<feature>/` containing `spec.md` (stable: requirements, validation,
design) and `state.md` (volatile: phase, next action, read budget, task table).
**There are no global mutable state files** — no single "current task" pointer, no
shared task board, no decision index. A fresh session finds its feature from the
branch/worktree it is in and reads that feature's `state.md`.

Consequences for IDs and decisions, also for parallel safety:

- Task / requirement IDs are **scoped per feature** (`T-1`, `REQ-1`), so no two
  branches collide on an ID.
- ADRs are **date-named** (`YYYY-MM-DD-slug.md`) with **no index file**; the
  directory listing is the index.
- Feature-specific ports / terms / conventions live in the feature's `spec.md`,
  not in shared `context/*`; project-wide promotion happens deliberately on main.

Tool-specific integration is optional, via thin adapter files at the repo root
that point to `ai-harness/START-HERE.md` (see
[`../tool-linking.md`](../tool-linking.md)); none are created by default.

## Alternatives considered

- **Global state files (one `current.md` pointer + one shared task board +
  sequential ADR numbering).** — Rejected: every parallel branch writes the same
  files, causing merge conflicts and duplicate IDs. There is also no single "now"
  when N agents work at once. This was an earlier draft of this very harness; the
  parallel requirement ruled it out.
- **One combined Markdown file per feature (spec + state in one file).** —
  Rejected: routine task-state churn would rewrite the spec, creating needless
  intra-feature merge friction. Splitting stable `spec.md` from volatile
  `state.md` avoids it.
- **A committed cross-feature dashboard/index file.** — Rejected as a default:
  any shared file every branch appends to becomes a conflict hotspot
  (append-to-end collides). `ls specs/` plus per-feature `state.md` gives the
  overview without a shared mutable file.
- **Per-tool instruction files with real content (e.g. a full `CLAUDE.md`).** —
  Rejected: duplicates guidance across tools and drifts out of sync.
- **Heavy tooling (scripts, CI, generators).** — Rejected for now: more to
  maintain and explicitly out of scope. Folders + Markdown only.

## Consequences

- Easier: consistent process across any AI tool; quick onboarding for fresh
  sessions; clear traceability from requirements → tasks → tests; **parallel
  feature development that merges cleanly.**
- Harder / obligations: no single-glance dashboard (use `ls specs/` + each
  `state.md`); discipline replaces automation — keep your feature's `state.md`
  current, and write only your own feature folder during feature work.
- Application-code merges are still ordinary merge work; the harness only
  guarantees the *harness* files merge cleanly (see
  [`../parallel-work.md`](../parallel-work.md)).
- Follow-up: customize `context/project.md` and the other `<!-- CUSTOMIZE -->`
  sections once the stack and first feature are chosen.
