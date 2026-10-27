# Repository checks at incoming integration baseline

The required full release library suite was run, with its original output in
checks/root-library-v2.*: **4,886 passed, 15 failed, 121 ignored**, exit 101.
Nine failures were missing sparse-checkout runtime fixtures. Those exact
tracked fixtures were materialized; the same built test binary then passed
the bounds module (16), challenge module (12), and claim module (8 passed,
1 ignored), with separate exit-zero receipts in checks/restored-*.

Six incoming integration regressions remain:

* curve_traits: extension registry length 1324 versus 1358; missing
  scalar-Frobenius registry lookup.
* ecbench compare: legacy_comparison_id is Some where the test expects None.
* Three ecbench hyper_ic tests: method ic.hyperelliptic has no parameter
  branches.

`git diff --exit-code 70130b159365c0ecd0db2545e2bfd5ea8cb28fa8 HEAD -- src
docs/curves` returns zero. These library/data paths match the incoming PR
integration baseline; this study does not change them or weaken their tests.
The full library suite is **not green**. The focused native experiment package
passes its release tests separately. No full-suite success or merge readiness
is claimed.

The later full experiment-package check also retained one intermittent legacy
Hilbert test failure: its expected count of cheaper examples is at least 20,
but that process recorded 17. The exact isolated test and the full serial
package replay both pass, including all 123 main tests and the new export/date
test. Its cause is not established; no concurrency explanation, threshold
change, test skip or solver modification is claimed. Raw checks are
runner-tests-post-export, hilbert-baseline-isolated and runner-tests-serial.
