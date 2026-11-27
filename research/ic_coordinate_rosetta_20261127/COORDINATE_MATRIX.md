# Coordinate and transfer matrix

## Scope key

| class | meaning in this study | comparable outcome |
| --- | --- | --- |
| IC function | a rational function on the fixed S59 curve used to define a factor-base predicate | matched density/yield, exclusions, algebraic class, extraction work |
| storage chart | a representation of a point on a curve model | round-trip, applicable formula correctness, counted operations, admitted wall time |
| curve model | a birational model when its torsion/precondition holds | compare only after an explicit map to the same curve is fixed |
| transfer | a homomorphism into another group/Jacobian | compare only with subgroup preservation, recovery and full transfer cost |

## Fixed-curve IC functions

| arm | algebraic reading | negation/scale expectation | role |
| --- | --- | --- | --- |
| `x` | degree-two Kummer coordinate | negation invariant; intrinsic | reference |
| `x_aa` | byte-identical repeat of `x` | same | A/A control |
| `jacobian_x_normalized` | `X/Z^2 = x` | negation and scale invariant | representation-equivalence control |
| `mobius_x` | `(7x+11)/(13x+17)` | negation invariant; intrinsic away from one pole | degree-two reparameterization |
| `jacobian_X_raw_negative_control` | raw `X=xZ^2` | scale dependent | negative control; not a curve function |
| `y` | degree-three function | changes under negation | descriptive |
| `x_plus_y` | degree-three function | changes under negation | descriptive |
| `x_squared` | degree-four composition through `x` | negation invariant, but image collisions can hide degree | protocol-gap control |
| `x_over_y` | rational function | changes under negation | descriptive on the primary curve; Pornin's degree-two class needs a compatible rational 2-torsion model |

Every arm uses the same `P,Q,R=-(P+Q)`, seed, sample count and threshold
`floor(p/16)`. The density-adjusted statistic is
`relation_rate / factor_base_density^3`; it is a finite-sample diagnostic, not
an asymptotic complexity exponent.

## Prime-field EFD inventory

The executable catalogue contains the following 33 charts. Each is checked
against the model-independent affine short-Weierstrass oracle produced by its
registered birational conversion; the oracle is not counted as a chart arm.

| model | charts | Rosetta interpretation |
| --- | --- | --- |
| short Weierstrass | `jacobian-0`, `jacobian-3`, `jacobian`, `modified`, `projective-1`, `projective-3`, `projective`, `w12-0`, `xyzz-3`, `xyzz`, `xz` | Jacobian, modified Jacobian, homogeneous projective, XYZZ and x-only/Kummer; special suffixes retain their coefficient preconditions |
| Montgomery | `xz` | x-only ladder chart |
| Edwards | `inverted`, `projective`, `yz`, `yzsquared` | inverted/projective and y-only quotient charts |
| twisted Edwards | `extended-1`, `extended`, `inverted`, `projective` | extended, inverted and projective charts |
| Hessian | `extended`, `standard` | natural and extended projective charts |
| twisted Hessian | `projective` | projective chart |
| Jacobi intersection | `extended`, `standard` | projective intersection coordinates |
| Jacobi quartic | `2xxyzz`, `2xxyzzr`, `2xyz`, `xxyzz`, `xxyzzr`, `xyz` | weighted/projective and cached variants |
| 2-DIK | `standard` | weighted projective chart |
| 3-DIK | `standard` | Jacobian-family chart |

The EFD catalogue has 199 addition, 161 doubling, 44 tripling, 19
differential-addition, 23 ladder and 25 scaling programs. Counts are taken from
the exact straight-line program selected by the instrument. A chart without an
ordinary addition/doubling formula remains a successful quotient-chart
round-trip and retains its best differential/ladder count; it is not mislabeled
as unsupported arithmetic.

The Rosetta page also names Chudnovsky Jacobian and co-Z storage. They are
covered as concepts in the source inventory, but the maintained prime-field EFD
snapshot does not expose them as separate chart IDs. No fabricated benchmark
row is added. Batched affine is an execution strategy rather than a point
chart, and complete RCB addition is a formula within the projective charts.

## Model applicability

No single generic NIST curve is guaranteed to have every special model over the
base field:

| model family | base-field prerequisite from the supplied Rosetta PDF | treatment |
| --- | --- | --- |
| short Weierstrass | characteristic not 2 or 3 | primary same-curve lane (`a=-3`) |
| Montgomery / (twisted) Edwards | suitable rational 4-torsion/Montgomery class | separate S59 model fixture unless an explicit same-curve map is supplied |
| Jacobi quartic | rational 2-torsion | separate S59 model fixture |
| Jacobi intersection | full rational 2-torsion | separate S59 model fixture |
| Huff | `Z/2 x Z/4` | classified; absent from maintained EFD snapshot |
| Hessian / twisted Hessian | rational order-3 point and stated cube condition | separate S59 model fixture |
| binary models | characteristic two | out of scope for prime-field S59 |

Consequently, cross-model wall times are engineering rows, not same-target IC
rankings. The decisive fixed-curve comparison stays within the first table.

## Higher-genus material

| supplied family | group used by a DLP | S59 status |
| --- | --- | --- |
| smooth plane quartic / C3,4 / Picard / Ciani / Riemann / Klein / Fermat | `Jac(C)(F_p)`, not `C(F_p)` | no explicit source-curve subgroup map and recovery path in the attachments; transfer benchmark not admitted |
| hyperelliptic curve, Mumford/Cantor/Harley/Kummer representations | `Jac(C)(F_p)` | arithmetic representations are valid within that Jacobian, but not coordinates of the elliptic source group |
| Smith `(2,2,2)` isogeny, GHS/Weil descent, pairing transfer | target Jacobian or finite-field multiplicative subgroup | only applicable after construction, map evaluation, subgroup preservation and recovery are certified |

These rows remain useful follow-up surfaces. They are not silently mixed into
the fixed-curve coordinate table.
