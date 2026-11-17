# L10 protocol: how does the joint-optimum F4 cost scale with r? (isogeny factor base, k = 16 … 24)

Wave 4, lead L10. Written 2026-11-04, before any L10 data. **Status: design, not yet frozen.**
The protocol is frozen when its commit hash is recorded here by whoever freezes it (git is
forbidden in this session). After that, nothing in it changes; deviations go to an addendum.

**Class.** Exploratory engine measurement in counted units (F4 multiply-adds from
`curve25519_family f4cost`). The outcome is a *modelled* F4 cost per solve and its fitted exponent
in r. It is not an end-to-end solve, not an ecbench session and not a leaderboard row. Rho remains
ahead; nothing here is an improvement claim.

## 1. Question

Wave 3 validated (T10 model, T12 end to end at k = 20) the cost model

    E_w(k, t, G, s) = columns(k, t, s) * r_k * (mpt(t, G) + w * nat) / covered_points(k, t, s)

- relations needed ≈ `columns`; trials per relation ≈ r / `covered_points`;
- `mpt` = `muladds_per_target`, which depends on (form, t, G) and not on the seed or the curve
  (T4 replication: 710,091.9 / 710,094.0 / 710,096.0 at k = 18 / 20 / 22–24 for t = 6, G = 4;
  177,112.0 at k = 20 and k = 24 for t = 5, G = 3; 17,592.0 for two seeds at t = 4, G = 3, k = 20);
- `nat` = per-target `native.*_uncharged` F_p multiplications, priced at w ∈ {0, 0.1, 1}.

At fixed t, `columns`, `covered_points` and `mpt` do not depend on k (the tower is a fixed object;
the J/columns-selected seed S*(k, t) gives a cols/covered ratio that varies by a bounded factor
across k, §2.2), so **E is linear in r**: the algebraic track scales like r¹ against rho's r^½,
unless the optimal t grows with k. **Question: over the five family miniatures, what is the
exponent b in E_best(k) ∝ r^b, is the optimal t constant in k, and what t-growth would be needed to
beat exponent 1?**

## 2. Inputs computed now from existing data (no new measurement)

### 2.1 Curves

| k | A | p | r | √r | rho-neg GAE at S = 1.5 (1.5·√r) |
|--:|--:|--:|--:|--:|--:|
| 16 | 1058 | 65437 | 8209 | 90.6 | 135.9 |
| 18 | 78 | 262133 | 32719 | 180.9 | 271.3 |
| 20 | 926 | 1048573 | 131203 | 362.2 | 543.3 |
| 22 | 43186 | 4194301 | 524731 | 724.4 | 1086.6 |
| 24 | 318 | 16777213 | 2097349 | 1448.2 | 2172.3 |

Rho reference: the brief fixes rho-neg S = 1.5 (T12 measured 1.544 / 1.654 / 2.651 at w = 0 / 0.1 / 1,
k = 20, 32 runs). It is applied as 1.5·√r GAE at every k; at k ≠ 20 it is a heuristic, not a
measurement (same limitation as the T4 replication's E3).

### 2.2 Seeds (`selection/seeds.json`, sha256 `1e6195046fbd445a3551a6726ac4df7e3ee08976f3814590ac904909e3338f7d`, by `scripts/select_seeds.py`)

Source: `research/curve25519_isogeny_seed_20261103/data/k{16,18,20,22,24}.jsonl` (2,000 seeds per
(k, t), t = 4 … 7), sha256 as in that lead's `SHA256SUMS` (k16 `3935c968…`, k18 `4e670176…`,
k20 `bc74d8ac…`, k22 `11903b11…`, k24 `39c75436…`; copied in full into `scripts/run.sh`).
Rule: S*(k, t) = argmax J / columns, ties by larger J then smaller seed (the T9/T10 selector).
Seed 0 is the control.

| k | t=4 S* (cols/cov) | t=5 S* (cols/cov) | t=6 S* (cols/cov) | t=7 S* (cols/cov) | seed 0 t=5 | seed 0 t=6 |
|--:|:--|:--|:--|:--|:--|:--|
| 16 | 1786 (8/58) | 1597 (18/160) | 1381 (40/614) | 278 (74/1514) | 13/56 | 28/198 |
| 18 | 1074 (6/50) | 997 (19/186) | 1236 (36/480) | 1955 (78/1644) | 9/14 | 30/230 |
| 20 | 1193 (8/52) | 1307 (22/210) | 1308 (36/498) | 1594 (83/1840) | 18/72 | 25/172 |
| 22 | 1002 (13/88) | 1034 (21/224) | 284 (34/458) | 454 (81/1906) | 13/40 | 32/270 |
| 24 | 1717 (12/78) | 978 (20/190) | 1340 (29/380) | 702 (86/1864) | 17/76 | 39/364 |

