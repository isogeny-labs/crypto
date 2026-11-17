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
- A clean six-round untallied follow-up completed 896/896 verified records and
  replayed all 768 measured records. Its paired median destination/source wall
  ratios widen over `n=9,13,19,23`, closing the toy gate against an infeasible
  exact generic `n=83` run. A separate deterministic ledger records the complete
  destination routes without contaminating timing.
- The standard full-Jacobian genus-three index calculus is not an applicable
  solver for this order-`r ~= q` image. Its ambient `q^(4/3)` ledger loses to
  source rho by about 70.17 exponent bits here, and no image-compatible
  factor-base projection has been supplied.

## Start here

- [Run report](RUN_REPORT.md)
- [Results and interpretation](RESULTS.md)
- [Operational transfer/measurement guidance](GUIDANCE.md)
- [Destination-calculus applicability screen](DESTINATION_CALCULUS_SCREEN.md)
- [Untallied four-size common-cost result](COMMON_COST_LOCAL_RESULTS.md)
- [Separate complete-route field accounting](FIELD_ACCOUNTING_RESULTS.md)
- [Validated v3 assessment](TRANSFER_ASSESSMENT_V3.json) and its
  [validator receipt](ASSESSMENT_VALIDATION.json)
- [Final exact certificate](runs/exact-n83-post-panel-20261031T044500Z/certificate.json)
- [Quantitative figure](figures/native-cost-and-wall.svg)
- [Open destination-calculus/common-cost obligation](../../problems/genus3-image-destination-calculus-and-common-cost-missing.md)

The source of truth is the sealed directories under `sessions/`; generated
tables, reports, and figures are derived convenience views. The directory named
`common-cost-local-lt-l2-20261031T064500Z` is deliberately retained as an
invalid instrumentation-overhead preflight and must not be used for timing
claims.
