# Global Spec Info — how to fill any feature's `spec.md`

**This is a shared, static reference. Read it on demand (like `context/*`); do
not copy it into a feature.** It defines the structure, vocabulary, and rules for
every `specs/<feature>/spec.md`. Each feature's `spec.md` is a **bare freeform
key:value file** — only the filled-in content. All the *how* lives here.

> **Parallel-safety:** this file is shared static. Treat it read-only during
> feature work; edit it deliberately on the main branch. See
> [`../parallel-work.md`](../parallel-work.md).

---

## What `spec.md` is

The **stable** part of a feature: **requirements**, **acceptance & validation**,
and **hexagonal design**. It changes slowly. The volatile part (phase, next
action, task states) lives next to it in `state.md` — see
[`global-state-info.md`](global-state-info.md).

A feature starts by copying `_template/` to `specs/<feature>/`, then filling
`spec.md` in order: requirements → acceptance → design.

---

## ID rules (scoped per feature — no global counter)

- Requirements: `REQ-1`, `REQ-2`, … — stable within the feature; **never
  renumber once cited**.
- Acceptance tests: `AT-1`, …  Deferred validations: `DV-1`, ….
- Cite across features with the folder name: `order-submission/REQ-2`.
- Because IDs reset per feature, two parallel branches can never collide.

---

## The three sections

### 1. requirements (Spec Author, before any code)

- A one-line **intent**, plus a problem statement and the value / why.
- **Functional requirements**, each with a `REQ-*` id and a priority
  (`must | should | could`), written as **observable behavior, not
  implementation**. If you cannot imagine a test for it, refine it.
- Note **scope** (in / out), **assumptions**, and **open questions** — never
  silently assume.

### 2. acceptance (Tester, ideally before implementation)

- One or more **acceptance tests** per requirement, written as
  **Given / When / Then**, each citing the requirement(s) it covers
  (`AT-3 covers REQ-2`) and a level (`unit | integration | e2e`).
- Mark tests meant to be written **first (red)** to drive TDD.
- For anything not testable first, a **deferred-validation** entry (`DV-n`)
  recording: reason, interim/manual check to run before `review`, future
  automated path, and a follow-up `todo` task added to `state.md`.
- **Coverage rule:** every `REQ-*` maps to at least one `AT-*` or `DV-*`. A
  requirement with no coverage is a gap.

See [`../context/testing.md`](../context/testing.md).

### 3. design (Spec Author, hexagonal)

Express in hexagonal terms — enough to cut tasks and build, no speculative
design. See [`../context/architecture.md`](../context/architecture.md).

- **domain** — entities, value objects, invariants. Pure, no IO.
- **ports** — interfaces the application declares, named for the **capability**
  not the technology (`OrderRepository`, not `PostgresClient`).
- **adapters** — driven (implement a port: `SqlOrderRepository`) and driving
  (call the app: `HttpOrderController`).
- **usecases** — orchestrate domain + ports; hold no business rules.
- **feature-local conventions** — new ports / terms this feature introduces go
  here, **not** in shared `context/*` (which would conflict across parallel
  branches). Promote project-wide later, deliberately, on main.
- Dependency direction points **inward** to the domain.

---

## Canonical freeform shape (worked example)

```
# spec: order-submission
intent: accept an order, enforce rules, persist it, return its id

requirements:
  REQ-1 (must): the system shall reject an order with zero items
  REQ-2 (must): a submitted order is persisted and its id returned

acceptance:
  AT-1 covers REQ-1 (unit, red):
    Given an empty cart / When submit / Then raise EmptyOrder
  AT-2 covers REQ-2 (integration):
    Given a valid order / When submit / Then it is saved and id returned

design:
  domain: Order (invariant: >= 1 item)
  ports: OrderRepository (save, byId), Clock (now)
  adapters: SqlOrderRepository (driven), HttpOrderController (driving)
  usecases: SubmitOrder (build Order -> enforce rules -> save via port -> return id)
  feature-local conventions: new port OrderRepository
```

Keep it short. Anything that needs more explanation than fits here is a sign the
requirement should be split, not that the file should grow prose.
