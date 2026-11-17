# Neutral reviewer-pattern outputs

## Evidence boundary

These outputs are deterministic regroupings of the two standardized inputs in
the parent review packet. They add no target execution. All candidate/rho
figures use the representative price `M + 0.5S + 10I` plus `0.1 GAE` per
native logical operation unless the file carries explicit price-cell columns.

| requested view | generated evidence | boundary |
| --- | --- | --- |
| paired deltas | `paired-deltas.csv`, 12,850 rows | 11,586 rows have verified paired costs; 1,264 rows retain an explicit null/exclusion reason |
| ECDF and pencil missing mass | `ecdf.csv`, `ecdf.svg`, `ecdf.png` | all curves are right-continuous empirical steps; pencil solid curves are conditional and lower/upper curves place all missing ratios above/below each threshold |
| 36-cell sensitivity | `price-cell-heatmap.csv`, SVG and PNG | four 36-cell panels; pencil pools ten observed curve aggregates by summing costs before division |
| descriptive tails | `descriptive-tails.csv` | nearest-rank empirical quantiles by arm and order size |
| model-cluster resampling | `cluster-bootstrap.csv` | 10,000 deterministic resamples of observed curve models; descriptive intervals, not confidence coverage claims |
| exact break-even coefficients | `break-even.csv` | 85 groups; every integer coefficient set reconstructs all 36 source deltas with relative error below `1e-8` |
| phase fractions | absent; N5 remains partial | no common phase counters in the standardized target input; source-specific normalization remains unimplemented |

`paired-deltas.csv` marks rows with `included_in_cost_analysis`. Four pencil
attempts have `null_reason=sparse_matrix_cap`. The 1,260
`coordinate_identity_gate` rows have
`null_reason=coordinate_only_kummer_no_dlp_cost`; they remain auditable but do
not enter a cost plot or statistic.

`ecdf.csv` has one row per arm and frozen workload key. For an observed ratio
at rank `r`, observed count `n`, and cohort size 1,260:

```text
conditional_ecdf = r / n
cohort_ecdf_lower = r / 1260
cohort_ecdf_upper = (r + 1260 - n) / 1260
```

The bounds make no assumption about unobserved ratios. An explicit missing
row has empty ratio and ECDF fields. There are 717 such rows for
`online_identity`, 719 for `peel_identity`, 1,079 each for `online_affine`
and `online_reverse`, and 1,200 for `markowitz_priority`.
The SVG uses horizontal segments followed by vertical jumps at each distinct
observed ratio, so each conditional and cohort-bound path is right-continuous.

`cluster-bootstrap.csv` resamples whole observed `p` clusters with replacement
and computes the ratio of summed candidate GAE to summed rho GAE. Full-cohort
arms resample 21 frozen curve models. Pencil arms resample only their observed
models and retain their missing-target count; the one-model Markowitz interval
therefore collapses to its point estimate.

For each row of `break-even.csv`, candidate and rho have raw counter
differences `dM`, `dS`, `dI`, and logical difference `dL`. With source-add
unit `U = 9 + 4S + I`, their exact equality surface is:

```text
dM + dS*S + dI*I + dL*N*(9 + 4S + I) = 0
```

The integer coefficients are recovered from the source grids and then checked
against all 36 prices. The CSV records absolute GAE and relative residuals.
Here “exact” means the unique integer coefficients implied by the declared
36-cell arithmetic; this appendix does not independently recount the
study-specific raw JSON operation counters. Across the 85 groups, the largest
reconstruction residual is `0.000091553 GAE` absolute and
`5.036573605255e-12` relative.

## Selected observations

The following are file summaries rather than conclusions about research
significance:

- full-16 factor base: ratio of summed costs `2.448829986555`; descriptive
  model-resampling interval `[2.259778467591, 2.698796658385]`;
- retained-state projective law: `4.179297043461`, interval
  `[3.918669427664, 4.466615573311]`;
- uncached differential Kummer: `14.356662228982`, interval
  `[13.324768754160, 15.208447019973]`;
- degree-peel pencil on 541 observed targets: `905.826373484126`, interval
  `[329.389076989494, 1548.138301016511]`, with 719 arm-specific missing targets.

Across the heatmap panels, the 36-cell ranges are `1.345253–13.126486` for
full-16 factor base, `1.472855–28.015265` for retained-state projective,
`8.959687–55.935424` for uncached differential Kummer, and
`18.080987–1068.927266` for the ten-curve pooled degree-peel pencil rows.
These are ratios of aggregate costs, not averages of per-target ratios.
