# L10: how does the joint-optimum F4 cost scale with r? (isogeny factor base, k = 16 … 24)

**Class:** exploratory engine measurement in counted units (F4 multiply-adds from
`curve25519_family f4cost`). The result is a *modelled* F4 cost per solve
E_w = columns · r · (mpt + w·nat) / covered_points and its fitted exponent in r. It is not an
end-to-end solve, not an ecbench session and not a leaderboard row. **Rho remains ahead on every
track; nothing here is an improvement claim.**

Protocol: [PROTOCOL.md](PROTOCOL.md) with `scripts/` (frozen at commit
`5a800bb3d06c69b32f32195c3aebccdfb3fee31e`, the checkout's HEAD during the run; neither was edited).
Run: 2026-11-04 06:33:21Z to 06:35:45Z, host `vm` (`nproc` 3 during the run), strictly sequential,
one heavy process. Analysis: `python3 -I scripts/analyze.py <lead dir>` → `analysis/summary.json`
(sha256 `6590e8d4b6801f71115d2a61537913552c987095185f52c776ad1ebbbc45b129`). Data hashes:
`data/SHA256SUMS` (written by the frozen runner; 124 files, `sha256sum -c` all OK).

## Answer

- **Exponent (P5 held).** Least squares of ln E_best(k) on ln r_k over the five curves:
  **b = 1.0297** (frozen script, w = 1, which prices nat as the 100-target total; see erratum E2), a = 7.968, rmse(ln) 0.072; b = 1.0315 at w = 0, which equals the per-target-corrected b = 1.0315 (w = 0.1 and w = 1). Point
  prediction was 1.03. Local slopes 0.894 (16→18), 1.180 (18→20), 0.971 (20→22), 1.029 (22→24).
  The fixed-t exponents are b = 1.030 / 0.980 / 1.002 / 0.937 at t = 4 / 5 / 6 / 7 (frozen, w = 1),
  i.e. **the algebraic track scales like r¹ against rho's r^½ at every t**, and the optimum does not
  move with k.
- **Optimal t (P4 held, the preregistered alternative to the brief's "t = 5").** argmin_t
  E(k, t, t−1, S*) = **4 at all five k**, second-best t = 5 everywhere, E(t=5)/E(t=4) = 2.16, 2.25,
  1.78, 1.66, 1.79 (frozen E_1; 2.40, 2.54, 2.04, 1.91, 2.05 at w = 0, against point predictions
  2.45, 2.56, 2.04, 1.91, 2.05).
- **k-invariance of mpt (P2 held by its 10 % rule, point predictions missed).** max/min of mpt
  across k at fixed (t, G = t−1): 1.0053 (t=4), 1.0273 (t=5), 1.0508 (t=6), 1.0915 (t=7). The point
  predictions (≤ 1.0001 at t = 4–6, ≤ 1.03 at t = 7) were missed in all four series: mpt is exactly
  k-invariant whenever a cell has `hits = 0` (17,592.0 / 52,816.0 / 146,464.0 at k ≥ 20 for the G = t−1 series
  t = 4 / 5 / 6; the non-deciding t = 6, G = 4 series is not exactly constant, see E4 below) and is *lower* by up to 8.4 % at k = 16, where the 100-target stream contains 1–14
  hits (a hit ends that target's branch work early). The deviation is a hit-rate artefact that
  shrinks with coverage/r; it is not a cost increase with k.
- **Seed control (P3 held).** mpt(S*)/mpt(seed 0) = 0.981–1.000 in all 10 pairs.
- **Instrument (P0, P1 held).** The 7 cells shared with T10 reproduce T10's mpt to 0.0 relative
  difference; f4cost `columns`/`covered_points` equal the census in all 40 cells.
- **Gap to rho grows (P6 held).** E_best / (1.5·√r) at w = 1: 2.42e5, 4.18e5, 1.07e6, 2.05e6,
  4.28e6 (strictly increasing; slope r^0.53). See the distance section for the per-target-nat
  correction (1.47e5 … 2.63e6) and the w = 0.1 figures.

## Instrument and run

- Binary `/home/user/ctarget/release/curve25519_family`, sha256
  `ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef` (= frozen; checked at start
  and before every cell; never rebuilt). Census files k16/18/20/22/24 and `selection/seeds.json`
  matched their frozen sha256 (`data/env.txt`).
- Command per cell: `curve25519_family f4cost <k> <isogeny|isogeny@S*> <t> <G> 100 <2026111500+t> 3600 2 full 0 30000000`
  (12 positional arguments, trace off, as T10).
- 40/40 cells exited 0 on the first try (no F2 reruns, no `*.try*` files), none capped,
  `undecided = 0` everywhere, `targets_done = 100` in every cell. No host restart interrupted the
  run (no `data.interrupted.*`).
- Sizing (not a result): the run envelope was 144 s (start/end in `data/env.txt`, including runner overhead; the per-cell `wall_s` sum to 139.8 s); per-cell wall 0.48–11.50 s (`data/run.log`,
  `wall_s` in each JSON). Counted work spent: 852,770,849 F4 mul-adds summed over the 40 cells.
  No ecbench session, no `/tmp/crypto-bench.lock` taken. Another lead's runner (L12) and the T11
  ecbench session shared the host; counts are deterministic and unaffected.

## Results: all 40 cells (verbatim from `data/cells/*.json` via `analysis/summary.json`)

cols/covered are from f4cost and equal the census (P1). deg = `solving_degree_max`. mpt =
`muladds_per_target`. nat_tot = Σ `native.*_uncharged` over the 100 targets (the frozen script's
`nat`; per target = nat_tot/100, see Deviation D5). E in F4 mul-adds per modelled solve; E_0 uses
no nat and is unaffected by D5; E_1 is the frozen script's value (nat_tot at w = 1).

| cell | cols | covered | deg | mpt | nat_tot | hits | E_0 | E_0.1 (frozen) | E_1 (frozen) |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| k16 t4 G3 S*=1786 | 8 | 58 | 4 | 17,499.43 | 11,512 | 1 | 1.9814e7 | 2.1118e7 | 3.2849e7 |
| k16 t5 G4 seed 0 | 13 | 56 | 4 | 52,422.01 | 23,480 | 1 | 9.9899e7 | 1.0437e8 | 1.4464e8 |
| k16 t5 G4 S*=1597 | 18 | 160 | 4 | 51,412.38 | 25,542 | 3 | 4.7480e7 | 4.9839e7 | 7.1068e7 |
| k16 t5 G3 S*=1597 | 18 | 160 | 5 | 173,865.38 | 17,874 | 3 | 1.6057e8 | 1.6222e8 | 1.7707e8 |
| k16 t6 G5 seed 0 | 28 | 198 | 4 | 141,928.06 | 61,590 | 4 | 1.6476e8 | 1.7191e8 | 2.3626e8 |
| k16 t6 G5 S*=1381 | 40 | 614 | 4 | 139,389.52 | 78,982 | 8 | 7.4544e7 | 7.8768e7 | 1.1678e8 |
| k16 t6 G4 S*=1381 | 40 | 614 | 5 | 673,835.41 | 63,676 | 8 | 3.6036e8 | 3.6376e8 | 3.9441e8 |
| k16 t7 G6 S*=278 | 74 | 1514 | 4 | 388,472.09 | 323,484 | 14 | 1.5587e8 | 1.6885e8 | 2.8566e8 |
| k18 t4 G3 S*=1074 | 6 | 50 | 4 | 17,592.0 | 11,200 | 0 | 6.9426e7 | 7.3846e7 | 1.1363e8 |
| k18 t5 G4 seed 0 | 9 | 14 | 4 | 52,815.6 | 22,400 | 0 | 1.1166e9 | 1.1639e9 | 1.5902e9 |
| k18 t5 G4 S*=997 | 19 | 186 | 4 | 52,572.47 | 23,550 | 1 | 1.7661e8 | 1.8452e8 | 2.5573e8 |
| k18 t5 G3 S*=997 | 19 | 186 | 5 | 176,237.82 | 15,576 | 1 | 5.9205e8 | 5.9729e8 | 6.4438e8 |
| k18 t6 G5 seed 0 | 30 | 230 | 4 | 146,462.83 | 44,800 | 0 | 6.2827e8 | 6.4749e8 | 8.2044e8 |
| k18 t6 G5 S*=1236 | 36 | 480 | 4 | 145,567.05 | 49,078 | 1 | 3.5904e8 | 3.7115e8 | 4.8009e8 |
| k18 t6 G4 S*=1236 | 36 | 480 | 5 | 708,608.92 | 33,290 | 1 | 1.7478e9 | 1.7560e9 | 1.8299e9 |
| k18 t7 G6 S*=1955 | 78 | 1644 | 4 | 418,475.28 | 156,984 | 4 | 6.5296e8 | 6.7745e8 | 8.9791e8 |
| k20 t4 G3 S*=1193 | 8 | 52 | 4 | 17,592.0 | 11,200 | 0 | 3.5510e8 | 3.7770e8 | 5.8117e8 |
| k20 t5 G4 seed 0 | 18 | 72 | 4 | 52,816.0 | 22,400 | 0 | 1.7324e9 | 1.8059e9 | 2.4671e9 |
| k20 t5 G4 S*=1307 | 22 | 210 | 4 | 52,816.0 | 22,400 | 0 | 7.2596e8 | 7.5675e8 | 1.0338e9 |
| k20 t5 G3 S*=1307 | 22 | 210 | 5 | 177,112.0 | 14,400 | 0 | 2.4344e9 | 2.4542e9 | 2.6323e9 |
| k20 t6 G5 seed 0 | 25 | 172 | 4 | 146,464.0 | 44,800 | 0 | 2.7931e9 | 2.8785e9 | 3.6474e9 |
| k20 t6 G5 S*=1308 | 36 | 498 | 4 | 146,464.0 | 44,800 | 0 | 1.3891e9 | 1.4316e9 | 1.8141e9 |
| k20 t6 G4 S*=1308 | 36 | 498 | 5 | 710,095.08 | 28,800 | 0 | 6.7349e9 | 6.7623e9 | 7.0081e9 |
| k20 t7 G6 S*=1594 | 83 | 1840 | 4 | 417,161.31 | 156,704 | 4 | 2.4689e9 | 2.5617e9 | 3.3964e9 |
| k22 t4 G3 S*=1002 | 13 | 88 | 4 | 17,592.0 | 11,200 | 0 | 1.3637e9 | 1.4505e9 | 2.2319e9 |
| k22 t5 G4 seed 0 | 13 | 40 | 4 | 52,816.0 | 22,400 | 0 | 9.0071e9 | 9.3891e9 | 1.2827e10 |
| k22 t5 G4 S*=1034 | 21 | 224 | 4 | 52,816.0 | 22,400 | 0 | 2.5982e9 | 2.7084e9 | 3.7001e9 |
| k22 t5 G3 S*=1034 | 21 | 224 | 5 | 177,112.0 | 14,400 | 0 | 8.7128e9 | 8.7836e9 | 9.4212e9 |
| k22 t6 G5 seed 0 | 32 | 270 | 4 | 146,464.0 | 44,800 | 0 | 9.1086e9 | 9.3873e9 | 1.1895e10 |
| k22 t6 G5 S*=284 | 34 | 458 | 4 | 146,464.0 | 44,800 | 0 | 5.7053e9 | 5.8798e9 | 7.4505e9 |
| k22 t6 G4 S*=284 | 34 | 458 | 5 | 710,096.0 | 28,800 | 0 | 2.7661e10 | 2.7773e10 | 2.8783e10 |
| k22 t7 G6 S*=454 | 81 | 1906 | 4 | 419,290.4 | 122,592 | 2 | 9.3500e9 | 9.6234e9 | 1.2084e10 |
| k24 t4 G3 S*=1717 | 12 | 78 | 4 | 17,592.0 | 11,200 | 0 | 5.6764e9 | 6.0378e9 | 9.2903e9 |
| k24 t5 G4 seed 0 | 17 | 76 | 4 | 52,816.0 | 22,400 | 0 | 2.4778e10 | 2.5829e10 | 3.5287e10 |
| k24 t5 G4 S*=978 | 20 | 190 | 4 | 52,816.0 | 22,400 | 0 | 1.1660e10 | 1.2155e10 | 1.6606e10 |
| k24 t5 G3 S*=978 | 20 | 190 | 5 | 177,112.0 | 14,400 | 0 | 3.9102e10 | 3.9420e10 | 4.2281e10 |
| k24 t6 G5 seed 0 | 39 | 364 | 4 | 146,464.0 | 44,800 | 0 | 3.2913e10 | 3.3920e10 | 4.2980e10 |
| k24 t6 G5 S*=1340 | 29 | 380 | 4 | 146,464.0 | 44,800 | 0 | 2.3443e10 | 2.4160e10 | 3.0614e10 |
| k24 t6 G4 S*=1340 | 29 | 380 | 5 | 710,096.0 | 28,800 | 0 | 1.1366e11 | 1.1412e11 | 1.1827e11 |
| k24 t7 G6 S*=702 | 86 | 1864 | 4 | 424,021.45 | 106,306 | 1 | 4.1031e10 | 4.2060e10 | 5.1318e10 |

### E_best(k) and the fit

| k | r | best (t, G, seed) | E_0 | E_1 (frozen) | E_1 (nat per target, D5) | E(t=5)/E(t=4), E_1 frozen / E_0 |
|--:|--:|:--|--:|--:|--:|--:|
| 16 | 8,209 | 4, 3, 1786 | 1.9814e7 | 3.2849e7 | 1.9945e7 | 2.163 / 2.396 |
| 18 | 32,887 | 4, 3, 1074 | 6.9426e7 | 1.1363e8 | 6.9868e7 | 2.251 / 2.544 |
| 20 | 131,203 | 4, 3, 1193 | 3.5510e8 | 5.8117e8 | 3.5736e8 | 1.779 / 2.044 |
| 22 | 524,731 | 4, 3, 1002 | 1.3637e9 | 2.2319e9 | 1.3724e9 | 1.658 / 1.905 |
| 24 | 2,097,349 | 4, 3, 1717 | 5.6764e9 | 9.2903e9 | 5.7125e9 | 1.787 / 2.050 |

| fit of ln E_best = a + b ln r | b | a | rmse(ln) | local slopes 16→18, 18→20, 20→22, 22→24 |
|:--|--:|--:|--:|:--|
| frozen, w = 1 (**decides P5**) | **1.0297** | 7.968 | 0.0725 | 0.894, 1.180, 0.971, 1.029 |
| frozen, w = 0.1 | 1.0312 | 7.517 | 0.0708 | 0.902, 1.180, 0.971, 1.029 |
| w = 0 (nat-free) | 1.0315 | 7.451 | 0.0704 | 0.903, 1.180, 0.971, 1.029 |
| nat per target (D5), w = 0.1 / w = 1 | 1.0315 / 1.0315 | 7.452 / 7.458 | 0.0704 / 0.0705 | 0.903, 1.180, 0.971, 1.029 |

The fit has 5 points and 2 parameters; the residual spread is the rmse above and no confidence
interval is claimed. The local-slope swing (0.89–1.18) is the census seed quality: the
cols/covered ratio at S*(k, 4) is 0.138, 0.120, 0.154, 0.148, 0.154 for k = 16 … 24 (§2.2 spread
≤ 1.3×), and k = 18 happens to have the best t = 4 seed.

Fixed-t exponents (S*, G = t−1; frozen w = 1 / w = 0): t = 4 1.030 / 1.031; t = 5 0.980 / 0.988;
t = 6 1.002 / 1.030; t = 7 0.937 / 0.997.

## Predictions (decided by `scripts/analyze.py`, frozen rules; nothing retuned)

| # | prediction (rule) | outcome | measured |
|:--|:--|:--|:--|
| P0 | 7 k = 20 cells shared with T10 reproduce mpt within 1e-3 relative, all 7 | **held** | all 7 at relative difference 0.0 (52,816.0; 52,816.0; 177,112.0; 146,464.0; 146,464.0; 710,095.08; 417,161.31) |
| P1 | f4cost columns and covered_points equal the census in all 40 cells | **held** | 40/40 match, no mismatches |
| P2 | k-invariance: max/min mpt across the 5 k ≤ 1.10 in 4/4 primary (t, t−1) series | **held** | 1.0053 (t4 g3), 1.0273 (t5 g4), 1.0508 (t6 g5), 1.0915 (t7 g6); non-deciding G = t−2 series 1.0187 (t5 g3), 1.0538 (t6 g4). **Point predictions missed** (≤ 1.0001 at t = 4–6, ≤ 1.03 at t = 7): see hit-rate note below |
| P3 | 0.90 ≤ mpt(S*)/mpt(seed 0) ≤ 1.10 at (t, t−1), t ∈ {5, 6}, all 10 pairs | **held** | 0.9807, 0.9821 (k16 t5, t6); 0.9954, 0.9939 (k18); 1.0 at k = 20, 22, 24 |
| P4 | argmin t = 4 at every k, second-best t = 5 at every k, E(t=5)/E(t=4) ≥ 1.5 at every k (all three clauses, all five k) | **held** | argmin 4 and second 5 at all five k; ratios 2.163, 2.251, 1.779, 1.658, 1.787 (frozen E_1); point predictions 2.45, 2.56, 2.04, 1.91, 2.05 were made at w = 0, where the measured ratios are 2.396, 2.544, 2.044, 1.905, 2.054 |
| P5 | exponent b of E_best in r in [0.85, 1.15] | **held** | b = 1.0297 (point prediction 1.03) |
| P6 | E_best/(1.5·√r) strictly increasing over k = 16 … 24 (w = 1) | **held** | 2.417e5 < 4.177e5 < 1.070e6 < 2.054e6 < 4.277e6 |

Inconclusive rule: not triggered (all 20 primary cells present, exit 0, uncapped, `undecided = 0`).

**Hit-rate note on P2 (the mechanism behind the missed point predictions).** In the four primary G = t−1 series mpt is
identical to the last digit across all cells with `hits = 0` (k ≥ 18 at t = 4; k ≥ 20 at t = 5, 6;
k = 24 alone at t = 7; the non-deciding t = 6, G = 4 series is the exception, see E4) and falls below that value in proportion to the hit count: at t = 7, G = 6
mpt/mpt(k=24) = 0.916 (14 hits), 0.987 (4), 0.984 (4), 0.989 (2), 1.0 (1). A target whose
specialisation hits a factor-base point stops its remaining branches early, so the 100-target
average is lower where coverage/r is larger. The per-target cost of a *non*-hitting target is
k-invariant to the digit; the modelled E uses the average and so inherits a downward bias of 8.4 % at
k = 16 (t = 7: 388,472.09 / 424,021.45 = 0.916; 0.53 % in E_best(16), t = 4) that vanishes as r grows. This is a reporting observation, not a retuned rule.

## What t-growth would be needed to beat exponent 1 (reported, §6 of the protocol)

Write E = g(t)·r with g(t) = columns·mpt/covered at S*. Measured g and its per-step ratio
g(t+1)/g(t), with the growth of columns·mpt and of covered_points per step:

| k | g(4) | g(5) | g(6) | g(7) | g5/g4 | g6/g5 | g7/g6 | cols·mpt growth 4→5, 5→6, 6→7 | covered growth 4→5, 5→6, 6→7 |
|--:|--:|--:|--:|--:|--:|--:|--:|:--|:--|
| 16 | 2,414 | 5,784 | 9,081 | 18,987 | 2.40 | 1.57 | 2.09 | 6.61, 6.02, 5.16 | 2.76, 3.84, 2.47 |
| 18 | 2,111 | 5,370 | 10,918 | 19,855 | 2.54 | 2.03 | 1.82 | 9.46, 5.25, 6.23 | 3.72, 2.58, 3.42 |
| 20 | 2,706 | 5,533 | 10,588 | 18,818 | 2.04 | 1.91 | 1.78 | 8.26, 4.54, 6.57 | 4.04, 2.37, 3.69 |
| 22 | 2,599 | 4,952 | 10,873 | 17,819 | 1.91 | 2.20 | 1.64 | 4.85, 4.49, 6.82 | 2.55, 2.04, 4.16 |
| 24 | 2,706 | 5,560 | 11,178 | 19,563 | 2.05 | 2.01 | 1.75 | 5.00, 4.02, 8.59 | 2.44, 2.00, 4.91 |

- g grows by **1.57–2.54× per t step** (median 2.0×) at every k, so raising t never pays in this
  model and the optimum sits at the smallest t measured (t = 4). The g(t) values at fixed t are
  k-invariant to within the seed spread (2,111–2,706 at t = 4), which is exactly why b ≈ 1.
- Exponent ½ (rho's) needs g(t(k)) ∝ r^−½, i.e. g must **halve for every 2 bits of r**. If t were
  raised by one step per 2 bits, each step would have to *lower* g by 2×; with columns·mpt growing
  4–9.5× per step, covered_points would have to grow **8–19× per step** (2 × the columns·mpt growth)
  instead of the measured 2.0–4.9×. Nothing in the census or in the measured series moves in that
  direction, so within this factor-base family the exponent stays at 1 unless the per-step cost
  growth of F4 (columns·mpt) is cut by ~4× per step, which is a different lead.
- Follow-ups that would need a new protocol (not run here, listed as open): t = 3 (the trend
  predicts g(3) ≈ 1,200 if the pattern holds, but the census has no t = 3 row and f4cost was not
  run at t = 3); seed 0 at t = 4 and 7.

## Distance remaining to rho-neg (model, F4 phase only; rho S = 1.5 as the brief fixes)

Unit-mixed (F4 mul-adds priced at w against GAE), F4 phase only; at w = 0 F4 work is unpriced and
the ratio is 0 by construction. Rho-neg S = 1.5 is a k = 20 measurement (T12: 1.544 / 1.654 / 2.651
at w = 0 / 0.1 / 1) applied heuristically at the other k.

| k | rho-neg GAE 1.5·√r | E_best/GAE, w = 1 (frozen, decides P6) | w = 1, nat per target (D5) | w = 0.1 (frozen) | w = 0.1 (D5) |
|--:|--:|--:|--:|--:|--:|
| 16 | 135.9 | 2.417e5 | 1.468e5 | 1.554e4 | 1.459e4 |
| 18 | 272.0 | 4.177e5 | 2.569e5 | 2.715e4 | 2.554e4 |
| 20 | 543.3 | 1.070e6 | 6.577e5 | 6.952e4 | 6.540e4 |
| 22 | 1,086.6 | 2.054e6 | 1.263e6 | 1.335e5 | 1.256e5 |
| 24 | 2,172.3 | 4.277e6 | 2.630e6 | 2.779e5 | 2.615e5 |

The gap grows as r^0.53 (fit over the five k, either column). The measured end-to-end distance at
k = 20 is T12's **6.06·10⁴× behind rho-neg at w = 0.1** (17.1× at w = 0) for the T10 winner
(t = 5, G = 4, seed 1307). The modelled F4 gain of t = 4 over that winner at k = 20 is 2.044× (w = 0);
if it held end to end the projected distance would be about **3.0·10⁴× at w = 0.1**. That is a
projection from a model, not a measurement, and it ignores linear algebra (∝ columns²), group
operations, the per-curve seed search and relation slack. **Rho remains ahead on every track.**
The next end-to-end candidate this suggests is `prime-isogeny:t=4,seed=S*(k,4)` with
`pkm-semaev:m=2,mode=full,guess=3` (T12-style ecbench session; not run here).

## Exploratory items (labelled; not predictions)

- E1. Seed-0 control cost ratio E(seed 0)/E(S*) at G = t−1, w = 0: t = 5: 2.10, 6.32, 2.39, 3.47,
  2.13; t = 6: 2.21, 1.75, 2.01, 1.60, 1.40 (k = 16 … 24). The J/columns selector gives 1.4–6.3×
  at every (k, t); the k = 18, t = 5 seed-0 outlier is its 9 columns / 14 covered points.
- E2. The second-best G (G = t−2) is 3.35–3.38× (t = 5) and 4.83–4.87× (t = 6) behind G = t−1 in E_0
  at every k, confirming T10's argmin G = t−1 across k.
- E3. Per-target nat (D5) is 0.04–0.83 % of mpt in every cell, so the w band changes E by < 1 %
  once nat is per target; the frozen E_1 column overstates it (see D5).
- E4. nproc reported 3 on the shared host during the run (the brief says 4 cores); irrelevant to
  counts.

## Deviations and pre-run steps (recorded, none changes a verdict)

- D1. **Resume checkpoint not pushed.** `scripts/checkpoint.sh status` ran (target
  `s3://setec-astronomy-artifacts/checkpoints/crypto/claude__curve25519-index-calculus-i674j4/`,
  "RESUME.md: ok", "last push: stale or none"). The container has no upload token or AWS
  identity, `.resume/RESUME.md` describes a different effort (arity completion) and this lead may
  write only inside its own directory, so `checkpoint.sh init`/`push` were not run. The frozen
  runner is not hook-gated and ran.
- D2. **Conductor absent** (`conductor: command not found`); no task card attached. Edits were
  confined to this lead directory (README, `analysis/`, `data/` via the frozen runner).
- D3. **Route.** Shared host as the wave brief directs, not `isolab-eks`; counted units are
  host-independent.
- D4. **One premature analysis.** A wait loop misidentified the runner's process and
  `analyze.py` was invoked once while 13 of 40 cells existed; its output was discarded
  (`analysis/` removed) and the analysis was re-run on the complete `data/` after `ALL_DONE`. The
  frozen runner was never restarted, so no cell was produced twice.
- D5. **nat scale in the frozen analysis.** PROTOCOL §1 defines nat as the *per-target*
  `native.*_uncharged` count; the frozen `analyze.py` sums the JSON fields, which are totals over
  the 100 targets (22,400 at t5 g4 = T10's 224 per target × 100). The frozen E_0.1/E_1 and the
  P4/P5/P6 decisions therefore use nat at 100× its protocol value. Decided strictly by the frozen
  script as the rules require; the corrected per-target values are reported alongside (tables
  above) and change no verdict: b 1.0297 → 1.0315, P4 argmin/second unchanged with ratios
  2.39–2.54 / 1.90–2.05, P6 still strictly increasing. No script was edited.
- D6. No git operations, no ecbench, no rebuilds, no extra cells, targets, seeds or t values.

## Files

- `PROTOCOL.md`, `scripts/{run.sh,analyze.py,select_seeds.py}`, `selection/seeds.json` (frozen,
  unchanged; seeds sha256 `1e6195046fbd445a3551a6726ac4df7e3ee08976f3814590ac904909e3338f7d`).
- `data/cells/k<k>_t<t>_g<G>_s<seed>.{json,stderr,status}` (40 cells), `data/run.log`,
  `data/env.txt`, `data/binary.sha256`, `data/seeds.used.json`, `data/SHA256SUMS`, `data/ALL_DONE`.
- `analysis/summary.json` (frozen analysis output), `analysis/SHA256SUMS`.

## Validator errata (applied)

Source: `validator-report.json` (verdict pass_with_errata, validated 2026-11-04T07:02:04Z). Only this README was edited; PROTOCOL.md, scripts/, data/, selection/, analysis/ and the report are untouched. **All prediction verdicts (P0-P6) are unchanged (all held).**

- **E1, applied.** The k = 18 prime-order size is r_18 = 32,887, not 32,719 (the frozen scripts and seeds.json still carry 32,719; disclosed, not fixable here). Corrected here: r in the E_best table; all 24 E values in the eight k = 18 rows of the 40-cell table (scaled by 32887/32719 = 1.0051, the model is linear in r); E_best(18) = 6.9426e7 (E_0), 1.1363e8 (E_1 frozen), 6.9868e7 (E_1 D5); 1.5·sqrt(r_18) 271.3 -> 272.0; k = 18 distance ratios 4.166e5 -> 4.177e5 (w = 1 frozen), 2.562e5 -> 2.569e5 (D5), 2.708e4 -> 2.715e4 (w = 0.1 frozen), 2.547e4 -> 2.554e4 (D5); local slope 18->20 1.179 -> 1.180 (16->18 stays 0.894). Recomputed exponent b = 1.02965 (frozen w = 1), 1.0315 (w = 0): unchanged to 4 decimals (moves by < 1e-4). P4 within-k ratios, mpt/g(t) and hits are unaffected; P6 stays strictly increasing. The frozen analysis/summary.json still shows the uncorrected k = 18 values.
- **E2, applied.** The frozen analyze.py prices nat as the 100-target total. The frozen b = 1.0297 (w = 1; decides P5) is therefore the frozen-script value; the per-target-corrected b = 1.0315 (D5 columns, w = 0.1 and w = 1) is shown beside it in the Answer and the fit table. The frozen E_0.1/E_1 columns overstate nat 100x (E_1 by up to 1.83x at k16 t7); E_0 is unaffected. Not fixable in the script; no verdict changes.
- **E3, applied.** Sizing: per-cell wall_s 0.48-11.50 s, summing to 139.8 s; 144 s is the env.txt start/end envelope including runner overhead.
- **E4, applied.** The "identical to the last digit" statement is restricted to the four primary G = t-1 series. In the non-deciding t = 6, G = 4 series hits = 0 at k = 20, 22, 24 but mpt is 710,095.08 / 710,096.0 / 710,096.0 (spread 1.3e-6, seed-dependent, as T4 documented); "710,096.0 at k >= 20" was removed from the P2 bullet.
- **E5, applied.** Downward bias at k = 16 is 8.4 % (t = 7: 388,472.09 / 424,021.45 = 0.916), 0.53 % in E_best(16) (t = 4).
- **E6, disclosed, not fixable here.** PROTOCOL.md's header still says "not yet frozen" and never records the hash; the freeze is established by commit 5a800bb3 (blob ids identical).
- **E7, applied (disclosure).** The distance columns at w = 0.1 and w = 1 use a fixed rho-neg S = 1.5. T12 measured rho-neg S = 1.544 / 1.654 / 2.651 at w = 0 / 0.1 / 1 (k = 20), so the quoted w = 0.1 and w = 1 gaps are conservative upper bounds, too large by at most 1.103x and 1.767x (about 1.77x) respectively; P6 monotonicity and the r^0.53 slope are unaffected.
- **E8, disclosed, not fixable here.** PROTOCOL §2.4's E(t = 7) predictions (1.69e8, 6.52e8, 2.49e9, 9.37e9, 4.06e10) use the seed-0 prior mpt(7,6) = 423,619.64, not the seed-1594 value 417,161.31 (which gives 1.67e8, 6.48e8, 2.47e9, 9.30e9, 4.04e10). No decision depends on t = 7.
