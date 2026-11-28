# RESULTS: kappa at genus 3 [pcc:unverified]

Every figure below is `[pcc:unverified]`: one VM (4 vCPU, no hardware PMU), single thread, wall clock.
Protocol and decision rule were frozen first (`PROTOCOL.md`, `frozen.sha256`). Raw timings: `raw/`.
Tables with all quantities: `tables.md`. Figure: `fig_kappa.svg`. Instrument: `examples/kappa_g3.rs`
(release build, `-j2`), driver `run_all.sh`. 16 measured trials after 2 warm-ups per p; medians.
Instances: first `instance` line of `research/genus_ladder_g3_highp_20261121/run_<p>.jsonl`; arm
C_double_lp+prefilter, SGE(8,3), cc 2.0, seed 1 (one attempt each).

## Method as run
* t_F in situ: `JCurve::mul_rtl` batches (64 random divisors, ~46-53-bit scalars), wall time over the instrument's
  own `fmul + 10 finv` deltas. Bare: `(x*y) % p` as a dependent chain and as independent throughput.
* t_R in situ: the LA phase is not separable in `IcResult`, so t_LA = wall(`run_ic`, full) - wall(`run_ic` with
  `max_trials = trials - 1`, which stops before the linear algebra; `la.mul == 0`, `attempts == 0` asserted;
  la_mul identical across all trials, so the relation stream is deterministic). t_R = t_LA / `la_mul`
  (the whole LA: build_rows, peel, SGE, Wiedemann, certify). Bare: copy of `ic_control::mulmod`/`addm`.
* A/A: split-half (even vs odd trials), |median difference| / median. Load average (1 min) read before and after each trial.

## Results (median ns; kappa = ratio of medians)

| p | r bits, mulmod branch | t_F in situ | t_F bare thr / chain | t_R in situ | t_R bare mulmod+addm / chain | kappa in situ | kappa bare (matched / chain) | kappa add |
|---|---|---|---|---|---|---|---|---|
| 65537 | 46, f64-Barrett | 5.55 | 1.90 / 5.62 | 99.6 | 4.26 / 16.0 | 17.9 (trials 15.6-23.7) | 2.25 / 2.85 | 0.99 |
| 131101 | 49, f64-Barrett | 6.76 | 2.39 / 5.98 | 114.9 | 4.25 / 15.9 | 17.0 (13.6-22.8) | 1.78 / 2.67 | 1.00 |
| 262147 | 53, u128 `%` | 6.79 | 2.87 / 6.40 | 107.1 | 5.16 / 6.89 | 15.8 (12.7-19.9) | 1.80 / 1.08 | 1.00 |

A/A spread (split-half): t_F in situ 1.8% / 10.6% / 6.5%; t_R in situ 2.3% / 0.5% / 0.8%; kappa in situ 5% / 10% / 6%;
bare timings 0.5-9%. Load average: 0.34-0.94 (p = 65537, first trials while the VM settled), 0.99-1.06, 1.00-1.08 (one
busy thread, which is this process; nothing else was running).
Note: r at p = 262147 is above 2^50, so `mulmod` takes the `u128 %` branch there; bare chain kappa falls to 1.08.

Per counted unit in situ: LA wall / `la_add` = 9.4, 10.4, 9.2 ns, i.e. 1.7, 1.5, 1.4 t_F. Real runs have
`la_add / la_mul` = 10.6, 11.1, 11.6.

## Phase split (to explain the gap), synthetic core, `#[ignore]` test in `genus_ladder.rs`
Synthetic 4100 x 4000 core of Cantor-run-like weights, same r values, `sge_reduce` and `wiedemann` timed apart
(`raw/phase_split_test.log`): SGE takes 8 ms (negligible); Wiedemann costs 30.6 / 31.6 / 23.0 ns per counted mul and
13.1 / 13.5 / 9.8 ns per counted add (its `la_add/la_mul` is 2.3 there, so per-mul is not comparable to real runs, per-add is).
The in-situ LA time is therefore Wiedemann, and it scales with the number of matrix entries visited (about 9-13 ns each,
gather plus compare-subtract plus loop overhead), not with the number of mod-r multiplications.

## Decision (rule from PROTOCOL.md)
* In situ kappa_wall = 15.8-17.9 (every trial above 12.6): above 4.1, so by the frozen rule the light-row selection
  is an improvement on this host if FPM is to track wall time, and the studies' kappa = 9 under-charged LA.
* Bare mulmod-versus-F_p-mul kappa = 1.1-2.9 (matched 1.8-2.25): at or below 2.5, consistent with main's PR #4852 (about 2).
* The two disagree because `la_mul` is not what the LA time is made of: in situ the cost is about 10 ns per
  matrix entry and `la_add` is 10-12x `la_mul`. My prediction (in situ 0.5-3, central 1.5) was wrong for the in-situ number
  and right for the bare number. The in-situ figure is an effective charge per counted mul that absorbs the additions and memory traffic.

## What it means
* R4 / the light-row floor: its win is not fragile in wall-clock terms: lighter rows cut entries visited, hence both
  `la_add` and `la_mul`. Measured per entry, the saving in wall time is at least the saving in `la_mul`, and larger when weight drops.
  Under the bare kappa (about 2) the replay's "no improvement below 2.5" holds for la_mul-only FPM. Which accounting the campaign
  adopts decides the verdict: either charge kappa about 16-18 per `la_mul` (this host), or charge `la_add` (about 1.4-1.7 F_p mul each) and keep kappa about 2.
* SGE arm FPM ratios: FPM with kappa = 9 under-weights LA by about 2x versus wall time here (about 16 measured), and
  over-weights it about 4.5x versus the bare multiply (about 2). Ratios of arms with equal `la_add/la_mul` are insensitive; arms that change
  that ratio (SGE merges trade adds for generic multiplies) are not.

## Limits
One host, one thread, no PMU, wall time on a shared VM; one instance per p (first line of each file), single attempt each.
t_F in situ is per counted `fmul + 10 finv` (the 10 is the campaign's weight, not measured here). t_R in situ is a difference of two
whole-run wall times (about 5-17 s LA out of 6-60 s runs); the stopped run omits one trial (about 30 us). The phase split uses a
synthetic core, not the real one. Not run: GAE pricing, additional instances, other hosts.

## Deviations / changes
* Source change: an `#[ignore]` test module `kappa_phase_split` appended to `src/cryptanalysis/genus_ladder.rs`
  (measurement only, no visibility change, rustfmt-clean). The example itself needs no source change.
* `mulmod`/`addm` are copied into the example (they are `pub(crate)`).
* A/A is split-half over trials, not a separate back-to-back arm.
* Clippy: no warning in `examples/kappa_g3.rs`.
