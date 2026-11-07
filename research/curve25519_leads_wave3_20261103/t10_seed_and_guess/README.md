# T10: joint choice of tower height t, guess split G and tower seed (k = 20, isogeny factor base)

**Class:** exploratory engine measurement in counted units (F4 multiply-adds from
`curve25519_family f4cost`). The result is a *modelled* end-to-end F4 cost
`E_w = columns * (mpt + w * nat) / coverage_exact`, not an end-to-end solve, not an ecbench
session and not a leaderboard row. Rho remains ahead; nothing here is an improvement claim until
an end-to-end test (T9-style) measures it.

Protocol: [PROTOCOL.md](PROTOCOL.md) with `scripts/` (frozen at `505ecbb154f907657e6b3865699d41a54c550406`
per the wave brief). Run: 2026-11-03 21:59:52Z to 22:04:17Z, host `vm`, 4 cores.
Analysis: `python3 -I scripts/analyze.py <lead dir>` -> `analysis/summary.json`
(sha256 `c283d71d42c7b5abd9a08e8a80be2049ea8c1a597a566c887b6591dbbd0f55be`).

## Answer

- **Winner (argmin E_0 over all 36 cells): t = 5, G = 4, seed 1307**, E_0 = 725,959,944 F4 mul-adds
  per modelled solve.
- Incumbent (t = 6, G = 4, seed 0): E_0 = 13,541,637,076. **Ratio 18.65x** (predicted 19.3x).
- Best (t, G) other than (6, 4), against (6, 4) at its better seed 1308: **9.28x** (> 1.5x trigger).
- **Recommendation (P6 held): `prime-isogeny:t=5,seed=1307` with `pkm-semaev:m=2,mode=full,guess=4`
  is the next end-to-end candidate.** Runner-up: t = 6, G = 5, seed 1308 (E_0 = 1.389·10⁹, 1.91x behind the winner).

## Instrument

- Binary `/home/user/ctarget/release/curve25519_family`, sha256
  `ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef` (= frozen; checked before every cell).
- Census `research/curve25519_isogeny_seed_20261103/data/k20.jsonl`, sha256
  `bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc` (= frozen).
- Command per cell: `curve25519_family f4cost 20 <isogeny|isogeny@S> <t> <G> 100 <2026111500+t> 3600 2 full 0 30000000`.
- 36/36 cells exited 0 on the first try (no F2 reruns), none capped, `undecided = 0` everywhere,
  100/100 targets in every cell.

## Results (verbatim from `data/cells/*.json` via `analysis/summary.json`)

cols / covered are from f4cost and equal the census in every cell (P1). nat = per-target
`native.pkm_system_fp_mults_uncharged`. E in F4 mul-adds per modelled solve.

