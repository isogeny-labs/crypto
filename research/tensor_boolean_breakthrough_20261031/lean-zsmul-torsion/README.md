# Scalar-torsion to division-polynomial bridge

This standalone Lean project checks the parity-safe group-law implication

```text
n • P = 0  ->  ΨSq n (x(P)) = 0
```

for every field, Weierstrass curve, nonsingular affine point, and integer
`n`.  The conclusion uses Mathlib's full univariate squared division
polynomial `ΨSq`, not `preΨ`; consequently it retains the two-torsion factor
at even indices.

The proof depends on
`WeierstrassCurve.zsmul_eq_smulEval`, currently supplied by upstream Mathlib
PR [#41197](https://github.com/leanprover-community/mathlib4/pull/41197).
For reproducibility, `lakefile.toml` pins the contributor fork at exact commit
`f46c499f84960839eac3fdbc517a930f02b3b252` and the project pins Lean
`v4.32.0-rc1`.

## Verified checkpoint

On 2026-11-19, the pinned Mathlib target
`Mathlib.AlgebraicGeometry.EllipticCurve.DivisionPolynomial.ZSMul` built from
source successfully (1,969 jobs), and `TorsionDivisionPolynomial.lean`
elaborated with exit status zero. The packaged project then built successfully
in 1,972 jobs. The repository axiom audit reports one theorem, zero failures,
and zero trusted/native declarations; the transitive axioms are only
`propext`, `Classical.choice`, and `Quot.sound`. An independent
`leanchecker --fresh TorsionDivisionPolynomial` replay also exited zero. The
checked proof file had SHA-256
`dd07b3c7faf95a39f0dd05407a0348c2ddac953443cf47bf3b61fd749c61d56d`.

This closes the scalar-torsion-to-`ΨSq` obligation.  It does not prove a
subexponential ECDLP algorithm.  The upstream theorem is still in an open WIP
PR and is not present in the main project's separately pinned Mathlib commit;
the bridge must be rebased or imported after that dependency is integrated.
