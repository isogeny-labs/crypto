# Executed n=83 genus-3 image transfer and destination solvers

This study closes the executable transfer and solver-reachability gates for the
pinned same-field degree-three cover. It does **not** claim that the genus-three
destination is weaker or faster.

## Outcome

- The exact `n=83` pullbacks of `P` and `Q=[37]P` are valid reduced Mumford
  classes of exact prime order `r=2417851639230796216685689`.
- Typed certificates prove `phi_*phi^*=[3]` for `P`, `Q`, infinity, and the
  exceptional rational two-torsion fiber. Since `gcd(3,r)=1`, the pullback is
  injective on the certified subgroup; `3^-1 mod r` is recorded.
- `jacobian_image.rho3` and `jacobian_image.bsgs` construct the cover, pull the
  source instance into the actual Jacobian image, solve there without receiving
  the planted scalar, and independently replay the answer on the source.
- Four faithful successful toy sizes (`n=9,13,19,23`) and the retained `n=7`
  failure are sealed and replayed. All 400 measured destination attempts
  verified. Both source copies exhausted at `n=7`; all 320 measured source
  attempts at the other four sizes verified.
- Destination BSGS is cheaper than destination rho in native Jacobian operations
  on the panel. Both are generic square-root algorithms. The source/destination
  operation totals have no certified common unit, and wall observations are L0
  descriptive. No net advantage is promoted.

## Start here

- [Run report](RUN_REPORT.md)
- [Results and interpretation](RESULTS.md)
- [Operational transfer/measurement guidance](GUIDANCE.md)
- [Validated v3 assessment](TRANSFER_ASSESSMENT_V3.json) and its
  [validator receipt](ASSESSMENT_VALIDATION.json)
- [Final exact certificate](runs/exact-n83-post-panel-20261031T044500Z/certificate.json)
- [Quantitative figure](figures/native-cost-and-wall.svg)
- [Open destination-calculus/common-cost obligation](../../problems/genus3-image-destination-calculus-and-common-cost-missing.md)

The source of truth is the two sealed directories under `sessions/`; generated
tables and figures are derived convenience views.
