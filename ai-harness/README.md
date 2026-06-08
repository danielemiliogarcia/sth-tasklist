# AI Harness

A **model- and provider-agnostic** development harness, built entirely from
folders and Markdown files. Any AI coding assistant that can read repository
files can follow it — Claude Code, Codex, GitHub Copilot, Pi, OpenCode, Gemini
CLI, Aider, or any future agent. Nothing inside `ai-harness/` depends on a
specific model, vendor, plugin, or runtime tool.

> **New session?** Don't start here — start at [`START-HERE.md`](START-HERE.md).
> This README explains *how the harness works*; you only need to read it once.

The harness combines six ideas into one lightweight, repo-local process:

- **Specification-Driven Development (SDD)** — write requirements before code.
- **Test-Driven Development (TDD)** — prove behavior first *when practical*.
- **Hexagonal Architecture (Ports & Adapters)** — keep the domain independent.
- **Persistent task state** — every task has an explicit, durable status.
- **Fresh-session resumability** — a new agent resumes from a few small files.
- **Minimal context loading** — read only what the current step needs.

It is also **parallel-safe**: each feature owns all of its mutable state in its
own folder, so multiple agents can build different features in separate branches
or git worktrees without merge conflicts. See
[`parallel-work.md`](parallel-work.md).

> `ai-harness/` is deliberately separate from your application source code. The
> harness describes *how* you work; build your app wherever you like (for example
> a top-level `src/`). Keep the two apart.

---

## What to read for a given goal

| Your goal | Read |
|-----------|------|
| Resume work right now | `specs/<your-feature>/state.md` (via [`START-HERE.md`](START-HERE.md)) |
| Understand the boot sequence | [`START-HERE.md`](START-HERE.md) |
| Run several agents in parallel | [`parallel-work.md`](parallel-work.md) |
| Know the responsibilities of each role | [`roles.md`](roles.md) |
| Learn the architecture rules (hexagonal) | [`context/architecture.md`](context/architecture.md) |
| Learn the testing / validation approach | [`context/testing.md`](context/testing.md) |
| Learn project-specific facts | [`context/project.md`](context/project.md) |
| Start or shape a feature | [`specs/README.md`](specs/README.md) |
| Record a significant decision | [`decisions/README.md`](decisions/README.md) |
| Connect a specific AI tool | [`tool-linking.md`](tool-linking.md) |

---

## The lifecycle

```text
idea → requirements → validation → design → tasks → implementation → review → done
```

| Phase | Question answered | Where it lives | Role |
|-------|-------------------|----------------|------|
| idea | Is this worth doing? | a new `specs/<feature>/` folder | Planner |
| requirements | What and why? | `specs/<feature>/spec.md` § Requirements | Spec Author |
| validation | How will we know it works? | `specs/<feature>/spec.md` § Acceptance & Validation | Tester |
| design | How is it built? | `specs/<feature>/spec.md` § Design (hexagonal) | Spec Author |
| tasks | What are the steps? | `specs/<feature>/state.md` § Tasks | Planner |
| implementation | Build it | your app code + task state in `state.md` | Implementer |
| review | Does it meet the spec? | review notes + state → `done` | Reviewer |
| done | Record and close | `specs/<feature>/state.md` marked `done` | Planner |

Phases are a default order, not a cage. A tiny change may collapse several phases
into one short pass; a large feature iterates (design often loops back to
requirements). The rule that matters: **the artifact for a phase must exist
before you rely on it.** Spec before code; validation defined before
implementation when practical.

---

## The rules (read once, then follow)

1. **Spec before code.** No implementation task starts without written
   requirements it can cite.
2. **Validation before implementation when practical.** Define acceptance tests
   or scenarios first. If tests must be deferred, **write down why** and define
   the future validation path — never skip silently.
3. **TDD is preferred, not mandatory.** Exploratory, infrastructure-heavy, or
   hard-to-test work may design validation first and automate it later (see
   [`context/testing.md`](context/testing.md)).
4. **Keep the domain pure.** Business logic never imports frameworks, databases,
   HTTP, or UI directly. External systems are reached through **ports**;
   **adapters** implement them. See
   [`context/architecture.md`](context/architecture.md).
5. **Every task has an explicit state:** `todo`, `doing`, `blocked`, `review`,
   `done`, or `cancelled`.
6. **State is durable and feature-local.** The truth lives in
   `specs/<feature>/state.md`, not in conversation memory. Update it; never rely
   on what a previous session "remembered".
7. **Load the minimum context.** Read what the step needs and no more; your
   feature's `state.md` read budget tells you what is enough.
