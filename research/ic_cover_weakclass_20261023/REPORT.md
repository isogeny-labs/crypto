# Hilbert early stop in the Joux-Vitse cover IC, end to end (2026-10-23 to 2026-10-28)

Branch `claude/ic-cover-weakclass-20261023`. Protocol: [`PROTOCOL.md`](PROTOCOL.md), frozen at commit `1f006b002` before any evidence run. Binary sha256 `223902e2...` (git `1f006b002`, no uncommitted source), host Intel Xeon 2.1 GHz, shared 4-vCPU VM at load 3-6, one thread per run (`RAYON_NUM_THREADS=1`). Raw records: [`raw/`](raw/) (`commands.txt`, `versions.txt`, `walls.txt`), aggregate [`summary.json`](summary.json) / [`table.md`](table.md), graph [`s_over_rho.svg`](s_over_rho.svg) / [`.png`](s_over_rho.png). Weak class, lower-bound framing; Table D is untouched. Everything below is **counted** F_p multiplications (`S = total_muls / (c_add_E sqrt(l))`) unless stated; no wall-time claim.

## Result

| statement | class | value |
|:--|:--|:--|
| Baseline A (full F4 + sparse FGLM) reproduces the recorded S | exact computation | S = 731.59, 328.79 (p = 53 seeds 1, 2), 224.41, 365.46 (p = 101 seeds 1, 2); recorded 731.6, 328.8, 224.4, 365.5 |
| S/rho of A falls with p | measured | mean over seeds (rho pooled per p): 1361 (n=8, heavy tail), 205 (n=8), 35.2 (n=6), 7.19 (n=3); log-log slope -2.27 +- 0.13 over 4 sizes (fit over few sizes) |
| **B (Hilbert early stop `hilbert:16`) lowers counted S at equal relations**, per cell S_A/S_B | measured | 1.848 (p=53, n=8, [1.825, 1.878]), 1.875 (101, n=8, [1.870, 1.879]), 1.868 (251, n=6, [1.866, 1.870]), 1.849 (503, n=3, [1.848, 1.850]); 25 of 25 cells pass every criterion |
| B moved no work to the counter that is emitted | measured | `f4_native_ops` B/A = 0.988, 0.985, 0.984, 0.983 (all 25 cells <= 1) |
| As found (B0: `hilbert:1`, scan staircase) is a relabelling | measured | S_A/S_B0 = 2.53-2.64 but `f4_native_ops` x8.3-8.5; break-even 1.48-1.54 multiplications per native op (0.0045 GAE) |

Classification (AGENTS.md section 3): B is **engineering** (counted S falls 1.85x-1.88x; the floor `unknowns x 720` residuals is unchanged, so not an advance). B0 is **relabelling**: the count fell 2.5x while the unpriced F4-driver work rose 8.4x. This study's contribution is the repair that turns B0 into B.

## What changed (and what did not)

