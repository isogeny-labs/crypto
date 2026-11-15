# Frozen validation of the n=7 genus-eight L-factorization

15 November 2026, before the first calculator call. This is a specialization check of Hess's published [Theorem 15](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf), not a new GHS family claim. Prior q4 and q16 exact equations and group-order receipts are committed in sibling directories. Magma's [EulerFactor API](https://docs.magma-maths.org/ArithmeticGeometry/HyperellipticCurves/point_structures_jacobian.html) returns the reciprocal Frobenius polynomial.

## Exact validation cases

1. q16: `K=F2[z]/(z^28+z^3+1)`, `a` and `b` as in the committed `ghs_n7_q16_ic_20261115/map.magma`; base `k=F16`, parameter `c=1`, `#E=268413186`, source trace `t=22271`. Construct `C,h := WeilDescent(E,k,1)`, assert genus 8, and independently compute `EulerFactor(Jacobian(C))`. Let `E0/k: y^2+xy=x^3+a x^2+1`, with `a` embedded from K; compute its trace `u=16+1-#E0`. Compare the full degree-16 polynomial with `(1-uT+16T^2)(1-tT^7+16^7T^14)`. Also compare evaluations at `T=1` to the committed `#J` and `#E*#E0`.
2. q4: same check on the committed exact q4 equation over `F2[z]/(z^14+z^5+1)`, `#E=16634`, `#J=33268`, genus 8, and `E0` over F4. Run only after the q16 result is preserved.

Each public request contains only published-equation-style curve coefficients, no private data. Preserve the exact submitted source, its SHA-256, response, HTTP status, and client duration in a new immutable receipt. A transport error, calculation failure, non-genus-eight result, or polynomial mismatch stops promotion of the factorization; preserve the failed receipt and diagnose before another run. The full polynomial comparison, not the group-order ratio alone, is the acceptance gate. This validation does not measure attack cost or prove a new vulnerable family.
