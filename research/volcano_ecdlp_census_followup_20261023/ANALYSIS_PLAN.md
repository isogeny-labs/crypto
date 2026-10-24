# Analysis plan (follow-up study; frozen before any Phase B session ran)

Frozen 2026-10-23, at the end of Phase A. No ecbench session has been run in
this study; no cost has been computed or viewed. This file is the census's
`ANALYSIS_PLAN.md` (research/volcano_ecdlp_census_20261017/) with only the
changes and additions that `PLAN.md` §4 lists, plus the census amendments
A2.1 and A2.3 written in as rules from the start (their post-hoc labels
apply to the census only). Sections that say "as the census" are the census
text unchanged. `analysis/analyze.py` (Phase C) must implement exactly this;
any departure goes into `AMENDMENTS.md` with a reason and is labelled
post hoc in the report.

## 1. Data

- **Sessions.** Included if `runs/progress.jsonl` has a line with
  `"status":"done"` and `audit_ok: true`. Lines without a status (cleanup
  re-verifies) are ignored. Each session counts once.
- **Rows.** From `runs/extract/<spec>.csv.xz` with `warmup == false`. Rows with
  `verified != true` are counted per (volcano, family, arm) and reported as
  the failure rate with a Clopper-Pearson 95% CI, and excluded from cost
  statistics; an arm with any unverified row is flagged.
- **Primary unit.** `S`. Secondary: `ops_gae`. Wall (`solve_wall_ns`) is a
  result only under §8; otherwise descriptive.
- **Uncharged counts (census A2.1, in force from the start).** Counts of the
  counters that `cost.unpriced` names come from each session's `records.jsonl`
  (not from `ecbench table` / `compare`, which print names only). A counter
  takes its value from the record's top-level `counters` if present, otherwise
  from the sum of the phases' `native` counters of the same name without the
  `_uncharged` suffix. They are extracted per session (unpacked from its
  archive, one at a time) into `runs/uncharged/<spec>.csv.xz`, one row per
  measured run: `total_gae`, `S`, the sum of the uncharged counts and the
  per-counter detail. Priced S at price p is
  `S * (total_gae + p * uncharged) / total_gae`; priced `ops_gae` is defined
  the same way. The totals (`total_gae`, `S`) must agree with the CSV extracts
  row by row; mismatches are reported. Only counters named in `cost.unpriced`
  are priced; any other native work is not counted, so both arms stay lower
  bounds.
- **Curve = census vertex (census A2.3, in force from the start).** Every
  bootstrap that resamples "curves" resamples **census vertices**, then rows of
  each chosen vertex (all families together), stratified by level. A
  `curve_slug` identifies a curve model, and a vertex has several (1 to 4)
  across families, so slugs are never the resampling unit. The `ctl` curves
  map to the vertex they re-scale. The M models map to their vertex; Tcr rows
  map to their *source* vertex.
- **Volcanoes.** V6 (ell = 3, 648 vertices) gets the full analysis (Q1). V5 (ell = 2,
  j = 0 crater, 64 vertices) gets Q2a, Q2b and H-T. M curves (H-M) and Tcr
  specs (H-T) are analysed only in their own sections and are never pooled with
  the isogenous families.

## 2. Arms and families

- **Arms:** `rho-neg` (reference), `rho-plain`, `bsgs`, `bsgs-neg`, `ic`,
  `rho-neg-aa` (wall A/A only), `rho-cubic` (V5 crater specs D, N, ctl, and every
  Tcr spec). M specs carry `rho-neg`, `bsgs`, `ic` only. Tcr specs carry
  `rho-cubic` and `rho-neg` only.
- **Families:** V6: D, Ua, Ub, S, N, ctl. V5: D, Ua, Ub, N, ctl (the crater is one
  vertex, so there is no S, and Ua and Ub, which `ca` cannot carry onto j = 0,
  have no level-0 rows), plus Tcr and M.
- "Isogenous" = D, Ua, Ub, S, N pooled (without S on V5). ctl is the negative
  control and is never pooled with the isogenous families. Tcr and M are
  never pooled with them.

## 3. Level effect (census PLAN §5 tests 1-2; Q1 on V6, Q2b on V5)

Run this for each volcano V in {V6, V5}, each arm in {rho-neg, rho-plain, bsgs,
bsgs-neg, ic}, and each level L >= 1. `rho-cubic` is excluded from the level
analysis on V5.

