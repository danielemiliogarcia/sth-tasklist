# Architecture — Hexagonal (Ports & Adapters)

The harness assumes a **hexagonal** layout so the domain stays pure, testable,
and independent of frameworks and infrastructure. This file is the practical
rulebook: the Spec Author designs with it, the Implementer builds with it, the
Reviewer checks against it. Keep it open during design and implementation.

---

## The layers

```text
┌─────────────────────────────────────────────────────────────┐
│ Adapters (outer edge)                                        │
│   driving: HTTP controllers · CLI handlers · schedulers · UI │
│   driven:  DB repositories · API clients · queues · clock    │
├─────────────────────────────────────────────────────────────┤
│ Application (use cases)                                       │
│   orchestrates the domain; declares Ports (interfaces)       │
├─────────────────────────────────────────────────────────────┤
│ Domain (pure core)                                           │
│   entities · value objects · domain services · rules         │
│   imports nothing external — no framework, IO, DB, HTTP, clock│
└─────────────────────────────────────────────────────────────┘
        dependencies point inward → toward the Domain
```

- **Domain** — entities, value objects, domain services, and the business rules
  binding them. Pure. No imports of frameworks, IO, DB, HTTP, time, or env.
- **Port** — an interface *owned by the application*, describing something it
  needs from outside, in the application's own language. Examples:
  `OrderRepository`, `PaymentGateway`, `Clock`, `EventPublisher`.
- **Adapter** — a concrete implementation at the edge.
  - **Driven adapter** implements a port using real tech (`SqlOrderRepository`).
  - **Driving adapter** calls the application from outside
    (`HttpOrderController`, `CliHandler`).
- **Use case / application service** — orchestrates domain objects and ports to
  fulfil one request. Holds no business rules and no infrastructure detail.

---

## The dependency rule

```text
Driving adapter ──► Use case ──► Domain
                       │
                       └──► Port (interface)  ◄── implemented by ── Driven adapter
```

- Compile-time dependencies point **inward** (toward the domain) — except
  adapters, which depend on the ports they implement.
- The domain depends on **nothing** in the outer rings.
- The application depends on the **domain** and on **its own port interfaces**,
  never on concrete adapters.
- Adapters are wired to the application at the **composition root** (startup),
  via dependency injection.

Why this helps an AI agent: the dependency rule is a single, checkable
constraint. It stops generated code from smuggling database or HTTP concerns into
business logic, and it makes the domain trivially unit-testable without mocking
the whole world.

---

## Do

- Put business rules and invariants **in the domain** (e.g. "an order needs at
  least one item" lives on the `Order` entity, not in a controller).
- Express external needs as **ports** in the application's language.
- Implement ports in **adapters** at the edge.
- Inject adapters into use cases from the **composition root**.
- Pass **time, randomness, and IO** through ports (`Clock`, `IdGenerator`,
  `Repository`) so the domain stays deterministic.
- Keep use cases thin: validate input shape, call domain, call ports, return.

## Don't

- ❌ Import a database/ORM, HTTP client, framework, filesystem, or `now()` inside
  the domain.
- ❌ Put business rules inside an adapter or controller.
- ❌ Let a use case depend on a concrete adapter class.
- ❌ Leak transport/persistence shapes (DTOs, ORM rows, JSON) into the domain;
  translate at the adapter boundary.
- ❌ Reach for the network or clock directly "just this once" — add a port.

---

## Worked example (language-neutral pseudocode)

```text
# domain/order.*  — pure
Order:
  items: list
  rule: an Order with zero items is invalid → raise EmptyOrder

# application/ports.*  — interfaces the app declares
interface OrderRepository:   save(order); byId(id) -> Order
interface Clock:             now() -> Timestamp

# application/submit_order.*  — use case orchestrates
SubmitOrder(repo: OrderRepository, clock: Clock):
  run(command):
    order = Order.fromCommand(command, clock.now())   # domain enforces rules
    repo.save(order)                                   # via port
    return order.id

# adapters/sql_order_repository.*  — driven adapter (edge)
SqlOrderRepository implements OrderRepository:
  save(order): ...translate to rows, write to DB...

# adapters/http_order_controller.*  — driving adapter (edge)
HttpOrderController(submitOrder: SubmitOrder):
  POST /orders: id = submitOrder.run(parse(request)); return 201, id

# main.*  — composition root: wire concrete adapters into use cases
repo = SqlOrderRepository(db)
clock = SystemClock()
submitOrder = SubmitOrder(repo, clock)
http = HttpOrderController(submitOrder)
```

The domain (`Order`) unit-tests with no database and no HTTP. The use case tests
with in-memory fakes of `OrderRepository` and `Clock`. The adapters get
integration tests at the edge.

---

## Naming conventions

- Name **ports** for the capability, not the technology (`OrderRepository`, not
  `PostgresClient`).
- Name **adapters** for the technology + capability (`SqlOrderRepository`,
  `HttpOrderController`).
- Translate external shapes to domain types **at the adapter boundary**, never
  deep inside.

---

## Self-check (use during review)

- Could I unit-test the domain logic **without any mocks** of DB/HTTP/time? If
  not, infrastructure has leaked inward.
- Does any domain file import something from `adapters/` or a framework? That is
  a violation.
- Does a use case reference a concrete adapter instead of a port? Introduce and
  inject the port.
- Are transport/persistence shapes translated at the boundary, not passed inward?

---

## Project-specific bindings

> `<!-- CUSTOMIZE -->` Record decisions that shape the whole system (module
> layout for `src/`, DI/wiring, error-handling and transaction boundaries), and
> list the real ports and their adapters as they appear so agents reuse them
> instead of inventing new ones. Link significant choices to a
> [`../decisions/`](../decisions/README.md) record.
>
> **Parallel-safety:** this is a *shared* file. While a feature is in flight, put
> its new ports in that feature's `spec.md`, not here — two feature branches both
> appending rows would conflict. Promote a port to this project-wide table
> **deliberately, on the main branch**, after the feature merges. See
> [`../parallel-work.md`](../parallel-work.md).

- Module / folder layout: _…_
- Dependency injection / wiring: _…_
- Error handling strategy: _…_
- Transaction / consistency boundaries: _…_

| Port (interface) | Purpose | Adapter(s) |
|------------------|---------|------------|
| _OrderRepository_ | _persist orders_ | _SqlOrderRepository_ |
| _Clock_ | _current time_ | _SystemClock / FixedClock (tests)_ |
