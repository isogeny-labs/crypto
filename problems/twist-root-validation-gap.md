# Root-library validation remains incomplete for the twist research PR

- **Status:** open
- **Kind:** existing implementation regressions and operational validation resource limit.
- **Affected work:** [PR2380](https://github.com/isogeny-labs/crypto/pull/2380), [twist delivery checks](../research/twist_routes_1_3_20261021/DELIVERY_CHECKS_20261027.md).
- **Next action owner:** unassigned library/validation lane.
- **Last updated:** 2026-10-27

## Problem

The verified twist research cannot claim a passing complete root release suite:
four existing ecbench regressions remain and the full run was terminated at its
one-hour operational deadline in an unrelated long isogeny-sieve test.

## Evidence and uncertainty

- **Observed:** `cargo test --release --offline --locked --lib -- --test-threads=1`
  was launched at c9db6fab0d658ab83a0ec6a35e1c050815130968. Compilation took
  18m21s. The run reached
  `cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`
  and was terminated by the explicit operational watchdog at15:53UTC, exit101/
  SIGTERM. This test uses p101,seed1,moves8,jumps3/5/7,cap multiplier3 and stop64.
  The time boundary was applied after observing the long validation run; it is
  not a pre-registered performance result or a passing full-suite verdict.
- **Observed:** isolated replays confirm one weighted-comparison legacy-ID
  assertion and three `ic.hyperelliptic` unknown-`branches` errors. The affected
  comparison/hyper-IC source files are unchanged from main7e271266f.
- **Observed:** three additional failure groups were missing tracked runtime
  fixtures. Restoring exact HEAD records, six T01 parameter files and cited
  READMEs makes the band/defaults/planner replays pass. The sparse helper had no
  applied profile; explicit tracked-file restoration was used, without editing
  fixture bytes. Historical model restoration also passes the complete native
  canonical checker (75 models,34 extension models).
- **Scope:** the root executable was compiled at c9db6fab0; runtime fixture
  materialization and one later catalog-only addition are separate validation
  changes. No current-head complete-suite success is inferred. Native experiment
  package tests and all v8/v9/v10 independent scientific checks pass.
- **Unknown:** why this root sieve test takes this long on this host; neither
  nontermination nor a mathematical obstruction is established. Later tests
  in the interrupted full run have no verdict.

Receipts, original failure probes, repaired-fixture probes and watchdog are in
[the checks inventory](../research/twist_routes_1_3_20261021/execution-v8/checks/.artifacts.json)
and adjacent small logs. Hydrate through the maintained artifact tool to read
large stdout records. Scientific raw archives/protocols remain unchanged.

## What would unblock it

- [ ] Fix the four existing API/comparison mismatches without deleting or weakening their assertions.
- [ ] Diagnose the long sieve test, then complete the original root suite on a pinned head and consistent materialized inputs, recording actual exit and all failures.
- [ ] Complete current-head CI/review; do not infer merge readiness from standalone scientific checks.

Meeting these conditions establishes library validation, not an IC speedup,
NIST result or completion of the original full research campaigns.

## Next discriminating action

Replay/profile the single unchanged sieve test with phase/resource receipts and
an explicit budget. A successful bounded replay permits a full-suite rerun; a
budget exit preserves the resource gap. Fix the independently reproduced four
regressions in their owning modules. Keep the research PR open for review.