- **Statistic.** R(L) = mean S at level L / mean S at level 0 (the crater),
  using isogenous-family rows. Carry `ops_gae` and `solve_wall_ns` on the same
  resamples (wall is descriptive unless §8 admits it).
- **CI: cluster bootstrap.** 10,000 replicates, seed 20261022, stratified by
  level. Each replicate resamples **census vertices** with replacement within
  each level, then rows with replacement within each chosen vertex. This gives a
  95% percentile CI and a 90% CI for TOST. (The census's frozen two-stage
  bootstrap is conservative by about sqrt(2) when the ICC is near 0; kept, as
  census A3 N1.)
- **TOST margins.** +-5% for rho and BSGS arms: [0.95, 1/0.95]. +-10% for `ic`:
  [0.90, 1/0.90].
  - **"equivalent"** if the 90% CI lies inside the margin.
  - **"different"** if the 95% CI excludes 1 *and* the Holm-adjusted p < 0.05
    (§7). p = 2*min(P(R* <= 1), P(R* >= 1)) with +1 smoothing, used only for Holm.
  - **"inconclusive"** otherwise. A result that meets both TOST and
    "different" is labelled "equivalent" (this order).
  - The achieved 95% half-width is always reported.
- **V5 levels 1-3 are descriptive.** They have 1, 2 and 4 vertices, too few to
  generalise across curves. Their R(L) and CIs are reported, their verdict is
  "descriptive", and they are not in the Holm family. **Levels 4-6** (8, 16 and
  32 vertices) get TOST verdicts. Level 0 is the single crater vertex and is the
  reference.
- **Secondary model.** `log S ~ C(level) + C(family) + (1 | curve)`, statsmodels
  `MixedLM`, REML, curve = census vertex. Report level coefficients as ratios
  with Wald 95% CIs, the curve variance component and the ICC; if it does not
  converge, report that. It never overrides the bootstrap.
- **Robustness.** R(L) per family (D, Ua, Ub, S, N separately), descriptive.

## 4. Curve effect against the negative control (census test 3; Q1 on V6 only)

Run for V6 and each arm. (Not run on V5: PLAN §4 lists only Q1 for this test.)

- **Data.** For every vertex v in the D family that has ctl copies: m_v = mean S
  of the D curve; c_{v,1}, c_{v,2} = mean S of its two re-scalings.
- **Variance-ratio test.** F = (variance of m_v across vertices within each level,
  pooled) / (pooled within-vertex variance of {m_v, c_{v,1}, c_{v,2}}).
  Permutation test: shuffle the three labels within each vertex, 10,000 times,
  seed 20261023.
- **Per-curve flags.** z_v = (m_v - mean(c_v)) / pooled control SD. Within-vertex
  label shuffles have 3 outcomes per vertex, so p_v is the share of the pooled
  |z*| (all vertices, all 10,000 shuffles) at least |z_v|, Holm over the curves
  of that (V, arm). Flag if adjusted p < 0.01. A (V, arm) with zero within-vertex
  variance (BSGS: S depends only on r and k) reports no F and no flags.
- If the V6 `ctl` family is reduced (PLAN §3), only vertices with ctl copies
  enter; the reduction, if applied, is recorded in `runs/manifest.json`
  (`ctl_subset`) and the report.

## 5. Transfer (census test 4; Q1 on V6 only)

- **Recovery.** The verified fraction per (V, family, arm) with a Clopper-Pearson
  95% CI (expected 1). Reported for every family on both volcanoes.
- **Paired source comparison (V6).** Within a vertex, D, Ua, Ub and S rows share
  target_index and the scalar k_i. The unit is the (vertex, target_index, round)
  triple, resampled jointly across the two families, pooled over levels,
  strata = level. N does not share k_i with D, so its pairing is nominal.
  - **BSGS arms:** the fraction of triples with identical `ops_gae` across
    families (a sanity check; expectation 1). No ratio.
  - **rho-neg, rho-plain, ic:** ratio of means Ua/D, Ub/D, S/D, N/D, §3
    bootstrap, §3 TOST margins. Holm family (c) = V6 x {rho-neg, rho-plain, ic} x
    {Ua, Ub, S, N}.
- **Transport cost.** From `runs/transfer_cost.jsonl` (`ca volcano
  --transport-bench`, own lock window): per path length d, ns per edge and per
  path (median, IQR) and Velu evaluations per path; transport/solve fraction =
  transport ns / median `solve_wall_ns` of each arm at the destination level.