| t | G | seed | cols | covered | deg | mpt (muladds_per_target) | nat | E_0 | E_0.1 | E_1 |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 5 | 0 | 0 | 18 | 72 | 5 | 3,957,255.0 | 18 | 1.298e11 | 1.298e11 | 1.298e11 |
| 5 | 0 | 1307 | 22 | 210 | 5 | 3,957,252.33 | 18 | 5.439e10 | 5.439e10 | 5.439e10 |
| 5 | 1 | 0 | 18 | 72 | 5 | 3,174,436.77 | 36 | 1.041e11 | 1.041e11 | 1.041e11 |
| 5 | 1 | 1307 | 22 | 210 | 5 | 3,174,438.37 | 36 | 4.363e10 | 4.363e10 | 4.363e10 |
| 5 | 2 | 0 | 18 | 72 | 5 | 820,171.95 | 72 | 2.690e10 | 2.690e10 | 2.690e10 |
| 5 | 2 | 1307 | 22 | 210 | 5 | 820,171.93 | 72 | 1.127e10 | 1.127e10 | 1.127e10 |
| 5 | 3 | 0 | 18 | 72 | 5 | 177,111.46 | 144 | 5.809e9 | 5.810e9 | 5.814e9 |
| 5 | 3 | 1307 | 22 | 210 | 5 | 177,112.0 | 144 | 2.434e9 | 2.435e9 | 2.436e9 |
| 5 | 4 | 0 | 18 | 72 | 4 | 52,816.0 | 224 | 1.732e9 | 1.733e9 | 1.740e9 |
| **5** | **4** | **1307** | 22 | 210 | 4 | **52,816.0** | 224 | **7.260e8** | **7.263e8** | **7.290e8** |
| 6 | 0 | 0 | 25 | 172 | 5 | 55,320,544.22 | 18 | 1.055e12 | 1.055e12 | 1.055e12 |
| 6 | 0 | 1308 | 36 | 498 | 5 | 55,320,550.78 | 18 | 5.247e11 | 5.247e11 | 5.247e11 |
| 6 | 1 | 0 | 25 | 172 | 5 | 23,998,404.31 | 36 | 4.577e11 | 4.577e11 | 4.577e11 |
| 6 | 1 | 1308 | 36 | 498 | 5 | 23,998,426.92 | 36 | 2.276e11 | 2.276e11 | 2.276e11 |
| 6 | 2 | 0 | 25 | 172 | 5 | 10,835,314.21 | 72 | 2.066e11 | 2.066e11 | 2.066e11 |
| 6 | 2 | 1308 | 36 | 498 | 5 | 10,835,321.87 | 72 | 1.028e11 | 1.028e11 | 1.028e11 |
| 6 | 3 | 0 | 25 | 172 | 5 | 3,065,449.78 | 144 | 5.846e10 | 5.846e10 | 5.846e10 |
| 6 | 3 | 1308 | 36 | 498 | 5 | 3,065,479.69 | 144 | 2.907e10 | 2.907e10 | 2.908e10 |
| 6 | 4 | 0 (incumbent) | 25 | 172 | 5 | 710,094.0 | 288 | 1.354e10 | 1.354e10 | 1.355e10 |
| 6 | 4 | 1308 | 36 | 498 | 5 | 710,095.08 | 288 | 6.735e9 | 6.735e9 | 6.738e9 |
| 6 | 5 | 0 | 25 | 172 | 4 | 146,464.0 | 448 | 2.793e9 | 2.794e9 | 2.802e9 |
| 6 | 5 | 1308 | 36 | 498 | 4 | 146,464.0 | 448 | 1.389e9 | 1.390e9 | 1.393e9 |
| 7 | 0 | 0 | 65 | 1188 | 5 | 745,197,464.48 | 190.8 | 5.349e12 | 5.349e12 | 5.349e12 |
| 7 | 0 | 1594 | 83 | 1840 | 5 | 745,338,004.73 | 709.2 | 4.411e12 | 4.411e12 | 4.411e12 |
| 7 | 1 | 0 | 65 | 1188 | 5 | 227,080,791.11 | 208.8 | 1.630e12 | 1.630e12 | 1.630e12 |
| 7 | 1 | 1594 | 83 | 1840 | 5 | 222,890,910.35 | 726.48 | 1.319e12 | 1.319e12 | 1.319e12 |
| 7 | 2 | 0 | 65 | 1188 | 5 | 89,804,859.16 | 244.44 | 6.447e11 | 6.447e11 | 6.447e11 |
| 7 | 2 | 1594 | 83 | 1840 | 5 | 88,236,443.15 | 761.58 | 5.222e11 | 5.222e11 | 5.222e11 |
| 7 | 3 | 0 | 65 | 1188 | 5 | 25,762,377.25 | 315.72 | 1.849e11 | 1.849e11 | 1.849e11 |
| 7 | 3 | 1594 | 83 | 1840 | 5 | 25,503,942.46 | 832.68 | 1.509e11 | 1.509e11 | 1.509e11 |
| 7 | 4 | 0 | 65 | 1188 | 5 | 9,624,332.46 | 458.46 | 6.909e10 | 6.909e10 | 6.909e10 |
| 7 | 4 | 1594 | 83 | 1840 | 5 | 9,455,075.95 | 971.82 | 5.596e10 | 5.596e10 | 5.596e10 |
| 7 | 5 | 0 | 65 | 1188 | 5 | 1,844,976.33 | 744.66 | 1.324e10 | 1.324e10 | 1.325e10 |
| 7 | 5 | 1594 | 83 | 1840 | 5 | 1,810,250.18 | 1252.26 | 1.071e10 | 1.071e10 | 1.072e10 |
| 7 | 6 | 0 | 65 | 1188 | 4 | 423,619.64 | 1062.22 | 3.041e9 | 3.042e9 | 3.049e9 |
| 7 | 6 | 1594 | 83 | 1840 | 4 | 417,161.31 | 1567.04 | 2.469e9 | 2.470e9 | 2.478e9 |

Top of the w = 0 ranking (exact E_0): t5_g4_s1307 725,959,944; t6_g5_s1308 1,389,145,749;
t5_g4_s0 1,732,404,412; t5_g3_s1307 2,434,417,934; t7_g6_s1594 2,468,925,910; t6_g5_s0 2,793,098,284;
t7_g6_s0 3,041,002,437; t5_g3_s0 5,809,388,722; t6_g4_s1308 6,734,935,285; ...; t6_g4_s0 13,541,637,076 (13th).
The predicted top five (§4) came out in exactly the predicted order.

Hits (in-base targets found among the 100): 0 at t = 5 and t = 6; at t = 7, 1 (seed 0) and 4 (seed 1594)
in every G, all verified (`hits_verified = hits_exact = hits`). Hits do not enter E.

## Predictions (decided by the frozen rules in `scripts/analyze.py`)

