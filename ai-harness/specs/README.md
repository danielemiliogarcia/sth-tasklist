# Specs

This folder holds **one subfolder per feature** — the heart of
Specification-Driven Development. Each feature owns all of its files, which is
what makes parallel work conflict-free (see
[`../parallel-work.md`](../parallel-work.md)).

```text
specs/
├── README.md            ← you are here
├── global-spec-info.md  ← static reference: how to fill any spec.md (read on demand)
├── global-state-info.md ← static reference: how to fill any state.md (read on demand)
├── _template/           ← copy this whole folder to start a feature
│   ├── spec.md          ←   bare freeform; structure/guidance in global-spec-info.md
│   └── state.md         ←   bare freeform; structure/guidance in global-state-info.md
└── <feature-name>/      ← one folder per feature (e.g. order-submission/)
    ├── spec.md
    └── state.md
```

---

## Two files per feature, on purpose

| File | Volatility | Holds |
|------|------------|-------|
| `spec.md` | **stable** — changes slowly | requirements (`REQ-*`), acceptance & validation (`AT-*` / `DV-*`), hexagonal design |
| `state.md` | **volatile** — changes every session | phase, next action, blocker, read budget, and the feature's task table (`T-*`) |

Splitting stable from volatile means routine task-state churn never rewrites the
spec, so even edits to the same feature merge cleanly. `state.md` is what a fresh
session reads first for the feature; `spec.md` sections are opened on demand per
the phase.

Both per-feature files are **bare freeform key:value** — only the filled-in
content. Their structure, legal vocabulary, and how-to-fill guidance live in two
shared static references, read on demand like `context/*`:
[`global-spec-info.md`](global-spec-info.md) and
[`global-state-info.md`](global-state-info.md).

---

## Start a new feature

1. **Copy the whole `_template/` folder** to `specs/<feature-name>/` (short,
   kebab-case, e.g. `order-submission`). Do this on a feature branch / worktree.
2. Fill `spec.md` § Requirements first (Spec Author). Give each requirement an ID.
3. Set `state.md`: phase = `requirements`, next action = "write acceptance tests",
   and record the branch name.
4. Proceed through the lifecycle, updating `state.md` at each handoff.

> **One feature per branch.** An agent works in one feature folder; that is how
> multiple agents stay out of each other's way.

---

## Naming and IDs

- Folder: kebab-case feature name (`user-login/`, `order-submission/`).
- IDs are **scoped to the feature** — there is no global counter:
  - Requirements: `REQ-1`, `REQ-2`, … (stable within the feature; never renumber
    once cited).
  - Acceptance tests: `AT-1`, …; deferred validations: `DV-1`, ….
  - Tasks: `T-1`, `T-2`, ….
- Cite across features with the folder name: `order-submission/REQ-2`,
  `user-login/T-3`.

Because IDs reset per feature, two parallel branches can never collide on the
same ID.

---

## Finished features

When a feature is `done`, leave its folder in place with `state.md` marked
`done` — it is the feature's permanent record. To see all work, `ls specs/` and
read each `state.md` header. There is deliberately no global dashboard file
(it would be a shared merge hotspot — see
[`../parallel-work.md`](../parallel-work.md)).

---

## When a feature gets large

If `spec.md` grows unwieldy, split a heavy section into a sibling file inside the
same folder (e.g. `design.md`) and link it from `spec.md`. Keep `state.md` and
the lighter sections intact so resumability is unaffected. Prefer staying in two
files until size genuinely hurts.

---

## Lifecycle reminder

```text
idea → requirements → validation → design → tasks → implementation → review → done
```

A feature is `done` when all its tasks are `done` (or `cancelled`), its
requirements are satisfied, and its validation has run or is scheduled via
follow-up tasks. Record completion in the feature's `state.md`.
