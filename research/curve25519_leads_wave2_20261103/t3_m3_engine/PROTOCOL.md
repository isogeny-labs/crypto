# T3: does m = 3 frame selection survive the F4 engine? (protocol, 2026-11-03)

**Status:** frozen design, written before any T3 data. No engine cell and no census
regeneration has been run for this lead. Everything below that is a number comes from
files already in the repository (cited per number).
**Class:** component measurement on the algebraic track (`pkm-semaev`, Curve25519-family
miniature k = 20). Not a leaderboard row. Exact counts and counted F4 mul-adds only; no
timing claims (wall time is recorded for sizing only).
**Parents:** `research/curve25519_torsion_leads_20261103` (Q4: m = 3 selection gain
2.8–5.4× over the mean frame at t ≤ 5), `research/curve25519_frame_search_20261103` (P5:
frame selection keeps the F4 system and cuts F4/relation 4.71× at t = 5, m = 2),
`research/curve25519_algebraic_m3_20261102` (m = 3 is 30–200× worse than m = 2 per relation
at lo = 0; m = 3 cells at t ≥ 6 exceed 3.5 GB).

## 1. Question

The torsion census selected the m = 3 frame by J3 (coset-cancelling signed triples) and
found its exact m = 3 coverage 3.4–4.4× the mean frame at k = 20, t = 4–5. In the m = 2
case the engine confirmed that a selected frame leaves the F4 system unchanged. Two
questions:

- **Q-A.** At m = 3, does the F4 engine on `bits@L3*` (the J3-selected frame) have the same
  solving degree, the same terms per specialisation and the same mul-adds per target as
  `bits` (lo = 0), with `coverage_exact` equal to the census m3_cov, so that the m = 3
  selection gain becomes an equal F4 mul-adds per relation gain?
- **Q-B.** At matched t and selected frames, is m = 3 ever cheaper per relation than m = 2?

## 2. Design

### 2.1 Fixed instrument and curve

- Binary: `/home/user/ctarget/release/curve25519_family`, sha256
  `7a438b1d8ce9d7a0b8049a4331599727e9574ea4201f9c7957befa3184ba98be` (read 2026-11-03).
  It differs from the frame-study binary (`0459a725…`) and the torsion-study binary
  (`8e6dff92…`). The control cells in §2.4 test whether the counts are unchanged. Not rebuilt.
- Curve: family rule, k = 20, A = 926, p = 1048573, r = 131203, E(F_p) ≅ Z/8 × Z/r.
- Every engine cell: `f4cost 20 <form> <t> 3 <targets> <seed> 3600 <m> full 0 30000000`.
  That is guess split G = 3, full mode, no lifts, max_nnz 30,000,000, wall cap 3600 s.
  Each cell also runs under `timeout 4200` and `ulimit -v 6000000`.
- One process at a time, in the fixed order of `scripts/run.sh`.

### 2.2 Frame choices (step S)

The census bytes behind `research/curve25519_torsion_leads_20261103/data/k20.json` live in
the artifact store (`.artifacts.json`, sha256 `dc08cc8e…1099f`, 1,555,899 bytes). They are
not on this machine: there is no `aws` CLI and no local cache. **So L3* cannot be read now as
a number.** It is fixed instead by a frozen rule over hash-pinned data:

1. `run.sh` regenerates the k = 20 census with the study's frozen script
   (`torsion_census.py --k 20 --p 1048573 --A 926 --n 1049624`, seed 20261124). It writes the
   result into this lead's `data/census/` and requires the sha256 to equal `dc08cc8e…1099f`.
   If it does not, the run aborts before any engine cell (F1).
2. `scripts/select_frames.py` applies the torsion study's frozen rules from its `analyse.py`,
   verbatim:
   - **L3\*(t):** over the first 300 frames, argmax J3; ties go to more columns, then to the
     lowest L.
   - **L2\*(t):** over all 2000 frames, argmax J[0], with the same tie-breaks.
