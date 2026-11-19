# Fresh paired n17 incumbent card-bound registration, version 1

Status: protocol + tip CLI for freeze/publish/replay/execute. Execute remains
blocked until a separate external authorization supplies
`separately_authorized_incumbent_arm_v1`. The four-arm freeze alone records
`not_granted_freeze_only` and does not admit consume. Candidate IC1 stays
null until a separate exact pre-card factor-base inventory is sealed.

## Problem

Arm order ends with **incumbent**. The target-free source publication
([incumbent-source-v1](../incumbent-source-v1/result-v1/RESULT.md)) archives
the pairinv worker, but tip `icprog` had no incumbent card-bound one-use path
parallel to [rho-target](../RHO_TARGET.md). Without a tip controller that binds
the public card to the archived worker under a one-use claim, the fourth freeze
arm cannot be consumed safely.

## Contract

1. Inputs are the committed incumbent source publication (seal
   `36923bdb…`), its source descriptor (`fea835bd…`), the public card
   (`bd969a40…`, target `[87633, 27]`), and four-arm `freeze.json`
   (`a94df27c…`).
2. `incumbent-target-freeze` copies the archived worker and build sidecars into
   a new capsule, writes the pair-table stdin job from freeze policy pins, and
   seals an unconsumed registration with status
   `INCUMBENT_CARD_REGISTERED_NOT_DISPATCHED`. No worker runs.
3. `incumbent-target-publish` / `incumbent-target-replay` are data-only custody
   of that unconsumed capsule
   (`PASS_DATA_ONLY_INCUMBENT_TARGET_SCIENTIFIC_REGISTRATION_CUSTODY`).
4. `incumbent-target-execute` refuses unless
   `--dispatch-authorization separately_authorized_incumbent_arm_v1`. On that
   token it consumes the claim before launching the archived worker once with
   the sealed stdin job, retains stdout/stderr/terminal even on failure, and
   never retries.
5. Closed controls, readiness sessions, and consumed F5/SAT/ordinary/rho
   registrations stay closed. This path does not mint IC1 or replace the
   required pre-card inventory gate.

## Non-claims

Freeze/publish/replay admit no online speedup, no fresh-paired qualification,
no IC1 identity, and no four-arm dispatch. Execute, when later authorized,
remains L0 unless an isolated-host receipt is separately registered.
