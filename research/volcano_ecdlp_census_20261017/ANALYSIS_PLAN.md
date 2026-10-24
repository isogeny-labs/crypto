# Analysis plan (frozen before any Phase B result was inspected)

Frozen 2026-10-22, while the last Phase B session was still running. Before
this point nobody had computed or viewed any Phase B cost summary:

- **The coordinator** looked only at counts, audits and the extract's column
  header.
- **The executor** produced only per-session `ecbench table` files, which
  nobody has read, and was told not to summarise results.

This file sets out PLAN §5 in operational form. `analysis/analyze.py` must
implement exactly this. Any departure goes into `AMENDMENTS.md` (A2…) with a
reason, and is labelled post hoc in the report.

## 1. Data

- **Sessions.** A session is included if it has a `runs/progress.jsonl` line
  with `"status":"done"` and `audit_ok: true`. Lines without a status (cleanup
  re-verifies) are ignored. Each session counts once.
- **Rows.** Rows come from `runs/extract/<spec>.csv.xz` with `warmup == false`.
  - Rows with `verified != true` are counted per (volcano, family, arm) and
    reported as the failure rate, with a binomial (Clopper–Pearson) 95% CI.
    They are excluded from cost statistics.
  - An arm with any unverified row is flagged.
- **Primary unit.** `S`, ecbench's normalised operations.
  - Secondary: `ops_gae`.
  - Wall (`solve_wall_ns`) is a result only under §8; otherwise it is
    descriptive.
- **Curve key.** `curve_slug`. Each slug is mapped to its census vertex (id,
  level, parent, subtree) through `census/<V>/vertices.jsonl`. The `ctl` curves
  map to the vertex they re-scale.
- **Volcanoes.**
  - V1 and V2 get the full analysis.
  - V3 gets the positive control (§6) and descriptive per-level tables only
    (A1.2: one curve per upper level).
  - V4 is not measured (A1.1).

## 2. Arms and families

- **Arms:** `rho-neg` (reference), `rho-plain`, `bsgs`, `bsgs-neg`, `ic`,
  `rho-neg-aa` (wall A/A only), and `rho-cubic` (V3 crater only).
- **Families:** D, Ua, Ub, S, N, ctl.
  - "Isogenous" = D ∪ Ua ∪ Ub ∪ S ∪ N.
  - ctl is the negative control and is never pooled with the isogenous
    families.

## 3. Level effect (PLAN §5 tests 1–2)

Run this for each volcano V ∈ {V1, V2}, each arm in {rho-neg, rho-plain, bsgs,
bsgs-neg, ic}, and each level L ≥ 1.

- **Statistic.** R(L) = mean S at level L / mean S at level 0 (the crater),
  using isogenous-family rows.
- **CI: cluster bootstrap.** 10,000 replicates, seed 20261022, stratified by
  level. Each replicate:
  1. resamples curves with replacement within each level;
  2. then resamples rows with replacement within each chosen curve.

  This gives a 95% percentile CI and a 90% CI for TOST.
- **TOST margins.** ±5% for rho and BSGS arms: [0.95, 1/0.95]. ±10% for `ic`:
  [0.90, 1/0.90].
  - **"equivalent"** if the 90% CI lies inside the margin.
  - **"different"** if the 95% CI excludes 1 *and* the result survives Holm
    (§7).
  - **"inconclusive"** otherwise.
  - The achieved 95% half-width is always reported.
- **Secondary model.** A linear mixed model, `log S ~ C(level) + C(family) +
  (1 | curve)`, fitted with statsmodels `MixedLM` and REML. Report:
  - the level coefficients as ratios with Wald 95% CIs;
  - the curve variance component and the ICC.

  If the model does not converge, report that. It never overrides the
  bootstrap.
- **Robustness.** Repeat R(L) per family (D, Ua, Ub, S, N separately), as
  descriptive values only.

## 4. Curve effect against the negative control (PLAN §5 test 3)

Run this for each volcano V ∈ {V1, V2} and each arm.

- **Data.** For every vertex v in the D family that has ctl copies, compute:
  - m_v = mean S of the original curve (D family);
  - c_{v,1}, c_{v,2} = mean S of its two isomorphic re-scalings (ctl family).
- **Variance-ratio test.**
  - Statistic: F = (variance of m_v across vertices within each level, pooled)
    / (pooled within-vertex variance of {m_v, c_{v,1}, c_{v,2}}).
  - Permutation test: shuffle the three labels within each vertex,
    10,000 times, seed 20261023.
