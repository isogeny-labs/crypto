# Branch factor forced by a norm identity

This is a conditional polynomial proof, not a target experiment or cost comparison. The [PDF](BRANCH_NORM_FACTOR.pdf), [diagram](branch-norm-factor.svg), [gate CSV](branch-norm-factor.csv), [Lean source](Common22BranchFactorization.lean), and [compiler log](branch-factorization-lean.log) preserve the exact checked boundary.

Let `f,u1,u2,n,support` be polynomials over a field, and let `a` be rational. If `f(a)=0`, the polynomial norm identity `n²-u1·u2·support=b²f` holds, and `u1(a)u2(a)support(a)=0`, then the checked theorem proves `(X-a) ∣ n`. Its proof specializes the already checked branch-overlap theorem at ordinate zero and applies the polynomial factor theorem. The repository's [source-style norm theorem](COORDINATE_RING_BRIDGE.md) is a separate checked input with concrete coefficient hypotheses; this packet does not assert those hypotheses for every runtime path.

The result supplies a candidate factorization for the local zero-order argument. It does not yet identify `n` with the compiled numerator in the actual projective stalk, establish the required unit `B` in every branch, count nonbranch and infinity orders, or prove a global principal divisor and `Pic⁰` class equation. Frozen 1,260-target rows and matched-rho ratios are unchanged.

Direct Lean 4.34.1/Mathlib `d13f23b723b8a846827a245b89c10fc7d3f11612` compilation exited zero. The theorem's axiom audit lists only `propext`, `Classical.choice`, and `Quot.sound`, without `sorryAx`. Source SHA-256: `7e583e29d15de7b4eba06d1cbb93a8eb52812c57039248791389d723317b8051`.
