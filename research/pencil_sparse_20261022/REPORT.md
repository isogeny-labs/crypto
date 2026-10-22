# Exact pencil variable-order and degree-peeling continuation

The artifact manifest has an explicit post-commit amendment: four untracked
Python bytecode cache entries were removed from its index. The amendment
records the original manifest digest and path list. All run records, replay
rows, counters, validation results, and the original run-binary digest are
unchanged; `Cargo.lock` is pinned with the same digest recorded in the
manifest.

The [frozen protocol](PROTOCOL.md) reuses the original public 21-curve,
1,260-target known-answer cohort. This report records the exact point-driven
pencil collector with three fixed sparse-matrix variable orders and a
separately frozen degree-peeling elimination method. Its source transfer,
every accepted three-point Jacobian relation, recovered scalar and
same-process source-negation-rho control use the parent implementation. The
variable-order screen changes the column permutation; the later peel arm
changes only exact relation elimination. The [peeling argument](PROOF.md)
states why degree-one rows can be removed without losing a dependency and
which source checks certify a recovered scalar.

The first completed gate has three p=739/1,151/1,187 curves, 60 targets per
curve and three orders: 540 arm-target pairs over 180 distinct source targets.
All 540 candidate and rho scalars were independently checked on the source
curve; nine fresh archive-input replays matched all 549 setup/target rows
after removing wall-time fields. The native validator recomputed all 19,440
target-level pricing cells, and 0/19,440 have candidate/rho below one.

At `M + 0.5S + 10I`, with 0.1 GAE per native logical operation, the recorded
cold-cost ratios and modular row-update counts are:

| p | Order | Verified pairs | Modular row updates | Pencil/rho |
| ---: | --- | ---: | ---: | ---: |
| 739 | identity | 60 | 6,288,519 | 12.931178 |
| 739 | reverse | 60 | 6,162,526 | 12.831728 |
| 739 | affine | 60 | 6,164,628 | 12.840416 |
| 1,151 | identity | 60 | 20,344,734 | 28.666756 |
| 1,151 | reverse | 60 | 20,783,127 | 29.084432 |
| 1,151 | affine | 60 | 21,253,537 | 29.519296 |
| 1,187 | identity | 60 | 23,112,414 | 31.080921 |
| 1,187 | reverse | 60 | 22,368,648 | 30.439502 |
| 1,187 | affine | 60 | 23,304,945 | 31.282567 |

The table gives complete one-target cold-cost sums for each 60-target slice
divided by that slice's matched rho sum. It includes variable-order logical
work. The [all-cell grid](results/price-grid.csv) and [raw target index](results/targets.jsonl)
preserve individual costs and losses. The [run PDF](reports/run-20261022T0516Z/RUN_REPORT.pdf),
[graph](reports/run-20261022T0516Z/matrix-order.svg),
[native summary](results/summary.json), and [validator](results/validation.json)
give reviewable versions of the same records.

The original pencil study already covered these 180 distinct targets. A
separate p=48,383 identity target was also already covered there. Further
upper-size targets and the requested full 1,260 distinct-target cohort are
listed as open until their raw solves and independent checks complete.
Wall time is a level-0 diagnostic because the runs overlapped other work.

The separately preregistered first 32-bit target at p=186,379 returned a
`sparse_matrix_cap` failure: 69,672 verified relations, rank 69,672,
10,011,236 stored coefficients and 1,233,576,833 modular row updates.
Its same-process rho scalar was source verified. The candidate scalar and
candidate/rho ratio are **null**. The [upper-size PDF](reports/run-20261022T0522Z/RUN_REPORT.pdf),
[graph](reports/run-20261022T0522Z/matrix-cap.svg) and
[raw row](results/p186379-identity-t0.jsonl) retain the unsuccessful run.

The separately frozen degree-peeling alternative solved and source-verified
all 60 p=739 targets, with exact non-wall replay. At the same representative
price, its cold cost was 3,881,713.942857 GAE against 4,321,445.680952 GAE
for the matched online collector and 334,188.080952 GAE for rho. Its paired
candidate/online ratio was 0.898244 and candidate/rho was 11.615357. Across
the 36-cell pricing band, 24 candidate cells were lower than online and 12
were higher; none was below rho. The candidate/online range was
0.873217–1.041611. The [peel PDF](reports/run-20261022T0529Z/RUN_REPORT.pdf),
[36-cell graph](reports/run-20261022T0529Z/peel-price-grid.svg),
[paired grid](results/p739-peel-vs-online.csv) and
[raw candidate](results/p739-peel.jsonl) give the complete observations.

The unchanged degree-peeling method subsequently solved every frozen target
on the other eight original pilot curves. The table pairs each 60-target
peel slice with the original online slice using identical workload IDs. A
ratio below one in the `peel/online` column means a lower complete cold
operation price at the representative cell; the last column retains the
full 36-cell range, including prices where peel was more expensive.

