# G2: constructor-helper and raw encoder coverage

Source ab0df82d147f39252adde0a280a2b17f4b19f8b7. Original PKM_GOALS acceptance
is unchanged. Results below are unsigned producer observations, PCC-20261106-59b811.

## What now stays with the caller

Disjoint setup parameter/constructor captures and per-oracle parameter captures
retain constructor-helper calls when a Result returns Err. Captures use the
existing current-thread Cell counter; calls between captures are excluded.
The helper tick flags underlying overflow, and checked interval/aggregate
arithmetic emits unavailable (null), not exact wrapped/saturated counts. A
sticky underlying overflow conservatively invalidates later captures. This
qualifies the new capture behavior, not every legacy producer counter.
[pcc:unverified]

The shared encoder does not use these coordinate helpers. A separate ledger
counts raw scalar-n additions/subtractions/remainders, coordinate-p orientation
subtractions/remainders and base-entry/lookup diagnostics. It retains a prefix
on invalid ordinates or missing base entries; any class/call overflow makes
integer counts unavailable without changing the row arithmetic. Moduli and
implementation units remain separate. Map diagnostics do not count internal
comparisons/allocations. No source-call class is a machine-instruction count.

Producer outputs gain an additive relation_encoding_work diagnostic (v1).
Replay recomputes and compares it when present, using the SHARED encoder.
This is consistency checking, not independent counter completeness/proof.
Older records without the diagnostic remain readable; missing producer work
is not thereby free or price-qualified. Replay always reports its own ledger.
Nested replay-work remainsv2 with additive helper/encoder fields; outer replay
remainsv2, request/producer outputsv1. Actual checkpoint remains before input.

Producer and replay are two real executions. Numerically equal encoding counts
must be charged for both executions, not deduplicated. In contrast, the legacy
completed-oracle enumeration view aliases attempted enumeration from the SAME
replay calls: do not add both. Within one helper capture inverse/sqrt diagnostics
overlap expanded multiplications. Current aggregates support an expanded-field
view with inverse/sqrt as diagnostics; an atomic inverse/root view needs child
exclusion attribution not yet emitted. Different implementation/modulus classes
still need separate calibrated weights before any GAE total.

## Actual checks

The full oracle suite passes32controls:5new and27prior,0fail/ignored. New cases
check excluded between-scope work and retained late errors, underlying/helper-
aggregate overflow without changed math, setup partition conservation and
invalid-root refusal, exact positive/negative encoder counts and retained error
prefix/overflow, and tampered diagnostic rejection with old-record readability.
Formatting/CLIbuild0 and actual checkpoint-refusal message+expectedexit1 pass.
Final scoped Cargo lint/supervisor FAIL101 on the same2inherited library errors;
final Cargo example strict-lint remains unqualified. No waiver. [pcc:unverified]

The fresh recipe checks22committed objects plus alltrackedcleanliness before
every phase. Exact phase terminals/UTF8 captures and2newbinaryhashes with stable
copies are in validation/helper-accounting-20261106/receipt.json. Eight unchanged
binaries, old213453-byte library log and exact formal/gate/Mathlib objects were
rechecked. No unchanged standalone/library/legacy/process-prerequisite/Lean runs
were duplicated. Prior failures/caps remain immutable and validate own revisions.
[pcc:unverified]

## Remaining coverage and acceptance

Captures cover selected synchronous current-thread helper calls only. Other
threads, primality/discriminant/raw guards, JSON/binding, uninstrumented
constructor/encoder/container/branch work, counter/serialization overhead,
complete F4/root/logical/native coverage, whole process/memory/persistence and
qualified runtime/resume remain incomplete/unpriced. Result-return retention is
not panic/crash/signal durability or execution attestation. Legacy affine and
other producer overflow contracts remain unqualified. No timer, shared encoder
or signed-looking producer field closes those gaps.

| Goal | Status retained |
|---|---|
| G1 | Partial, multi-curve complete-output qualification open |
| G2 | Partial, new helper/encoding coverage; full pricing/runtime open |
| G3 | Partial, qualified equal-domain external comparisons open |
| G4 | Unrun held-out optimization panel |
| G5 | Unrun scaling/reach panel |
| G6 | Partial, unchanged formal scope; Rust/polynomial/cost bridges open |

No scientific panels; all cost/GAE/scaling metrics null, comparison_admissible=false,
priced_total=null, coverage_complete=false. Full-panel historical card remains
blocked/unfunded/unimplemented. No access/funding/lease/IAM/quota/pool change or
gate bypass. Frozen original-ledger fork remains report-only; required exact-
head checks/reviews and sanctioned reconciliation still gate merging.

Sources reviewed before implementation: G2 audit/reference-stage notes, actual
algebra/encoder/replay paths, ICMS window/unit rules and official Rust thread_local,
Cell and overflowing_add documentation. The same arithmetic is reused; no novel
ECDLP mechanism or performance result follows. Companion PDF/SVG show ownership
because there are no comparable performance values. Canonical IC scoreboard/
progress-timeline, curve and performance views are unchanged: no admitted metric,
curve edge, phase price or reference. Older reports remain preserved.
