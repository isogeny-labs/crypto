---
title: "Protocol: the break-even growth ladder for structured-base coincidences"
date: 2026-10-29
status: registered before any pilot or main run (hashes in REGISTRATION.sha256)
---

# Protocol: the break-even growth ladder for structured-base coincidences

Lead: `LEAD-ICP-coincidence-harvest-structured-base` (crypto-autoresearcher atlas), next step "run the break-even growth ladder (IDEA-20261018-a0830c)". Everything is computed by `examples/coincidence_ladder.rs` (standard library only), driven by `run.sh`.

## 1. Classification, stated first

- **What is measured.** κ is a relation-yield statistic: the number of group-additive pair-sum coincidences `P + Q = R + S` of a structured base, over the mean for matched random bases. It is not a cost, so it is not an IC/rho ratio, and no §7a dashboard point or `benchmarks/` entry follows from it.
- **Which track.** A table or enumeration method that harvests such coincidences is a generic-decomposition method (AGENTS.md "Tracks of index calculus", 2026-10-29): bounded by √r, and it cannot beat rho.
  - **Generic prediction.** For any x-coordinate-structured base on a generic group, κ = 1.
  - **What would reopen it.** A κ that *grows with N* would be non-generic structure in the base, outside the generic bound. That is why the lead is worth closing or reopening.
