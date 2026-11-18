# H116 scalar division-polynomial degree experiment

Frozen experiment `H116-SCALAR-DIVPOLY-DEGREE-001` passed all eight cells.
The independent Python implementation reproduced every correctness field and
accepted 8/8 cells. [pcc:unverified]

## Receipts

```text
8b8c4e8ad103d86be63018f4e6e4ee869f6afa8450df7a6af5b22b4fc3bb90f2  SCALAR_DIVPOLY_DEGREE_PROTOCOL.json
225da661ef80694e2652f94dd62993c73935ac059a147b22fb4b71c8af38de1e  h116_scalar_divpoly_degree.rs
4e802fb2bd6e6889869f5b899b729f99d0cc23778b4b4684e6e34704e5c2e6bb  h116_scalar_divpoly_degree_verify.py
be46e7359388b42d981d2c92f31a0111028e513ce01cb28097a3cdf022a316e5  SCALAR_DIVPOLY_DEGREE_RESULT.tsv
f7efde7c060962d487492ea3d49253b03036856287052369808057d2bb7716a5  SCALAR_DIVPOLY_DEGREE_VERIFY.json
```

## Exact-degree and pointwise results

For the three base curves, the division-polynomial rational x-maps for
`[N]` had the locked exact degrees and leading coefficients:

| p | N | degrees of (Phi_N, Psi_N^2) | leading coefficients |
| ---: | ---: | ---: | ---: |
| 5 | 7 | (49,48) | (1,4) |
| 7 | 5 | (25,24) | (1,4) |
| 11 | 14 | (196,195) | (1,9) |

Across all eight extension fields, the exhaustive census covered 5,058 affine
points: 4,884 regular images and 174 points sent to infinity. There were zero
rational-map mismatches, zero denominator/pole mismatches, and zero observed
numerator/denominator cancellations. Adding one to the numerator constant
rejected all 4,884 regular evaluations.

The construction operation counts for the primary base-field polynomials were
`(add,mul,inv)=(3247,3227,4)`, `(904,893,2)`, and
`(54418,54222,10)` for `(p,N)=(5,7),(7,5),(11,14)`.
Extension-field and group-operation counts are retained per cell in the TSV;
they measure this exhaustive certificate, not an ECDLP solver.

## Characteristic control

The excluded scalar `[p]` exhibited the locked failure of the characteristic-
zero denominator degree formula in every family:

| p | expected p^2-1 | observed degree of Psi_p^2 |
| ---: | ---: | ---: |
| 5 | 24 | 20 |
| 7 | 48 | 42 |
| 11 | 120 | 110 |

In every case the coefficient at the expected degree was zero. This is the
finite counterpart of the explicit Lean premise `(n:F) != 0` in
`scalarMultiplicationXMap_exact_degrees`.

The experiment supplies coordinate-level evidence for the scalar-degree
certificate. It does not prove that Mathlib's division-polynomial quotient
implements its affine point scalar multiplication, identify coordinate degree
with scheme/function-field degree, construct the pullback tower for the dual
factor pair, prove duality, solve ECDLP, or establish subexponential runtime.
[pcc:unverified]
