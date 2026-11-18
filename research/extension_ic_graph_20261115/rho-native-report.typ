#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let display_value(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Rho reference: count the native tower
16 November 2026 · Runpod CPU · Same public walk, two arithmetic price modes

Source #raw(s.source_revision.slice(0,9)), p = #display_value(s.p), seed
#display_value(s.seed). Both public-input workers recover scalar
#display_value(s.actual_scalar), with native and additional elliptic replay
plus held-out agreement. The generator's answer stays outside workers.
[pcc:unverified]

#table(columns:(210pt,120pt,120pt),align:left,inset:5pt,
 [Arithmetic observation],[Harness],[Native tower],
 [Product: Fp products / additions],[55 / 44],[#display_value(s.native_calibration.mul.field_mul) / #display_value(s.native_calibration.mul.field_add)],
 [Inverse: Fp products],[150],[#display_value(s.native_calibration.inverse.field_mul)],
 [Inverse: Fp additions],[104],[#display_value(s.native_calibration.inverse.field_add)],
 [Base inversions per inverse],[1],[#display_value(s.native_calibration.inverse.field_inv)],
 [Observed subtotal (bitops)],[#display_value(s.outcomes.at(0).accounting.observed_primitive_subtotal_bitops)],[#display_value(s.outcomes.at(1).accounting.observed_primitive_subtotal_bitops)],
 [Complete operation total],[Unknown],[Unknown],
)

The actual semantic walk and high-level counters match:
#display_value(s.same_semantic_work), with #display_value(s.steps) steps.
Additional elliptic replay primitive vectors match:
#display_value(s.same_outer_replay_primitives). The copied Rust qualifier
independently reconstructs explicit phase counts, checks inversion once,
binds calibration vectors and recomputes every phase price and subtotal.
These are producer checks without an independent signed verdict.
[pcc:unverified]

The native inversion uses an adjugate formula, whereas the IC harness uses
a different inversion. Multiplication and squaring retain the same source
algebra counts. A const-generic counted instantiation executes the native
source for calibration; production rho removes the extra hooks. Deferred
products/sums use the existing prime-field bitop/v1 convention. These are
model weights, not physical circuit or instruction measurements.
[pcc:unverified]

The explicit rho tally is the source for walk costs; only the separate
additional elliptic replay is added. The whole process hook snapshot is
never added to it. Legacy TUNED multiplication credits stay separate.
Native bookkeeping, RNG internals and tuned-reference pricing remain
open. This corrects arithmetic coverage without changing the algorithm
or reconstructing any earlier experiment. There is no IC comparison,
speed result, full-cost ranking or rho-parity claim. [pcc:unverified]

One public instance and validation round, zero warmups; harness mode then
native mode, negation/batch16/r256 and the same DP policy. CPU0/8GiB/one
claimed thread on the recorded allowed CPU; 60 seconds for fixture, 900 per worker, 1900 whole; no
retries. Walls are practicality observations without A/A intervals.
Exact raw/summary/export/flow/argv/status, copied immutable binaries/source
and preworker/prepost hashes: #raw(e.run_path). [pcc:unverified]
