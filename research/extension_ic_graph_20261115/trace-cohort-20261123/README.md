# Full saved-cohort comparison of traced coordinate backends

The native analysis reads every one of the3415saved Run51systems, checks all3415coordinate root sets containing
3419roots across original-unit/five-trace/six-trace, and totals only
the emitted F4and linear-stage Fp multiplication counters. It makes zero new
solver, natural-yield or timing calls. The initial saved-counter-summary.json
is retained; proof-bound-counter-summary.json adds explicit embedded replay
proof bindings without changing any reported total. [pcc:unverified]

| Path | Reported F4 | Reported linear | Sum of those counters |
|---|---:|---:|---:|
| Unit without trace |2179269070|716225084|2895494154|
| Unit with trace |1097651810|716225084|1813876894|
| Six with trace |1065026764|686480526|1751507290|

The six-coordinate trace has the lower reported subtotal on this complete
captured cohort. Keep it as a serious comparison arm. Elimination to five
coordinates is exact, but fewer variables alone do not establish a cheaper
solver. Producer data: unsigned PCC-20261123-400f7d. [pcc:unverified]

The tracked RREF used to learn an F4trace is missing from field_ops in the
pinned engine. Its events were not recorded in these JSON rows, so the sums
cannot recover its cost. The existing Nov18descent-finder study already identifies
that gap and provides its own bound. That study's six-trace counter totals agree
with these, but its bound is not applied to this binary/five arm and no signed
validation follows. Other graph/setup/reduction/RNG/native-control costs remain
outside these subtotals. No phase-complete, whole-pipeline, wall or parity claim.

A source audit also distinguishes ordinary internal replay divergence from the
invalid-row-index early return carrying trace_mismatch. Both fixed captured
controls match all expected roots; they do not certify every future exceptional
shape. Before broadening the worker's domain, independently test the report
acceptance/fallback policy. Do not silently upgrade the fixed-cohort result to
a generic completeness theorem.

Native source: ../trace_cohort_summary.rs and ../trace_cohort_report.rs. Three
negative/overflow/canonicality tests and strict scoped Clippy pass. The analyzer
checks immutable raw SHA256s, exact record order/denominator, status flags, full
coordinate multisets and replay/mismatch counts; all totals use checked u128.
Builds link only the native JSON/hash dependencies; no cryptographic solver is
invoked by this program. Original failures, binaries, records and earlier reports
are unchanged. [pcc:unverified]

[Root-set wording clarification](ROOT_COUNT_CLARIFICATION.md) ·
[Readable report](report/REPORT.md) · [PDF](report/report.pdf) ·
[Source review](SOURCES.md) · [Analysis plan](PLAN.md).

The requested Runpod comparison remains blocked by provider funds and checkpoint
upload. No new paid allocation/call occurred. Next: include learning work and
certified fallback, then compare both traced arities with existing IC/rho under a
separately frozen native isolated cold protocol. Current root/benchmark and
artifact-store gates remain open; no benchmark/leaderboard row is promoted.
