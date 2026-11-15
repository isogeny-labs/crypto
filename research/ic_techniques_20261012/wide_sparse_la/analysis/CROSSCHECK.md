# Cross-check: Rust port against the earlier Python analysis

Compared on the real data (`raw/seed1..3`, 3 RunPod pods): `results.json` written by the
removed `analysis.py` (python3 + numpy, run once before deletion, byte-identical to the
previously committed `results.json`) against `results.json` written by
`wide-sla-analysis --raw raw --out results.json`. Comparison tool: `wide-sla-crosscheck A.json B.json`
(`src/bin/crosscheck.rs`, native). Every numeric leaf is compared by relative difference
|a-b|/max(|a|,|b|); strings, booleans, nulls, key order and array lengths must match exactly.

Bootstrap-dependent leaves are every `*ci95` field (size-law `a_ci95`, `beta_ci95`, curvature `ci95`,
extrapolation `t3_ci95_ns`, `row_ops_ci95`, `time_*_ci95`). They differ because the generator differs
(numpy PCG64 in Python; xoshiro256** seeded by SplitMix64 from 20261112 in Rust). The t-based
hold-out `pi95` is not bootstrap-dependent and is compared as a point estimate.

Edge cases also run with both implementations (empty raw dir; seed1 only with a truncated grid,
a malformed line and no done.json): 0 structural differences, markdown identical, non-bootstrap
max rel diff 1.5e-13.

## Result (full data)

```
numeric leaves compared: 4478 (exactly equal: 3722)
  non-bootstrap: 4262   bootstrap-dependent (*ci95): 216
non-bootstrap leaves with rel diff > 1e-9: 0
max rel diff, non-bootstrap: 5.094e-13 at /falsification/curvature/U192@129|full-bl/coef_log2D_sq (0.0030905789437610243 vs 0.00309057894375945)
bootstrap-dependent leaves: max rel diff 2.745e-1, median 7.213e-5, #>1e-9: 212, #>1e-3: 35, #>1e-2: 11
  BOOT /falsification/curvature/U128@127|full-bl/ci95[0] : -0.00016760533574876509 vs -0.00023101777046169718 (rel 2.745e-1)
  BOOT /falsification/curvature/U192@129|full-bl/ci95[0] : -0.0001690774062162061 vs -0.0002312788831357797 (rel 2.689e-1)
  BOOT /falsification/curvature/U256@255|full-bl/ci95[0] : -0.00016687110293474972 vs -0.00022457415646567885 (rel 2.569e-1)
  BOOT /falsification/curvature/U192@190|full-bl/ci95[0] : -0.00019923675867222342 vs -0.00025684736540595993 (rel 2.243e-1)
  BOOT /falsification/curvature/U128@127|full-bw/ci95[0] : -0.00025797025711250164 vs -0.0003049151396845806 (rel 1.540e-1)
  BOOT /falsification/curvature/U192@129|full-bw/ci95[0] : -0.00026009147315149607 vs -0.000306054788214441 (rel 1.502e-1)
  BOOT /falsification/curvature/U192@190|full-bw/ci95[0] : -0.000279750081686854 vs -0.00032751228253674873 (rel 1.458e-1)
  BOOT /falsification/curvature/U256@255|full-bw/ci95[0] : -0.000256850887431275 vs -0.00029943334849204245 (rel 1.422e-1)
  BOOT /falsification/curvature/u64@61|full-bl/ci95[0] : 0.0004864599436442891 vs 0.0004609449007766234 (rel 5.245e-2)
  BOOT /falsification/curvature/U128@61|full-bl/ci95[0] : 0.0004864599436442891 vs 0.0004609449007766234 (rel 5.245e-2)
structural / string / bool / null differences: 0
```

Non-bootstrap fields (OLS fits, medians, ratios, coverage counts, A/A bands and determinism,
width-law and hold-out/curvature statuses, point estimates of the extrapolation, notes, pods,
integrity): maximum relative difference 5.1e-13 (QR least squares against numpy lstsq); none above 1e-9.
All test statuses and verdicts are identical. The curvature lower CI bounds near zero (e.g.
U192@129 full-bw: -0.00026 Python, -0.00031 Rust) carry the largest relative bootstrap differences
because the quantity is close to 0; no curvature status changes.
