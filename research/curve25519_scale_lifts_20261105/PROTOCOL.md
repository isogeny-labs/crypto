# Scale-up S3: cofactor lifts (lead 2) and the shared lift system (lead D) at k = 24 and 28 — protocol

Frozen 2026-11-05, before any data in this directory. Component study of one oracle (`pkm-semaev`), unit = F4 elimination multiply-adds (native, unpriced, AGENTS.md section 5a). No rho arm, no distance to rho, no ranking, no leaderboard row, no benchmark row. Every statement is scoped to the measured cells.

## Question

Lead 2 (`research/curve25519_cofactor_algebraic_20261102`, `lifts=1`: one F4 system per cofactor lift) and lead D (`research/curve25519_cofactor_shared_20261102`, `lifts=2`: one shared system with the lift abscissa as a free variable) were measured at k = 20 only. Do their k = 20 cost ratios, solving degrees and per-relation ratios persist at k = 24 and k = 28?

## Instrument (frozen, no code change, no build)

- Binary `/home/user/frozen-bin/curve25519_family_scale`, sha256 `d66b85b0dcb60c2d1b551a2e3ce19de4cc76e6897ce359eafce1bf579606ca90`; the hash is checked by the driver before every run and the driver aborts on a mismatch.
- CLI: `f4cost <k> <form> <t> <G> <targets> <seed> <cap_s> <m> <mode> <lifts> <max_nnz> <trace>` with `cap_s = 600`, `m = 2`, `mode = full`, `max_nnz = -`, `trace = 0`.
- k = 32 is refused by the instrument (tower engine needs p < 2^31), so the scope is k in {20 (reproduction), 24, 28}. If a code change seems needed the study stops and reports.

## Cells, arms, targets, seeds

Lead 2's five cell shapes (form, t, guess G): isogeny t = 4, 5, 6 with G = 0; bits t = 5, 6 with G = 3; at k = 24 and k = 28: 10 cells. Arms per cell: `lifts` 0, 1, 2, run in that order on the same seed (so the same targets), one process at a time.

Requested targets: 200 per cell and arm. The only truncation mechanism is the 600 s per-cell cap (checked between targets by the instrument). This is fixed in advance; the number of targets is not lowered by hand.

Fresh seeds (none used by leads 2, D or the k = 20 cells): `seed = 2026110500000 + 100*k + 50*[bits] + t`, e.g. isogeny t = 4 at k = 24 is 2026110502404, bits t = 6 at k = 28 is 2026110502856.

