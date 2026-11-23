# Binary genus-two publication: inherited CI and security-review gates

- **Status:** open
- **Kind:** inherited build/CI integration failures and required external security review
- **Affected work:** [crypto PR1830](https://github.com/aburan28/crypto/pull/1830), [continuation receipt](../research/binary_genus2_speed_20261014/maintenance/20261019-resume-ci/README.md)
- **Next action owner:** base-branch maintainers for shared failures; human security reviewer for alert 456; PR author for the narrow dependency checkout repair
- **Last updated:** 2026-10-19

## Problem

PR1830 cannot meet its current-head CI and review gates while shared base
code/configuration fails and the explicit human security review is outstanding.
This blocks publication merge, not continuation of the separately pinned,
unchanged full-subgroup campaign binary. The original m=83 workload, both
prime orders, full-target recovery and measurement requirements remain open.

## Evidence and uncertainty

- **Observed, PR-specific:** native job [117507898326](https://github.com/aburan28/crypto/actions/runs/39070828371/job/117507898326)
  stops because its sparse checkout lacks the new `volcano-cli` path dependency.
  The maintenance change includes `tools/volcano-cli` and its
  `isogeny_algos` dependency, adds their path triggers and regenerates only
  the standalone tool lockfile with Cargo. Strict YAML and locked Linux
  dependency-closure checks pass.
- **Observed, shared source:** native package validation then stops on 25
  compiler errors; the required release library-test attempt stops on 45
  compiler errors, before executing tests. Examples include duplicate
  `Instance::Fp4`, unresolved `ecbench::fbm`, and missing `sparsest`.
  Raw diagnostics are in the linked maintenance record.
- **Derived by Git object identity:** the complete `src` tree
  `401b7bf9620d4bb39757d06d5d00254a37093ae7`, root Cargo files,
  `tests`, `isogeny_algos`, all pre-existing workflow files,
  `.github/scripts/ci-scope.mjs` and `tools/lean/AuditAxioms.lean`
  match base `c6d64436a33359320ad9d5b42c4acebee2968b9a` at the inspected
  parent head `a320d9dc3100ec553ccda0fc43e2de7f863380af`.
  These shared failures are not repaired by changing campaign arithmetic.
- **Observed, shared CI:** workflow syntax job [117507898693](https://github.com/aburan28/crypto/actions/runs/39070828422/job/117507898693)
  finds duplicate YAML keys; scope job [117507898700](https://github.com/aburan28/crypto/actions/runs/39070828421/job/117507898700)
  finds a duplicate `const root`; the global Lean audit
  [117518971095](https://github.com/aburan28/crypto/actions/runs/39070828275/job/117518971095)
  fails on unknown `String.ofList`. The separate construction Lean job
  succeeded. This does not complete the geometric/Picard formalization.
- **Observed, review:** [thread PRRT_kwDOSMdswM6sfA9-](https://github.com/aburan28/crypto/pull/1830#discussion_r4275317021)
  / [alert 456](https://github.com/aburan28/crypto/security/code-scanning/456)
  remains unresolved. Approval-agent feedback explicitly requires human
  review. The existing public-input explanation is not review disposition.
- **Missing capability:** accepted security disposition and successful
  applicable checks on the actual final PR head. No performance advantage,
  complete common price or ecbench audit follows from this maintenance.

## What would unblock it

- [ ] The native job checks out its complete local dependency closure and
      passes locked package validation on a compatible base.
- [ ] Base maintainers repair the shared Rust, scope/workflow and global Lean
      audit failures; all required/applicable final-head checks succeed.
- [ ] A human security reviewer disposes of alert 456 and the unresolved
      review requirement is actually satisfied.

These are merge requirements. Meeting them does not complete a=0 full solves,
establish durable compute, formalize the Picard bridge or admit a speedup.

## Next discriminating step

- **Action:** publish the narrow checkout/lock repair and its exact resume
  record; recheck current-head CI/reviews at the repository cadence.
- **Controls:** the parent/base Git identities and raw compiler/job diagnostics
  above; never substitute old-head or manual-dispatch results.
- **Limits:** existing resources only; pause the running search before heavy
  local validation; no unrelated base rewrites, disabled checks, review
  bypass, credential replacement or paid infrastructure.
- **Outcomes:** new change-caused failures receive a scoped fix; unchanged
  base/security failures remain open with evidence. Merge only after all
  actual gates pass. Stop routine PR follow-up after the prescribed unchanged
  checks; do not stop the authorized search for that reason.

## Resolution and history

Unresolved. This record preserves the current merge gates; it is not a new
ECDLP result and does not supersede the a=0 compute or ecbench-adapter problems.
