# Decisions (ADRs)

Lightweight **Architecture Decision Records** for **cross-cutting** choices —
ones that shape the whole project or constrain future work. When future-you will
ask "why did we do it this way?", record it here as a short, self-contained note.

```text
decisions/
├── README.md                    ← you are here (format + when)
└── YYYY-MM-DD-<slug>.md         ← one file per decision (date-named, self-sorting)
```

A worked example exists:
[`2026-06-07-adopt-ai-harness.md`](2026-06-07-adopt-ai-harness.md).

> **Feature-local choices stay in the feature.** A trade-off that only affects
> one feature belongs in that feature's `spec.md` § Design, not here. Reserve
> `decisions/` for genuinely project-wide choices.

---

## Why date-named files and no index

Files are named `YYYY-MM-DD-<slug>.md`, **not** with a global sequential number,
and there is **no index file**. Both choices are about parallel safety: a global
counter (`0002-…`) and a shared index are exactly the things two branches collide
on. A date + slug rarely collides, and the **directory listing, sorted by name,
is the index**. To browse decisions, list the folder.

(If two branches happen to add the same `YYYY-MM-DD-<slug>.md` on the same day,
disambiguate one slug — a trivial, rare fix.)

---

## When to write one

Write an ADR when you:

- choose an architecture pattern, boundary, or major library;
- pick one approach over a viable alternative with real trade-offs;
- adopt a convention everyone must follow;
- make a choice you might otherwise re-litigate later.

Do **not** write one for trivial or easily-reversed choices — keep signal high.

## How to write one

1. Create `YYYY-MM-DD-<short-slug>.md` (today's date).
2. Use the format below. Keep it to roughly one screen.
3. Set status to `accepted` (or `proposed` if still under discussion).
4. Link it from the relevant `spec.md` § Design or
   [`../context/architecture.md`](../context/architecture.md) so it is
   discoverable in context.

## Rules

- **Immutable.** Do not rewrite history. To change a decision, write a **new**
  ADR and set the old one's status to `superseded by <new-file>`.
- **Self-contained.** A reader should understand the decision without chasing
  links, though links help.

## Format

```markdown
# <short decision title>

- **Status:** proposed | accepted | superseded by <file> | deprecated
- **Date:** <YYYY-MM-DD>
- **Deciders:** <human(s) and/or agent>
- **Related:** <feature, REQ-…, or other ADRs>

## Context
What situation and forces prompted a decision? State constraints that matter.

## Decision
The choice, stated plainly. "We will …"

## Alternatives considered
- <Option A> — pros / cons; why not chosen.
- <Option B> — …

## Consequences
What becomes easier and harder; new constraints; follow-up work; how it will be
revisited if assumptions change.
```
