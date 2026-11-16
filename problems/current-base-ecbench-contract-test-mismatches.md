# Current-base ecbench contract tests disagree with their implementation

- **Status:** open
- **Kind:** observed library validation failures; unrelated source contracts
- **Affected work:** [PR2655 source validation](../research/ic_algebraic_crossover_20261023/PUBLISH_REVIEW.md)
- **Next action owner:** ecbench comparison and hyperelliptic method maintainers
- **Last updated:** 2026-10-27

## Problem

Four unchanged library tests fail after exact runtime inputs are restored,
so the current integrated source cannot claim a passing full library gate.

## Evidence and uncertainty

- **Observed:** source974f2e9cd46b and published feature562f7075 have the same
  library tree `b65dd608474242d7bb415218a399340f5fd0ca8f`. The new query source
  changes only examples. The bounded full command exits124 with one unfinished
  JV test; its earlier23named failures include19missing-runtime-input failures.
- **Observed:** restore187tracked runtime inputs plus two audit JSON files
  from their exact Git blobs, then rerun the23failed tests with the same binary.
  The recheck has19passes/four failures. No source, assertion or test budget
  is changed. [Raw recheck](../research/ic_algebraic_crossover_20261023/pipeline-receipts/current-base-runtime-fixture-recheck-20261027-attempt-2/tests.log).
- **Observed:** `weighted_comparison_prices_verified_pairs_and_records_weight_identity`
  fails `zero.legacy_comparison_id.is_none()` at ecbench/compare.rs2448.
  Three hyper_ic tests fail resolution of method ic.hyperelliptic because its
  supplied `branches` parameter is rejected. Exact names and messages are in
  the raw receipt. They are distinct from missing files and the JV watchdog.
- **Derived:** source identity establishes that these are inherited library
  contracts in this feature base. It is not a fresh full run of a separate
  base checkout or a diagnosis of every cause.
- **Suspected explanation:** contract/registration drift; not proved.
- **Missing capability:** agreement between intended comparison/version and
  method-parameter behavior and their unchanged tests.
- **Limits:** own16example tests and scoped proofs pass separately. Neither
  that success nor fixing sparse inputs turns this full command into a pass.

## What would unblock it

- [ ] Resolve the four exact contract mismatches according to intended behavior,
  retaining the recovered-scalar, pricing-identity and applicability assertions.
- [ ] Pass the four tests and the complete unchanged required library command;
  resolve the separately tracked JV runtime without skipping/quarantining tests.

That establishes software validation, not an IC performance/scaling result.

## Next discriminating step

- **Action:** maintainers compare the weighted-ID compatibility rule and the
  registered hyperelliptic parameter contract with their tests before editing.
- **Inputs and controls:** exact source tree above and the same four test names.
- **Execution limits:** development correctness checks; retain current assertions
  and scope. No scientific workload or paid resource is authorized by this note.
- **Outcomes:** resolve actual implementation/test drift or preserve the failure
  with an explicit explanation; do not delete the assertions to obtain green.
- **Recheck trigger:** upstream contract change or new passing source receipt.

## Resolution and history

2026-10-27: recorded after exact sparse-input restoration. Unresolved. The
[separate JV watchdog](jv-nonweak-end-to-end-library-watchdog.md) also remains open.
