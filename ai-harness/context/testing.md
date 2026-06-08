# Testing & Validation Strategy

How this project proves behavior. The Tester writes acceptance tests from this;
the Implementer drives code with it; the Reviewer checks against it.

**Core stance: TDD is preferred when practical, but not mandatory.** Validation
is *always* defined — either as automated tests written first, or as a
documented, scheduled plan to add them later. Nothing ships "untested forever."

---

## The test pyramid (mapped to hexagonal layers)

```text
        /\        e2e / acceptance  — few, slow, high confidence
       /  \                          through driving adapters, real-ish edges
      /----\      integration       — adapters against real tech (DB, queue, API)
     /      \                         verify ports' implementations
    /--------\    unit               — many, fast
   /__________\                        domain rules + use cases with fakes
```

- **Unit (most tests).** Domain entities/value objects/services in isolation, and
  use cases with **in-memory fakes** of their ports. Fast, deterministic, no IO.
  This is where hexagonal architecture pays off: a pure domain needs no mocks.
- **Integration (some).** Driven adapters against real technology (a real DB, a
  test broker/container). Confirms a port's adapter actually works.
- **End-to-end / acceptance (few).** Through a driving adapter (HTTP/CLI) across
  the whole slice. Confirms the requirement holds in the assembled system.

Default to the **lowest level** that can prove the requirement.

---

## How acceptance tests connect to requirements

- Each acceptance test cites the requirement(s) it covers (`AT-3 covers REQ-2`).
- Write tests as **Given / When / Then** so they read clearly and translate to
  code at any level.
- Mark tests intended to be written **first (red)** to drive implementation.
- Coverage goal: **every requirement maps to at least one test or an explicit
  deferred-validation entry.** No requirement is left unvalidated.

---

## The TDD loop (when practical)

```text
red → green → refactor
```

1. Write a failing test for the next small behavior (from the spec's Acceptance
   & Validation section).
2. Write the minimal code to pass it.
3. Refactor with tests green.
4. Repeat.

Prefer a thin **vertical slice** first (one path end to end) before broadening.

---

## When tests are deferred (allowed, but tracked)

Some work resists test-first: spikes/exploration, scaffolding, large refactors,
infrastructure/environment setup, integrations you do not yet understand, or UI
needing human judgment. In those cases — **defer deliberately, never silently:**

1. Record a **Deferred validation (`DV-n`)** block in the spec's Acceptance &
   Validation section: the **reason**, the **interim/manual validation** to run
   now, and the **future automated path**.
2. Run the interim/manual validation **before** moving the task to `review`.
3. Create a **follow-up `todo` task** in the feature's task table
   (`specs/<feature>/state.md`) to add the automated test when feasible.

Deferral is a scheduling decision, not an excuse to skip validation. A task is
either tested, or it carries an explicit, scheduled plan to become tested.

---

## What makes a good test here

- Tests **behavior and requirements**, not implementation details.
- Domain tests use **no mocks** (pure inputs/outputs).
- Use-case tests use **fakes of ports**, not heavyweight mocks of the world.
- Deterministic: inject `Clock`, `IdGenerator`, randomness via ports.
- One reason to fail per test where practical; names state the behavior asserted.

---

## Project-specific testing notes

> `<!-- CUSTOMIZE -->` Fill in once the stack is chosen.

- Test runner / framework: _…_
- How to run all tests: _…_
- How to run a single test: _…_
- Integration test setup (containers, fixtures, seed data): _…_
- Coverage expectations, if any: _…_
- Naming / location convention for tests: _…_
