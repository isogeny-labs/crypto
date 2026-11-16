# T10 protocol: joint choice of tower height t, guess split G and tower seed (isogeny factor base, k = 20)

Wave 3, lead T10. Written 2026-11-03, before any T10 data. **Status: design, not yet frozen.**
The protocol is frozen when its commit hash is recorded here by whoever freezes it. After that,
nothing in it changes; deviations go to an addendum.

**Class.** Exploratory engine measurement in counted units (F4 multiply-adds from
`curve25519_family f4cost`). The outcome is a *modelled* end-to-end F4 cost. It is not an
end-to-end solve, not an ecbench session, and not a leaderboard row. T9 covers end-to-end.

## 1. Question

The F4 work per relation is `mpt(t, G, seed) / coverage(seed, t)`. A solve needs about
`columns(seed, t)` relations. So the modelled F4 work of one solve is

    E_w(t, G, s) = columns(s, t) * (mpt(t, G, s) + w * nat(t, G, s)) / coverage_exact(s, t)

- `mpt` is `muladds_per_target`.
- `nat` is the per-target sum of the `native.*_uncharged` counters (Fp multiplications that F4
  does not charge), priced at w ∈ {0, 0.1, 1}.
- `coverage_exact` is `covered_points / r`, with r = 131203.

`columns` and `coverage` depend only on (seed, t), and the census already gives them. `mpt` depends on
(t, G) and, by the T4 engine check, only weakly on the seed: seed 1093 ran at 0.991× of seed 0 at t = 6, G = 4.
**Which (t, G, seed) minimises E, and does it beat the incumbent (t = 6, G = 4)?**

The model already predicted 0.599 against a measured 0.641 for the seed change (T5). Here it is
only used to rank configurations. A ranking is a candidate for T9-style end-to-end tests, not a result.

## 2. Inputs computed now from existing data (no new measurement)

### 2.1 Seeds (`selection/seeds.json`, by `scripts/select_seeds.py`)