cols/cov at S* across the five k: t = 4 0.120–0.154, t = 5 0.094–0.118, t = 6 0.065–0.076,
t = 7 0.042–0.049. The k-to-k spread at fixed t is ≤ 1.3×, so with k-invariant mpt the model's
exponent is 1 up to this spread.

### 2.3 Prior per-target costs (isogeny form, m = 2, full, lifts 0; k = 20 unless stated)

| t | G | mpt | source |
|--:|--:|--:|:--|
| 4 | 3 | 17,592.0 (seeds 595 and 1093) | `curve25519_leads_wave2_20261103/t7_t0_frames` |
| 5 | 4 | 52,816.0 (seeds 0, 1307) | T10 |
| 5 | 3 | 177,112.0 (seeds 0, 1307; k = 24 seeds 0, 797) | T10, T4 replication |
| 6 | 5 | 146,464.0 (seeds 0, 1308) | T10 |
| 6 | 4 | 710,094.0 / 710,095.08; 710,091.9–710,096.0 at k = 18 … 24 | T10, T4 replication |
| 7 | 6 | 423,619.64 (seed 0), 417,161.31 (seed 1594; 4 hits) | T10 |

T10 found argmin G = t − 1 in every series (G = t is refused by `pkm-semaev`), so G = t − 1 is the
primary G here and G = t − 2 the second-best G. **T10 did not include t = 4** (its brief fixed t ∈
{5, 6, 7}); the census does, and with mpt(4, 3) = 17,592 the model puts t = 4 ahead at every k (§4).

### 2.4 Cells predicted now (model with the §2.3 mpt and §2.2 census; E in F4 mul-adds per modelled solve, w = 0)

| k | E(t=4) S* | E(t=5) S* | E(t=6) S* | E(t=7) S* | argmin t | 2nd/best | E(t=5) seed 0 | E(t=6) seed 0 | E(t=5,G=3) S* | E(t=6,G=4) S* |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| 16 | 1.99e7 | 4.88e7 | 7.83e7 | 1.69e8 | 4 | 2.45 | 1.01e8 | 1.70e8 | 1.64e8 | 3.80e8 |
| 18 | 6.91e7 | 1.77e8 | 3.59e8 | 6.52e8 | 4 | 2.56 | 1.11e9 | 6.25e8 | 5.92e8 | 1.74e9 |
| 20 | 3.55e8 | 7.26e8 | 1.39e9 | 2.49e9 | 4 | 2.04 | 1.73e9 | 2.79e9 | 2.43e9 | 6.74e9 |
| 22 | 1.36e9 | 2.60e9 | 5.71e9 | 9.37e9 | 4 | 1.91 | 9.01e9 | 9.11e9 | 8.71e9 | 2.77e10 |
| 24 | 5.68e9 | 1.17e10 | 2.34e10 | 4.06e10 | 4 | 2.05 | 2.48e10 | 3.29e10 | 3.91e10 | 1.14e11 |

Least-squares fit of ln E_best = a + b ln r over the five predicted points: **b = 1.031**, a = 7.46;
local slopes 0.90 (16→18), 1.18 (18→20), 0.97 (20→22), 1.03 (22→24). Predicted
E_best / (1.5·√r): 1.47e5, 2.55e5, 6.54e5, 1.26e6, 2.61e6 (strictly increasing, slope ≈ r^0.53).

### 2.5 Sizing (labelled; from existing wall times only, no new pilot)

T10 (k = 20, same binary, 100 targets): every G = t − 1 or t − 2 cell took 1–12 s wall; the
slowest T10 cell at all (t = 7, G = 0) took 25 s. The T4 replication ran k = 24 at t = 6, G = 4 and
k = 16 … 24 at t = 5, G = 3 with 100 targets, none capped. No cell of this design is sized as too
slow, so **100 targets are frozen in every cell** and no reduced-target cell exists. Expected
total: 40 cells, well under an hour on the shared host. Wall time is sizing only, never a result.

## 3. Exact design

- Instrument: `/home/user/ctarget/release/curve25519_family`, sha256
  `ba1e3c0518eae58ae39a9b08e1f757c7e9fe1eb02c86b158b883244bc6a5ecef` (the T10 binary; checked at
  start and before every cell; never rebuilt).
- Forms: `isogeny` for seed 0; `isogeny@S*` for the selected seed. m = 2, mode full, lifts 0,
  max_nnz 30,000,000, trace off (12 positional arguments, as T10).
- Target seed `2026111500 + t`, the same for every k, seed and G at a given t (paired on the same
  target stream per t; at k = 20 this makes 7 cells identical to T10 cells, used by P0).
