# L2: both [d]G = Q checks of the tw-lazy arms from the stored [2^i]G ladder (frozen before data, 2026-11-15)

Question. The target-walk lazy arms compute [d]G = Q twice by plain double-and-add (inside TargetWalk and in the
pipeline's recovery check) although their set-up already paid for the ladder [2^i]G. Routing both checks through
`ladder_mul` keeps both checks and the walks unchanged. What is the measured saving in counted GAE?

Instrument. ecbench single-target sessions (one DLP per run, `targets: stride-generator`, as the snc-panel),
spec per k in this directory (`spec-k20.json`, `spec-k24.json`): arms `rho-neg` (reference, `rho.negation`),
`tw2cj-lazy` (baseline, `wwalk:x=2.0,g=1,grow=2,tw=2,cj=1,lazy=1,seed=run`) and `tw2cj-lazy-lcheck` (candidate,
the same with `lcheck=1`), factor base `prime-projected-abscissa:size=2,scan_cap=32`, the snc-panel's five family
curves per k, 8 rounds, alternate order, seed 20261130 (so the walks of the two tw arms are identical per run).
Paired ratios from `ecbench compare` and `ecbench band` at w = 0, 0.1, 1 (§5a); `ecbench verify --replay 8`.
k = 20 first, then k = 24; k ≥ 28 only if time allows (the saving there is below 1 %).

Predictions (from the red team's recount: two checks, about 16 doublings + 7.5 additions each by double-and-add
against about 4.7 additions each over the ladder at log2 r = 17). P1: `tw2cj-lazy-lcheck` / `tw2cj-lazy` paired
ratio at w = 0 is 0.959 [0.950, 0.970] at k = 20 and 0.983 [0.975, 0.990] at k = 24, with the order holding at
w = 0.1 and 1. P2: the two tw arms have identical walk operations, entries and rows per run; only the check
counters differ. P3: the distance to rho-neg falls by the same factor and stays above 1.5× at both k.

Decision. P1 holding is a measured improvement of the C = 2 index-calculus arms under the owner's 2026-11-04 ruling
(the arms count as IC); it changes no exponent. P2 failing means the change touched the walk and the run is void.
Classification of `tw=2` arms remains the owner's call (red team R8); the result is reported for both readings.