- **Transport, then solve (V6).** Per (V, source family, arm, destination level):
  [mean `ops_gae` at the destination + p x Velu point maps of the path] / mean
  `ops_gae` of the same family's rows at the source vertex, for p = 0, 0.1, 1
  GAE per point map. Both point maps (G and Q) are charged to every instance,
  with no amortisation. The control is the same ratio computed with the native
  (N) rows at the same destinations and the same source vertex, no Velu cost.
  A signal needs the CI to exclude 1 on the lower side at all three prices and to
  be disjoint from the control CI. No such ratio is a speedup claim unless it
  does. The source mean comes from one vertex, so it is noisy and shared by all
  destination levels.

## 5b. H-T: carry up, then solve with the automorphism (V5; new)

**Data.** The 63 Tcr specs (source vertex v, one spec each, born at levels 1-6)
and, for each v, the N-family rows of v's own curve (`rho-neg`, native generator
gn_v, same target seed, same k_i). The Tcr rows are on the census crater model
with generator psi_v(gn_v); the rounds of Tcr_v equal those of the N family at
v's level.

**Cost unit.** `ops_gae`, with uncharged counters priced as in §1 at the same
price p. M_v = Velu point maps of the carry-up path of v, both point maps (G and
Q) charged, no amortisation: `velu_point_maps` of the level-1 ancestor's line in
`runs/transfer_cost.jsonl` (`src = tcr-v<id>`, the path v to w) **plus 2** for the
last edge w to j = 0, which `ca` does not take (it is done by the study's own
verified helper; counted as 2 point maps, modelled like `ca`'s 9 field
multiplications and 1 inversion per ell = 2 map). Its wall time is not measured
and is not part of any result.

**Statistic.** For each v and p in {0, 0.1, 1}:

    T_v(p)  = [ mean C_p(rho-cubic, Tcr_v) + p * M_v ] / mean C_p(rho-neg, N_v)
    T0_v(p) = [ mean C_p(rho-neg,   Tcr_v) + p * M_v ] / mean C_p(rho-neg, N_v)

where C_p is priced `ops_gae` and the means are over the rows of the named spec
and arm. T0 is T's control: the same transport and the same crater curve, no
automorphism. **Pooled** values: the same ratios with the numerators and
denominators averaged over the vertices of a stratum (ratio of means); the
**per-level** pooled values are descriptive; the **overall** pooled value (all 63
vertices, strata = level 1-6) carries the verdict.

**CI.** Cluster bootstrap over source vertices: 10,000 replicates, seed 20261023,
stratified by level; each replicate resamples vertices with replacement within
each level, then (target_index, round) pairs within each chosen vertex jointly
across the Tcr_v and N_v rows. 95% percentile CI at each price, for T and T0.

**Verdict** (PLAN §4, applied at each of the three prices):

- **Supported:** the overall pooled CI of T lies below 1 at all three prices, and
  the CI of T0 includes 1 at all three prices.
- **Refuted:** the overall pooled CI of T includes 1 or lies above it at any price.
  (The price enters only through the added transport term, so p = 1 is the binding
  price.)
- **Inconclusive** otherwise (T below 1 at all prices, but the control's CI
  excludes 1 at some price).

Also reported (descriptive): the theory value 1/sqrt(3) = 0.577 and the measured
pooled rho-cubic / rho-neg ratio on the Tcr rows alone (the same-curve automorphism
gain, in S), the transport term as a share of the numerator at each price, and the
per-level and per-vertex T_v with their CIs.

## 5c. H-M: does IC cost depend on the Weierstrass model? (new)

**Data.** The 12 M specs: V5 crater + one V5 vertex each at levels 4, 5, 6, and 8
V6 vertices (levels 0, 0, 1, 1, 2, 2, 3, 4). Each spec holds the 8 models of one
curve, each with 16 targets and 4 rounds (64 non-warmup rows per model and arm),
arms `rho-neg`, `bsgs`, `ic`. Response: S.

**Test**, for each chosen curve and each arm: one-way test across its 8 models.

- Statistic F = between-model mean square / within-model mean square (one-way
  ANOVA on S, equal group sizes). A (curve, arm) whose within-model variance is
  zero reports no F and is counted as not significant.
