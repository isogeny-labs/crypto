# Exact good-function section basis, frozen gate

Correctness/algebra gate only. No general-class map, isogeny alignment, timing
or full10xclaim is registered. Existing failed cold result remains unchanged.
Source: Tian4.2.3 https://arxiv.org/html/2007.03172#S4.SS2.SSS3 . Instead of
sampled evaluations on the diagonal, compute its restriction coefficientwise.

Use the existing Q1/F127 coefficient list from coldbuild13. Candidate anchors
are the already decoded points(18,52),(28,86),(43,125),(101,43), in that order.
Select first whose tangent is y=ax+b, with tangent residual of degree2
and not containing O; log preceding rejections. All tested anchors must lie
on Q1. No field scan, point resampling, scalar labels or expected images.

Let K be the infinity-line canonical divisor. Tangent at O cuts2O+R, with R
degree2 disjoint from O and affine under the chart conditions. Cubic numerators
N vanishing on R give the8-dimensional space V=L(2O+2K), represented by N/l.
Compute that space as the kernel of the2coefficient residual-line restriction
conditions on the10plane cubics. Required rank2/dimension8.

Multiply numerator pairs and reduce modulo quartic F by its monic x^4 term.
The36symmetric products must have rank18, the Riemann-Roch dimension of2D
for D=2O+2K, degD10. For120symmetric cubic tensors in V, restrict(P1,P2,P3)
to(P,P,Q), including all6permutations with multiplicities (2and6are units).
Record the220x120exact coefficient matrix in the22normal degree<=6monomials
on P and10degree<=3monomials on Q. Require rank112/nullity8, all original
coefficient residuals zero, and a corrupted section rejected. Emit8basis vectors.

Symmetry and odd characteristic make vanishing on a pair diagonal at least
order2. This holds for every pair by symmetry. Tian's determinant omega has
divisor D0+Delta-sum(pr_i*K), hence h/omega^2 is a second-order theta section
under j(P1,P2,P3)=sumPi-O-delta, with2delta=K. Dominance of j and theta
Riemann-Roch bound h0(2Theta)=8 turn the8independent tensor functions into a
complete basis; this is an exact coefficient identity, not a sample-rank claim.

Remaining obligations: specify rational delta/group centering, projective
alignment with source quotient coordinates, nonzero/diagonal/pole chart atlas,
finite algebra solving (generic ordered-triple Kummer fiber degree12), sign
recovery, general-class source/destination verification, then full cold cost.
No runtime speedup follows from this basis gate. Genus2/4 mechanisms separate.

Commit/fingerprint/checkpoint before one native worker run. Retain stdout,
stderr, exit including failure. No rerun without a committed amendment.
No Python scientific code, no measurement benchmark record for this untimed gate.
