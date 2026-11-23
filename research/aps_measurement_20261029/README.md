# APS measurement instrument

Continuation of four-lead PR #2803 and independent replay PR #2867. Native
method `ic.aps_v1` instruments the existing bounded APS algorithm. No benchmark
has launched. Practical speed/scaling and the complete-cost panel remain open.

One monotonic timeline charges setup and every target-dependent attempt: live
validation, labeled-base sampling, equation/export construction, F4 and roots,
signed checks, modular recovery and independent affine witness verification.
Setup plus five exclusive online phases sum exactly to the cold solver interval.
The online boundary precedes conversion of the live target into Montgomery form.
Failures before a transcript, retries, partial enumeration and deadlines remain
recorded. Final report aggregation/serialization is outside the solver interval; equation
export construction/binding is inside PDP and its overhead is included.

The adapter exposes six existing encodings, exact/legacy roots,
exhaustive/first-informative stopping, random/guaranteed labels and certified
cubic-orbit line pruning. Fields remain capped at 65,521, abscissae at 32 and
attempts at 128. This is not a paper-scale reproduction.

| Requirement | Implemented behavior | Remaining evidence |
| --- | --- | --- |
| Timing | Closed cold and five-phase online wall intervals, failed work included | No admitted L2/A-A measurement |
| Abort records | Each failed retry retains wrapping seed, stage and reason | Discarded draws before a sampling abort are not individually exposed |
| Four leads | Matched constructor/seed controls; certified orbit/pruning parameters | Held-out ladder and frozen protocol incomplete |
| Answer check | Separate ordinary-residue affine checks of labels, signed sum, A/B and scalar recovery | Does not prove missing-root completeness or execution history |
| External input | Actual equations/characteristic, canonical digest, msolve renderer | No external solver execution/output equality |
| Native work | F4/root diagnostics, a counter of missing algebra reports, and counted independent-check group operations | Absent reports leave diagnostic counts unknown; core group/field, polynomial, allocation and hashing work incompletely counted |
| Export/replay | Existing fixed-fixture replay unchanged; native audit classification matches cooperative-deadline nondeterminism | APS counts are not eligible for deterministic native replay; unified export and independent metric/full-set validation open |

`total_gae` prices only counted independent affine checks: a lower bound,
not complete S or bitop/v1. Missing counts are unknown, never invented as zero.
`native_work_accounting_complete`, `costs_attested` and
`complete_sets_independently_audited` stay false. A closed wall interval does
not repair incomplete operation pricing. Development test durations are not
performance evidence. No leaderboard, benchmark or progress chart is promoted.

See [CHECKS.md](CHECKS.md) and [PROTOCOL_DRAFT.md](PROTOCOL_DRAFT.md).
The original [complete-cost panel](../../backlog/XP-20261026-aps-complete-cost-panel/README.md)
and [obstruction record](../../problems/aps-single-relation-complete-cost.md)
remain open. This instrument addresses their timing-support gap; it does not
resolve their measurement, full accounting or independent-audit requirements.

Sources checked: the APS revised manuscript sections 3.1–3.4 and SEC 1 v2.0
sections 2.2.1/3.2.2.1, already primary-source reviewed in the linked
[four-lead](../aps_followup_20261028/README.md) and
[replay](../aps_replay_20261029/README.md) studies; the
[ecbench standard](../../docs/ecbench/README.md) sections 3/6/7/8/11;
sibling atlas LEAD-ICP-algebraic-membership-factor-base; canonical
experiment.protocol/v1 and experiment.export/v1 schemas in
crypto-autoresearcher; maintained EKS harness contract. These require one
informative relation, independent checks, complete intervals, paired controls
and explicit accounting gaps. No new mechanism/degree/scaling claim is made.

T-9 continues T-8 unchanged. During stale-lease recovery the native CLI default
release cancelled T-8; its terminal history is preserved. T-9 holds the actual
implementation scope. No research panel was marked done.

The [editable diagram](instrument.dot), [SVG](instrument.svg) and
[source-linked report PDF](report.pdf) describe the instrument and its open
gates. Rebuild with `./render-report.sh` (Graphviz, LaTeX, Ghostscript and
Poppler); the font-subsetting step checks extracted text equality. Inspect
every final PDF page. No chart is updated because no performance data exists.

Streaming and exhaustive recovery share one validated, immutable recovery
context per attempt, preserving the validation hoisting in `find_relation`.
Every candidate still passes coordinate/sign/nonzero-B/scalar checks; no full
base check is repeated merely because roots are streamed. Unexpected solver,
construction or check errors end the execution; sampling/orbit overlap failures
can retry and remain charged. Valid late witnesses remain evidence and cannot
become timed successes. The development specs include both rho and APS A/A
pairs. Hybrid/cover measurement support remains open.
