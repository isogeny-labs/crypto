# Rule v4: a recipe against rule v3's, paired, on Table A's six sizes

**Verdict: PARTIAL, by the frozen rule. One of six sizes improves (`n` = 57, cold
cost 0.953 [0.904, 0.998] of the v3 recipe's); the other five are not separated
from 1. That is not a Table A improvement (the rule asks for four).** Class:
**engineering at `n` = 57; a null result elsewhere.** The free-relation lever of
the task, implemented in this pipeline, **costs 5 to 9% more** and is not in the
v4 arm. **The distance to rho, with rho priced in the same process at the
canonical batched step, is 9.9 to 43 times (cold, index calculus over rho),
1.76 to 1.94 times the as-built figure rule v3 used; the v4 recipe moves it by
-4% to +3%.** A null result for the recipe, measured with its controls: the v3
recipe is within a few percent of the best the grid shows for the cold objective.

Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen before any timed row, with eight
amendments (A1 to A8; table below; [`FROZEN.txt`](FROZEN.txt) has the digests).
Report: [`report/report.pdf`](report/report.pdf); graphs
[`report/paired-cold.svg`](report/paired-cold.svg) and
[`report/gap-to-rho.svg`](report/gap-to-rho.svg) (drawn by
[`report/plot.py`](report/plot.py) from [`analysis.json`](analysis.json));
replay: [`replay-certificate.json`](replay-certificate.json). Benchmark copy:
[`benchmarks/20261024-table-a-rule-v4/`](../../../../benchmarks/20261024-table-a-rule-v4/).

## What this is

The single-target comparison of rule v3 (Koblitz `K_0`/`K_1`, `n` = 41, 47, 53, 57,
59, 61; `ic price --single-target`: one unseen public point, the index calculus
against Pollard rho on that point, set-up included, one `isolated_bench` process
per row), re-run with **two recipes of the index calculus in the same binary, paired**:

- **`v3`**: rule v3's parameter files, unchanged;
- **`v4`**: the same with K (columns), the collection window and the unit size
  retuned on training seeds disjoint from the evaluation (below).

32 pairs a size, on **16 recipe seeds** (201 to 216; rule v3's single seed 201 is
one of them) and the first 32 targets of rule v3 (`public_hash_seed` 23001 to
23032), arms alternating order, plus an A/A (the v3 recipe again, 8 pairs a size)
and a positive control (the v3 recipe with the free-relation lever on, 8 pairs a
size). 485 clean timed processes and 8 contended attempts that were re-run, no
row missing; all 493 replayed outside `ic`, every logarithm equal to rule v3's
for its target; 24 untimed re-runs agree. Host: one L0 shared VM, Intel Xeon
2.80 GHz with AVX-512 F and BW and **no GFNI or VBMI**, 4 vCPUs, kernel
6.18.44-fc-v153; the binary is this tree's `ic` (not v3's `995ea207`), so a kernel
gain since v3 is in neither arm's ratio. Ran 2026-10-24: training 05:46 to 07:02,
panel 07:55 to 10:28 UTC.

## Results