- Source: `research/curve25519_isogeny_seed_20261103/data/k20.jsonl`, sha256
  `bc74d8ac5151713c59f6c6d5bf58df1c0bf5e3442c6b50c7d234163c4c807adc` (matches that lead's `SHA256SUMS`).
  It holds 2,000 seeds per t.
- Rule: argmax J / columns (log-free), with ties broken by larger J and then by smaller seed. This is the T9 selector applied
  per t. Seed 0 is the reference.

| t | seed 0: cols / covered | selected seed | cols / covered | census (cols/cov) ratio sel ÷ seed 0 |
|--:|:--|--:|:--|--:|
| 5 | 18 / 72 | **1307** | 22 / 210 | 0.419 |
| 6 | 25 / 172 | **1308** | 36 / 498 | 0.497 |
| 7 | 65 / 1188 | **1594** | 83 / 1840 | 0.824 |

For reference only (not measured here): the J-only selections are 1550 (t = 5), 1093 (t = 6) and
804 (t = 7). Seed 1308 at t = 6 is also T9's seed.

### 2.2 Prior per-target costs that the predictions rest on (seed 0 unless stated; k = 20, isogeny, m = 2, full)

| t | G | mpt | source |
|--:|--:|--:|:--|
| 5 | 0 | 3.957·10⁶ | `curve25519_cofactor_algebraic_20261102`; `t7_t0_frames` isosel/iso1093 |
| 5 | 3 | 1.771·10⁵ (seeds 1093, 1550; k24 seeds 0 and 797 give 177,112) | `t7_t0_frames/data/engine`, `t4_seed_replication` |
| 5 | 1, 2, 4 | not measured | — |
| 6 | 0 … 4 | 5.55·10⁷, 2.40·10⁷, 1.08·10⁷, 3.07·10⁶, 7.10·10⁵ | `curve25519_guessing_20261102/analysis/f4cost_guess.jsonl` |
| 6 | 5 | 1.465·10⁵ | `curve25519_guess_optimum_20261102` (stage 1) |
| 7 | 0 … 4 | 7.46·10⁸, 2.30·10⁸, 9.03·10⁷, 2.60·10⁷, 9.70·10⁶ | `curve25519_guessing_20261102` |
| 7 | 5, 6 | 1.83·10⁶, 4.13·10⁵ | `curve25519_guess_optimum_20261102` |

Lead B found that the argmin over G is G = t − 1, the largest that `pkm-semaev` accepts, for every t in 6…10. Every
step G → G+1 cut the cost by ≥ 2.19× (`curve25519_guess_optimum_20261102`). The t = 5, G = 4 value is
interpolated here as 5.1·10⁴: t = 4, G = 3 is 17,592 and t = 6, G = 5 is 146,464, and 5.1·10⁴ is about their geometric mean.
This interpolation is the weakest input to the predictions.

### 2.3 G range

The brief asks for G ∈ {0 … t}. `pkm-semaev` refuses G ≥ t ("guess must be below the tower height",
`src/cryptanalysis/ic_framework/pkm_semaev.rs`, `prepare`), so G = t is infeasible by construction.
The design uses **G ∈ {0 … t − 1}**: 5 + 6 + 7 = 18 (t, G) pairs.

### 2.4 Sizing (labelled pilot, from existing data only; no new pilot run)

- The prior seed-0 cells of exactly these (t, G) took at most 25.1 s wall per 100 targets (t = 7, G = 0,
  `curve25519_guessing_20261102`). The other cells took 0.4 to 16 s.
- Selected seeds give the same polynomial system shape (T4 README), with more columns.
- **Frozen: 100 targets in every cell.** The cap is 3600 s per cell. No cell is expected to come near it.
- Wall time is used for sizing only and never as a result.

## 3. Exact design

- Instrument: `/home/user/ctarget/release/curve25519_family`. The run must use the binary with sha256
  `ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef`, as measured on 2026-11-03 while this protocol was written.
  - This differs from the `7a438b1d…` binary of the T4/T6 runs, so P0 checks for drift.
  - Do not rebuild the binary.
- Curve: the Curve25519-family miniature k = 20 (A = 926, p = 1048573, r = 131203).
- Cells: t ∈ {5, 6, 7} × G ∈ {0 … t − 1} × seed ∈ {0, sel(t)} = **36 cells**, each with 100 targets.
- Forms:
  - `isogeny` for seed 0. This is identical to `isogeny@0` and keeps the prior cells' exact form string.
  - `isogeny@1307`, `isogeny@1308`, `isogeny@1594` for the selected seeds.
- Target seed: `2026111500 + t`, the same for both seeds and for every G of a given t. The comparisons are therefore paired on
  the same targets. At t = 6 this is 2026111506, the target seed of the T4 engine check, which P0 uses.
- Command per cell:

      curve25519_family f4cost 20 <form> <t> <G> 100 <2026111500+t> 3600 2 full 0 30000000

- Order: for each t, G from t − 1 down to 0, with seed 0 and then the selected seed in each (t, G). Cells run strictly
  sequentially, one heavy process at a time. Counts are deterministic, so order and load do not change them.
- Runner: `scripts/run.sh`. It checks the binary and census hashes, refuses to overwrite data, and writes
  `data/cells/t<t>_g<G>_s<seed>.{json,stderr,status}`, `data/run.log`, `data/env.txt`, `data/binary.sha256`,
  `data/SHA256SUMS` and `data/ALL_DONE`.
- Analysis: `scripts/analyze.py <lead dir>` writes `analysis/summary.json`. It is frozen with this protocol and only applies
  the rules below.
- No ecbench spec in this lead: the brief puts the end-to-end measurement in T9.
- Pre-run steps (repository rules, done by the executor and recorded in the README):
  - `scripts/checkpoint.sh init` resume checkpoint.
  - The Conductor check.
  - The route. This lead runs locally on the shared 4-core host as the wave brief directs, not through `isolab-eks`. That
    is a recorded deviation from the default route; counted units are host-independent.

## 4. Predictions (numbered; each with its numeric decision rule)

Computed from §2 with mpt(sel) = mpt(seed 0). The predicted E_0 values, in F4 mul-adds per modelled solve, are:

| rank | cell (t, G, seed) | predicted E_0 |
|--:|:--|--:|
| 1 | (5, 4, 1307) | 7.0·10⁸ |
| 2 | (6, 5, 1308) | 1.39·10⁹ |
| 3 | (5, 4, 0) | 1.67·10⁹ |
| 4 | (5, 3, 1307) | 2.43·10⁹ |
| 5 | (7, 6, 1594) | 2.44·10⁹ |
| — | incumbent (6, 4, 0) | 1.35·10¹⁰ |
| — | (6, 4, 1308) | 6.73·10⁹ |

- **P0 (instrument).** mpt(6, 4, seed 0) reproduces the T4 engine value 710,094.0 within 1·10⁻³ relative.
  If it fails, the cause is binary drift. The analysis continues on the frozen binary, and every comparison with prior leads is flagged.
- **P1 (census consistency).** In all 36 cells, the `columns` and `covered_points` reported by f4cost equal the census
  values of §2.1 exactly. If not, E uses the f4cost values, and the mismatching cells are listed.
- **P2 (seed invariance of per-target cost).** For each of the 18 (t, G) pairs, 0.90 ≤ mpt(sel) / mpt(0) ≤ 1.10.
  Held only if all 18 pairs are measured and inside the band.
- **P3 (guessing monotone).** In each of the 6 (t, seed) series, mpt falls at every step G → G + 1 by a ratio
  ≥ 1.25, and the argmin is G = t − 1. Held only if all 6 series are complete and satisfy both conditions.
- **P4 (winner).** The argmin of E_0 over the 36 cells is **(t = 5, G = 4, seed 1307)**. Held if and only if it is
  exactly that cell.
- **P5 (gain over the incumbent).** E_0(6, 4, 0) / E_0(argmin cell) ≥ 10. The point prediction is 19.3. Held if ≥ 10.
- **P6 (recommendation trigger).** Take the best (t, G) other than (6, 4), minimised over its two seeds, and
  compare it with (6, 4) minimised over its two seeds (expected to be seed 1308). Held if the other (t, G) is cheaper by > 1.5×.
  The prediction is 9.6×. If held, T10 recommends that (t, G, seed) as the next end-to-end candidate. If not, it
  recommends none.
- **P7 (band robustness).** E_w(6, 4, 0) / E_w(argmin cell) > 1 at every w ∈ {0, 0.1, 1}, where native
  `*_uncharged` Fp multiplications are priced at w each. The order is stated only if it holds at all three weights.
- **P8 (the interpolated input).** 3.0·10⁴ ≤ mpt(5, 4, seed 0) ≤ 8.0·10⁴. The point prediction is 5.1·10⁴.

**Inconclusive rule for P4 to P7.** These four are reported as **inconclusive** rather than held or failed if any cell
they depend on is missing, failed (F2), capped or has `undecided > 0`. Those cells are every G = t − 1 cell (both seeds) and
both (6, 4) cells. `analyze.py` lists such cells under `P4.inconclusive`.

Failed predictions are reported as failed. No rule, threshold or seed is retuned after data are seen.

## 5. Failure handling

- **F1. Instrument identity.** If the binary or census sha256 differs from the frozen value at start or before any
  cell, `run.sh` stops and runs nothing more. A new protocol addendum (new hash, new P0 reference) is required. The
  binary is never rebuilt by this lead.
- **F2. Non-zero exit.** `run.sh` keeps the failed attempt (`*.try1.*`) and makes one identical rerun. If that also
  fails, the cell is failed, is not replaced, and its predictions are evaluated on the remaining cells with the denominator stated
  (see the inconclusive rule).
- **F3. Cap.** If a cell reports `capped: true`, it counts with its `targets_done` when that is ≥ 10, and it is
  flagged. Below 10 targets it is treated as failed. No re-run with a larger cap.
- **F4. `undecided > 0`.** The cell's mpt is a lower bound, and the cell is flagged (inconclusive rule).
- **F5. Census mismatch** (P1 fails). E uses the f4cost columns and coverage for that cell, and the mismatch is reported.
- No extra cells, targets or seeds are added after data are seen. Any follow-up needs a new protocol.

## 6. Scope and distance remaining

- **Scope.** One curve (k = 20), isogeny form, m = 2, mode full, lifts 0, t ∈ {5, 6, 7}, G ∈ {0 … t − 1},
  two seeds per t, 100 targets per cell.
  - E is a model of F4 work. It ignores linear algebra (∝ columns²), group operations, the seed search
    cost (1.35·10⁷ group ops at t = 6, T5 README) and the slack of relations needed over columns.
  - A winner here is a candidate for an end-to-end ecbench test (T9-style), not an improvement claim. Relabelling or
    re-accounting is not an improvement.
- **Distance remaining (from the T5 session `ECBS1h405f20219036`, k = 20, 6 targets).**
  - The best algebraic arm (seed 1093, t = 6, G = 4) is 18.3× behind rho-neg at w = 0, 6.9·10⁵× at w = 0.1 and 4.4·10⁶× at w = 1.
  - F4 mul-adds are unpriced native work in ecbench's accounting, so an F4 reduction changes S only at w > 0.
  - Even the predicted 19× F4 reduction, if it carried over end-to-end in full, would leave the algebraic track about 3.6·10⁴× behind
    rho-neg at w = 0.1. That is a projection, not a measurement. **Rho remains ahead; no row is a leaderboard claim.**