- **No ranking against k-sum.** κ ≈ 1 gives a structured base no advantage over a random base inside the same table method, so there is nothing to rank against `ksum:m=5` (#2852, #2866). A ranking or end-to-end pricing runs **only if a cell reads ALIVE** (§7), as the IDEA specifies, and then on the same instances as `research/prime_ksum_vs_incumbent_20261029` with the §5a band.

## 2. Question

Does κ for x-coordinate-structured bases (window, rank-2 and rank-3 label GAP) grow with N at the break-even rate `N^δ`, `δ ≥ δ*(h) = 1/(2h−1)` (1/3, 1/5, 1/7 for h = 2, 3, 4; IDEA-20261017-bbc53e), on prime-order curves of 16 to 32 bits?

## 3. Stage 1 (free): the slope already recorded

From the recorded cells of EXP-PFDR-ca5e01 (`runs/RUN-PFDR-b7c582/analysis.json`, crypto-autoresearcher at `8ac32f7`, sha256 `a0244ada624f007261293cb9e832dcffc63e0fba34f08580cfe2094062c9d3d0`), the 16 powered cells at θ = 0.30 and 0.35 and d = 1, 2, 3 are in `stage1/ca5e01-cells.tsv`. `coincidence_ladder stage1` fits `ln κ` on `ln N = bits·ln 2` per (θ, d) with a parametric bootstrap from each cell's recorded 95% interval. Only groups with at least three rungs get a slope. This is a cross-implementation prior, not a registered reading.

## 4. Instances

- **Rungs.** 16, 20, 24, 28, 32 bits. `p < 2^32` so every pair-sum key `x·p + y` is an exact 64-bit integer.
- **Curves.** Prime-order `y² = x³ + a x + b` over F_p, `p` a random `bits`-bit prime (top bit set), random `(a, b)`. The order is proved, not assumed: the Hasse interval holds exactly one multiple of the order of a random point; it is prime and exceeds half the interval's top, so it is `#E` (`prime_order_of`). `selftest` checks the proved order against a brute-force point count on 16 curves.
- **Seeds.** Every base is a pure function of the label string `pch20261029|<kind>|<block>|<bits>|<curve>|<θ index>|<k>`, hashed by FNV-1a into splitmix64. Pilot block 8,000,000; main block 8,100,000; replication block 8,200,000. Blocks are disjoint.
- **Base size.** `b = 2·round(2^(bits·θ)/2)` for θ ∈ {0.30, 0.35}, as in EXP-PFDR-ca5e01 (checked by `selftest` against its cells b16-t0.30 = 28 and b20-t0.30 = 64). θ = 0.30 and 0.35 bracket `U = N^(1/3)` (IDEA-20261018-a0830c, (iii)).

## 5. Bases, statistic, sizing, estimator

- **Count.** For a symmetric base `F` of exactly `b` points, `C(F) = Σ_s binom(m_s, 2)`, `m_s` the number of unordered pairs `{P, Q} ⊂ F` with `P ≠ Q`, `Q ≠ −P` and `P + Q = s`. Exact; no sampling.
- **Structured bases** (all three are run; none is selected afterwards, so there is no winner's-curse step):
  - **window (d = 1):** `x0` uniform, `x = x0, x0+1, …`;
  - **rank-d label GAP (d = 2, 3):** `x0 + a + c·w1 (+ e·w2)`, box side `ceil(b^(1/d))` (at least 2), innermost index fastest, outermost unbounded.
  - In all three, both lifts of each `x` with `f(x)` a nonzero square are added, repeated `x` skipped, until exactly `b` points.
- **Null.** 20 uniformly random symmetric bases of the same size per (curve, θ).
- **Statistic per rung.** `S` = Σ over curves of `C(structured)`; `M` = Σ of per-curve null means; `V` = Σ of per-curve null sample variances; `κ = S/M`; `z = (S − M)/sqrt(V(1 + 1/20))`. The 95% interval is a curve-cluster bootstrap (2000 resamples, seeded per θ).
- **Slope.** OLS of `ln κ_i` on `x_i = mean ln r` over the five rungs, per (θ, geometry). Its 95% interval is the joint curve-cluster bootstrap: each iteration resamples curves within every rung, recomputes the five κ and the slope.
- **Sizing.** A pilot of 40 curves per (rung, θ) on block 8,000,000 gives the per-curve null mean `M1` and variance `V1`. Then `n_main = clamp(ceil(1.05·V1 / (0.02²·M1²)), 40, cap)`, cap = 30,000 / 12,000 / 5,000 / 2,500 / 1,200 at 16 / 20 / 24 / 28 / 32 bits. The target is 2% relative standard error per cell (the IDEA's figure). `design.json` records the rule, the cap and the predicted relative sd before the main run.

## 6. Controls

- **Z/N interval (positive, exact slope 1 − θ).** The symmetric interval `{±k : 1 ≤ k ≤ b/2}` modulo the curve order `N`, against 20 random symmetric sets, counted by the same kernel logic. Its κ is `Θ(N/b)`, so the slope is `1 − θ`: 0.70 and 0.65. It fixes the estimator's scale.
- **Weighted-base plants (positive).** `F' = {±c·P_i : 1 ≤ c ≤ c0}`, `c0 = ceil(N^γ)`, `m = floor(b/(2·c0))` rows, topped up with random points to `b`.
  - **Independent prediction.** The rows' coincidences are counted in the integers (`formal_row`, no group arithmetic). The predicted `κ = 1 + m·C_row(c0)/M` and its slope are compared with the measured group count.
  - **Why this prediction differs from the IDEA.** The IDEA notes' `κ_eff = 2·c0` and slope γ concern a different quantity. For this count the slope is `2γ + 1 − 3θ`, up to the same constants.
- **Null-vs-null.** Null replicate 0 as a pseudo-structured base against replicates 1–19 (`null_cal`), and the γ = 0 plant (a random base).

### Deviation R1 from the IDEA (recorded before any run)

IDEA-20261018-a0830c asks for plants at γ = 0, 1/6, 1/3. **γ = 1/3 cannot be built at θ = 0.30 or 0.35 on any rung:** it needs `c0 = N^(1/3) ≤ b/2 ≈ N^θ/2`, so `m = 0`. At 32 bits `c0 = 1625` against `b/2 = 388` (θ = 0.30) and 1177 (θ = 0.35). The registered plants are **γ = 0, 1/6 and 1/5** (all give `m ≥ 1` on every rung). 1/6 and 1/5 exceed the weakest break-even exponent `δ*(4) = 1/7`, so they are positive controls for the ALIVE reading. The IDEA's "recovered within 0.03" is applied as: measured slope within 0.03 of the formal-count slope.

## 7. Gates and the registered reading

- **G1 calibration.** Among the four null-type slope estimates (`null_cal` and γ = 0 plant, at two θ), at most 1 has a 95% interval excluding 0 (IDEA: more than 10% of cells is miscalibrated).
- **G2 Z/N.** Both slopes within 0.05 of `1 − θ`.
- **G3 plants.** Slopes of the γ = 1/6 and γ = 1/5 plants within 0.03 of their formal-count slopes, at both θ.
- If any gate fails, **no closure or ALIVE reading is made** (inconclusive; never a reading of the lead).
- **Real-geometry slopes.** Six: {window, gap2, gap3} × {θ = 0.30, 0.35}. Per slope, with 95% interval `[lo, hi]` and the weakest break-even `δ*(4) = 1/7`:
  - `ALIVE`: `lo > 0` and `hi ≥ 1/7`;
  - `inconclusive_wide`: `lo ≤ 0` and `hi ≥ 1/7`;
  - `positive_subcritical`: `lo > 0` and `hi < 1/7`;
  - `negative`: `hi < 0`;
  - `null`: otherwise.
- **Reading** (computed by `analyze`):
  - `ALIVE` if any real slope is ALIVE. Then the replication block runs and the lead stays open; it must be priced end to end against matched rho and against k-sum on the same instances.
  - `SUB_BREAK_EVEN_AT_SCALE` if all six `hi < 1/7`. This is `exhausted_at_scale` at 32 bits, not a proof.
  - `INCONCLUSIVE` otherwise.
- The upper bounds are also reported against δ*(h) = 1/3, 1/5 and 1/7.

## 8. Distance remaining

For the harvest law `T = sqrt(2·c·U·N/κ) < sqrt(πN/2)` (IDEA-20261017-bbc53e), a harvest wins only if `κ > 1.27·c·U`, `U = N^(1/(2h−1))`. The report gives, per rung, `1.27·N^(1/(2h−1))` for h = 2, 3, 4 beside the measured κ (taking `c = 1`; a larger stopping constant only widens the gap).

## 9. Predictions (registered)

- **P1.** All six real-geometry slopes have `|slope| ≤ 0.05` and `hi < 0.07`; at least five of six intervals contain 0; every real κ lies in `[0.95, 1.05]`.
- **P2.** Z/N slopes are 0.70 and 0.65 within 0.05. Plants recover their formal slopes within 0.03.
- **Consistency with ca5e01.** At 16 and 20 bits my window and GAP κ at θ = 0.30 should lie within the recorded intervals of ca5e01's cells (different curve generator, so statistical, not exact).

## 10. Audits

- `selftest`: 10 checks, including brute-force order and the batched sorted count against an unbatched hash-map count.
- A recount of `null`, `window`, `gap` and `plant` bases by the unbatched path on every 97th curve (`--verify-every 97`).
- Determinism: the 24-bit main rung rerun with a different worker count is byte-identical.
- `verify --replay-all` of ecbench does not apply (no ecbench session); the above replaces it.

## 11. Disclosures

- **A dry run preceded this registration.** To test the estimator on its controls I ran the census on a throwaway block (9,999,000), rungs 16–28 only, and looked at its analysis. All real slopes there were within ±0.016 with upper bounds below 0.035, and the controls recovered (Z/N 0.701 and 0.652, plants within 0.001 of their formal slopes). **P1 and P2 were therefore written after seeing a dry run on other curves; they are not independent predictions.** That block is not used anywhere in the registered analysis.
- The pilot data were produced after this registration. The only timing runs before it were at block 8,000,000 (rungs 16, 20, 24: line counts only) and block 9,999,000; the 8,000,000 files are regenerated by `run.sh`.
- The atlas lead's `next_action` names the bb0d26 independent review as pending. Its `analysis.md` (EV-PFDR-07d608, DEC-20261029-01405d) shows that review done. This study does not edit the atlas.

## 12. Limits

- Scale: 32 bits (`p < 2^32`). A bound from these rungs says nothing about growth beyond them.
- Only x-coordinate-structured families (window, rank-2 and rank-3 GAP). bb0d26's log-GAP label sets and non-geometric structures are outside this ladder.
- One curve family; the generator differs from ca5e01's.
- No theorem is stated or claimed, so no Lean file is owed. The controls are derived in the IDEA notes and in §6.
