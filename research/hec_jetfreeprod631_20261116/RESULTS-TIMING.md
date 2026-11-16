# Production shortcut cohort: no demonstrated speedup,10xgate failed

All10cells pass and recover both original classes. Frozen de0bddf85eea/checkpoint20261116T033149Z precedes one benchmark cohort with corrected production binaries. No cells replaced or rerun. Record20261116T0331Z-hec-jetfree631; raw archive/input/output/statuses and table retained in content-addressed S3.

| Arm | n | Median component s | Observed min–max s | Median process s |
|---|---:|---:|---:|---:|
|Previous direct|5|5.887982083|2.295271584–8.760866375|5.890616792|
|Production no-base shortcut|5|6.329883375|2.136444166–9.979136209|6.334192042|

Derived primary medianPrevious/medianProduction=0.930188083126, below10. The numerical gate fails. Strong run-order drift occurs in both arms: block0times2.295/2.136s, block4times8.761/9.979s. Alternating order and successful correctness checks do not remove this uncertainty. The observed ratio is not persuasive evidence of a stable algorithmic regression or speedup. No repeat run was silently substituted. No confidence interval, isolation or cache control is claimed.

Only the reconstruction executable changes between arms. Both include identical direct generation/solver/exact verification, class reconstruction/adjacency and final independent class comparison. Common quotient setup and public-coordinate preparation remain reused outside this inverse-component boundary; conic descent is excluded. This is not a cold end-to-end result. Historical1.621xcomparison is a different cohort and is neither overwritten nor pooled/multiplied with this ratio.

The algebraic shortcut remains correctness-supported: generic path skips redundant Hensel/reference work, exceptional cancellation retained. This cohort supplies no reliable performance benefit. Before another timing claim, establish stable same-host controls and finer charged phase attribution under a separately frozen protocol; do not retune acceptance using these timings. The previous cost gap and genus2/4/full10xobjective remain unresolved.
