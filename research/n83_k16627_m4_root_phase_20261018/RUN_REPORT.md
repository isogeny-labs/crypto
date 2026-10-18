# N83 K=16,627 four-summand root-propagation control

**Run status:** `UNKNOWN_wall_cap`. The frozen positive arm ran from
2026-10-18 05:22:09 to 05:52:09 UTC, exited 124 at the outer 1,800-second
cap, and was not OOM-killed. Its cgroup peak was 2,661,863,424 bytes.
The dependent unpinned arm was not attempted because the checked positive
prerequisite did not complete.

## Question and frozen method

The [preceding complete-witness arm](../n83_k16627_m4_witness_20261018/RUN_REPORT.md)
imported the exact retained signed-Frobenius base, built the full m=4
orbit-domain model and entered SAT, then reached its 1,800-second outer cap
without a solver return. Its phase records could not separate level-zero
root propagation from later CDCL work. The [protocol](PROTOCOL.md) froze the
same object, four summand orbit columns, two intermediate x values and target
x before the solver was modified. The independent, append-only phase file
adds a checkpoint immediately before and after one state-preserving
`Solver::propagate_root()` call.

The primary curve is `icv1-f2m83-tm6151469093347-debefd74`, policy
`public_x_hash`, seed `2026100801`. The exact object BLAKE3 is
`1aa70cc1571f4cb25af42936699378329128a869f2ae8ad35efb375663d4e59d`
and point-set BLAKE3 is
`564c6bb72b346b73112240bb0bddc4ef68b7525f7ef2556d6046e52c32f976e0`.
The base contains 16,627 signed-orbit columns and 2,760,082 distinct
points. All five retained inputs and six compiled source files passed
SHA-256 checks; [the build receipt](BUILD_RECEIPT.md),
[validation ledger](VALIDATION.md), and [exact commands](COMMANDS.md)
pin the source, worker, image and execution envelope. The accepted worker
source is clean pre-stack commit
`43c9850a73f1d960f65ed470fdb8adac981a5ffe`.

The run uses one CPU, a hard 4 GiB cgroup, zero swap, a read-only root,
no network, the pinned arm64 `rust:1.93.1-bookworm` image, 100,000 solver
conflicts, one model, and a 1,800-second outer process cap. An unpinned
arm may start only after a checked SAT positive control.

## Observed phase receipts

The worker selected stored point indices `[0, 166, 332, 498]`. It checked
four distinct summand x values, affine partial sums, a rank-one group row,
and rejection of an `R+G` target tamper before SAT construction.

| Synced phase | Elapsed from worker start | Observation |
| --- | ---: | --- |
| Base import | 35.040 s | exact retained object imported |
| Group preflight | 35.049 s | rank 1, tamper rejected |
| Full orbit model | 35.221 s | 201,598 SAT variables, 5,915,238 clauses, 3,901 XOR rows; model-domain work 160.561 ms |
| Seven x blocks pinned | 35.227 s | target block checked against fixed builder value |
| Root entered | 35.233 s | 0 decisions and 0 conflicts |
| Root returned | 35.427 s | `Fixpoint`; call duration 185.023 ms |
| Ordinary SAT entered | 35.437 s | last durable phase; no `sat_returned` before outer cap |

At the completed root fixpoint the solver reported 89,094 clause visits,
3 XOR passes, 6,806 XOR repivots, 84,549 XOR row operations, 9,047 XOR
row scans, 63,910 propagated literals, zero decisions and zero conflicts.
Its timers charged 2.256 ms to clause propagation and 182.759 ms to XOR
propagation. These are one-arm phase counters, not an external-tool or
complete-index-calculus performance comparison.

The [diagram](root-phase.svg) and rendered [PNG](root-phase.png) show the
four observed stages. The [phase JSONL](positive/worker.json.phases.jsonl)
is the source of the timestamps, memory readings and solver counters;
the [outer outcome](positive/outcome.json) is derived fail-closed from that
file, the [container state](positive/container-state.json),
[host configuration](positive/host-config.json), exact UTC stamps and
cgroup peak by [derive-outcome.jq](derive-outcome.jq). The worker did not
write `worker.json`; stdout and stderr were empty. The [PDF report](RUN_REPORT.pdf)
is a rendered summary of this source-linked account. The raw and derived
files are pinned by [SHA-256 manifest](positive-files.sha256) with a
[successful check](positive-files-check.log).

## Interpretation and open obligations

The initial root propagation completed quickly in this instrumented run.
That makes it an unlikely explanation for the preceding arm's cap under the
same model and underlying propagation code, but the preceding arm has no
root marker of its own. This result does not distinguish one long later
propagation from many decisions, conflicts or restarts. In the current
solver, the internal deadline is
checked after a completed conflict, while the separate outer process cap
applies even if that check is never reached. Because this arm capped without
`sat_returned`, post-root conflicts, decisions, model status,
original-field/model lift, post-SAT group relation and post-SAT rank remain
unknown. The root-stage zero conflicts and decisions do not describe the
later solver stage.

The rank-one row above is the pre-SAT planted group preflight. It is not a
returned SAT relation or a natural-query useful-rank measurement. No
factor-base winner, cold total or matched-rho ratio follows from this
stage control. A complete positive result would require a SAT model,
original-field verification, exact seven-block x match, checked group
lift, rank one and rejection of the tampered target. Only then can a
separately frozen unpinned and natural-query comparison proceed.

After this cap, the next bounded control will emit
completed decision, conflict and propagation checkpoints under a new
protocol; if one propagation call itself exceeds the cap, instrumentation
must enter the CNF and XOR subphases. Any search-policy change remains a
separate measured candidate against this frozen source.