3. The selection must reproduce the published census summary
   (`torsion…/analysis/summary.json`, k = 20). If it does not, the run aborts (F1).

| t | L3\* m3_cov (census) | mean m3_cov, 300 frames | L2\* (published) | L2\* m2 cov |
|--:|--:|--:|--:|--:|
| 4 | 458 | 103.97 | 797820 | 78 |
| 5 | 2578 | 757.22 | 233902 | 184 |
| 6 | 11746 (not run, see §5) | 5856.95 | n/a | 582 |

The L2\* values were confirmed in the engine (torsion `engine_confirm/`).

### 2.3 Cells (k = 20, G = 3, full; executed in this order by `scripts/run.sh`)

| # | tag | form | t | m | targets | target seed | role |
|--:|:--|:--|--:|--:|--:|--:|:--|
| 1 | m2_t4_lo0 | bits | 4 | 2 | 100 | 2026111504 | control + m = 2 lo = 0 |
| 2 | m2_t4_L2 | bits@797820 | 4 | 2 | 100 | 2026111504 | control + m = 2 selected |
| 3 | m2_t4_L3 | bits@L3\*(4) | 4 | 2 | 100 | 2026111504 | exploratory (m = 2 on the m = 3 frame) |
| 4 | m2_t5_lo0 | bits | 5 | 2 | 100 | 2026111505 | control + m = 2 lo = 0 |
| 5 | m2_t5_L2 | bits@233902 | 5 | 2 | 100 | 2026111505 | control + m = 2 selected |
| 6 | m2_t5_L3 | bits@L3\*(5) | 5 | 2 | 100 | 2026111505 | exploratory |
| 7 | m3_t4_lo0 | bits | 4 | 3 | 100 | 2026111304 | control + T3 arm A |
| 8 | m3_t4_L3 | bits@L3\*(4) | 4 | 3 | 100 | 2026111304 | T3 arm B |
| 9 | m3_t5_L3 | bits@L3\*(5) | 5 | 3 | 30 | 2026111305 | T3 arm B (runs first at t = 5) |
| 10 | m3_t5_lo0 | bits | 5 | 3 | 30 | 2026111305 | T3 arm A |
| 11 | m3cov_t4_L2 | bits@797820 | 4 | 3 | 0 | – | coverage only, exploratory |
| 12 | m3cov_t5_L2 | bits@233902 | 5 | 3 | 0 | – | coverage only, exploratory |

How the sizes were fixed, from existing data only (no pilot):

- `curve25519_algebraic_m3_20261102/analysis/f4cost_m3.jsonl` gives the m = 3 per-target
  costs at lo = 0:
  - bits t = 4, G = 3, full: 1.198·10⁹ mul-adds/target, 100 targets in 56 s;
  - bits t = 5, G = 3, full: 2.90·10¹¹ mul-adds/target, 19 targets in 608 s (about 32 s each).
- So t = 5 runs **30 targets** per arm: about 16 min, and under 30 min at a 1.8× slowdown on
  the shared machine. This keeps each cell well under the hour.
- m = 3 runs only at t ∈ {4, 5}:
  - G = 0 at full mode is oversize (F4 does no work and the targets come back undecided);
  - t = 6 failed at 3.5 GB in the m = 3 study.
- The two arms at each t use the same target seed, so they solve the same targets (paired).
- The m = 2 and m = 3 cells use the seeds of their source studies, so the controls can be
  compared exactly. The m = 2 and m = 3 targets therefore differ. Q-B compares per-target
  means, which is acceptable at a 10²–10³ gap.
- Frozen sizes are not changed after data.

### 2.4 Controls (instrument identity, binary 7a43… vs older builds)