**Cold cost of one complete solve** (set-up plus the online interval, in each
process's own batched-addition unit), **v4 over v3**: ratio of sums over the 32
pairs, 95% cluster bootstrap over the 16 seeds. Verdict by the frozen rule:
IMPROVEMENT when the interval's upper end is below 1.

| n | log2 r | pairs | **cold, v4 / v3 [95%]** | verdict | set-up | online | cold, least repetition | wall | A/A (v3 again / v3) | lever control / v3 | Table A instance (seed 201) |
|--:|--:|--:|:--|:--|:--|:--|:--|:--|:--|:--|:--|
| 47 | 36.6 | 32 | **1.007 [0.960, 1.056]** | NOT_SEPARATED | 0.999 [0.951, 1.048] | 1.42 [1.13, 1.79] | 0.993 [0.939, 1.051] | 0.991 [0.908, 1.082] | 0.963 [0.906, 1.021] (8 pairs) | 1.100 [1.043, 1.150] | 1.104 (2 pairs) |
| 57 | 38.0 | 32 | **0.953 [0.904, 0.998]** | IMPROVEMENT | 0.951 [0.901, 0.995] | 1.12 [0.99, 1.33] | 0.917 [0.860, 0.971] | 0.962 [0.883, 1.034] | 0.972 [0.907, 1.029] (8 pairs) | 1.130 [1.030, 1.261] | 0.966 (2 pairs) |
| 41 | 39.0 | 32 | **0.963 [0.910, 1.014]** | NOT_SEPARATED | 0.965 [0.912, 1.018] | 0.71 [0.44, 0.99] | 0.939 [0.858, 1.022] | 0.966 [0.919, 1.015] | 0.978 [0.929, 1.032] (8 pairs) | 1.114 [1.058, 1.172] | 1.104 (2 pairs) |
| 53 | 44.3 | 32 | **1.010 [0.964, 1.060]** | NOT_SEPARATED | 1.009 [0.961, 1.061] | 1.26 [0.98, 1.76] | 1.015 [0.952, 1.082] | 0.988 [0.955, 1.025] | 0.947 [0.839, 1.074] (8 pairs) | 1.141 [1.050, 1.255] | 1.242 (2 pairs) |
| 59 | 44.5 | 32 | **1.013 [0.966, 1.061]** | NOT_SEPARATED | 1.013 [0.965, 1.060] | 1.10 [1.00, 1.24] | 1.013 [0.948, 1.077] | 1.021 [0.970, 1.077] | 0.961 [0.823, 1.115] (8 pairs) | 1.153 [1.021, 1.297] | 1.196 (2 pairs) |
| 61 | 47.2 | 32 | **0.966 [0.917, 1.018]** | NOT_SEPARATED | 0.967 [0.918, 1.019] | 0.69 [0.51, 0.89] | 0.987 [0.910, 1.065] | 0.952 [0.921, 0.987] | 1.019 [0.928, 1.115] (8 pairs) | 1.122 [1.029, 1.227] | 1.188 (2 pairs) |

- The positive control reads above 1 at all six sizes and **1.130 [1.090, 1.172]**
  pooled over its 48 pairs (the training pairs said 1.05 to 1.09): the panel sees
  a 10 to 15% slowdown. **The detector is demonstrated** (frozen condition: pooled
  interval above 1 and at least 5 of 6 sizes above 1).
- Every A/A interval contains 1 (0.947 to 1.019); five of six point estimates are
  below 1 (a mean near 0.97, standard error about 2%): noise, not an order effect
  (odd and even pairs read 0.980 and 0.966). No size is UNRESOLVED.
- **The Table A instance itself (seed 201, 2 pairs) is worse under v4 at five of
  six sizes** (1.10, 0.97, 1.10, 1.24, 1.20, 1.19 at `n` = 47, 57, 41, 53, 59, 61): one
  seed's collection path is luck of a relation or two (the per-seed ratios of the
  panel run from 0.70 to 1.24 across the sizes). A v4 row on rule v3's seed alone
  would not be an improvement; the panel's claim is about the recipe.
- Pooled over all 192 pairs, v4 over v3 reads 0.985 (standard error 1.0%): the
  retune is worth about 1.5% on average on this host, concentrated where the fit
  predicted the most.

## Distance remaining to rho (the figure the success criterion asks for)

`S` is cost over `sqrt(r)`, in the process's own unit; the gap is index calculus
cold over rho cold, **rho in the same process on the same point**. Above 1,
rho is cheaper. Rho at the **canonical step** (one batched addition plus the table
canonicalisation, measured in that process) is the primary price; "as built" is
rho's measured walk, and the last column the strongest rho available (the faster
per step of that walk and the strong fixture).