8. **One feature per branch; write only your feature folder.** This is what keeps
   parallel work conflict-free (see [`parallel-work.md`](parallel-work.md)).
9. **Leave the campsite ready.** Update `specs/<feature>/state.md` before you
   stop, so the next session resumes without rediscovery.

---

## The state model

Every task is in exactly one state:

| State | Meaning | Typical next state |
|-------|---------|--------------------|
| `todo` | Defined, not started | `doing` |
| `doing` | Actively being worked | `review` or `blocked` |
| `blocked` | Cannot proceed; reason recorded | `doing` or `cancelled` |
| `review` | Implementation done, awaiting verification | `done` or `doing` |
| `done` | Meets requirements and validation | — |
| `cancelled` | Will not be done; reason recorded | — |

- A `blocked` task **must** record the blocker and what would unblock it.
- A task moves to `review` only when its done-criteria are met **and** its
  validation has been run or explicitly scheduled (deferred-validation plan).
- A task moves to `done` only after review confirms requirements + validation.
- **The source of truth for task state is the feature's
  `specs/<feature>/state.md`.** There is no global board.

---

## Where state lives (and why it is small)

V2 has **no global mutable state files** — no single "current task" pointer, no
shared task board, no decision index. That is deliberate: a shared mutable file
is exactly what makes parallel branches conflict. Instead:

- **`specs/<feature>/state.md`** — the *now* for one feature: phase, next action,
  read budget, and that feature's task table. The first thing a session reads.
- **`specs/<feature>/spec.md`** — the stable spec: requirements, validation,
  design. Changes slowly.
- **`decisions/YYYY-MM-DD-slug.md`** — one self-contained file per cross-cutting
  decision; the directory listing is the index.

Each feature folder is written by exactly one branch at a time, so the harness
files always merge cleanly. Full rationale and the worktree workflow are in
[`parallel-work.md`](parallel-work.md).

---

## Traceability

Requirements, validation, and tasks cite each other by IDs **scoped to the
feature** (there is no global counter), so a reviewer can confirm coverage
without reading the whole codebase:

```text
requirement (REQ-3) ── satisfied by ──> task (T-7) ── proven by ──> acceptance test (AT-3)
                                                     └─ or deferred ─> validation plan (DV-1)
```

- Requirements: `REQ-1`, `REQ-2`, … (stable within the feature; never renumber
  once cited).
- Acceptance tests: `AT-1`, …; deferred validations: `DV-1`, ….
- Tasks: `T-1`, `T-2`, … (per feature). Cite across features as
  `<feature>/T-1`, `<feature>/REQ-2`.

---

## Folder map

```text
ai-harness/
├── START-HERE.md       ← boot sequence (read first, every session)
├── README.md           ← you are here: how the harness works
├── parallel-work.md    ← file ownership + worktree workflow + merge rules
├── tool-linking.md     ← connect any AI tool via a thin adapter
├── roles.md            ← the "hats" an agent wears per phase (all roles, one file)
├── context/            ← stable, shared project knowledge (read-only mid-feature)
│   ├── project.md      ← project-specific facts (CUSTOMIZE)
│   ├── architecture.md ← hexagonal (ports & adapters) rulebook
│   └── testing.md      ← validation strategy: TDD-when-practical + deferral
├── specs/              ← one FOLDER per feature (the heart of SDD)
│   ├── README.md       ← how the feature-folder model works + naming
│   ├── global-spec-info.md   ← static reference: how to fill any spec.md
│   ├── global-state-info.md  ← static reference: how to fill any state.md
│   ├── _template/      ← copy this whole folder to start a feature
│   │   ├── spec.md     ← bare freeform (guidance in global-spec-info.md)
│   │   └── state.md    ← bare freeform (guidance in global-state-info.md)
│   └── <feature>/      ← e.g. order-submission/  (spec.md + state.md)
└── decisions/          ← architecture decision records (ADRs)
    ├── README.md       ← when/how to write one (no index — dir is the index)
    └── 2026-06-07-adopt-ai-harness.md
```

---

## What this harness deliberately excludes

To stay focused on the local idea→implementation loop, the harness does **not**
set up CI pipelines, pull-request or issue templates, release automation,
package files, or any runtime tooling. It is folders and Markdown only. When you
later want those, add them outside `ai-harness/`; the harness still describes how
you work.

---

## Customizing this harness

Sections marked `<!-- CUSTOMIZE -->` are placeholders for your project. The
harness ships with generic but usable starter content; replace the marked parts
with project specifics as they become known. Start with
[`context/project.md`](context/project.md).