The Hilbert stop (`F4Options::stopping_by_hilbert`, in `main` from #2305) certifies that the six quadrics' leading forms are a regular sequence, then drops every pending F4 pair that the complete intersection's Hilbert function proves reduces to zero; the reduced basis is the same. It was reachable from `--exp ccov` only. This branch (a) plumbs it into `--exp dlp` (`run_cover_dlp_mode`), (b) adds the diagnostic counter `f4_native_ops` (divisibility tests and monomial-table probes of F4's symbolic preprocessing and staircase checks; a lower bound, see below), (c) replaces the scan staircase check (every monomial of degree <= d tested against every leading monomial at every step: 2.4e9 of B0's 3.8e9 native operations in the pilot) by an incremental tracker (`StairTracker`, asserted equal to the scan in `cfg(test)`), prunes the staircase walk by degree, and (d) takes 16 pairs per F4 step. Chunk 16 was chosen on one pilot cell (p=53 seed 2) as the smallest of {1, 4, 8, 16, 32} whose native count did not exceed the baseline's; that cell is reported like the others. The baseline arm's counts, rng draws and algorithm are unchanged (A reproduces the recorded S exactly).

## Controls and gates

| gate | outcome |
|:--|:--|
| equal relations, every cell | `residuals`, `relations`, `decompositions` and `relation_digest` (FNV-1a over every relation, in order) equal for A and B in 25/25 cells; the digest is computed by the full-F4 path of A, which does not execute the Hilbert code |
| logarithm correct, relations verified | `solved` and `correct` true, `mismatches` 0 in every A, B and B0 run (brute-force cross-check every 64th residual at p <= 101 where the table fits) |
| A/A | arm A repeated from a fresh process at (53,1), (53,2), (101,1): `total_muls`, residuals, relations, digest identical, ratio of S = 1.000 |
| independent replay | B re-run from a fresh process at (53,2) and (101,4) ([`replay/`](replay/)): `total_muls`, digest and native count identical to the recorded B run, and the digest equals A's |
| positive control | `--exp ccov --constructed 20`, p=101 seed 1 ([`controls/`](controls/)): 20/20 planted decompositions found in A and in B, oracle mismatches 0 in both (B: Hilbert applied on 118 of 120 systems, 2 fallbacks) |
| unit tests | `cargo test --release --lib -- cryptanalysis::f4_fp cryptanalysis::jv_cover`: 75 passed, 0 failed, 1 ignored ([`test.log`](test.log)), including tracker-versus-scan and Hilbert-versus-oracle |
| counter consistency | the global F4 counter is 1.000-1.003x the sum of per-test reports in B (and 1.003x in A): nothing is missing from the headline counter; the check run and the fallback runs (0.2-2.6 % of tests) are inside it |

## Relabelling and accounting exclusion

Counted S falling is not enough, and the pilot showed why (B0). In B, per cell: the stage split (Weil, F4 including the Hilbert check, linear, post, verification, 16 x Wiedemann, setup, stream) is in `summary.json`; only F4 changes (-60 %: for example 7.44e5 -> 2.90e5 per residual at p=503), every other stage is equal to the last digit; the regularity-check multiplications (about 11 % of B's F4 multiplications) are inside it. The emitted native counter is no higher in any cell (0.983-0.990). Wall time, a practicality note only on a loaded shared host, was B/A 0.70-1.08 (mean 0.94) per cell, so no hidden slowdown appears; B0's was 1.06-2.10.

**Unpriced work not emitted, for every IC arm:** pair selection and the Gebauer-Moller criteria, matrix assembly, sorting, factor-base hash lookups, and the integer work of the Weil restriction; `f4_native_ops` therefore bounds only part of the native work from below. B has more F4 steps than A (16 pairs per step against all of a degree), so an unemitted per-step cost could still favour A; the wall-time ratio above is the only (indirect, noisy) evidence against it. **Rho side:** the harness's `rho_e` leaves a table insert and a lookup per step unpriced (steps = 0.97-1.00 per group addition; emitted as `rho_steps_mean`). Priced at 0, 0.1 and 1 GAE per native operation on the rho side only (the IC side's unpriced counters are not emitted), mean S/rho of B is 18.8 / 15.7 / 6.3 at p=251 and 3.89 / 3.24 / 1.30 at p=503 (A: 35.2 / 29.3 / 11.7 and 7.19 / 5.99 / 2.40; the p=503 band used 1.0 steps per addition, since no rho was run at 503). Because only the rho side is priced, no cross-method rank is made.

## Distance to rho (lower bound only)

Matched rho is the harness's plain r-adding stored-table rho (`rho_e`, S_rho = 1.455 / 1.269 / 1.360 at p = 53 / 101 / 251, pooled over seeds, 64 / 64 / 24 runs; at p=503 no runs were taken and the pooled mean of all 152 runs, 1.362, is used). A stronger rho (negation map, `--exp rhostrong`) is outside this protocol, so the true gap is larger. S/rho (mean over seeds, [min, max], n):

| p | A | B | formula-predicted A / B (derived) |
|--:|:--|:--|:--|
| 53 | 1361 [226, 6062] n=8 | 730 [123, 3229] n=8 | 509 / 304 |
| 101 | 205 [100, 288] n=8 | 109 [53, 153] n=8 | 161 / 83 |
| 251 | 35.2 [22.5, 47.0] n=6 | 18.8 [12.0, 25.1] n=6 | 28.7 / 13.8 |
| 503 | 7.19 [4.9, 8.7] n=3 | 3.89 [2.7, 4.7] n=3 | 6.5 / 3.5 |

The across-seed spread is the relation-count noise (the residuals needed per seed are heavy-tailed: 8.5k to 233k at p=53), not an interval for the method; the paired ratio S_A/S_B is almost free of it. The earlier record's 664 / 286 / 151 / 293 used 4-run rho means (1.10-1.48); with the 64-run pool the same S gives 503 / 226 / 169 / 276. **L5 answer:** S/rho keeps falling with slope -2.27 +- 0.13 (expected -2: S ~ p, rho ~ p^3). **Extrapolation (not a measurement)**, with that slope and the plain rho: the line meets S/rho = 1 near p ~ 1.2k for A and p ~ 0.9k for B (#2216 estimated 1.7k to 12k); a strong rho moves it up, and a larger p changes the group size, so it is a statement about this family only. S/rho at p=503 is 7.2 (A) and 3.9 (B): distance remaining, not a pass/fail.

## Deviations from the protocol (all recorded here)

1. The counter-consistency criterion was written as "`f4_muls_per_test_sum = f4_muls`"; the global counter is 0.3 % above the sum in the **baseline** too (work no per-test report claims), so the aggregator tests `0 <= (global - sum)/global <= 1 %`. Changed after the data were collected, before they were analysed; A and B differ by 0.0-0.3 percentage points.
2. The p=101 seed-5 A run was lost when the session was reset and rerun from scratch by the idempotent driver (log empty, no record existed); `run.sh` has a harmless unset-variable test on its first call.
3. p=503: `--rho-runs 0` as protocolled; the S/rho band for those cells uses 1.0 rho steps per group addition (hand-corrected in `summary.json`, field `rho_steps_note`; the generator printed 0.023 because it pooled absolute step counts over sizes and divided by this p's sqrt(l)). The generator now pools each run's own steps-per-group-op ratio, so a rerun gives the measured pooled ratio for these cells instead of 0.023; `summary.json` has not been regenerated with it yet and still carries the hand-set 1.0.
4. B was tuned (chunk 16) on one pilot cell that is also a grid cell; B0 and the pilot intermediates are in `pilot/` with `NOTES.txt`.
5. Total run time 6.3 core-hours (wall, one thread, shared host) plus the controls; the 10 core-hour cap held and p=503 passed the protocol's affordability test (4.74 h < 5 h).
6. A/A was run on three cells, B0 on three; per-cell native and wall values are in `summary.json`.

## Next improvement to try

B's native counter is flat, not lower, and F4's non-multiplication work is unpriced in both arms; the next step is to emit the remaining F4-driver counters (pair selection, matrix assembly) so that the F4 stage can be compared at the 5a band, then to cut the linear stage (now 45 % of B's counted cost per residual at p=503: 2.37e5 of 5.3e5 muls: target-pinned deferred sparse solver stop, or fewer eigenvector solves) and the Weil/F4 per-test share. L2 (F5 signatures, about 1.08x) and L6 (large primes, closed at p <= 12k) are unchanged.