| cell | published f4_muladds (exact) | covered_points | source |
|:--|--:|--:|:--|
| 1 m2_t4_lo0 | 8,884,000 | 12 | `curve25519_edwards_semaev_20261102/cells/bits_t4_g3.json` |
| 2 m2_t4_L2 | 8,884,000 | 78 | `curve25519_torsion_leads_20261103/engine_confirm/bits_797820_t4.json` |
| 4 m2_t5_lo0 | 303,691,176 | 42 | `curve25519_frame_search_20261103/engine/bits_t5_g3.json` |
| 5 m2_t5_L2 | 303,690,989 | 184 | `curve25519_torsion_leads_20261103/engine_confirm/bits_233902_t5.json` |
| 7 m3_t4_lo0 | 119,824,176,893 (800 calls) | 60 | `curve25519_algebraic_m3_20261102/analysis/f4cost_m3.jsonl` + `coverage.jsonl` |
| 10 m3_t5_lo0 | (30 targets, not comparable) | 224 | `curve25519_algebraic_m3_20261102/analysis/coverage.jsonl` |

The prior m = 3 cells ran without the max_nnz argument. Cell 7 is therefore also the check
that the 30,000,000 cap does not bind at m = 3 (`undecided` = 0).

### 2.5 Derived quantities (definitions, fixed now)

- cov(c) = `covered_points` (order-r points that are a sum of m base points);
  `coverage_exact` = cov / r.
- C(c) = `muladds_per_relation_at_exact_coverage` = `muladds_per_target` / `coverage_exact`.
  This is derived. No relation count enters it.
- Per-target ratio ρ_t = mpt(bits@L3\*) / mpt(bits), m = 3.
- Selection gain at m = 3: g3_t = C(m3_t_lo0) / C(m3_t_L3). At m = 2: g2_t = C(m2_t_lo0) / C(m2_t_L2).
- m = 3 vs m = 2 at selected frames: q_t = C(m3_t_L3) / C(m2_t_L2).

## 3. Predictions (numbered; each with its decision rule)

Expected values come from the files in §2.2–2.4, assuming per-target cost is
frame-independent at m = 3 as it is at m = 2.

- **P0 (instrument identity).** Cells 1, 2, 4, 5 reproduce the published `f4_muladds` and
  `covered_points` exactly. Cell 7 reproduces 119,824,176,893 mul-adds over 800 F4 calls with
  `undecided` = 0. *Rule:* exact integer equality in all five cells. A mismatch is reported as
  instrument drift (binary 7a43… vs the older builds). It does not void P1–P5, which compare
  arms run on the same binary. It does void any comparison against older published per-target
  numbers.
- **P1 (coverage = census).** `covered_points` of cell 8 = **458** and of cell 9 = **2578**
  (the census m3_cov of L3\*). Cells 7 and 10 give 60 and 224. *Rule:* exact equality in all
  four cells. Otherwise P1 fails.
- **P2 (same F4 system).** At each t ∈ {4, 5}, compare the paired m = 3 arms (7 vs 8, 10 vs 9).
  *Rule:* P2 holds at t when **all** of the following are true; otherwise it fails at t:
  - `solving_degree_max` is equal;
  - `poly_terms_per_specialisation` is equal;
  - `undecided` = 0 in both arms;
  - **0.90 ≤ ρ_t ≤ 1.10**.
- **P3 (gain survives the engine).** Expected g3 is about 7.6 at t = 4 (458/60) and about 11.5
  at t = 5 (2578/224), times ρ_t⁻¹. *Rule:* **g3_t ≥ 5.0 at both t = 4 and t = 5**. Otherwise
  P3 fails, and the cells where it fails are named.
- **P4 (m = 3 selection pays more than m = 2 selection, in F4/relation).** Expected g2 is 6.50
  at t = 4 (9.71·10⁸ / 1.49·10⁸) and 4.38 at t = 5 (9.49·10⁹ / 2.17·10⁹), from the controls.
  *Rule:* **g3_t > g2_t at both t**. Otherwise P4 fails at the t where it does not hold. At
  t = 4 the expected margin is small (7.6 vs 6.5).
