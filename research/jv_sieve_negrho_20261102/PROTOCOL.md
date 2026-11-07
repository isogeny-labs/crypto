# Protocol: matched negation-map rho for the sieve route at p = 1511 (frozen 2026-11-02, before any run)

Labels: **measured** (read from a run record), **derived** (computed from measured values by `fit/`, Rust), **model** (extrapolation).
Scope: the `jv_cover` weak class only (full rational 2-torsion, `#E = 4l`, genus-3 cover); never prime order. Lead B (L3) of `research/jv_cover_next_leads_20261102/LEADS.md`. Accounting only: no improvement is claimed, nothing enters the leaderboard or `benchmarks/`.

## Question
V2 (`research/jv_sieve_strong_rho_v2_20261020/`) measured the sieve route at p = 1511 on seeds 17-32 but its rho A/A rule A4 failed (rho comparisons withheld). `research/jv_cover_negrho_rescore_20261101/` (#3278) measured the negation-map rho on other instances (126 runs, pooled 0.9322 steps/sqrt(l), homogeneity p = 0.31). This study measures the matched negation rho on the sieve route's own p = 1511 instances and gives the sieve route's S/rho there.

## Producer and arms
* Binary: `jv_cover_rho_frozen` (built at commit 3764c93e98, main c2b49d7682 code); its sha256 is in `results/FROZEN.sha` before any run. No cargo build of the main crate is made. The binary is the only producer.
* **Reference arm (new runs):** `--exp rhostrong --neg 1 --batch 16 --r 256 --dp-bits 16 --sizes 1511 --first-seed s --seeds 1 --run k`, seeds s = 17..24, run indices k = 0 and 1 (16 runs; the run index fixes the walk seed independently of the instance seed). One process per run, single thread, 4 concurrent.
* **Sieve arm (no new runs):** V2's records `research/jv_sieve_strong_rho_v2_20261020/raw/ic_1511_sparse.json` (16 rows, seeds 17-32; `JV_LIN=sparse`), copied unchanged into `results/inputs/` with sha256. All must be `solved`, `correct`.
* **Pooled reference (#3278):** its 126 runs (`results/inputs/old_rhostrong/` 18 + `results/new/` 108 of that study, p in {1301, 1451, 1601}), read in place from `research/jv_cover_negrho_rescore_20261101/results/`, same settings (negation, batch 16, r 256, dp-bits 16).
* Identity check (tool, hard): for every new run, `l` and `c_add_e` equal those of V2's sieve row with the same seed.

## Units (as #3278, verified there in `src/cryptanalysis/jv_rho_strong.rs`)
Per run: `s_harness = muls_harness/(c_add_e sqrt l)` (U1) and `s_tuned = muls_tuned/(c_add_e sqrt l)` (U2). Sieve: `S = total_muls/(331 sqrt l)`, `S+ = total_plus/(331 sqrt l)`. c_add_e = 331 everywhere.

## Pre-registered tests
1. **Homogeneity (frozen).** Metric: steps/sqrt(l). Groups: the 16 new runs vs #3278's 126 runs. Tests: (a) permutation test (20000 permutations, group sizes kept) on the absolute difference of means; (b) two-sample Kolmogorov-Smirnov statistic with a permutation p-value (20000). **Homogeneous iff both permutation p-values >= 0.05.** Printed for information: Kruskal-Wallis H (permutation p), the same two tests on s_harness and s_tuned. **Primary reference: the pooled 142 runs if homogeneous, else the 16 matched runs.** Both are reported.
2. **Positive control.** The matched mean of steps/sqrt(l) (16 runs) within 3 se of theory sqrt(pi/4)/sqrt(1-1/256) = 0.88796, se = sd/sqrt(16). If |z| > 3 the reference is flagged and so is every verdict.
3. **Replay control (before launching).** #3278's record `p=1451 seed=1 run=0` is rerun with the frozen binary and compared field by field (all but `wall_ms`); must be bit-identical, else the pooled 126 runs are excluded from the reference and the primary is the 16 matched runs.
4. Every new run must be `correct`, not `capped`, negation, batch 16, r 256, dp_bits 16; an excluded run is listed, never replaced.

## Statistics (frozen)
Reference mean = plain mean of the unit's per-run values over the reference runs. **S/rho = geometric mean over cover seeds of S, divided by the reference mean** (secondary: ratio of arithmetic means, V2's convention). Cover seed sets: **seeds 17-24 (the matched 8)** and **all 16 seeds**. S+/rho: cover S+ over the same reference mean (rho unpriced), and, as V2 does, cover S+ over rho S+ (rho (muls + fp_adds + native)/(331 sqrt l)). Interval: 95% percentile bootstrap, **20000 resamples**, jointly resampling cover seeds with replacement and reference runs with replacement within each instance (groups sizes kept; the same draw is shared by units, w values and layers); splitmix64 fixed seed. "How far below rho" = 1/(S/rho) with the interval inverted. Verdict on [lo, hi]: below if hi < 1, above if lo > 1, else not resolved.

## Section 5a band (frozen)
Native operations priced at w = 0, 0.1, 1 GAE (1 GAE = 331 F_p mults) each on both arms: `ratio_w = geomean over seeds of (cover S + w nat_cover/sqrt l) / (rho s + w nat_rho/sqrt l)`.
* **Layer A (V2's upper bound):** cover native = `sieve_adds + sieve_lookups + enum_adds + unemitted`, where `unemitted = total_muls - sieve_muls - enum_muls` (F4/descent multiplications repriced as a native upper bound as V2 does); rho native = `canon_cmp + hash + dp_check + scalar_ops + table_ops + cycle_cmp + fp_adds`.
* **Layer B (as emitted):** cover native = `sieve_lookups`; rho native = the six #3278 counters (without `fp_adds`, which #3278 lists as an unpriced counter).
* U2 uses `muls_tuned` for rho. Order holds across the band for a cell iff the verdict is the same at all three w.

## Budget and cut (frozen)
Host: 4 vCPU box shared with another agent's short runs (counted metrics are host-independent). Budget 3 h wall. Order: all run=0 jobs (seeds 17-24), then all run=1 jobs. No job launches after T0 + 2 h. If the run=1 round is not complete, the analysis uses run=0 only (1 run per seed, 8 runs) and says so; the homogeneity test and control 2 then use 8 runs. A job over 5400 s is recorded and its round is incomplete.

## Freeze record
`results/FROZEN.sha`: sha256 of this file, `run.sh`, the binary, with a UTC timestamp, before any run. The analysis tool `fit/src/main.rs` and `fit/Cargo.toml` are hashed there too, after being tested on #3278's data only and before the new records are read (a second timestamped line). Analysis is Rust only; no Python computes a number.