- **Per-curve flags.**
  - Each original curve gets z_v = (m_v − mean(c_v)) / pooled control SD.
  - Two-sided p-values come from the permutation distribution of the same
    statistic, Holm-corrected across all curves of that (V, arm).
  - A curve is flagged if its adjusted p < 0.01.

## 5. Transfer (PLAN §5 test 4)

- **Recovery.** The verified fraction per (V, family, arm), with a
  Clopper–Pearson 95% CI. The expectation is 1.
- **Paired source comparison.** Within a vertex, D, Ua, Ub and S rows share
  target_index and the scalar k_i.
  - **BSGS arms:** report the fraction of (vertex, target_index, round)
    triples with identical `ops_gae` across families. This is a sanity check;
    the expectation is 1.
  - **rho and ic:** the ratio of means Ua/D, Ub/D, S/D and N/D, with the §3
    cluster bootstrap (curves as the resampling unit) and the same TOST
    margins.
- **Transport cost.** Taken from `runs/transfer_cost.jsonl`:
  - per path length d: ns per edge and ns per path (median and IQR), and Vélu
    evaluations per path;
  - the transport/solve fraction = transport ns ÷ median `solve_wall_ns` of
    each arm at the destination level;
  - in GAE, Vélu evaluations priced at 0, 0.1 and 1 GAE each, relative to the
    arm's mean `ops_gae`.

  "Transport, then solve" is reported against "solve where born" for each arm,
  as a ratio with its band. No such ratio is a speedup claim unless it excludes
  1 at all three prices and exceeds the control spread.

## 6. Positive control (PLAN §5 test 5)

On the V3 crater curve, with the D, N and ctl rows pooled, compute the ratio
mean S(`rho-cubic`) / mean S(`rho-neg`), with a bootstrap 95% CI.

- Theory gives about 1/√3 ≈ 0.58.
- **Pass:** the CI upper bound is below 0.85.
- **Fail:** the report declares the detector broken, and every
  "equivalent" verdict in §3–§5 is relabelled "uninformative".

## 7. Multiplicity

Holm–Bonferroni at α = 0.05 is applied separately to two families:

- **(a)** all §3 "different" tests across V × arm × L;
- **(b)** the §4 variance-ratio tests across V × arm.

§4 per-curve flags are corrected within their own (V, arm). TOST verdicts are
not adjusted. §5 paired ratios form their own family (c), also Holm-corrected.

## 8. Wall time

Wall is a result for a (V, arm) only if both conditions hold:

1. Every included row is at isolation `L2`.
2. The A/A ratio mean wall(`rho-neg-aa`) / mean wall(`rho-neg`) has a 95% CI
   inside [0.95, 1.05] for that volcano.

If either fails, wall tables are labelled "descriptive (not admitted)". Wall
ratios use the same cluster bootstrap.

## 9. IC against matched rho (PLAN §5 test 8)

For each V and level, compute mean S(`ic`) / mean S(`rho-neg`) with a 95% CI.

- Each arm's uncharged counters come from the sessions' `ecbench table` /
  `ecbench compare --save` output for (rho-neg, ic) on each session.
- The ratio is recomputed with every uncharged count priced at 0, 0.1 and
  1 GAE per operation on both arms.
- The two arms are ranked only if their order holds at all three prices. An
  arm with an unemitted count is a lower bound.
- The ratio is the distance remaining, not pass/fail.

## 10. Outputs

- `analysis/results.json`: every statistic above, with n (curves, rows),
  seeds, and the input file hashes.
- Figures, each as SVG + PNG:
  1. Forest plot of R(L) per V × arm, with TOST bands.
  2. The volcano diagram per V, coloured by per-curve mean S for rho-neg and
     ic (`tools/volcano-diagram`).
  3. IC/rho ratio by level with the pricing band.
  4. Per-curve mean S against the control band (§4).
  5. Transfer cost against solve cost by path length.
  6. A CV and achieved-precision table rendered as a figure.
- `REPORT.md` and `report.pdf`, following the research-visuals skill. Copies
  go in `benchmarks/20261017-volcano-ecdlp-census/`, with a row in
  `benchmarks/README.md`.
- The report states every limit: shared 4-vCPU VM; one volcano per ℓ; V3
  descriptive; V4 unmeasured; the 5-day pause; A1 amendments; and that this
  file was frozen after data collection but before inspection.
