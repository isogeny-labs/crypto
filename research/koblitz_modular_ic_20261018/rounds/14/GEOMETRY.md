# Proper polarized geometry: an intrinsic target with explicit remaining work

The requested proper F2 equations and full quotient h-saturation remain open.
G14 checks finite data attached to the four actual graph quotients; it does
not provide their missing global sections or projective ideals.

Let q:B=E^4×E'→A be the retained degree361 isogeny, with
q^dual*lambda_A*q=diag(H,[19]) and lambda_A principal. The actual q5 kernels,
H bindings, Frobenius permutations and six-chart Kummer atlas are prior inputs.
The product principal polarization of the split base cannot simply identify
its two descent actions; round09 already corrected that issue.

There is a canonical way to specify a line bundle over F2 without first
choosing a rational theta divisor. For the rigidified Poincare bundle P on
A×A^dual, set M=(id_A,lambda_A)^*P. Its polarization is2*lambda_A.
The ample bundle L=M^3 represents6*lambda_A and is very ample in arbitrary
characteristic. These are the constructions in
[Genestier–Ngo, Theorem2.2.3 and Lemma2.3.2](https://www.math.uchicago.edu/~ngo/Shimura.pdf).
This is a written geometric bridge, not a new Lean theorem about abelian schemes.

Applied to these fivefolds, ample vanishing and Riemann–Roch give h0(L)=6^5=7776.
Thus the specified complete linear system has target P^7775, Hilbert
polynomial7776*t^5 and degree5!*7776=933120. Pulling back by q gives
h0(q^*L)=361*7776=2807136 and polarization matrix6*diag(H,[19]). G14
computes these actual Hermitian matrices, checks their determinants and
matches each to its exact graph coefficients/coset/trace. The Lean arithmetic
certificate verifies these integer consequences; it assumes the written bridge.

G14 also derives the kernel Frobenius matrices from all361 actual images:

    trace -1: [[0,17],[1,17]] over F19
    trace +1: [[0,17],[1, 2]] over F19.

It checks every ordered addition pair and both linear/Cartier dual pairings,
130321 of each per graph. The F19-linear dual has action M^(-transpose),
whereas the mu19-valued characters have Frobenius2*M^(-transpose): coefficient
Frobenius squares the root of unity. The corrected Cartier action has six
orbits per graph. Their lengths and representatives are exported. The
character multiplicity7776 follows from the written eigensheaf/ample
vanishing argument for q_*O_B; a permutation of characters does not compute
the semilinear7776-dimensional blocks. The first linear-dual-only output is
retained and explicitly superseded by this correction. Lean checks both
actual Cartier matrix identities M^transpose*C=2*I. No section coefficients or block
matrices have been inferred from it.

An executable construction still needs a basis of H0(B,q^*L), the actual
Gamma linearization and invariant subspace, the Frobenius action on that
subspace, its F2 descent basis, and the coordinate multiplication map.
Its kernel must then be compared with the four dense-open circuit models
and the global atlas. Saturation of a chosen circuit closure alone need not
give this smooth proper polarized model: a compactification of an affine
embedding can have a different boundary. Identifying that boundary is part
of the requirement, not a consequence of projectivity alone.

# Full saturation criterion

For an actual homogeneous circuit ideal I and homogenizing coordinate h,
the full h-saturation is the contraction of I localized at h. Equivalently,
x lies in it exactly when some h^n*x lies in I. KoblitzRound14 proves this
using mathlib's actual Localization.Away and Ideal.map/comap, and proves a
usable equality certificate: I⊆J, every element of J has an h-power in I,
and h-regularity of R/J imply J=I:h^infinity. No equality certificate for
all four actual J has yet been produced.

Q16 and Q17 supply further certified families but no new separating model.
Their surviving boundary models, checked in Lean against all imposed
relations, do not prove that the ideals remain unsaturated. The bounded
restricted scalar searches also cannot certify full scheme saturation.
A complete computation can use the localization/elimination method of
[Barakat–Lange-Hegermann–Posur](https://arxiv.org/abs/1707.00925), but no
whole-ideal elimination or h-regularity computation was completed here.

The next discriminating geometry step is to construct the actual invariant
section space and multiplication maps, or to supply a full saturation
certificate for the existing circuit ideal and identify its boundary with
the global quotient. The former targets the retained polarization directly;
the latter must still prove the boundary and line-bundle identification.
