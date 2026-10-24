# Four-follow-up next steps

This plan preserves the original 1,260-target requirement and the existing
reporting boundary. It does not reinterpret partial pencil coverage as a full
cohort. New results use the existing standardized target rows, 36-cell cost
grid, matched source-negation rho, independent replay, graphs, and neutral PDF
format.

## Current evidence boundary

| Lead | Current verified evidence | Remaining gap |
| --- | --- | --- |
| factor-base/graph sweep | five arms over all 1,260 targets; 6,300 candidate and 6,300 matched-rho solves | replication is needed before interpreting the 0.999323 full8/full16 difference |
| projective law | wrapper and retained-state arms over all 1,260 targets; 1,224,000 checked additions; exact certificate equalities | a mechanized divisor-class theorem and independent mathematical review |
| point-driven pencil | 543 distinct verified targets, one target with cap-only records, 716 targets without a pencil record | 717 primary and independent-replay target records; a complete aggregate ratio requires verified candidate solves |
| sign-safe Kummer | fixed-difference collector over all 1,260 targets; 8,820 labels and 1,260 signed recoveries checked | an arbitrary-jump construction remains open; the fixed-difference result is complete for its frozen scope |
| review packet | 19 checksum-bound files, 12,850 target-arm rows, 3,067 grid rows, three graphs, standardized CSVs, and a three-page PDF | regenerate only when admitted evidence changes |

## Ordered work

### N1. Freeze the exact open pencil inventory

Generate `open-workloads.jsonl` from the frozen workload archive and subtract
only source-verified candidate IDs. Pin the archive, source revision, generator,
and output digests. Assert set equality for 1,260 unique workload keys, 543
verified keys, and 717 open keys, with no duplicate `(bits,p,target_index)` or
workload ID.

The frozen source is
`research/five_leads_e2e_20261019/sessions_projective_chart_v2.tar.zst`,
SHA-256 `550f7178e30b60db62931a752b0c179beeef7ea99a96cf94486ff8aee631d339`.
It contains 1,260 unique workload identities, each represented by one rho and
two IC records. The only cap-only identity is `(32,186379,0)`, workload
`Wa0fc1e72d7a0`. A read-only construction of the canonical 717-row inventory
produced SHA-256
`16d33685b156b3ba753fff577c11c4958335a4ce80384ca292100bc5bbadda5a`;
the generator, stored output, and regression check remain N1 deliverables.
Because the full JSONL is 371,572 bytes, it must use the repository artifact
store rather than be added directly to Git.

The expected open shards are 177 targets at 28 bits and 180 targets at each of
32, 36, and 40 bits. A mismatch stops the plan before execution.

### N2. Build a non-solving handoff and intake gate

Write an operator handoff that fixes every workload ID and seed, the protocol,
algorithm and parameter digests, resource guards, record schema, output paths,
failure semantics, and primary/replay separation. Generalize the current
nine-curve summarizer so it reads the archive inventory. Test ingestion only
with fabricated public records, including negative fixtures for duplicate or
unknown workloads, seed/hash drift, missing setup, candidate prices on failed
records, unverified rho, incomplete replay, and non-wall replay mismatch.

This assisted session does not run matrix export, exact nullspace, collector,
or replay commands rejected by automatic approval review. The handoff makes an
independently authorized execution mechanically reviewable; it does not count
as execution.

Implement the gate as a standalone, standard-library-only reader at
`research/pencil_sparse_20261022/intake_gate.py`, with a versioned contract,
fabricated `SYNTH-*` workloads, and negative fixtures. Its executable mode is
synthetic-only: it rejects real cohort IDs, absolute or escaping paths,
duplicate or unknown workloads, seed/hash drift, missing setup, non-disjoint
primary and replay processes, incomplete replay, non-wall replay mismatch,
unverified rho, and a non-null candidate price on a failed or capped record.
It emits a `pencil-intake-validation/v1` receipt and cannot invoke a collector,
replay, matrix, nullspace, or recovery command.

### N3. Execute the open cohort under independent authorization

The primary pass returns one retained record for each of the 717 open workload
IDs, including caps and failures. A disjoint replay pass reruns the same IDs in
fresh processes and compares every non-wall field. Shard identity is
`(archive_sha256, workload_id, solver_mode, parameter_digest)`; retries and
alternate arms do not increase distinct coverage.

Resource caps are guards rather than measured costs. Every stopped record must
retain phase and operation counts, coefficient visits, row updates, peak memory,
same-process rho and a null candidate ratio. No value is extrapolated from one
target to another.

### N4. Admit returned evidence and regenerate the review packet

Quarantine returned bundles until checksum, revision, archive, plan-set,
process-receipt, source-scalar, replay, and pricing checks pass. Attempt-complete
means all 1,260 workload keys have primary and replay records, including honest
failures. Solve-complete means all 1,260 candidate and rho scalars verify; only
then may a full aggregate candidate/rho ratio be reported.

For admitted solves, export all 36 price cells and complete cold phase counts.
Regenerate standardized JSONL/CSV, coverage and cost graphs, neutral PDF,
manifest, validation certificate, and the problem record. Failed targets retain
explicit null candidate ratios.

### N5. Add reviewer-pattern outputs from existing verified rows

Without running a solver, derive per-target paired deltas, phase fractions,
model-cluster bootstrap intervals, ECDFs, heatmaps, outlier tables, and exact
break-even surfaces from the current operation counts. Publish the underlying
CSV beside SVG/PNG graphs and a neutral PDF appendix. All 1,260 workload keys
remain visible, including pencil nulls.

The existing rows support 11,586 verified priced target observations, four
failed pencil attempts with null candidate ratios, and 1,260 coordinate-only
Kummer rows that remain outside cost plots. Pencil ECDFs are conditional on the
observed verified subset and show the missing cohort mass. Bootstrap intervals
are labeled descriptive resampling intervals over the 21 frozen curve models.
For the common source-add unit `9M+4S+I`, the exact candidate-minus-rho
break-even surface is
`dM + dS*S + dI*I + dL*N*(9 + 4S + I) = 0`; the generator must reproduce all
36 source price cells before publishing that surface.

### N6. Close the projective-law proof boundary

Pin the proof environment and formalize reduced divisors, branch predicates,
exceptional composition, and the divisor-class bridge used by the runtime law.
Connect the runtime formulas to the eight existing certificates. An exhaustive
small-field reduced-divisor regression is a separate check and is not labeled a
general proof. Independent mathematical review remains a separate acceptance
item.

### N7. Formalize the Kummer sidecar obstruction

State and prove the fixed-offset orbit requirement as a finite group-action
theorem. For each of the 21 frozen prime-order groups and 16 jumps, emit the
minimum offset-orbit sidecar size and its memory/cost lower bound. Label-free
recurrence synthesis and branch checking may be tested separately; no DLP
collection is part of this step.

### N8. Replicate the factor-base comparison

Run preregistered seed-replication arms for full8, full16, and full32 on the
same 1,260 public targets, with matched same-process rho and all 36 price cells.
Report the distribution and interval for full8/full16 rather than treating the
current 0.999323 point estimate as stable. Preserve losses and failed records.

## Completion rule

N1, N2, and N4 can be implemented without solving a new target. N5-N8 improve
reviewability, proof scope, or robustness but do not close the pencil cohort.
The original objective is complete only after N3 and N4 establish the required
1,260-target pencil evidence, together with the already verified full-cohort
factor-base, projective, and Kummer results.