- Cells (**40**), each 100 targets, cap 3600 s:
  - for every k ∈ {16, 18, 20, 22, 24} and t ∈ {4, 5, 6, 7}: (t, G = t − 1, S*(k, t)) — 20 cells;
  - for t ∈ {5, 6}: (t, G = t − 1, seed 0) control — 10 cells;
  - for t ∈ {5, 6}: (t, G = t − 2, S*(k, t)) second-best G — 10 cells.
- Command per cell:

      curve25519_family f4cost <k> <isogeny|isogeny@S*> <t> <G> 100 <2026111500+t> 3600 2 full 0 30000000

- Order: k ascending, t ascending, within (k, t): S* at G = t − 1, then seed 0 at G = t − 1, then S*
  at G = t − 2. Strictly sequential, one heavy process; engine runs do not take
  `/tmp/crypto-bench.lock`. Counts are deterministic, so order and host load do not change them.
- Runner `scripts/run.sh` (frozen): checks the binary, the five census files and `selection/seeds.json`
  against their hashes, refuses to overwrite data, writes
  `data/cells/k<k>_t<t>_g<G>_s<seed>.{json,stderr,status}`, `data/run.log`, `data/env.txt`,
  `data/binary.sha256`, `data/seeds.used.json`, `data/SHA256SUMS`, `data/ALL_DONE`.
- Analysis `scripts/analyze.py <lead dir>` (frozen) → `analysis/summary.json`; it applies §4 only.
  - E_best(k) = min over t ∈ {4 … 7} of E(k, t, t − 1, S*) (the 20 primary cells).
  - Exponent: ordinary least squares of ln E_best(k) on ln r_k over the five curves (w = 1 for the
    headline; w = 0 and 0.1 reported; nat is small, so they agree to < 1 %).
  - k-invariance: at fixed (t, G) over the S* series, max/min of mpt across the five k.
  - Secondary fits (reported, not predictions): b at each fixed t; local slopes between
    neighbouring k; g(k, t) = columns·mpt/covered at S* and its per-t-step ratio.
- Pre-run steps (repository rules, done by the executor and recorded in the README): the
  `scripts/checkpoint.sh init` resume checkpoint and the Conductor check where available; the route
  is the shared 4-core host as the wave brief directs (recorded deviation from `isolab-eks`;
  counted units are host-independent).
- No ecbench session in this lead; nothing is end to end.

## 4. Predictions (numbered; each with its numeric decision rule, decided by `scripts/analyze.py`)

- **P0 (instrument reproduction).** The 7 k = 20 cells shared with T10 (t5 g4 s0 / s1307, t5 g3 s1307,
  t6 g5 s0 / s1308, t6 g4 s1308, t7 g6 s1594) reproduce T10's mpt within 1·10⁻³ relative (expected:
  identical). Held only if all 7 do.
- **P1 (census consistency).** In all 40 cells, f4cost `columns` and `covered_points` equal the census
  values of §2.2 exactly. If not, E uses the f4cost values and the mismatching cells are listed.
- **P2 (k-invariance of mpt, brief item i).** For each of the 4 primary (t, G = t − 1) series over the
  S* seeds, max_k mpt / min_k mpt ≤ 1.10 across the five k. Rule: **held only if 4/4 series are
  complete and within 10 %.** Point prediction: ≤ 1.0001 at t = 4, 5, 6 and ≤ 1.03 at t = 7 (hit
  counts differ by seed at t = 7). The two G = t − 2 series are reported under the same band but do
  not decide P2.
- **P3 (seed invariance control).** At (t, G = t − 1) for t ∈ {5, 6} and every k,
  0.90 ≤ mpt(S*) / mpt(seed 0) ≤ 1.10. Held only if all 10 pairs are inside the band.
- **P4 (optimal t, brief item ii, stated alternative).** The brief's default was "t = 5 at every k".
  With t = 4 in the design and mpt(4, 3) = 17,592 the model predicts otherwise, and the
  preregistered alternative is: **argmin_t E(k, t, t − 1, S*) = 4 at every k, the second-best t is 5
  at every k, and E(t = 5) / E(t = 4) ≥ 1.5 at every k** (point predictions 2.45, 2.56, 2.04, 1.91,
  2.05). Held only if all three clauses hold at all five k. If t = 5 wins anywhere, P4 fails and the
  brief's default is reported as the outcome for that k.
- **P5 (exponent, brief item iii).** The least-squares exponent b of E_best(k) in r_k over the five
  curves lies in **[0.85, 1.15]**. Point prediction 1.03 (§2.4). Held iff 0.85 ≤ b ≤ 1.15.