Truncation rule (generalising lead D's, written before data). After the three arms of a cell: `n* = min(targets_done)` over the arms that produced a JSON. Every arm with `targets_done > n*` is re-run with `targets = n*` and the same seed (label `prefix<n*>`), so that all three arms are compared on the same first n* targets (the target generator is a sequential LCG, so a prefix is the same sequence). The full-length rows stay in the tables. A cell where lifts=2 fails to produce one target within the process limit is reported as failed with its exit code; lifts 0/1 then stand on their own counts and the ratios against lifts=2 are undefined for that cell.

Process limits: `ulimit -v 3500000`, `timeout 1200` (above the cap); `uptime` recorded before every run. Wall times are not compared (shared 4-core box). Runs are local, not isolab-eks, as instructed by the task. The driver is resumable: a run whose JSON exists and parses is skipped.

Reproduction run: k = 20, isogeny t = 4, guess 0, seed 2026111204 (lead D's seed), lifts 0, 1, 2, 200 targets. Compared field by field with lead D's JSONs (`f4_calls`, `f4_muladds`, `hits`, `hits_verified`, `hits_exact`, solving degrees, matrix sizes, columns, coverage counts) - an exact match is expected because the count is deterministic.

Pre-data disclosure: a 2-target smoke run (k = 24, isogeny t = 4, lifts = 1, seed 2026110524, not a protocol seed) was done to check that the binary accepts k = 24; it showed fiber size 8, complete, 5 solving degree, 16 F4 calls (8 per target), mul-adds per target 4182968, covered/projected/all-coset points 10/128/129. It is not in the tables.

## Equal base size

The factor base is a function of (form, t, guess) and k. The analysis asserts equal `columns`, `covered_points`, `covered_points_all_cosets` and `covered_projected_points` across the arms of a cell and aborts otherwise (the file `analysis/base_check.txt` records one distinct signature per cell).

## Metrics (per cell and arm)

- F4 calls per target; F4 multiply-adds per target; max (and mean) solving degree; matrix size (max rows, columns, non-zeros over calls); undecided count.
- Hits, hits verified (`[8]Sum = [8]R`), hits exact (`Sum = R`); `mul-adds per hit` (undefined at 0 hits, never imputed).
- Per relation at exact coverage: `mul-adds per target / (cov/r)` with `cov = covered_points` for lifts=0 and `covered_projected_points` for lifts 1, 2 (the points R with some R+K a sum of two base points); the all-coset count `covered_points_all_cosets / r` is shown beside it as an upper bound (coverage_exact over all cosets).
- Yield gain of lifts=1 over lifts=0: (a) expected, `projected / covered` (the exact ratio of coverages); (b) observed, hits(lifts=1)/hits(lifts=0) (undefined at 0 hits in the denominator). Per-relation ratio lifts=1/lifts=0 = (mul-adds per target ratio) / (a).
- Ratios lifts=2/lifts=1 and lifts=2/lifts=0 on the same targets (full rows where all arms finish, prefix rows otherwise); k-scaling ratios of every metric, k = 24 over k = 20 and k = 28 over k = 24, within a cell shape (cells of different t are never pooled). The k = 20 values for lifts 0, 1 come from the reproduction of lead 2 and D (lead 2's five-cell table, unchanged in the leads' JSONs and read from them, for the shapes other than the reproduction cell).

Hit counts are Poisson and the exact coverage is about 1e-5 to 1e-3, so with 200 targets the expected number of hits per cell is far below 1 at k = 24, 28 (the analysis prints `200 * cov/r` per cell). Hit-based statements are therefore descriptive; the cost side is an exact count.

## Predictions, frozen before data, with falsifiers

- **S0 (reproduction).** The k = 20 isogeny t = 4 rerun equals lead D's counts exactly for all three arms (calls, mul-adds, hits, degrees, matrices). Falsifier: any field differs.
- **S1 (lifts=1 costs about 8x lifts=0).** In every cell at k = 24 and 28, calls per target of lifts=1 over lifts=0 are in [7.5, 8.05] and mul-adds per target in [7.0, 8.5]. Falsifier: any cell outside either interval.
- **S2 (per-relation ratio near 1).** In every cell, per-relation cost at exact coverage of lifts=1 over lifts=0 is in [0.5, 2.0]. Falsifier: any cell outside. (k = 20 values were 0.89, 0.89, 1.10, 1.39, 1.02; isogeny t = 4 at k = 24 has coverage ratio 128/10 and is predicted to land near 0.63 and not to cross 0.5.)
- **S3 (lifts=2 stays far above lifts=1).** In every cell where lifts=2 produced at least one target, mul-adds per target of lifts=2 over lifts=1 on the same targets is above 10 (k = 20: 30.7 to 628). Falsifier: any ratio at or below 10; a ratio below 1 would reverse lead D.
- **S4 (degree rise persists).** Max solving degree of lifts=2 exceeds lifts=0's by at least 2 in every cell where lifts=2 produced a target (k = 20: 8 vs 5, 8, 10 vs 5, 8 vs 5, 8 vs 6). Falsifier: a rise of at most 1 in any cell.
- **S5 (lifts 0, 1 degree does not move with k).** Max solving degree of lifts=0 and 1 at k = 24, 28 differs from the k = 20 value of the same shape (5, 5, 5, 5, 6) by at most 1. Falsifier: a difference of 2 or more.
- **S6 (cost per target of lifts=0 is flat in k).** Mul-adds per target of lifts=0 at k = 24 and 28 is within [0.8, 1.25] of the k = 20 value of the same shape. Falsifier: outside, in any cell.
- **S7 (fiber).** The cofactor fiber is complete with size 8 in every lifted cell. Falsifier: any lifted cell with a different size or incomplete.
- **S8 (hits).** At k = 24 and 28 the lifts=0 arm has 0 hits in all ten cells, and the lifts>=1 arms together have at most 2 hits. Falsifier: any lifts=0 hit, or 3 or more hits in the lifts>=1 arms.
- **S9 (caps, weak because load dependent).** lifts=2 reaches the 600 s cap with fewer than 200 targets in the cells isogeny t = 5, 6 and bits t = 6 (the k = 20 set) at both k, and finishes 200 targets in isogeny t = 4 and bits t = 5. Falsifier: any cell with a different status. A cell where lifts=2 produces no target is reported as a failure and counts as consistent with S3 only in the sense that the limit was hit.

## Classification (decided before data)

No new component improvement is claimed from this study: it re-measures two existing designs at larger k. A cell supports "persists" for a prediction when its falsifier does not hold; any falsified prediction is reported as falsified with its numbers. Relabelling or accounting changes are not improvements. The gap to rho is not reported.
