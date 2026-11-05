# Independent review: bounded Kummer branch-pruning check

Verdict: supports the packet's bounded software-equivalence claim. Independent rebuild/replay exited0 and produced byte-identical stdout. No scientific workload, collector cohort or performance measurement was run. Review plan was recorded before producer source and results were read. Producer files were not modified.

## Exact checked result

The two compiled source copies differ only in `walk::step`: eager computes both doubles and the mixed addition; pruned computes the mixed addition plus the selected double. Both compute branch choice first. Selected-call arguments and their relative order are unchanged; arithmetic helpers read no operation counters, so removed counter changes do not influence their return values. This source argument is conditional on the executed arithmetic terminating without panic/overflow; the fixed debug execution supplies that evidence for these fixtures, not arbitrary u64 inputs.

The wrapper exercises exactly three ordinary transitions and one forced-zero-difference transition per retained model. The comparison asserts exact `next`, `choice`, `labels`, `fallback`, `fallback_checks`, `transition_checks` and `validation_failed`. The wrapper separately rejects validation_failed and missing successors. Independently recomputed raw output contains84 rows over21 distinct primes:63 ordinary,21 forced fallback. Both output vectors in every successor pair are nonzero. Ordinary choices split33/30; forced choices13/8, so both pruned branches are represented in both families.

Each ordinary transition invokes existing signed Jacobian expected-pair computation and projective comparison. That signed arithmetic is a distinct representation from Kummer xADD, though it shares source dependencies between arms. Forced fallback uses the same unchanged signed reconstruction in both arms; it is not independently reimplemented by the wrapper. The report correctly distinguishes this.

## Accounting and repair coverage

Recomputing eager minus pruned counters from raw rows yields ordinary deltas24m,36a,4i and forced deltas304m,376a,24i uniformly. These are exactly the source's walk counters. The pow loop uses raw_mul without tally_m; native counters therefore must not be represented as a complete field-operation census or complete collector cost. The report states this limit and reports no ratio.

All84 eager repair-root arrays contain onlyNone: zero successful geometric repairs. Forced zero difference remains zero under the linear repair actions, explaining unsuccessful attempts before fallback. The recorded m/a/i difference is consistent with skipping one double plus its unsuccessful repairs. Success of geometric repair, exceptional projective-coordinate behavior and arbitrary-step coverage remain open.

The54-case control enumerates2 choices and3 outcomes in each of3 slots. Its old/new expressions select the same Option pair. It does NOT call arithmetic or verify repair telemetry, despite the initial PLAN mentioning skipped-telemetry logic. The final REPORT accurately limits this control to Option selection. This discrepancy is a retained coverage gap, not evidence of successful repair correctness. Pruned unused slots containNone, which is also used for a selected slot needing no successful repair; an explicit evaluated-slot mask/new candidate identity is still required before interpreting these telemetry fields in a new cohort.

The comparison uses zip without an explicit equal-length assertion. In the inspected wrapper both arms have identical fixed loop bounds and panic on missing successor, and the raw84-row count is verified, so this does not weaken the present fixture result. A later variable-length wrapper should add the length assertion.

## Custody, failure and bounded replay

All38 files listed in the producer manifest matched hashes at inspection. `eager.rs` equals the retained historical main_labelled source plus the appended fixture wrapper. The first compilation's stderr is E0308 in the wrapper's Map key type, before execution. The retained before-fix wrapper differs from the successful wrapper by exactly `key` to `key.to_string()` in deltas.insert. Amendment and before/after hashes match. No arithmetic or fixture change was hidden by this fix.

Independent execution copied src/native/Cargo files and fixtures to a separate crate and target directory, used --locked --offline with timeout120s and kill grace5s, and returned0. Replay stdout was byte-identical to producer run2.stdout. Frozen baseline cohort and production code were not touched. Review does not establish the production build correspondence or cost of an unexecuted full collector.

## Recommendation at component scope

Retain this packet as fixed-fixture evidence for the proposed branch pruning. Before cohort admission, bind a distinct candidate identity, an explicit executed-call telemetry mask, and fixed successful-repair witnesses. Existing timing, full-target and cost-accounting obligations are unchanged.
