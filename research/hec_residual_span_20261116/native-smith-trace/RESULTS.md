# Native trace inverse results [pcc:unverified]

Freeze8a056b49bd3, INPUT hashes pass, sole bounded invocation exits0. Both original F61 maps retained, no sigma search.

| Map | Trace systems | Reconstructed points | Exact previous b matches |
|---|---:|---:|---:|
| 0 | 58 | 55 | 55 |
| 1 | 57 | 54 | 54 |

All115 trace systems are nonsingular, every cubic Gprime inverse and S=-u/2 identity passes. All109 b-polynomials satisfy the full congruence and match the frozen exhaustive chart record. The separate F101 control verifies four known sign-orbit roots against independently specified x/lambda values. Raw points and specialized trace coefficient polynomials are in CONTROL.txt. No missing or extra eligible points.

DERIVATION.md supplies the generic rational inverse via the quartic trace pairing: its determinant is disc(Q), giving coefficients in F61(t) without splitting the cubic. This argument is distinct from the finite implementation check; it has not been independently reviewed or formally verified. It provides an inverse on a common affine open, not a smooth projective canonical model. Exceptional places, canonical adjoints, nonhyperellipticity, complete divisor transport and cost remain open.

This stage replaces exhaustive sigma reconstruction by fixed-dimensional algebra. It makes no measured wall-time, complete-solver or10x speed claim. The full genus2+ objective remains unachieved. Git custody used;noS3polling/no cloud/no merge.