- **P5 (m = 3 is not cheaper per relation than m = 2, even after selection).** Expected q is
  about 2.3·10³ at t = 4 (3.43·10¹¹ / 1.49·10⁸) and 6.8·10³ at t = 5 (1.48·10¹³ / 2.17·10⁹).
  *Rule:* P5 holds if **q_t ≥ 100 at both t**.
  - If q_t < 1 in any cell, m = 3 is cheaper per relation there. That is reported as an
    unconfirmed improvement candidate, which needs its own preregistered confirmation, and P5
    fails.
  - If 1 ≤ q_t < 100, P5 fails, and the result is "m = 3 still costlier, by less than
    predicted".

Not preregistered (reported as exploratory only):

- cells 3 and 6, the m = 2 cost on the m = 3 frame;
- cells 11 and 12, the m = 3 coverage of the m = 2-selected frame;
- any observed relation (`hits`, `hits_verified`).

## 4. Failure handling

- **F1:** the census sha256 or the selection check fails. Abort before any engine cell
  (`data/ABORTED`). Report T3 as **not run**. Do not substitute another census or frame.
- **F2:** a cell fails (no JSON, timeout, out of memory) or is `capped`. Record it as failed
  or capped. Do not rerun it with other G, targets, cap or nnz. The predictions that need it
  are reported as **not decided** at that t. Capped cells are analysed on `targets_done` and
  labelled.
- **F3:** `undecided` > 0 in an m = 3 cell. P2 fails at that t, as stated in its rule. The
  per-target figure is reported with its undecided count, and its C is labelled a lower
  bound on work.
- A failed prediction is reported as failed, with its numbers. Thresholds, G, t, targets and
  frames are never retuned after data. Any follow-up is a new protocol.
- `hits_verified` must equal `hits` in every cell. A mismatch is a correctness finding, and
  that finding is reported first.

## 5. Scope

- In scope: k = 20, `bits` translation frames, t ∈ {4, 5}, G = 3, full mode.
- Out of scope:
  - t = 6, where the census selection has 11,746 points but the m = 3 cells failed at 3.5 GB;
  - other k;
  - the isogeny tower (T4);
  - chain mode;
  - 255-bit extrapolation.
- No timing claim.
- No cross-method ranking is made here. Native, uncharged work (system construction, root
  grid) is not priced. Any comparison with rho would need the §5a 0 / 0.1 / 1 GAE band, and
  none is claimed.
- The frame search cost is as priced in the parent studies: 300 · 2^t Legendre-symbol class
  evaluations plus [r]P per column. It is small against one m = 3 relation's F4 work, and
  not free against a k = 20 rho solve.

## 6. Distance remaining

Rho remains ahead on every track, and this lead cannot change that order.

- At k = 20 the generic index-calculus baseline is 7.8× rho.
- The best algebraic arm measured is 18× (w = 0) and 9.6·10⁵× (w = 0.1) behind rho-neg
  (`curve25519_algebraic_ecbench_20261102`).
- A rho solve at k = 20 takes 547–936 group operations.
- m = 3 per relation is expected near 10¹¹–10¹³ F4 mul-adds even after selection. That is
  10³–10⁴× the m = 2 selected frame (about 10⁸–10⁹).
- If P3 holds, it is a verified ≥ 5× cut **within m = 3**. It is not an improvement over the
  index-calculus baseline unless P5 also fails with q < 1.

## 7. Files

- `scripts/run.sh` executes §2.2–2.4 exactly into `data/`.
- `scripts/select_frames.py` is step S.
- Outputs:
  - `data/census/k20.json` (hash-checked);
  - `data/selection.json`;
  - `data/frames.txt`;
  - `data/cells/<tag>.json` and `.stderr`;
  - `data/binary.sha256`, `data/env.txt`, `data/run.log`, `data/SHA256SUMS`.
