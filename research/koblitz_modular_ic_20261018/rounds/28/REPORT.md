# R28 observations: adjugate norms select ordinary Hecke images

This report records exact native functions and point evaluations under the
retained modularity, good-reduction, pole and Riemann–Roch bridge. It makes
no IC cost/rank claim and does not construct the full colored source lift.

The curve equation alone leaves three rational second-x candidates at each
of the four ordinary supports of M15's two coupled functions. R28 uses the
actual 132-branch Hecke norm to choose among them. For F and a weight H,
write Norm(F+epsilon H)=Norm(F)+epsilon D_H, epsilon²=0. At a simple zero
where D_1 is nonzero, x_infinity=D_x/D_1. The finite-factor specialization
and cofactor nonvanishing are formalized in Lean. Identification with the
actual modular norm and its fiber is a separate written argument. The norm
as a multiplication determinant is reviewed in [Sutherland, Definition 7.1](https://math.mit.edu/classes/18.782/2013fa/LectureNotes7.pdf).

Native Rust extends the original binary-series cyclic-121 determinant to
dual numbers, together with the ten middle branches and the large branch.
All dual inverses and planted multiplicativity controls pass. Four derivative
functions are recovered in the original degree-792/660 base RR spaces.
Every one of their 1,024 coefficients is checked, and every underlying norm
reproduces the exact original M15 A/B coefficients. The input precision is
123,904 coefficients. Python performs only assembly, hashes and metadata.

| Coupled function | First degree | Multiplicity | R28 outcome | Second degree |
|---|---:|---:|---|---:|
| y0*yinf+1 | 436 | 1 | Exact selected image | 436 |
| y0*yinf+1 | 12 | 2 | Both first adjugates zero | unresolved here |
| x0*yinf+1 | 284 | 1 | Exact selected image | 284 |
| x0*yinf+1 | 83 | 1 | Exact selected image | 83 |

Each selected image satisfies the actual elliptic cubic and matches exactly
one of its three rational roots. A complete Frobenius orbit check confirms
the stated closed degree and residue pushforward multiplicity one. The
zero denominator at degree 12 is retained as a failure of first-order
recovery, not divided away. R29 resolves its image multiset with second-order
norm coefficients and classifies the base Frobenius cancellations.

The first compiler attempt fails before mathematical execution because a
copied module doc comment is in the wrong position. Its full source and log
are retained. The corrected build and both native runs pass. Lean compiles
three named theorems and audits four declarations, including one generated
congruence theorem; zero failures/zero trusted-native. These proofs cover
finite-factor algebra, not the modular/finite-flat geometric identification.

The explicit output pairs and all four derivative A/B functions are in the
raw archive. These recover ordinary paired Hecke images. They do not by
themselves construct the scalar-C5 lifts, exceptional source fibers, complete
reverse signed divisors, desired modular-factor action or independent rows.