| n | S_IC cold v3 | S_IC cold v4 | S_IC set-up v3 / v4 | S_IC online v3 / v4 | S_rho cold, canonical step | S_rho cold, as built | canonical step (units) | rho units / step as built | **gap v3, canonical** | **gap v4, canonical [95%]** | gap v3, as built | gap v4, as built | gap v4, strongest rho |
|--:|--:|--:|:--|:--|--:|--:|--:|--:|--:|:--|--:|--:|--:|
| 47 | 8.848 | 8.912 | 8.673 / 8.662 | 0.1758 / 0.2491 | 0.9015 | 1.6383 | 3.77 | 9.2 | **9.89** | **9.89 [9.22, 10.65]** | 5.55 | 5.44 | 5.68 |
| 57 | 8.531 | 8.133 | 8.428 / 8.017 | 0.1033 / 0.1160 | 0.8842 | 1.5506 | 4.33 | 10.0 | **9.56** | **9.20 [8.58, 9.89]** | 5.42 | 5.25 | 5.91 |
| 41 | 10.741 | 10.345 | 10.657 / 10.286 | 0.0843 / 0.0597 | 0.6243 | 1.2057 | 3.75 | 8.1 | **17.01** | **16.57 [14.59, 19.12]** | 9.05 | 8.58 | 8.74 |
| 53 | 14.725 | 14.875 | 14.662 / 14.795 | 0.0629 / 0.0793 | 0.5474 | 1.0846 | 4.00 | 8.2 | **26.33** | **27.17 [24.29, 30.22]** | 13.59 | 13.71 | 13.71 |
| 59 | 14.023 | 14.211 | 13.958 / 14.139 | 0.0653 / 0.0719 | 0.5859 | 1.1083 | 4.33 | 8.8 | **24.08** | **24.25 [19.91, 30.49]** | 12.82 | 12.82 | 12.84 |
| 61 | 21.968 | 21.228 | 21.902 / 21.183 | 0.0657 / 0.0451 | 0.5040 | 0.9866 | 4.39 | 8.7 | **43.46** | **42.12 [37.09, 49.28]** | 23.41 | 21.52 | 21.79 |

**Against rule v3's figures** (cold, rho as built, 5.23 to 20.97) the as-built gap
here reads 5.4 to 23 at the v3 recipe (same-binary rho, a host without GFNI), and
**at the canonical step the gap is 9.9 to 43.5, 1.76 to 1.94 times the as-built**:
the understatement validation sweep A found (1.67 to 1.96 times) is reproduced.
Neither recipe closes any part of it: the v4 gap at the canonical step is
9.89, 9.20, 16.57, 27.17, 24.25 and 42.12 at `n` = 47, 57, 41, 53, 59, 61.

