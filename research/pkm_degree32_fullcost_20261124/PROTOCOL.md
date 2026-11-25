# PKM degree-32 cold full-rank same-target protocol — 2026-11-24

## Question

Does the repeated degree-32 PKM relation-extraction lead survive collection to a checked full-rank joint relation matrix on the same 16 planted targets, and does that complete cold joint solve cross the repository's counted same-target rho reference?

This protocol uses the existing prime-field pipeline's strongest complete mode. `incremental-gauss-full-rank-checked` continues until every one of the 16 factor-base columns and the target column are independent, recovers the planted target logarithm, and point-checks one representative of every factor-base column. It is a cold single-target **joint solve** per repeat: factor-base construction, relation attempts, F4 extraction, dense incremental linear algebra, base-column checks and final target verification are charged. The current prime PKM path does not persist the solved base logarithms or expose a separate saved-database individual-log descent; that missing offline/online bridge is outside this measurement and must not be inferred from it.

## Frozen workload

- Binary: SHA-256 `06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59`.
- Generic prime instance flags: `--bits 13 --family generic --seed 20261149 --repeats 16`.
- Expected instance: `icv1-fp13-t19-0d05ae31`.
- Candidate: `prime-isogeny:t=5,seed=5,aux=ordinary`, five degree-2 Vélu maps composed to degree 32.
- Control: `prime-abscissa:below=32`.
- Both bases must have exactly 16 lifting abscissae and 16 columns.
- Oracle: m=2 Semaev F4, chain mode, degree bound 8. The control uses the exact five-bit interval equation.
- Matrix: `incremental-gauss-full-rank-checked`.
- Targets: the fixed `walk` stream. Every arm must recover the same 16 non-null planted scalars.
- Maximum relation targets: 5,000 per repeat. Every exhausted or failed repeat is retained and fails completion.
- Solver-call cap: five seconds.
- Same-target rho: 16 counted runs, one on each planted scalar. The exact `rho_reference.per_run[].planted` list must equal the recovered scalar list.
- Invocation order: interval, candidate, candidate, interval. Each whole invocation has a 600-second outer wall cap.

## Accounting and gates

Preserve complete commands, stdout JSON, stderr tail, exit status, external wall and child CPU in one append-only JSONL store. Require, per row:

1. verified recovery and equal recovered scalar across all four invocations;
2. `linear_algebra.rank = factor_base.columns + 1 = 17`;
3. `linear_algebra.cost.native.full_rank_reached = 1`;
4. identical instance and factor-base size;
5. null top-level `cost_qualification`;
6. a verified rho run for the corresponding planted scalar.

Report each mirrored pair and combined totals for relation trials, relations, F4 multiply-adds, linear-algebra row operations/GAE, total GAE, row wall, external wall and child CPU. The candidate/control full-rank component gate requires total GAE ratio below 0.9 and both row-wall and child-CPU ratios below 1 in both mirrored pairs. The complete cold same-target crossover gate additionally requires candidate aggregate total GAE below the matched aggregate rho GAE in both candidate invocations. Report failed attempts and censoring without estimating a successful ratio.

The F4 operation unit is currently priced from host measurement rather than a repository-pinned prime-field calibration. Therefore solver operation counts, wall and CPU remain separate primary observables; a passing GAE ratio still requires source-pinned replay and controlled-load confirmation before promotion.
