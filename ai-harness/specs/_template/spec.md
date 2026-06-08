# spec: <feature>
intent: <one line — what this delivers>
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): <testable, observable statement>

acceptance:
  AT-1 covers REQ-1 (unit, red):
    Given <ctx> / When <action> / Then <observable outcome>

design:
  domain: <entities/VOs + invariants — pure, no IO>
  ports: <capability ports, e.g. OrderRepository>
  adapters: <driven / driving>
  usecases: <name: steps; ports used>
  feature-local conventions: <new ports/terms introduced here>
