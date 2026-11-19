# Fresh paired n17 rho card-bound registration, version 1

Status: protocol + tip CLI for freeze/publish/replay/execute. Execute remains
blocked until a separate external authorization supplies
`separately_authorized_rho_arm_v1`. The four-arm freeze alone records
`not_granted_freeze_only` and does not admit consume.

## Problem

Arm order starts with **rho**, but only a target-free source publication exists
([rho-source-v1](../rho-source-v1/result-v1/RESULT.md)). The frozen F5 capsule
`icprog` has no rho/incumbent card-bound one-use path. Without a tip controller
that binds the public card to the archived `ecbench` binary under a one-use
claim, the first freeze arm cannot be consumed safely.

## Contract

1. Inputs are the committed rho source publication (seal
   `f74601e7…`), its source descriptor (`24d65abf…`), the public card
   (`bd969a40…`, target `[87633, 27]`), and four-arm `freeze.json`
   (`a94df27c…`).
2. `rho-target-freeze` copies archived `ecbench`
   (`60fa3f7a…`) into a new capsule, writes the strong-rho ecbench spec from
   freeze policy pins, and seals an unconsumed registration with status
   `RHO_CARD_REGISTERED_NOT_DISPATCHED`. No worker runs.
3. `rho-target-publish` / `rho-target-replay` are data-only custody of that
   unconsumed capsule (`PASS_DATA_ONLY_RHO_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY`).
4. `rho-target-execute` refuses unless
   `--dispatch-authorization separately_authorized_rho_arm_v1`. On that token
   it consumes the claim before launching the archived `ecbench` once, retains
   stdout/stderr/terminal even on failure, and never retries.
5. Closed controls, readiness sessions, and consumed F5/SAT/ordinary
   registrations stay closed.

## Non-claims

Freeze/publish/replay admit no online speedup, no fresh-paired qualification,
and no four-arm dispatch. Execute, when later authorized, remains L0 unless an
isolated-host receipt is separately registered.