- Permutation p: 10,000 shuffles of rows across the 8 models, seed 20261024,
  p = (1 + #{F* >= F}) / (1 + 10,000).
- Holm across the 12 curves of each arm separately, alpha = 0.05. "Significant"
  means Holm-adjusted p < 0.05.

**Verdict.**

- **Supported for IC:** `ic` is significant on at least half of the chosen curves
  (6 of 12) while `rho-neg` and `bsgs` are each significant on at most 1 curve.
- **Refuted:** `ic` is significant on at most 1 curve.
- **Inconclusive** otherwise.

**Effect size.** Per curve and arm, the max/min ratio of model means with a 95%
percentile CI (10,000 replicates, seed 20261024 on a separate generator, rows
resampled within each model). The census's V3 crater values (IC 57 / 67 / 73
on three models, A3 F2) are quoted as the motivating observation and are not
pooled. BSGS is expected to be exactly model-independent (S depends on r and
k_i only; k_i are identical across models), which also serves as the check that
the models carry the same instances.

## 6. Positive control (census test 5; Q2a on the V5 crater)

On the V5 crater curve, with the D, N and ctl rows pooled (the M and Tcr rows
are not pooled), compute mean S(`rho-cubic`) / mean S(`rho-neg`), the primary
bootstrap resampling (workload, target, round) pairs of `rho-cubic` and
`rho-neg` (the crater is one curve, so a vertex cluster has nothing to resample),
10,000 replicates, 95% percentile CI; a slug-cluster resample is reported as a
sensitivity.

- Theory gives about 1/sqrt(3) = 0.58. **Pass:** the CI upper bound is below 0.85.
- **Fail:** the report declares the detector broken, and every "equivalent"
  verdict in §3-§5c is relabelled "uninformative".
- The V3 result of the census is quoted for comparison, not pooled.

## 7. Multiplicity

Holm-Bonferroni at alpha = 0.05, applied separately to:

- **(a)** all §3 "different" tests across V x arm x L (V5: levels 4-6 only);
- **(b)** the §4 variance-ratio tests across V x arm (V6 only);
- **(c)** the §5 paired ratios (V6 only), defined above;
- **(d)** H-M: across the 12 curves of each arm (§5c).

§4 per-curve flags are corrected within their own (V, arm). TOST verdicts and
the H-T and Q2a verdicts, which are CI rules fixed in advance, are not adjusted.

## 8. Wall time

As the census. Wall is a result for a (V, arm) only if both hold: every included
row is at isolation `L2`, and the A/A ratio mean wall(`rho-neg-aa`) / mean
wall(`rho-neg`), pooled over the same (workload, target, round) triples with the
level-stratified vertex bootstrap, has a 95% CI inside [0.95, 1.05] for that
volcano. If either fails, wall tables are labelled "descriptive (not admitted)".
Only families with a `rho-neg-aa` arm enter (D, Ua, Ub, S, N, ctl); M and Tcr
specs have no A/A arm, so their wall times are descriptive.

## 9. IC against matched rho (census test 8, with A2.1 pricing)

For each V and level, mean S(`ic`) / mean S(`rho-neg`) with a 95% CI, from
(ic, rho-neg) pairs of the same (workload, target, round); the uncharged counts
are priced at 0, 0.1 and 1 GAE on both arms as in §1. The two arms are ranked
only if their order holds at all three prices; an arm with an unemitted count
is a lower bound. The ratio is the distance remaining, not pass/fail.

- On the V5 crater the matched rho is `rho-cubic`, the strongest available: the
  crater's (ic, rho-cubic) pairs from D and N, at 0 / 0.1 / 1 GAE, as census A3 F1.
- The M specs give an additional per-curve IC/rho-neg ratio (descriptive).

## 10. Outputs

- `analysis/results.json`: every statistic above, with n (curves, rows), seeds
  and input file hashes.
- Figures, each as SVG + PNG: (1) forest plot of R(L) per V x arm with TOST bands;
  (2) the volcano diagram per V coloured by per-vertex mean S for rho-neg and ic;
  (3) IC/rho ratio by level with the pricing band; (4) per-vertex mean S against
  the control band (V6); (5) transfer cost against solve cost by path length;
  (6) a CV and achieved-precision table; (7) H-T: T_v by source level with CIs at
  the three prices, with the T0 control; (8) H-M: model means per curve for the
  three arms with the permutation verdicts.
- `REPORT.md` and `report.pdf` (research-visuals), copies in
  `benchmarks/20261023-volcano-ecdlp-census-followup/` (no README row until the
  validation gate is valid).
- The report states every limit: shared 4-vCPU VM; one volcano per question;
  V5 levels 1-3 descriptive; the last carry-up edge is outside `ca`; any guard
  stop or reduction with its denominator; and that this file was frozen before
  the first Phase B session.
