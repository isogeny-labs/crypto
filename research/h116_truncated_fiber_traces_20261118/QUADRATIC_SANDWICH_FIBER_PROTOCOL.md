# H116 quadratic-sandwich fiber protocol

Protocol `H116-QUADRATIC-SANDWICH-FIBERS-001` is frozen before either native
implementation and before any affected fiber count is computed.

The experiment tests the finite splitting-field shadow of the newly proved
quadratic function-field sandwich.  On each of three fixed short-Weierstrass
curves it uses scalar `[2]`.  A canonical source point `P` is the first affine
point in `(x-index,y-index)` order for which `Q=[2]P` is affine, nonzero and
not fixed by negation.  The primary cell enumerates the full curve over the
locked extension, the two point fibers over `Q` and `-Q`, and their quotient
by source negation.
[pcc:unverified]

If all geometric two-torsion is rational in the locked extension, the expected
regular counts are:

```text
#[2]^-1(Q) = 4
#[2]^-1(-Q) = 4
#(union) = 8
# distinct source x-values in the union = 4
source and target generic x-lift counts = 2.
```

Thus the finite count realizes `4*2=2*4`: coordinate-fiber degree times the
source quadratic projection equals curve-map degree times the target quadratic
projection.  This is evidence for the inputs of the Lean theorem, not a
function-field or scheme proof.

Three controls are locked.

1. The base-field cell repeats the canonical construction before the full
   two-torsion splits.  Its `[2]` point fiber must have fewer than four points.
2. The branch cell targets the first nonzero two-torsion point in the primary
   extension.  Since `T=-T`, the `+T/-T` union must not have the generic
   eight-point count and the target x-lift count is one rather than two.
3. The characteristic cell counts the kernel of `[p]` in the same extension.
   Its number of distinct rational points must be strictly below the total
   degree `p^2`, exposing the inseparable-degree boundary of point counting.

The producer and verifier are separate Rust programs.  The verifier must not
include the producer or `h116_field.rs`; it implements its own field and group
arithmetic.  Every row and failed expectation is retained.  Operation counts
are diagnostics only; no timing or speedup claim is in scope.
[pcc:unverified]

