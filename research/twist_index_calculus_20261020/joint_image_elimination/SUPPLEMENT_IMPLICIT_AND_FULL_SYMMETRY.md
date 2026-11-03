# Implicit image coordinates and the fully symmetric control

Frozen before constructing or solving these representations. This extends
the joint experiment; it does not replace the global and orbit failures.
Their original input files, binaries and limits remain intact.

## Implicit coordinates

Introduce W in Fp and retain U as the source abscissa. For phi use

    Z = -omega^2/3 * (U + 4 B W),    U^2 W = 1.

This avoids clearing a power of the image denominator. For doubling use
C=B^4 in Fp and

    Z = U/4 + 9/4 * W * (C U - B U^10 + B^2 U^7 - B^3 U^4),
    (U^12-C) W = 1.

Substitute these polynomial coordinates in the identical generic S5
resultant. Use the existing finite-domain constraints for the three
ordinary factors and source U. Those constraints require distinct allowed
ordinary factors, and each mixed image is checked disjoint from L0. No
additional saturation variable is needed in this finite-domain encoding.
The fifth variable W is an inverse, not a free relaxation. Verify every
map on every allowed source abscissa, and evaluate the composed polynomial
against the direct generic polynomial on nonzero probes.

For phi, the auxiliary equation has degree three. For doubling it has
degree thirteen. These are representation diagnostics, not predicted
solver costs or asymptotic claims. Both maps receive the same inverse
coordinate treatment, and the better existing representation remains a
control.

## Four-factor symmetry of pure L0

The pure-L0 baseline must receive the full permutation symmetry among its
four ordinary abscissae. Starting with partial elementary coordinates
(e1,e2,e3,U), substitute

    e1=S1-U,
    e2=S2-U*S1+U^2,
    e3=S3-U*S2+U^2*S1-U^3.

Reduce the resulting polynomial modulo

    U^4-S1*U^3+S2*U^2-S3*U+S4.

Require that the remainder is independent of U. This is an executable
coefficient check on each constructed input, not an assumption of
symmetry. Check evaluations against the original polynomial on distinct
four-tuples. Require the monic quartic to divide the same squarefree
allowed-base polynomial g; this enforces all four factors in L0 with no
repetitions. Roots are decoded as quartic elementary coefficients and all
16 sign choices are checked under the actual group law. A geometric
encoding is a separate extension, not silently covered by this finite-domain
control.

## First experiment and acceptance

Construct p=37, seed=0, query=2 in both implicit mixed arms and the fully
symmetric pure arm. Solve all three under the unchanged one-thread,
8 GiB, 1,200 s outer and 1,180 s cooperative limits. Compare the complete
signed physical sets and independent RREF against the same earlier oracle.
Use a fresh record for each representation. No result is complete unless
every field branch was exhausted and the exact signed set matches.
Extend correctness construction to all 297 preregistered inputs after this
regression. A population cost result still requires the existing cold,
A/A, matched-reference, rho and GAE gates. An implicit-coordinate or
symmetrization win alone is not a twist-specific end-to-end gain.
