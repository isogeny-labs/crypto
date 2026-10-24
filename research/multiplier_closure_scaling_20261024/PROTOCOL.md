# PROTOCOL (frozen before any run in this directory) -- multiplier-closure scaling

**Date.** 2026-10-24. **Branch.** `research/multiplier-closure-scaling-20261024`, based on
`origin/main` 5cb553810e (contains `cf7bb19e8a` cm-multiplier and `0333e9d5c8`
binary-multiplier). Source: REVIEW.md section 1 item 2, section 3 item 3, section 5.2 M2.

## Hypothesis (H)
The 10-20x (prime CM) and 3-7x (binary) lower counted S of the multiplier-closure
bases (`research/ic_improve_20261023/{PRIME,BINARY}_README.md`) are constants of the toy sizes
that shrink as `r` grows and as the folded column count becomes a real factor base.
Operationally: `log2(S_mult / S_inc)` has a fitted slope against `log2 r` that is `>= 0`.
We try to falsify H: H is falsified if the 95% interval of the slope lies entirely below 0.

## Frozen inputs
* Binary: `ic` built from this branch (`cargo build --release --bin ic`,
  `CARGO_TARGET_DIR=/Volumes/SSD990-2/cargo-target-ic-followthrough-20261024` (amended, see below)); its sha256 goes in every JSONL row.
* Oracle for every row: `mitm:negation_folded=1,m=3`; `--targets walk`; `--repeats 4`;
  `--rho-runs 4` (rho reference is context only, not part of any decision).
* Instance seeds 5, 6, 7 (three instances per size). Metric: `S = total_gae / sqrt(r)` from the
  JSON report (counted, deterministic; wall time is inadmissible at load 60-100 and is not used).
  Per configuration, S is the mean over the 4 repeats.
* Regimes and sizes (`--bits` = prime subgroup bits; `--char2-degree` = binary field degree).
  A size that fails to generate (or times out at 20 min per invocation) is recorded and dropped;
  nothing is substituted.
  * prime `j0`: bits 24, 28, 32, 36, 40. prime `j1728`: bits 24, 28, 32, 36.
  * random binary: degree 20, 22, 24, 26, 28 (degree 30 exceeded 250 s before).
* Incumbent grids (best over the grid is the incumbent, per instance):
  * prime: `glv-orbit:size=S`; S = {50,100,200,400} (24), {100,200,400,800} (28),
    {200,400,800,1600} (32), {400,800,1600,3200} (36), {800,1500,3000,6000} (40).
  * binary: `binary-subspace:dimension=d`, d = {6..9} (20), {7..10} (22), {8..11} (24), {9..12} (26, 28);
    and `binary-multiplier:size=K,mult=1` (plain seeds), K in {64,128,256,384,512,1024}.
* Multiplier grids (best over the grid, per instance):
  * prime: `cm-multiplier:size=K,norm=N`, K in {4,8,16,32,64,256,512}, N in {8,16,32,64,128,256},
    kept iff `256 <= K*N <= 8192` (about 3.75*N points per column on j0, so at most about 30k points).
  * binary: `binary-multiplier:size=K,mult=M`, K in {2,3,4,8,16,32,256,512}, M in {2,4,8,16,32,64},
    kept iff `64 <= K*M <= 4096`.
  The K >= 256 cells are there on purpose: they are the only multiplier cells that can be
  admissible factor-base measurements (rule below).
* Reference for correctness: every row's `verified` and `correct` must be true; a configuration
  with any false is excluded from the minimum and counted in the README.

## Recorded per configuration
S, log2 r, folded columns (`factor_base.columns`), points (`signed_points`), relations needed
(`linear_algebra.rows`, rank), correctness, setup/factor-base/oracle gae split.

## Decision rule (frozen)
1. **Admissibility.** A configuration row with fewer than 256 folded columns is flagged
   inadmissible as a factor-base measurement (it is a table/BSGS-like object). An instance is
   an *admissible pair* only if both sides have at least one admissible row; the admissible ratio uses
   the best admissible row on each side.
2. Per instance: `R_all = min S_mult / min S_inc` over all rows (the READMEs' statistic) and
   `R_adm` over admissible rows only. Boundary flag: the minimising cell lies on an edge of its grid
   (then that S is an upper bound for that side and the ratio is biased; reported, not repaired).
3. Fit per regime (prime j0+j1728 pooled and each separately; binary): OLS of `y = log2 R` on
   `x = log2 r` over instances, slope with a 95% t-interval (n-2 dof); secondary fit on
   per-size means (n = number of sizes, n-2 dof). Primary verdict statistic: `R_adm` per-instance fit;
   `R_all` is reported alongside.
4. **Verdict.** Slope point estimate `>= 0` => the gain is a constant or shrinking (H not falsified);
   interval entirely `> 0` => the gain demonstrably shrinks; interval entirely `< 0` => H falsified
   (gain grows). Fewer than 4 sizes with admissible pairs => "not testable as a factor-base
   measurement"; the `R_all` fit is then reported only as a toy-size table-pipeline statement.
5. Stop conditions: any invocation capped at 20 min (timeout recorded as a row, not retried with a
   different grid). No post-hoc grid changes; any extra run is added as a labelled follow-on.

## Cost accounting
Counted gae only; all costs including factor-base construction, table build, oracle, linear algebra
are in `total_gae` as the bench reports them. No wall time is used.

## Amendment 1 (2026-10-24, before any run; coordinator instruction)
The 20-minute cap is a **CPU-time** cap (`RLIMIT_CPU` 1200 s per invocation, i.e. `ulimit -t 1200`), not a wall cap,
because host load is 60-100; a 4 h wall guard exists only to prevent hangs. Build target dir is the shared
`/Volumes/SSD990-2/cargo-target-ic-followthrough-20261024`. Nothing else changes.
