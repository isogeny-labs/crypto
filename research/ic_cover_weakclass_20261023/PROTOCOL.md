# Protocol: Hilbert early stop in the Joux-Vitse cover IC, end to end (frozen 2026-10-23, before any evidence run)

Branch `claude/ic-cover-weakclass-20261023`. Weak class, lower-bound framing only; Table D is not touched.
No leaderboard, no `benchmarks/README.md`, no PR from this study.

## Question

After the sparse-FGLM linear stage (#2244), `jv_cover --exp dlp` gives S/rho = 664 / 286 / 151 / 293 at
p = 53 (seeds 1, 2) and p = 101 (seeds 1, 2) (`research/jv_cover_sparse_fglm_20261018/summary.json`).

1. **L5.** Does S/rho keep falling with p? Measure the same baseline at p = 251 (and 503 if the budget allows).
2. **One lever.** Does one named change lower counted S at equal relations and correct logarithms, without moving
   work to a counter that is not priced?

## Lever, and what the pilot showed before the protocol was frozen

Candidates (`research/ic_cover_leads_20261018/README.md`, `research/f4_early_stop_20261018/`):

| lead | mechanism | status | decision |
|:--|:--|:--|:--|
| L2 F5 signatures | skip Koszul zero reductions (12 % of rows) | not implemented; a signature engine is a new F4 core | not built; about 1.08x is a heuristic |
| L6 large primes | exponent change | PR #2224: does not pay at p <= 12k | not redone |
| L4 Hilbert early stop | after the six quadrics' leading forms are certified a regular sequence, drop every pending F4 pair that the Hilbert function of the complete intersection proves reduces to zero (`F4Options::stopping_by_hilbert`; the output is the same reduced basis) | in `f4_fp`, reachable from `--exp ccov --stop hilbert:1`, **not from `--exp dlp`**; never measured end to end | **chosen** |

Pilot (not evidence; one cell, p = 53 seed 2, 8,486 residuals, one thread, run before this protocol was frozen;
files kept in `pilot/`): plumbing the stop into `dlp` gives the same relations (digest equal) and about 2.5x fewer counted
multiplications, **but** the staircase check of the existing implementation recounts every monomial of degree <= d against all
leading monomials at every step, and the new diagnostic counter `f4_native_ops` (divisibility tests and monomial-table probes of
F4's symbolic preprocessing and staircase checks, a lower bound) rose 8.6x (4.4e8 -> 3.8e9) and wall time rose 1.7x. That is the relabelling
pattern of AGENTS.md section 3: counted multiplications fell, work moved to a counter nobody prices. The lever is therefore tested in two forms:

- **B0, as found**: `--stop hilbert:1` with the scan staircase (`F4_STAIR=scan`), the only form reachable before this study.
- **B, repaired**: an incremental staircase (`StairTracker`: the uncovered monomials per degree, updated when a basis element
  is appended; a leading monomial is only tested against uncovered monomials of degree >= its own), a staircase walk that skips
  leading monomials of too high a degree, and a coarser step (`--stop hilbert:16`, 16 pairs per F4 step). The chunk 16 was
  chosen on that one pilot cell as the smallest tried (1, 4, 8, 16, 32) whose `f4_native_ops` did not exceed the baseline's
  (tuning on a grid cell; disclosed, and the cell is reported like the others). B is the **primary candidate**; B0 is secondary.

Code (counters and plumbing do not change the baseline arm's counts, rng draws or algorithm):
`run_cover_dlp_mode(.., StopMode, ..)` (`run_cover_dlp` keeps its signature), `--stop hilbert:C` honoured by `--exp dlp`;
`f4_fp`: `StairTracker`, degree-pruned `staircase_at_most`, native counters `native_ops_total`, `native_stair_ops_total`;
report fields `stop_mode`, `hilbert_*`, `f4_muls_per_test_sum`, `f4_native_ops`, `f4_stair_ops`, `relation_digest`, `rho_steps_mean`.
`F4_STAIR=scan` selects the old staircase check inside the same binary.

## Arms (one binary, selected on the command line / one env var)

- **A (baseline)**: `--exp dlp`, full F4, default sparse-FGLM linear stage. Reproduction check: must give the recorded
  `total_muls` structure at p = 53, 101, seeds 1, 2 (S 7.316e2, 3.288e2, 2.244e2, 3.655e2), else the difference is reported.
- **B (candidate)**: A plus `--stop hilbert:16`.
- **B0 (as found)**: A plus `--stop hilbert:1` with `F4_STAIR=scan`. Cells (53,1), (53,2), (101,1) only.
- **A2 (A/A control)**: arm A repeated from a fresh process. Pass: identical `total_muls`, `residuals`, `relations`, `relation_digest`,
  so the A/A ratio of S is exactly 1.000. Cells (53,1), (53,2), (101,1).
- **Positive control**: `--exp ccov --sizes 101 --seeds 1 --residuals 100 --constructed 20 --oracle-base 60` for A and B: all 20 planted
  decompositions found in both and the brute-force oracle reports 0 mismatches.

## Instances, seeds, command

Cells (p, seed), arms A and B in each: p = 53 seeds 1-8; p = 101 seeds 1-8; p = 251 seeds 1-6; p = 503 seeds 1-3 (only if affordable).

```
RAYON_NUM_THREADS=1 jv_cover --exp dlp --sizes P --seeds 1 --first-seed S --rho-runs R --rho-ref 1.3 --check-every 64 [--stop hilbert:16] --json OUT
```

One thread: the global F4 counters are then exact per test. `R` (stored-table r-adding rho, the harness's `rho_e`): 8 at p = 53, 101; 4 at p = 251;
0 at p = 503 (reference = pooled measured rho S). Rho runs are made in arm A only (they do not depend on the arm); S/rho for B uses the same
per-p pooled rho. Order: p ascending, within p by seed, A then B, A2 and B0 at their cells. **Budget:** one core, at most 10 core-hours of run time
(estimated 5-7 h from the pilot's 3.6 ms per test). p = 503 is run only if, after p = 251, cumulative run time is under 5 h and 2.2 x the p = 251
per-seed cost fits in the remainder; otherwise it is reported as not affordable. At the cap, unfinished cells are reported as not run.

## Unit and what is measured

`S = total_muls / (c_add_E * sqrt(l))`; `total_muls` = setup + factor base + residual stream + Weil restriction + F4 (including the Hilbert
regularity check and any fallback run) + linear stage + post-processing + relation verification + 16 x Wiedemann operations (the harness's counter).
S/rho = `total_muls` over the matched rho cost in the same unit (`rho_S * sqrt(l) * c_add_E`). Also recorded per run: `f4_native_ops` (above), wall time
(a practicality note on a shared, loaded host; never a criterion).

## What counts as a verified lowering (all must hold)

1. Per cell, A and B both `solved` and `correct`, `mismatches = 0`, every relation verified, and **equal relations**: equal `residuals`,
   `relations`, `decompositions`, `relation_digest`.
2. `total_muls(B) < total_muls(A)`, with `f4_muls_per_test_sum = f4_muls` in B (check and fallback work are inside the counter).
3. **No work moved to an unpriced counter**: `f4_native_ops(B) <= f4_native_ops(A)`. Then B is lower in every emitted counter, so the order holds at every
   weight of the section 5a band (0, 0.1, 1 GAE per native operation, GAE = `c_add_E` = 331 multiplications). Where `f4_native_ops(B) > f4_native_ops(A)` the cell
   is **unresolved**: the break-even weight w* = (muls(A) - muls(B)) / (native(B) - native(A)) multiplications per native operation is reported, and
   the cell is not an improvement at any band weight above it.
4. A/A ratio of S exactly 1.000 in all A/A cells; A reproduces the recorded baseline S where one exists, else the difference is reported.
5. At least one B run replayed from a fresh process (identical report), and its `relation_digest` equals the **A** run's (the full-F4 path does not execute the Hilbert code).
6. The `f4_fp` and `jv_cover` unit-test modules pass in release, including the tracker-versus-scan assertion (`cfg(test)`) and the Hilbert-versus-oracle test.

The claim "B lowers S" is made only for the set of cells where 1-3 hold, and it is stated per p. If 3 fails in some cells the claim is narrowed to the cells
where it holds and the others are reported as unresolved; if it fails in most, the result is a relabelling and is reported as such.

## Statistics

Counts are deterministic given the seed. The paired ratio S_A/S_B is per cell. Per p: mean, sample sd, min-max over seeds (n stated). S/rho observed
(pooled per-p rho) and formula-predicted (`predicted_s_over_rho`, derived, lower variance). Log-log least-squares slope of per-p mean S/rho against p, with
its standard error over 3-4 sizes (labelled a fit over few sizes). Across-seed spread is the relation-count noise (about 1/sqrt(|F|) per seed), not a confidence
interval for the method.

## Matched rho and the 5a band

Matched rho: the harness's `rho_e` (r-adding walk, exhaustive storage, r = 32), same group, unit = group additions x `c_add_E`. It leaves its table insert and lookup
per step unpriced (`rho_steps_mean` is emitted); the rho side of the band prices them at 0, 0.1, 1 GAE per step (2 operations per step). The IC arms leave native work
unpriced beyond `f4_native_ops` (pair selection and its criteria, matrix assembly, sorting, factor-base hashing); those counters are not emitted, so `f4_native_ops` is a
lower bound, no cross-method rank is made, and S/rho is the distance remaining. A stronger rho exists (`--exp rhostrong`, negation map) and is outside this protocol,
so the true gap is larger than reported.

## Classification

If 1-6 hold in the stated cells: **engineering** (S falls; the floor `unknowns x 720` residuals is unchanged, so not an advance). If 3 fails: relabelling or unresolved, with numbers. A negative result is recorded.

## Raw data and replay

`raw/p<P>_s<S>_<arm>.json|log`, `commands.txt`, `versions.txt` (git revision, binary sha256, host), `walls.txt`. Aggregation:
`examples/jv_cover_weakclass_report.rs` (Rust). Tests: `test.log`.
