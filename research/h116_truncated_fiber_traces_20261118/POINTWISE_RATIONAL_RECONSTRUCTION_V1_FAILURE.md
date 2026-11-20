# H116 pointwise rational reconstruction v1 failure

The first and only execution of frozen protocol
`H116-POINTWISE-RATIONAL-RECONSTRUCTION-001` failed.  The sealed independent
verifier reproduced every exact producer field with zero mismatches and also
rejected the result.  Both outcomes are retained. [pcc:unverified]

```text
1b837dfb420183e2ba0702060494687cdf78652f5fd4073d5a64c8af26d277e3  POINTWISE_RATIONAL_RECONSTRUCTION_PROTOCOL.json
d7ff21da47cd866f21fdb21422d0dbf48cd661e65e5963c32d6a2cc114faead9  POINTWISE_RATIONAL_RECONSTRUCTION_RESULT.tsv
3f349b7add0c7d5f5f2f95e57497d5895010ad97b2dab0fba391e8463d8c4870  POINTWISE_RATIONAL_RECONSTRUCTION_VERIFY.json
```

| Cell | Predicted N | Observed pole x | Pole multiplicity | Two windows equal | Full-sample mismatches |
|---|---:|---:|---:|---:|---:|
| fp5-a2-b1-d2 | 7 | 0 | 0 | no | 9 |
| fp5-a2-b1-d3 | 7 | 0 | 0 | no | 49 |
| fp5-a2-b1-d4 | 7 | 0 | 0 | no | 289 |
| fp7-a1-b1-d2 | 5 | 0 | 0 | no | 20 |
| fp7-a1-b1-d3 | 5 | 0 | 0 | no | 185 |
| fp7-a1-b1-d4 | 5 | 2 | 4 | yes | 0 |
| fp11-a1-b1-d2 | 14 | 1 | 1 | no | 54 |
| fp11-a1-b1-d3 | 14 | 7 | 13 | yes | 0 |

## Falsified assumption and corrected prediction

The v1 protocol predicted that the finite poles of `V-1` were the x-values of
nonidentity `E(F_p)` points.  That is false: `E(F_p)=ker(pi-1)`, whereas the
tested map has kernel `ker(V-1)`.

On `ker(V-1)`, `V=1`; the relation `V+pi=[t]` therefore predicts that Frobenius
acts as scalar `t-1`.  If the separable kernel is cyclic of order
`N=deg(V-1)=#E(F_p)`, its points visible over `F_(p^d)` number

```text
gcd(N, (t-1)^d - 1).
```

This predicts visible kernel sizes 1,1,1 for the three `p=5` cells; 1,1,5 for
the three `p=7` cells; and 2,14 for the two `p=11` cells.  The observed pole
multiplicities plus infinity are exactly those eight values.  The two original
interpolation windows agree precisely in the two cells where the complete
kernel splits and the prescribed full denominator was therefore available.

V1 is not amended or rerun.  The follow-up must reconstruct numerator and
denominator jointly, using the raw regular-sample capacity, so it does not
assume that denominator roots lie in the chosen extension. [pcc:unverified]