**Admissibility of the reference** (rule v2's check, kept): `ParallelRho` against
the strong fixture on T01 to T08:

| n | ParallelRho ns/step (T01-T08) | strong fixture ns/step | parallel / strong | admissible |
|--:|--:|--:|--:|:--|
| 47 | 122.9 | 120.1 | 1.02 | false |
| 57 | 140.5 | 116.7 | 1.20 | false |
| 41 | 120.4 | 111.8 | 1.08 | false |
| 53 | 119.0 | 158.5 | 0.75 | true |
| 59 | 129.9 | 141.0 | 0.92 | true |
| 61 | 142.4 | 131.5 | 1.08 | false |

`ParallelRho` is slower per step than the strong fixture at four sizes (1.02 to
1.20) on this host, where its 16-lane canonicalisation has no GFNI; and its
processes at `n` = 53, 59, 61 and half of 41 ran with neighbours' jobs active, the
fixture rows without, which lowers its apparent speed there. The strongest-rho
gap column takes whichever is faster per step. The canonical-step gap is a price
and does not depend on either.

## Section 5a

| n | gap v4 (w = 0 / 0.1 / 1) | v4 / v3 counted (w = 0 / 0.1 / 1) | order holds across the band |
|--:|:--|:--|:--|
| 47 | 17.88 / 18.84 / 23.14 | 1.092 [1.039, 1.145] / 1.101 [1.049, 1.155] / 1.136 [1.084, 1.189] | false |
| 57 | 18.30 / 19.36 / 24.08 | 1.004 [0.943, 1.059] / 1.013 [0.952, 1.069] / 1.045 [0.982, 1.102] | false |
| 41 | 25.96 / 27.56 / 34.62 | 0.888 [0.840, 0.935] / 0.900 [0.851, 0.946] / 0.939 [0.890, 0.987] | true |
| 53 | 37.01 / 39.79 / 52.19 | 1.079 [1.048, 1.116] / 1.078 [1.046, 1.114] / 1.072 [1.041, 1.108] | false |
| 59 | 35.73 / 38.41 / 50.43 | 1.099 [1.062, 1.142] / 1.098 [1.061, 1.141] / 1.094 [1.056, 1.137] | false |
| 61 | 46.98 / 50.40 / 65.70 | 0.782 [0.762, 0.801] / 0.779 [0.759, 0.798] / 0.769 [0.750, 0.788] | true |

Counted group-addition equivalents (method below) are a **lower bound** and rank
the recipes differently from wall time: at `n` = 47, 53, 59 the v4 recipe counts
8 to 10% **more** (a smaller table needs more trials) while its wall time is the
same; at `n` = 41 and 61 it counts 6 to 22% **less** while its wall-time interval
contains 1; at `n` = 57, where the wall time is below 1, the counted ratio is
1.00 to 1.05. The counted table prices a table's build at one addition a pair
where a build costs about 9 (cache misses), so it favours bigger tables. **By
§5a no size ranks the two recipes:** the counted band and the wall time agree on
a direction at none of the six (n = 57: wall below 1, counted not; n = 41 and
61: counted below 1 at all three weights, wall interval containing 1; the rest:
counted above 1, wall interval containing 1). Rho is above both recipes at all
three weights at every size (gap 17.9 to 65.7).

## The recipe

| n | columns v3 / v4 | window v3 / v4 | unit trials v3 / v4 | predicted v4/v3 (training fit) | collection trials, seed 201, v3 / v4 | relations collected v3 / v4 |
|--:|:--|:--|:--|--:|:--|:--|
| 47 | 56 / 48 | 164 / 262 | 164 / 164 | 0.944 | 2460 / 2460 | 56 / 49 |
| 57 | 64 / 54 | 228 / 365 | 181 / 181 | 0.979 | 3801 / 2896 | 65 / 57 |
| 41 | 80 / 96 | 205 / 328 | 623 / 623 | 0.960 | 9968 / 6853 | 86 / 98 |
| 53 | 264 / 224 | 874 / 874 | 1014 / 254 | 0.992 | 16224 / 25146 | 267 / 224 |
| 59 | 256 / 218 | 944 / 944 | 935 / 935 | 0.950 | 15895 / 21505 | 256 / 230 |
| 61 | 320 / 464 | 1220 / 732 | 3504 / 876 | 0.890 | 66576 / 68328 | 336 / 469 |

(columns, window and unit size at the values of [`recipe.json`](recipe.json); the
lever is off. "Predicted" is the training fit's prediction, not a result. The
collection trials and relations are seed 201's own counts from the panel.)

## How the recipe was chosen, and how the rule changed on the way

[`PROTOCOL.md`](PROTOCOL.md) was frozen before any timed row ([`FROZEN.txt`](FROZEN.txt)
holds the digests of it, the grid, the harness and the binaries at each step).
**Eight amendments were made, each written down before the rows it touches
(`FROZEN.txt` has the digests); A1 to A4 after training data had been seen, A6 to A8
during the evaluation.** They are listed because they decide what the v4 recipe
is and how the controls read, not because the verdict depends on them: the
verdict is read from 32 pairs per size on 16 recipe seeds that no step of the
selection saw. A6 and A7 were decided after seeing control rows (and, for the
first five pairs of `n` = 47, the v4 over v3 ratios); neither can move a
verdict (see the table).

| step | what | why |
|:--|:--|:--|
| frozen | best of the 24 lever-on grid configurations, 3 training seeds, accepted at 2% | the rule as first written |
| A1 | a candidate must repeat on 6 fresh training seeds (304 to 309) | the same configuration reads 7.8 to 9.8 `S` across seeds; the best of 24 is mostly luck |
| result of A1 | **0 of 5 candidates confirmed** (cost ratios 1.014, 0.961, 1.020, 0.979, 1.023; `n` = 41 had none) | [`training/summary-stage1-and-confirmation.json`](training/summary-stage1-and-confirmation.json) |
| A2 | the lever is off; the retune is the least-predicted grid configuration of an OLS fit over the 72 lever-off runs a size, taken if predicted at most 0.97 with a two-sigma bound below 1 | the lever costs 5 to 9% (below); the fit smooths the noise |
| result of A2 | **no size taken** (predicted 0.89 to 0.99, standard errors 3 to 7%) | `training/model.json` |
| A3 | the same, gated knob by knob | took only `n` = 41 |
| A4 | **no gate: the least-predicted grid configuration is the v4 recipe** | the panel of 32 pairs is the powered test (about 3%); training cannot resolve a 5% gain at 10 to 14% noise a run |
| A5 | four retries, holes allowed | a quarter of this host's isolated runs are contended by neighbours |
| A6 | the positive control is the lever, not 0.6 x columns | the first control was not slower at `n` = 47 (cost is flat in K there): decided after 5 pairs of `n` = 47 |
| A7 | the A/A gate is "interval contains 1", not "point within 3% and interval contains 1" | eight pairs' noise is 3%, so the old clause fails by chance about a third of the time: decided after the A/As of `n` = 47 and 57 were read (their verdicts were NOT SEPARATED either way) |
| A8 | the rest of the panel runs with `isolated_bench --max-other-cpu 3.2` | other sessions' jobs outside the lock (1.5 to 2.9 cores of `dreg_closure`, 1 to 2 of `solve_bench`) made the default refuse every start; `n` = 47, 57 and the first pairs of 41 are in the clean regime, the rest in the loaded one (`neighbour_cpu_per_wall` in `analysis.json`: median 1.5 to 2.3 CPUs against 0.03) |

## Lever (i): free relations in the pair-table pipeline. Measured; a net loss; off.

What was built (additive, off by default; `KIC_FREE_RELATIONS=1` in `ic price`):
the partitioned fold build carries each pair's place and hash through its
counting sort and finds pairs of stored sums with one canonical key; each
resolves to four base points that sum to `O`; the solver re-checks it in the
group and takes it as a row with right-hand side 0. Every relation was
accepted (6,187 made, none rejected), every logarithm verified, in all 510 runs
that ran it (462 training runs and the panel's 48 lever-control processes).

Its yield is small, because a 4-term relation shows as three collisions and the
count goes as `N^2 n / 3r`: 2 to 11 relations a solve at the v3 columns,
against 58 to 327 needed, and it cuts collection trials by 0.5 to 9% there
(`training/lever.json`: 72 pairs a size, the same configuration and seed with
the lever off and on). **Its cost is larger than its saving.** The harvest
makes the build phase 1.4 to 1.55 times slower (a third and fourth array
through the counting sort, and a pass over the buckets), and on the cold cost:

| n | lever on / off, cold (72 pairs, geometric mean) | standard error (log) |
|--:|--:|--:|
| 47 | 1.062 | 0.016 |
| 57 | 1.058 | 0.018 |
| 41 | 1.056 | 0.019 |
| 53 | 1.078 | 0.021 |
| 59 | 1.088 | 0.019 |
| 61 | 1.053 | 0.023 |

So the lever is **5 to 9% more cost on this host at every size**, at every
column scale the grid covers (it saves up to 22% of the trials at 1.45 x the
columns, where the free count grows as K^4, but that is not where the cost is
least). It is not in the v4 arm; the evaluation measures it again as the
positive control. The free-relation lever of
`research/ic_improve_20261015/02-freerel-production/` (0.960 x pooled) is a
different pipeline: S3 root tables with K = 100 to 500, where one complete
solve costs 59 s at `n` = 61 against 3.7 s here, so neither its K retune
(2.2 x at `n` = 41) nor its 0.960 transfers to Table A's pipeline by
arithmetic. Not pursued further because even a free harvest caps the lever at
the trial cut above (0.5 to 9% of the collection, so under 7% of the cold
cost), and a harvest at about a tenth of the build (detecting inside the
scatter on the line it has just written, ids only for candidates) would net
about 0 to 3% at v3's columns. A named follow-up, about a day.

## Levers (iii) and (iv): not ported, with reasons

- **(iii) the blocked pair-table builder** (`research/ic_oracle_setup_blocked_20261020`)
  is for the prime-field `ic.pipeline` oracle: entries packed in a word, `p < 2^32`.
  The Koblitz folded table is already built that way (rows keyed once and
  counting-sorted into 256 cache-sized partitions, the presence filter filled with
  prefetching, `fold_partitioned`). Nothing to port.
- **(iv) the n = 83 ratchet** (`cryptanalysis/experiments/koblitz-n83-*`):
  `lift_scan` canonicalisation and the rank finder's linear run finder are, in
  Rust, the 16-lane least-rotation of `koblitz_fast.rs` (AVX-512F, funnel shift
  where VBMI2 exists) and its scalar run finder; the index build, the rank finder
  and the guided rank stage belong to the S3 root-index pipeline
  (`koblitz_orbit_dlp_fast_online`, K = 255 to 600), which `ic price` does not
  run. **Named follow-up:** a third solver in `ic price` for that pipeline, 3 to 5
  days; it is not expected to help Table A below `n` = 61 (one solve costs 16 x
  more there), and would be priced against the pair-table pipeline first by
  running the compact-orbit example on the same 32 pairs.
- **Free relations in `ic workflow`:** moot while the lever is a loss.

## The reference

Rule v3 priced rho "as built" (9.7 to 11.8 units a step against a canonical
4.6 to 5.4: validation sweep A). Here both arms of every pair, and the
reference they are read against, come from the same process:

- **Rho** is `ParallelRho` (64 lanes in lockstep, signed-Frobenius canonical
  form, distinguished points), in the same `ic price` process as the index
  calculus, on the same point, with the same unit (one batched addition,
  measured before and after every repetition).
- **Canonical step**: its counted walk operations times one batched addition
  plus the table canonicalisation, measured in that process. This is the
  primary price; it is a price of the operation, not of an implementation, so
  it does not depend on which rho ran.
- **As built**: its measured walk, and, next to it, the **strongest rho
  available** (the faster, per step, of that walk and the strong fixture
  `koblitz_rho_fixture ... strong`, the optimised strong rho of
  `research/ecbench_rho_optimized_20261014`, on T01 to T08 of each size, isolated).
- **This host has no GFNI or VBMI**, so the 16-lane canonicalisation of rho's
  "matched" path (rule v8) is not available: rho's as-built step here is 2.2 to
  2.4 times the canonical one, as in rules v3 and v7, not v8's 1.3 to 1.7.
  The canonical-step gap does not depend on it.

## §5a

The unit of the primary figures is wall time in batched additions, for both
arms in one process, so no native operation is unpriced in them. The band
column re-prices the same runs from the counters the processes emit, at 0,
0.1 and 1 group-addition equivalent per unpriced native operation:

- charged, index calculus: the build's stored pairs and the scan's subtractions
  (one addition each), descent trials, and every scalar multiplication (a probe's
  `[a]G`, the cofactor multiplications, the target) at 1.5 n additions;
- unpriced, index calculus: one canonical key and one presence-filter probe for
  each stored pair, scanned summand and descent trial, and a table insert for each
  stored pair (the linear algebra's operations and the descent's bookkeeping
  are not emitted: **a lower bound**);
- rho: its walk, setup, start, look-ahead and verification additions and its
  scalar multiplications at 1.5 n; unpriced, `canonicalisations_uncharged` and
  twice its distinguished-point stores.

Two arms are ranked only where the order holds at all three weights.

## What ran, and what was interrupted

[`drive.log`](drive.log) records each lock hold. Every timed process ran under
`flock heavy.lock`, in chunks of at most 25 minutes, through
`isolated_bench run --wait --cpus 2`. **Interruptions:**
- chunk 3 of the panel was aborted at `k0n41/T03-v3b` after 60 refusals
  of 15 s: other agents' processes (`dreg_closure`, `solve_bench_final`),
  which did not hold the lock, used 2.5 cores. The harness now gives the lock
  back after 20 refusals and is re-queued (exit code 3, resumable), without a
  row being lost;
- the lock queue itself: this study waited for the lock for 10 to 25 minutes
  between chunks.


## Next

1. **A cheaper harvest for the free-relation lever**: detect inside the counting
   sort's scatter (on the line just written), keep ids only for candidate
   pairs; expected harvest overhead about a tenth of the build instead of 45%.
   Worth about 0 to 3% at v3's columns, more at 1.2 to 1.45 times the columns
   (free count grows as K^4), where the K retune says not to go. About a day, and
   only if something else frees the 3%.
2. **Retune where the data point**: the fit puts the optimum at the grid's
   edge at several sizes (0.85 x columns at `n` = 47, 53, 57, 59; 1.45 x at 61); a
   second grid beyond the edge, with 12 or more training seeds, resolves a 3 to
   5% gain. The panel here shows the recipe gain is at most about 5% at any size.
3. **The cold cost is the collection** (65 to 82% of it), a window scan at about
   40 ns a summand on this host (the pair table's cost model says 2.5 additions): cut the scan
   itself (the presence-filter probe and the canonical key), not the recipe. That is
   where a 2x lives; it is kernel work, not a retune.
4. **A GFNI host** for the same panel: rho's matched path and the index calculus's
   canonicalisation both change there (rule v8's 1.3 to 1.6 rho gain).

## Files

| file | what it is |
|:--|:--|
| [`PROTOCOL.md`](PROTOCOL.md), [`FROZEN.txt`](FROZEN.txt) | the rule, with amendments A1 to A8; digests at each step |
| [`analysis.json`](analysis.json) | every figure above: `ic_rule_v4 analyse` |
| [`recipe.json`](recipe.json), [`training/`](training/) | the selected recipe; the grid, stage-1 candidates, confirmation, the model fit (`model.json`), the lever's paired effect (`lever.json`); raw runs in `training/raw.tar.xz` |
| [`replay-certificate.json`](replay-certificate.json) | independent replay of 493 processes and 24 untimed re-runs |
| `runs.tar.xz` (+ `.sha256`) | the raw run tree: each process's report, isolation record and stderr, the reference walks, `host.json` (binaries' digests), the refusal and PSI logs; also `replay-rerun/`, the replay's 24 untimed re-runs |
| [`drive.log`](drive.log), [`scripts/`](scripts/) | each lock hold; the thin orchestration |
| [`report/`](report/) | report PDF, graphs (SVG, PNG), `plot.py`, `make_report.py` |

Re-check: extract `runs.tar.xz` and `training/raw.tar.xz` in place
(`tar -xJf runs.tar.xz`), then `ic_rule_v4 analyse --root . --v3 ../v3` rewrites
`analysis.json` byte for byte, `ic_rule_v4 replay --root . --v3 ../v3` rewrites the
replay (add `--rerun 2 --ic <ic> --recipe recipe.json` for the re-runs), and
`ic_rule_v4 select --model --root . --v3 ../v3 --grid training/grid.json` and
`ic_rule_v4 lever ...` the training read-outs. The harness is
[`src/bin/ic_rule_v4.rs`](../../../../src/bin/ic_rule_v4.rs).

**Shared source changed (additive, off by default):**
`src/cryptanalysis/koblitz_index_calculus.rs` (harvest in `fold_partitioned`,
`PairSumTable::free_relations`, `verify_collected_relation` for `a = 0`),
`src/bin/ic/workflow.rs` (`build_pair_table_harvest`), `src/bin/ic/price.rs`
(`KIC_FREE_RELATIONS=1`); new `src/bin/ic_rule_v4.rs`. Formatted with `rustfmt`
after the panel (no change of meaning); `cargo test --release --lib
the_harvest_leaves_the_table_unchanged` passes (the table is bit for bit the plain build's, and
every relation verifies at `n` = 23 and 31, with relations found; at `n` = 41
none are expected), and `cargo build --release` builds `ic`, `icprog` and `ic_rule_v4`.

## Proposed `benchmarks/README.md` row

| 2026-10-24 | [`20261024-table-a-rule-v4`](20261024-table-a-rule-v4/) | Table A's index calculus (Koblitz K_0/K_1, n = 41 to 61, `ic price --single-target`, cold): a v4 recipe (K, collection window and unit size retuned on training seeds) against rule v3's recipe in one binary, paired on 32 (recipe seed, target) pairs a size on 16 held-out seeds, one `isolated_bench` process per row, arms alternating, A/A and a positive control (the free-relation lever); rho priced in each process at the canonical batched step; 485 clean processes, all verified, 493 replayed | **PARTIAL**: cold v4/v3 0.953 [0.904, 0.998] at n = 57 (the one improvement), 1.007, 0.963, 1.010, 1.013, 0.966 (intervals contain 1) at n = 47, 41, 53, 59, 61; free-relation lever 1.05 to 1.09 (a net cost, off); gap to rho at the canonical step 9.9 to 43 (v4 9.89, 9.20, 16.6, 27.2, 24.3, 42.1), rho as built 1.8 to 2.1 times cheaper than v3 priced it | Intel Xeon 2.80 GHz (no GFNI), 4 vCPU L0 shared VM, `isolated_bench` one core; neighbours' unlocked jobs from n = 41 on (amendment A8) | [`research/ic_tool_program/rule/v4`](../research/ic_tool_program/rule/v4/) |
