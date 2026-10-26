# Base JV end-to-end test exceeds the local library watchdog

- **Status:** open
- **Kind:** runtime/resource bottleneck; cause unknown
- **Affected work:** PR2550 final release-library verification
- **Next action owner:** upstream JV/isogeny-walk maintainer
- **Last updated:** 2026-10-26

## Problem

The required full release library command reached its2,700-second outer
watchdog with one unfinished test. The exact full-suite command did not pass.

## Evidence and uncertainty

- Source d3c9f02c7 incorporates main e8fcadf62; the library tree is unchanged by
  subsequent recovery-metadata/example changes. Build jobs1, test threads2,
  RAYON/OMP threads1, existing shared busy lock. The source now compiles after
  four existing include_str fixtures were materialized from Git.
- The full suite lists4,991tests. Printed completion/ignore records cover4,990
  distinct names; the only unfinished name is
  `cryptanalysis::jv_isogeny_walk::tests::end_to_end_from_a_non_weak_curve_recovers_and_verifies`.
  It calls `run_end_to_end(101,1,8,&[3,5,7],3,1.3,1.25,Some(64))`.
- Exit124 and the full raw log are retained in
  `research/ic_algebraic_crossover_20261023/pipeline-receipts/upstream-fixed-hydrated-library-gate-20261026/`.
  `unfinished-tests.json` records the test-discovery comparison. Its first
  parser omitted expected-panic labels; corrected metadata preserves the actual
  timeout verdict. A nested one-test success is not the full-suite result.
- `git diff origin/main -- src/cryptanalysis/jv_isogeny_walk.rs` is empty.
  The task does not modify this prime-field JV routine. No assertion failure
  or mathematical obstruction is established by the timeout.

## What would unblock it

- [ ] Explain/resolve the unchanged test's runtime on this host, or run it to
  completion under an appropriately recorded source-validation envelope.
- [ ] Complete the required full source gate without skipping, quarantining or
  changing the unrelated test merely to obtain green.

Native recovery examples have their own tests/build/clippy gates. Passing
those does not turn the timed-out global command into a pass or authorize a
merge with failed required CI.

## Next discriminating step

Report the exact pending test and unchanged source on the existing PR. Inspect
the upstream runtime issue before repeating the full command. A focused check
with a recorded bound can distinguish reproduction from aggregate compile/test
overhead; retain any further cap. The research engine uses a separate frozen
native executable and does not infer a performance or security result here.

## Resolution and history

Unresolved. The earlier SAT syntax issue was fixed upstream; this is the actual
remaining full-library runtime failure, not that superseded parse error.
