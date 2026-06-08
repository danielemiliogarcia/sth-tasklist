# Parallel Work & File Ownership

This file explains **why the harness is structured the way it is** — one folder
per feature, no global state files — and how to run **multiple AI agents in
parallel** (separate branches or git worktrees, one feature each) without merge
conflicts.

Read this once before starting parallel work. Day to day you only need
[`START-HERE.md`](START-HERE.md) and your feature's `state.md`.

---

## The problem this structure solves

A naive harness keeps *global* mutable files — a single "current task" pointer, a
single task board, a global decision index with sequential numbering. That is
fine for one agent working one thing at a time. It breaks the moment two agents
work in parallel:

- Both rewrite the global "now" pointer → **merge conflict** (and conceptually,
  there is no single "now" when N agents work at once).
- Both add rows to the one task board, and both grab the next free task ID
  (`T-001`) → **conflict + duplicate IDs**.
- Both create the next decision file (`0002-*.md`) and both append to the
  decision index → **conflict + duplicate numbers**.

Every one of those is a *shared mutable file written by more than one branch*.
That is the root cause of merge pain.

## The rule that fixes it

> **The feature is the unit of parallelism, and it owns all of its mutable state
> inside its own folder.** No file outside a feature folder is written during
> normal feature work.

If two agents only ever write inside `specs/<their-own-feature>/`, their branches
touch disjoint files and merge cleanly — git does not even have to think.

---

## How each potential hotspot is handled

| Mutable thing | Where it lives now | Why it is conflict-free |
|---------------|--------------------|--------------------------|
| "What is active now" | `specs/<feature>/state.md` | One per feature; your branch writes only yours. The global "now" is *derived* (which feature your branch is on), not stored. |
| Task list + states | task table inside `specs/<feature>/state.md` | Per feature; no shared board. |
| Task / requirement IDs | scoped per feature (`T-1`, `REQ-1`) | No global counter, so two branches cannot collide on an ID. Cite across features as `<feature>/T-1`. |
| Decisions (ADRs) | `decisions/YYYY-MM-DD-slug.md`, **no index file** | Date + slug rarely collides; the directory listing *is* the index, so there is no shared index to conflict on. |
| The feature spec | `specs/<feature>/spec.md` | Separate file per feature. Split from `state.md` so churning task state never rewrites the stable spec. |

The result: **there is no committed file that every feature branch must write.**

---

## Keep shared static files out of feature branches

A few files *are* shared and mostly static: `README.md`, `roles.md`,
`tool-linking.md`, everything in `context/`, the `specs/global-spec-info.md` and
`specs/global-state-info.md` references, and the `specs/_template/` folder.
If two feature branches both edit one of these, you get a conflict — so don't,
during feature work:

- **New port, domain term, or convention introduced by a feature?** Record it in
  *your* `specs/<feature>/spec.md` (it has a "Feature-local conventions" block),
  **not** in `context/architecture.md` / `context/project.md`.
- **Want it to become project-wide?** Promote it into the shared `context/*` file
  **deliberately, on the main branch**, as its own small change — after the
  feature merges. One writer, no race.

Treat shared static files as read-only while a feature is in flight.

---

## Worktree workflow (suggested)

```text
main
 ├─ worktree A → branch feat/user-login    → writes specs/user-login/
 ├─ worktree B → branch feat/order-export  → writes specs/order-export/
 └─ worktree C → branch feat/rate-limit    → writes specs/rate-limit/
```

1. **Start a feature.** From main, create a branch + worktree named for the
   feature. Copy `specs/_template/` → `specs/<feature>/`. Commit that as the
   feature's first step.
2. **Work.** The agent in each worktree reads/writes only its own feature folder
   (plus *reading* shared `context/*` as needed). It updates
   `specs/<feature>/state.md` as it goes.
3. **Resume.** A fresh session in a worktree finds its feature from the branch
   name (see [`START-HERE.md`](START-HERE.md)) and reads that feature's
   `state.md`. No global pointer to consult or fight over.
4. **Finish & merge.** When the feature is `done`, merge to main. Because only
   the feature folder (and the app code it produced) changed under `ai-harness/`,
   the merge is clean. Leave the feature folder in place with `state.md` marked
   `done` — it is the feature's record.

## Application code conflicts (out of scope here)

This harness guarantees the *harness* files merge cleanly. Your **application
code** can still conflict if two features edit the same source files — that is
ordinary software merge work, not a harness concern. Hexagonal architecture helps:
features that add new adapters/use cases in new files conflict less than features
that all edit one big module. Keep the composition root (where adapters are wired)
small and expect to merge it by hand.

---

## Seeing everything at a glance

There is deliberately **no global dashboard file** (it would be a shared mutable
hotspot). To see all work:

- `ls specs/` — every feature is a folder.
- Read each `specs/*/state.md` header (phase + overall state) — they are tiny.

If you genuinely want a committed overview, add one **on main only** and update it
at feature start/finish (lifecycle boundaries), never mid-feature — but prefer
living without it. The feature folders are the source of truth.
