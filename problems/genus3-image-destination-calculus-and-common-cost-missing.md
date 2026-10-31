# Genus-3 image destination calculus and common-cost calibration are missing

Status: open

## Blocked claim

The executed degree-three pullback from the pinned ordinary binary elliptic
curve into a genus-three Jacobian is a valid, scalar-preserving transfer, and
generic BSGS/rho solvers reach its cyclic image. That is not yet evidence that
the destination DLP is weaker or that the route beats the source solver.

Two linked obligations block that promotion:

1. no complete, named non-generic destination calculus has been implemented
   on the transported image subgroup; and
2. native Cantor additions, polynomial arithmetic, table work, pullbacks, and
   source GAE do not have a certified common cost unit. The measured wall
   samples earned only L0 isolation.

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
- The four successful sizes show destination BSGS using fewer native
  Jacobian group operations than destination rho, as expected from their
  generic algorithms. The L0 descriptive candidate/source wall ratio grows
  from about 1.2 at `n=9` to about 13.6 at `n=23`. This is evidence against a
  practical win for this implementation on this host, not a lower bound or an
  impossibility theorem for another destination calculus.

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

Implement a small-field genus-three index-calculus adapter behind a new named
`ecbench` method. Run it first on `n=9,13,19,23` against the existing image BSGS
control. Stop if relation yield, matrix dimension, or individual-log success
does not follow the preregistered model. If it does, extend native counters to
field multiplications, inversions, memory bytes, and linear-algebra work before
considering the exact `n=83` family.
