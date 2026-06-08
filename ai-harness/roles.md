# Roles

A **role** is a hat an agent wears for one phase of the lifecycle. The same AI
(or human) wears different hats at different times. Each role says *what to
read*, *what to produce*, and *when to hand off*. You do not read every role —
just the one matching the current phase (named in your feature's `state.md`).

All roles are in this one file to keep the harness compact and to make handoffs
visible. Jump to the one you need.

| Phase | Role | Section |
|-------|------|---------|
| (any — bootstrap) | Session Loader | [↓](#session-loader) |
| idea · tasks · done | Planner | [↓](#planner) |
| requirements · design | Spec Author | [↓](#spec-author) |
| validation | Tester | [↓](#tester) |
| implementation | Implementer | [↓](#implementer) |
| review | Reviewer | [↓](#reviewer) |

**How handoff works:** roles hand off through *files, not memory*. When a role
finishes, it writes its artifact and updates the feature's `state.md` (phase,
last step, next action, task states). The next role reads it, opens only what it
needs, and proceeds. A handoff and a fresh-session resume are the same operation.

> All paths below are relative to a feature folder, e.g.
> `specs/<feature>/state.md` and `specs/<feature>/spec.md`. You write only your
> own feature folder (see [`parallel-work.md`](parallel-work.md)).

---

## Session Loader

**Purpose:** orient a fresh session with the *minimum* context, then switch to
the role matching the current phase. This is the first hat in any new session.
You are optimizing for two things at once: **enough context to act correctly**
and **as little reading as possible**.

**Read (in order, stop early):**

1. [`START-HERE.md`](START-HERE.md) — the boot sequence (if not already read).
2. Identify your feature (branch / worktree name, the human's instruction, or
   `ls specs/`).
3. `specs/<feature>/state.md` — phase, next action, read budget, task table.
4. Only then, if the next action needs it, the relevant section of
   `specs/<feature>/spec.md`.

**Stop.** You now know the phase and the next action. Do not open other features
or the whole `context/` folder. Trust the `state.md` read budget about what to
skip.

**Produce:** nothing yet — a clear mental model ("we are in phase X on feature Y;
the next action is Z; the files I need are …"), then adopt the matching role.

**Edge cases:**

- **A task is `blocked`.** Read its blocker. Clear it if you can; otherwise
  surface it and pick the next unblocked task or ask the human.
- **`state.md` points to a missing file.** Treat as a small bug: locate it,
  repair the pointer, note the fix.
- **No feature for your branch / no features at all.** Switch to **Planner**;
  shape or create the feature (copy `specs/_template/`).

---

## Planner

**Purpose:** keep work flowing. The Planner decides *what to do next*, turns
ideas into features, breaks features into stateful tasks, and closes work when it
is done. It directs; it does not write production code or detailed specs.

Serves the **idea**, **tasks**, and **done** phases.

**Read:** the feature's `state.md`,
[`specs/global-state-info.md`](specs/global-state-info.md) for the task-line
format + legal states, and the spec's design when breaking work down.

**Produce, by phase:**

- **idea → feature:** a new `specs/<feature>/` folder (copy of `_template/`),
  with a one-paragraph intent in `spec.md` and `state.md` initialized (phase =
  `requirements`). Ideally on a dedicated feature branch.
- **feature → tasks:** populate the Tasks table in the feature's `state.md`, each
  task with an ID (`T-1`, … scoped to this feature), state `todo`, and the
  requirements + acceptance tests it satisfies.
- **done:** mark tasks `done` in `state.md`, set the feature's overall state to
  `done`, and (if a choice was cross-cutting) record a `decisions/` ADR.

**How to break a feature into tasks:** walk the spec's Design from the outside in
(driving adapter → use case → domain → driven adapter/port); each slice is
usually one or two tasks. Give each a stable per-feature ID, observable
done-criteria, the requirement/validation IDs it serves, and a short "context
needed" list. Order so each task is independently testable — prefer a thin
vertical slice that proves the path end to end before broadening. If a task
cannot be tested first, mark it and record the deferred-validation plan.

**Prioritize, in order:** (1) unblock or cancel a `blocked` task; (2) finish a
`review` task (cheap, high closure value); (3) the smallest `todo` that proves
the riskiest assumption. Record the choice and reasoning in `state.md` so the
next session inherits the rationale, not just the result.

**Checklist:**

- [ ] Anything `blocked`? Clear or cancel it first.
- [ ] Anything in `review`? Get it confirmed and closed.
- [ ] Does each new task cite requirements + validation?
- [ ] Is `state.md`'s "Next action" the genuine next step?

---

## Spec Author

**Purpose:** define *what* to build and *why* (requirements), then *how* it will
be built (design) in hexagonal terms. Writes the documents the rest of the
lifecycle depends on. No production code in this role.

Serves the **requirements** and **design** phases.

**Read:** [`specs/global-spec-info.md`](specs/global-spec-info.md) for the spec
structure + ID rules, the feature's `spec.md` (current draft),
[`context/project.md`](context/project.md) for domain and goals, and — for the
design phase — [`context/architecture.md`](context/architecture.md). Cite any
constraint already fixed in [`decisions/`](decisions/README.md).

**Produce — `spec.md` § Requirements:**

- A clear **problem statement** and the **value / why**.
- **Functional requirements**, each with a per-feature ID (`REQ-1`, …), written
  to be testable (observable behavior, not implementation).
- **Non-functional requirements** that matter.
- Explicit **in-scope / out-of-scope**, **assumptions**, and **open questions**
  (list them — never silently assume).
- Hand off to the **Tester** to turn requirements into acceptance tests *before*
  design hardens, when practical.

**Produce — `spec.md` § Design (hexagonal):**

- The **domain model** (entities, value objects, invariants) — pure, no IO.
- The **ports** the feature needs (named for capability, not technology).
- The **adapters** that implement them and the **use cases** that orchestrate.
- The **dependency direction** (everything points inward toward the domain).
- New ports / terms / conventions go in the spec's **Feature-local conventions**
  block, not in shared `context/*` (keeps parallel branches conflict-free).
- **Trade-offs considered**; link a `decisions/` ADR for anything cross-cutting.
- Enough detail for the Planner to cut tasks and the Implementer to build — no
  speculative design for requirements that do not exist.

**Checklist:**

- [ ] Every requirement has an ID and is testable.
- [ ] Scope (in/out), assumptions, and open questions are explicit.
- [ ] Design names domain, ports, adapters, and use cases distinctly.
- [ ] Dependency direction points inward to the domain.
- [ ] `state.md` updated: phase, last step, next action.

---

## Tester

**Purpose:** define how each requirement will be *proven*, ideally before
implementation. Turns requirements into acceptance tests, or — when test-first is
impractical — into an explicit, scheduled deferred-validation plan. No silent
gaps.

Serves the **validation** phase. See [`context/testing.md`](context/testing.md).

**Read:** the spec's requirements,
[`specs/global-spec-info.md`](specs/global-spec-info.md) for how acceptance /
`DV` entries are written, and [`context/testing.md`](context/testing.md).

**Produce — in `spec.md` § Acceptance & Validation:**

- One **acceptance test** per requirement (or several), written as
  **Given / When / Then**, each citing the requirement(s) it covers
  (`AT-3 covers REQ-2`) and its level (unit / integration / e2e). Mark tests
  meant to be written **first (red)** to drive TDD.
- For anything not testable first, a **deferred-validation block** (`DV-1`): the
  **reason**, the **interim/manual check** to run before `review`, the **future
  automated path**, and a **follow-up task** (added to the feature's task table).
- A **coverage map**: every requirement maps to at least one `AT-*` or `DV-*`. A
  requirement with no row is a gap.

**Checklist:**

- [ ] Every requirement maps to an `AT-*` or a `DV-*` (no gaps).
- [ ] Tests are behavior-focused and use Given/When/Then.
- [ ] Each deferral records reason + interim check + future path + follow-up task.

---

## Implementer

**Purpose:** build the feature, guided by the design and proven by the tests.
Write the smallest correct code that satisfies the current task, keep the domain
pure, and keep state honest.

Serves the **implementation** phase.

**Read:** the **active task** in the feature's `state.md` (task states + line
format in [`specs/global-state-info.md`](specs/global-state-info.md)), the
relevant **slice** of the spec's design (not the whole thing), the cited
acceptance tests, and [`context/architecture.md`](context/architecture.md). Read
only the slice the task touches.

**The loop (TDD preferred):**

1. Set the task to `doing` in `state.md`.
2. Pick the next acceptance/unit test the task must satisfy; write it so it
   **fails** (red) if it does not exist yet.
3. Implement the **smallest** change that makes it pass (green).
4. Refactor with tests green; remove duplication; keep the domain clean.
5. Repeat until the done-criteria are met, then move the task to `review`.

If TDD is impractical, follow the deferred-validation plan: implement, run the
documented manual check, and confirm the follow-up test task exists. **Never
silently skip validation.**

**Hexagonal discipline while coding:** domain code imports nothing external
(no DB driver, HTTP client, framework, filesystem, clock, env). Need something
outside? Call a **port**; implement it in an **adapter** at the edge. Use cases
orchestrate only. Quick self-check: *could I unit-test this domain logic with no
mocks of the outside world?* If not, infrastructure has leaked inward.

**Keep state honest:** update the task state as you go; if you discover new work,
add a task to *this feature's* table (or note it for a future feature) rather
than expanding the current task silently; record cross-cutting technical choices
as a `decisions/` ADR.

**Checklist:**

- [ ] Task set to `doing` at start.
- [ ] Tests drove the change (or the deferral plan was followed).
- [ ] Domain stays free of infrastructure imports; external access via port + adapter.
- [ ] Done-criteria met; tests green (or manual validation run).
- [ ] New discoveries captured as tasks, not smuggled into this task.
- [ ] Task moved to `review`; `state.md` updated.

---

## Reviewer

**Purpose:** verify a task against its requirements and validation before it is
called `done`. The Reviewer confirms coverage and architectural integrity, not
personal taste.

Serves the **review** phase.

**Read:** the spec's Requirements + Acceptance & Validation sections for the
task, and the changed code.

**Check:**

- **Requirements coverage** — does the work satisfy every cited `REQ-*`?
- **Validation evidence** — do the cited `AT-*` pass, or has the `DV-*` interim
  check been run and the follow-up task created?
- **Hexagonal boundaries** — domain free of infrastructure; external access via
  ports/adapters; use cases hold no business rules. (See the self-check in
  [`context/architecture.md`](context/architecture.md).)
- **State hygiene** — the feature's `state.md` is consistent and resumable.

**Outcome:** on pass, mark the task `done` in `state.md` and hand to the Planner
to close (set the feature's overall state when all tasks are done). On fail, move
it back to `doing` with specific, actionable notes.

**Checklist:**

- [ ] Every cited requirement is satisfied.
- [ ] Validation passed or the deferral plan is honored.
- [ ] Hexagonal boundaries intact.
- [ ] `state.md` consistent and resumable.