| | prediction | outcome |
|:--|:--|:--|
| P0 | mpt(6,4,0) = 710,094.0 within 1e-3 | **held**: 710,094.0, rel. 0.0. No drift from the `7a438b1d…` binary. |
| P1 | f4cost cols/covered = census in all 36 cells | **held**: 0 mismatches |
| P2 | 0.90 ≤ mpt(sel)/mpt(0) ≤ 1.10 for all 18 (t,G) | **held**: t = 5, 6 within 1.0 ± 1·10⁻⁵; t = 7 ratios 0.9812–1.0002 |
| P3 | every G -> G+1 step ratio ≥ 1.25 and argmin G = t−1, all 6 series | **failed**: argmin is G = t−1 in all 6 series, but the t = 5 step G 0 -> 1 is 1.2466 (seed 0) and 1.2466 (seed 1307), below 1.25. All other steps 2.21–5.22. |
| P4 | argmin E_0 = (5, 4, 1307) | **held** (no inconclusive cells) |
| P5 | E_0(6,4,0)/E_0(winner) ≥ 10 (point 19.3) | **held**: 18.65 |
| P6 | best other (t,G) beats best (6,4) by > 1.5x (point 9.6) | **held**: t5_g4_s1307 vs t6_g4_s1308, 9.28x -> recommend |
| P7 | incumbent/winner > 1 at w = 0, 0.1, 1 | **held**: 18.653, 18.646, 18.582 |
| P8 | 3.0·10⁴ ≤ mpt(5,4,0) ≤ 8.0·10⁴ (point 5.1·10⁴) | **held**: 52,816.0 |

P3 fails on a threshold, not on the argmin: guessing still helps at every step, but the first t = 5
step is weaker than the frozen 1.25. Nothing was retuned.

## Exploratory observations (not preregistered)

- Seed invariance of mpt is near-exact at t = 5 and 6; at t = 7 the selected seed is 1–2% cheaper
  per target, and its larger hit count (4 vs 1) is the likely cause (hit targets stop early). Not tested.
- Derived F4 mul-adds per relation at exact coverage (`muladds_per_relation_at_exact_coverage`):
  winner 3.30·10⁷ against 5.42·10⁸ for the incumbent seed 0 (T4).
- The model's input `nat` is small relative to mpt, so P7 is near-automatic; the band ratio barely moves with w.
- t = 5 has fewer columns (22) than t = 6 seed 1308 (36), so the ignored linear-algebra term (∝ cols²) would
  also favour the winner. Not measured.

## Costs

- This lead: 36 f4cost cells, 3,600 targets. Total F4 work is the sum of `f4_muladds` over cells
  (each cell's `f4_muladds` = 100 × mpt). Wall about 4.5 min (sizing only, not a result).
- Seed search: none new. The selected seeds come from the existing 2,000-seed census. Its recorded group
  operations (build + selector, summed over 2,000 seeds) are 4,592,473 + 2,342,798 = 6.94·10⁶ (t = 5),
  6,766,535 + 6,762,024 = 1.35·10⁷ (t = 6), 11,122,302 + 21,635,056 = 3.28·10⁷ (t = 7). E excludes them;
  an end-to-end comparison must charge them or argue them away as one-time per curve.

## Distance remaining to rho-neg

- Measured (T5 session `ECBS1h405f20219036`, k = 20, 6 targets): the best algebraic arm (seed 1093, t = 6,
  G = 4) is 18.3x behind rho-neg at w = 0, 6.9·10⁵x at w = 0.1, 4.4·10⁶x at w = 1. That is still the
  measured distance; T10 measured nothing end-to-end.
- **Projection only:** if the 18.65x F4 reduction against the seed-0 incumbent carried over end-to-end in full,
  the seed-0 arm's S would fall from 1.674·10⁶ to about 8.97·10⁴ at w = 0.1 (about 5.8·10⁴x behind rho-neg) and from
  1.674·10⁷ to about 8.97·10⁵ at w = 1 (about 3.7·10⁵x behind). F4 work is unpriced in ecbench, so w = 0 is unaffected
  by this change apart from relation-collection effects. **Rho remains ahead; no row is a leaderboard claim.**

## Deviations

- **Route:** run locally on the shared 4-core host as the wave brief directs, not through `isolab-eks`
  (recorded in the protocol; counted units do not depend on the host).
- **Resume checkpoint** (`scripts/checkpoint.sh init`) and **Conductor check** were not run: the wave
  brief allows writes only inside this lead directory, and `conductor` is not installed on this host.
- **PROTOCOL.md header** still reads "Status: design, not yet frozen"; the freeze commit
  `505ecbb154f9…` comes from the wave brief. Git use was forbidden, so this session did not verify it.
- **G range:** G = t is infeasible (`pkm-semaev` refuses guess ≥ t), so G ∈ {0 … t−1}, as frozen.
- **Visual report** (graph, PDF, `benchmarks/` entry) was not produced: out of scope for this lead's brief,
  and the result is exploratory and not a benchmark row.

## Files

- `data/cells/t<t>_g<G>_s<seed>.{json,stderr,status}` (36 cells), `data/run.log`, `data/env.txt`,
  `data/binary.sha256`, `data/seeds.used.json`, `data/ALL_DONE`.
- `data/SHA256SUMS` (written by the frozen runner, 112 entries; `sha256sum -c` passes).
- `analysis/summary.json`.
