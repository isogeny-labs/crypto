# Genus-3 image destination calculus and common-cost calibration are missing

Status: open; standard full-Jacobian calculus and exact generic run rejected,
common-cost L2 pending

## Blocked claim

The executed degree-three pullback from the pinned ordinary binary elliptic
curve into a genus-three Jacobian is a valid, scalar-preserving transfer, and
generic BSGS/rho solvers reach its cyclic image. That is not yet evidence that
the destination DLP is weaker or that the route beats the source solver.

Two linked obligations block that promotion:

1. no complete, named non-generic destination calculus has been implemented
   on the transported image subgroup; and
2. source and destination do not yet share a certified arithmetic conversion.
   Separately named accounting-only destination adapters now tally
   instrumented `F2mElement` multiplications, squarings, and inversions over
   the complete route, but the optimized source solver uses a different
   kernel. The untallied timing adapters stay free of ledger overhead. The
   earlier wall samples earned only L0 isolation, and the paired-host L2 panel
   remains unexecuted.

The exact object is the order-
`2417851639230796216685689` image in the Jacobian of
`v^2+u^2*v=u^7+u` over the pinned `GF(2^83)` basis. The toy family is the same
`a=0`, `b=1`, degree-three cover at `n=7,9,13,19,23`.

## Evidence

- The final exact certificate is
  [`research/n83_genus3_image_solver_20261031/runs/exact-n83-post-panel-20261031T044500Z/certificate.json`](../research/n83_genus3_image_solver_20261031/runs/exact-n83-post-panel-20261031T044500Z/certificate.json).
- The sealed sessions and all-record replay receipts are under
  [`research/n83_genus3_image_solver_20261031/sessions/`](../research/n83_genus3_image_solver_20261031/sessions/).
- [`TRANSFER_ASSESSMENT_V3.json`](../research/n83_genus3_image_solver_20261031/TRANSFER_ASSESSMENT_V3.json)
  separates valid transfer and solver applicability from the unknown advantage.
- The clean six-round untallied session in
  [`COMMON_COST_LOCAL_RESULTS.md`](../research/n83_genus3_image_solver_20261031/COMMON_COST_LOCAL_RESULTS.md)
  completed 896/896 verified records and reproduced all 768 measured records.
  Its paired median BSGS/source ratio grows from 5.36 at `n=9` to 131.32 at
  `n=23`; destination-rho/source grows from 9.82 to 364.98. A/A stays within
  2.4% of one. This L0 result closes the toy go/no-go gate against an exact
  generic `n=83` attempt, but it is not an admitted cross-host speed claim.
- The separate deterministic
  [`FIELD_ACCOUNTING_RESULTS.md`](../research/n83_genus3_image_solver_20261031/FIELD_ACCOUNTING_RESULTS.md)
  reports complete-route binary-field counts without putting instrumentation
  overhead into the timing arms.
- [`DESTINATION_CALCULUS_SCREEN.md`](../research/n83_genus3_image_solver_20261031/DESTINATION_CALCULUS_SCREEN.md)
  rejects the off-the-shelf full-Jacobian genus-three calculus for this
  order-`r ~= q` image: its `q^(4/3)` ledger is about 70.17 exponent bits
  above source rho here, and the current API cannot project arbitrary
  factor-base classes into the image.
- [`n83-genus3-image-common-cost-l2.json`](../docs/ecbench/specs/n83-genus3-image-common-cost-l2.json)
  freezes the same four sizes, three substantive arms, and A/A control for
  identical EKS Spot runs on `c8a.2xlarge` and `m8a.2xlarge`.

## Exit criteria

This record can be resolved only when both of the following are met.

### Destination weakness gate

A named destination algorithm must be complete over the exact transported
image: factor-base or search space definition, smoothness/relation rule,
relation collection, linear algebra, individual logarithm/descent, proof that
the image is reachable by every stage, scalar recovery, and independent source
replay. A formula-only or relation-generation-only prototype does not pass.
Its cost model must predict a lower exponent than generic square root on the
actual family, or a checkable finite-size crossover.

### Net measurement gate

Every phase in source reference, route, destination generic control, and
destination candidate must be charged in one versioned unit, with no material
unpriced work. Alternatively, paired cold one-target wall trials must earn L2
or stronger isolation on two hosts, include an A/A control, verify every
target, replay deterministic records, and put the complete source/candidate
ratio interval above one at four or more faithful sizes. Memory, storage,
precomputation reuse, and amortization must be reported separately.

## Next discriminating experiment

Do not implement the ambient full-Jacobian calculus under a new method name.
First design an image-compatible factor base and either an image projection or
image-closed relation rule, with a relation probability and exponent stated in
the actual order `r`. Do not spend resources on an exact `n=83` generic BSGS or
rho run: the 81-bit image implies roughly `2^40.5` generic group steps and the
four-size panel shows a widening loss. In parallel, execute the frozen
common-cost L2 spec once on both registered Spot host classes. Only an
image-compatible candidate that passes the six admission criteria in
`DESTINATION_CALCULUS_SCREEN.md` should be implemented and measured against the
existing generic controls.