- **P6 (gap to rho grows, brief item iv).** E_best(k) / (1.5·√r_k) is **strictly increasing** over
  k = 16, 18, 20, 22, 24 (reported at w = 1 as w·E_w / GAE; also at w = 0.1; at w = 0 F4 work is
  unpriced and the ratio is 0 by construction). Point predictions 1.47e5, 2.55e5, 6.54e5, 1.26e6,
  2.61e6. Held iff all four consecutive differences are positive.

**Inconclusive rule.** P4, P5 and P6 are reported as **inconclusive** rather than held or failed if
any of the 20 primary cells is missing, failed (F2), capped, or has `undecided > 0`; `analyze.py`
lists such cells. P0–P3 are evaluated on the cells present, with the denominator stated; a
prediction whose rule requires all cells fails when a cell is missing.

Failed predictions are reported as failed. No rule, threshold, seed or cell is retuned after data.

## 5. Failure handling

- **F1. Instrument identity.** If the binary, any census file or `selection/seeds.json` differs from
  its frozen sha256 at start or before any cell, `run.sh` stops and runs nothing more. A protocol
  addendum (new hash, new P0 reference) is required. The binary is never rebuilt by this lead.
- **F2. Non-zero exit.** `run.sh` keeps the failed attempt (`*.try1.*`) and makes one identical rerun.
  If that also fails, the cell is failed, not replaced, and the inconclusive rule applies.
- **F3. Cap.** A cell with `capped: true` counts with its `targets_done` when that is ≥ 10 and is
  flagged; below 10 targets it is treated as failed. No rerun with a larger cap.
- **F4. `undecided > 0`.** The cell's mpt is a lower bound; the cell is flagged (inconclusive rule).
- **F5. Census mismatch** (P1 fails). E uses the f4cost columns and coverage for that cell; the
  mismatch is reported.
- **F6. Host restarts.** The container restarts every 40–60 min. `run.sh` refuses to overwrite a
  started `data/cells/`; if a restart interrupts the run, the executor moves the partial `data/` to
  `data.interrupted.<n>/` (kept, reported) and relaunches the whole runner. Cells are deterministic,
  so a relaunch reproduces the completed cells exactly.
- No extra cells, targets, seeds or t values are added after data are seen. Any follow-up (for
  example t = 3, or seed 0 at t = 4 and 7) needs a new protocol.

## 6. Scope and distance remaining

- **Scope.** Five family miniatures (k = 16 … 24), isogeny form only, m = 2, mode full, lifts 0,
  t ∈ {4 … 7}, G ∈ {t − 1, t − 2}, S* and seed-0 seeds, 100 targets per cell. E is a model of F4 work:
  it ignores linear algebra (∝ columns²), group operations, the per-curve seed search (6.94·10⁶
  group ops at k = 20, t = 5; the census recorded it for every (k, t)) and the slack of relations
  over columns. The exponent fit has 5 points and 2 parameters; its residual spread is reported, no
  confidence interval is claimed. A smaller optimal t is a candidate for an end-to-end test (T12
  style), not an improvement claim; relabelling or re-accounting is not an improvement.
- **What t-growth would beat exponent 1 (reported from the data, §3 secondary fits).** Write
  E = g(t)·r with g(t) = columns·mpt/covered at S*. The exponent falls below 1 only if the optimal
  t moves with k so that g(t(k)) falls with r; exponent ½ (rho's) would need g(t(k)) ∝ r^−½, i.e.
  g must halve for every 2 bits of r. The census and §2.3 priors give the opposite sign: per t step
  g grows by about 2.0–2.6× (covered points grow ≈ 2.5–3× per step while columns×mpt grows ≈ 6–8×),
  so raising t never pays in this model and the predicted optimum sits at the smallest t measured.
  The report states the measured g(t + 1)/g(t) per k and the per-step coverage gain that would be
  needed (≈ (columns·mpt growth) × 2 per 2 bits) for the track to reach exponent ½.
- **Distance remaining to rho-neg (from the model, predicted; the report replaces these with the
  measured values).** With rho-neg at S = 1.5: 1.5e5× (k16), 2.5e5× (k18), 6.5e5× (k20), 1.3e6×
  (k22), 2.6e6× (k24) at w = 1, a tenth of that at w = 0.1, and unpriced at w = 0. These are
  unit-mixed (F4 mul-adds against GAE) and F4-phase only; the measured end-to-end distance at k = 20
  is T12's **6.06·10⁴× behind rho-neg at w = 0.1** (17.1× at w = 0) for the T10 winner. If t = 4 holds
  end to end with the modelled 2.0× F4 gain at k = 20, the projected distance would be about
  3·10⁴× at w = 0.1; that is a projection, not a measurement. **Rho remains ahead on every track; no
  row is a leaderboard claim.**
