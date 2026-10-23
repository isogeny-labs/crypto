# Integral level77 maps: local controls and a written existence result

CM18 uses CM16's actual two degree8 n=5 trace-zero Frobenius factors,
Pminus and Pplus. Each has eight distinct nonzero roots modulo11; the
derivative is nonzero at every root. They have six common roots with
each other, explaining the previous resultant valuation and denominator704.
Squarefreeness of each individual factor is the new local fact used below.
Generated Lean checks the actual finite root/derivative assertion.

## Actual source matrices and integral controls

The actual trace-zero source lattice is the augmentation kernel in five
copies of the elliptic Tate module. In the basis e_i-e_4, the cyclic
rotation S sends e_i-e_4 to e_(i+1)-e_0 (with cyclic indices). The matrix
is S tensor [[0,-2],[1,t]], for elliptic trace t=-1 or+1. CM18 verifies
the correct degree8 polynomial annihilates it. A cyclic basis C has
exact determinant-64 for both signs and satisfies F*C=C*Companion(P).
Thus the source and companion lattices are isomorphic at11, with no
11-denominator introduced.

For the glued control N=Companion(Pminus*Pplus), multiplication by the
other polynomial is an integral inclusion from the corresponding degree8
companion module. Composing with adj(C) gives an integer16-by8 matrix J
such that N*J=J*F. Both actual fixed11 lines have nonzero images. Generated
Lean proves the integer intertwiner identity and the finite nonzero images.

The natural reverse is polynomial reduction modulo the source polynomial,
followed by C. Its composition with J is det(C)*P_other(F). Since both
P_other(1)=11, it kills the fixed11 line. This is checked exactly and in
Lean. A failed reverse therefore does not by itself refute the forward map.

A split direct-sum control and the cyclic glued control have the same
Frobenius polynomial and det(F-1)=121. Their fixed11 dimensions are2
and1 respectively. For a companion module, quotienting by F-1 is
Z/(P(1)); the two11-primary cokernels are consequently(11,11) and(121).
These are explicitly constructed integral controls, not a computation of
the actual level77 Jacobian's integral lattice or rational point group.

## Written proposition: a prime-to11 isogeny to the isotypic image

Let A/F2 be the actual n=5 trace-zero variety, and J/F2 an abelian
variety whose Frobenius polynomial contains P_A. Suppose P_A modulo11
is squarefree. Then there is an F2-homomorphism A→J whose kernel has
order prime to11. In particular it is injective on all geometric11-torsion.

First, Tate's rational isogeny theorem gives an abelian subvariety B of J
isogenous to A, with characteristic polynomial P_A. At11, Hensel lifting
of the eight distinct roots gives

    Z_11[F]/P_A(F) = product_(i=1)^8 Z_11.

Both T_11(A) and T_11(B) decompose under these integral idempotents.
Each component has rank one over Z_11, because P_A has multiplicity one.
Every such lattice is free of rank one, so the two Tate modules are
isomorphic as Frobenius modules. Tate's integral Hom theorem realizes
this isomorphism in Hom(A,B) tensor Z_11. Approximate its finitely many
coefficients by integers modulo11. The resulting actual homomorphism
A→B has invertible Tate map modulo11, hence is an isogeny of degree
prime to11. Compose with B→J.

This is our deduction from the integral Tate theorem, with the primary
proof source in SOURCES.md. It does not require a polynomial projector
splitting both signs integrally. For the smooth compact level77 modular
normalization, the known CM factor supplies P_A under the frozen written
modular/Jacobian bridge. Subject to that bridge, the proposition supplies
an existence result for a subgroup-preserving integral map.

The proof is nonconstructive here. No actual integral correspondence,
Jacobian divisor evaluator, saturated modular Hom lattice or map kernel
has been computed on that curve. The finite controls do not identify the
unknown actual Tate lattice. Full Lean formalization would require the
abelian/Tate-module definitions, the integral Hom theorem and its link to
this modular factor. We have not replaced those with axioms encoding the
claim. Relevant actual linear-algebra and torsion certificates are checked
in Lean; the global existence argument remains written.

The next construction target is an explicit integral correspondence with
an11-adically primitive image, followed by evaluation on the public
order11 fixtures. Dividing a projector by704 remains invalid at11.
