# Validation contract

- Explicit parameters; plan-time refusal of unsupported policy, field width,
  uncertified orbit and pruning combinations.
- Streaming/exhaustive recovery; legacy/exact preserve seeded bases, equations
  and complete coordinate sets.
- Sampling retries and wrapping seeds retained; invalid live targets recorded
  before any transcript.
- Independent affine checks reject corrupt labels/signs, duplicates and B=0;
  existing producer verification remains enabled.
- Exact cold/five-phase online sums, with live-target conversion inside online
  time. Independent check operations are charged where they occur. Missing
  solver reports leave diagnostic counts unknown; incomplete pricing stays so.
- Export binds actual equations; corruption changes digest. Renderer output
  alone is not an external solver equality receipt.
- Native method/report determinism must agree. A one-round L0 correctness
  scratch session statically audits APS while deterministic rho alone replays;
  it does not supply an APS count-replay certificate or performance result.
- Existing APS/F4/example and ecbench library/integration checks, full release
  library, formatting and strict lint. Retain original logs privately; publish
  normalized verdicts and source pins only.

No new theorem is asserted; existing Lean statements/gaps are unchanged.
No existing assertion or gate is weakened.

Observed boundaries:

- `pre-lint-input-sha256.txt`: 14 adapter, 34 existing APS and 67 F4 passed.
  Public integration passed 3/0, fixture example 10/0 and ecbench integration
  22/0. Strict Rust1.98 lint rejected one manual-divisibility expression.
- `pre-audit-input-sha256.txt`: its equivalent predicate correction passed
  strict Rust1.98/1.99 lint, adapter14/0 and the directly executed public
  integration binary3/0. Cargo's driver was cancelled130 during collateral
  binary builds after audit review; that driver is not a completed pass.
- `pre-query-input-sha256.txt`: the exact-method audit classification and
  fourth integration control passed both strict lint versions. Its next
  adapter build was cancelled130 before tests after the online-boundary
  issue was found. No test pass is claimed for that interrupted build.
- `final-input-sha256.txt` plus `integration-source-sha256.txt`: the final
  online-boundary/missing-diagnostics corrections pass strict Rust1.98/1.99
  lint. Adapter14/0, audit regressions43/0, APS34/0, F4 67/0, public integration4/0
  and fixture example10/0 all completed successfully. See the normalized
  [final summary](validation/final-summary.txt) and original-log digests.

The original broad pre-lint library run finished: 5397passed,6failed,123ignored,
exit101. Its failure output confirms five device and one historical-bounds
failure from absent sparse inputs. Restoring 18 exact
tracked blobs let the six-device subset and historical-bounds control pass
separately. This does not turn the original full run into a pass, and it is
not a final-source or later-head full-suite run. The isogeny case passed; no
library test remains in flight.

Earlier sparse setup exhausted disk before testing; original logs and the
initial 8-pass/1-failure adapter boundary remain preserved. The callback test
fixture was corrected without weakening its rejection assertion. Formatting,
workflow actionlint, report build/text-preservation and visual inspection
passed. Development test durations are not performance evidence.