| p | Peel/online | Peel/rho | Cells below online | Cells below rho | Peel/online 36-cell range |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 739 | 0.898244 | 11.615357 | 24/36 | 0/36 | 0.873217–1.041611 |
| 1,151 | 0.796603 | 22.836038 | 24/36 | 0/36 | 0.768822–1.057416 |
| 1,187 | 0.736053 | 22.877204 | 24/36 | 0/36 | 0.704783–1.047696 |
| 3,371 | 0.379821 | 93.480568 | 24/36 | 0/36 | 0.368897–1.023695 |
| 3,527 | 0.393296 | 105.157423 | 24/36 | 0/36 | 0.383283–1.029953 |
| 3,931 | 0.365357 | 121.367036 | 24/36 | 0/36 | 0.357003–1.026616 |
| 11,731 | 0.278124 | 1,204.654657 | 24/36 | 0/36 | 0.277039–1.022758 |
| 11,887 | 0.282829 | 1,090.655477 | 24/36 | 0/36 | 0.281761–1.022072 |
| 12,143 | 0.269806 | 1,146.700226 | 24/36 | 0/36 | 0.268781–1.021602 |

The [nine-curve PDF](reports/run-20261022T0534Z/RUN_REPORT.pdf),
[full-band graph](reports/run-20261022T0534Z/peel-nine-curves.svg),
[native-price graph](reports/run-20261022T0534Z/peel-nine-native.svg), and one
`results/p*-peel-vs-online.csv` file per curve preserve these prices and
their 36-cell losses. All nine degree-peel 60-target slices have exact
fresh non-wall replays. The [native validator](results/validation.json)
independently checked 2,894 source scalars, 1,480 replay rows, 52,056 priced
target cells, 72 deliberately unpriced cap cells, and 432 paired aggregate
cells. It reports `pass` with no missing replays. The
[replay certificate](results/replay-certificate.json) lists every raw/replay
pair, including the three metadata defaults normalized on old online rows.

On the first p=186,379 target, the unchanged peel method also stopped at the
10,000,000-coefficient cap: rank 84,696, peak 10,000,145 coefficients,
18,203,354 row updates and 971,612,198 recorded index operations. The
original online method stopped at the same cap after 1,233,576,833 row
updates. Both rho controls passed source multiplication; both candidate
scalars and ratios are null. The [paired upper-size PDF](reports/run-20261022T0545Z/RUN_REPORT.pdf),
[graph](reports/run-20261022T0545Z/upper-two-caps.svg) and
[peel raw record](results/p186379-peel-t0.jsonl) retain the failed attempt.

The first frozen p=48,383 target solved under all three column orders and
the degree-peeling solver. Two other 28-bit first targets, at p=48,823 and
p=49,451, also solved with online identity order. The representative
complete cold costs are one-target observations:

| p | Arm | Pencil/rho | Modular row updates | Logical index ops |
| ---: | --- | ---: | ---: | ---: |
| 48,383 | online identity | 96,957.133 | 20,814,919,424 | 0 |
| 48,383 | online reverse | 95,449.814 | 20,491,226,949 | 0 |
| 48,383 | online affine | 92,525.430 | 19,863,241,061 | 0 |
| 48,383 | degree peel | 22,644.844 | 1,719,301,303 | 9,411,215,080 |
| 48,823 | online identity | 104,109.643 | 21,197,946,803 | 0 |
| 49,451 | online identity | 73,044.348 | 20,860,404,273 | 0 |

At p=48,383, peel/online identity is 0.233555 in the representative cell
and 0.233492–1.018367 over the 36-cell band, with 24 cells below online
and 12 above. Both methods cost more than matched rho in every cell. The
online reverse/identity and affine/identity representative ratios are
0.984454 and 0.954292; each has 24 cells below identity and 12 equal.
Their full paired grids are in `results/p48383-*-vs-identity.csv`. The
[28-bit PDF](reports/run-20261022T0550Z/RUN_REPORT.pdf),
[graph](reports/run-20261022T0550Z/p28-first-targets.svg),
[36-cell heatmap](reports/run-20261022T0550Z/p28-price-band.svg),
[paired price grid](results/p48383-peel-vs-online.csv), and raw records
retain exact costs. These runs add only the p=48,823 and p=49,451 targets
to the original 541 distinct solved targets; the p=48,383 target overlaps
the original pilot. All six 28-bit first-target raw rows have fresh
field-equivalent non-wall replays with source scalars checked. The
[cohort status](results/status.json) therefore records 543 distinct solved
targets out of the original 1,260, leaving 717 target workloads open. The
failed 32-bit target remains open.

The separately frozen 25-million-coefficient resource extension for that
32-bit target is queued after the completed 28-bit replays. It has no
observed outcome or cost ratio in this report until its run record is added.

The [study validation gate](VALIDATION.json) pins the independently checked
[raw-data validator](results/validation.json), [replay certificate](results/replay-certificate.json),
and matched rho prices at native logical-work prices 0, 0.1, and 1 GAE.
The [source symmetry check](results/rho-symmetry-check.json) computes the
source (j)-invariant for all 15 inspected archived models. Each has
(j^p\ne j) and (j\ne 0,1728), so these source curves have no Frobenius
self-map or extra geometric automorphism fold beyond negation. The gate
passes `scripts/validation_gate.py evidence research/pencil_sparse_20261022`.
