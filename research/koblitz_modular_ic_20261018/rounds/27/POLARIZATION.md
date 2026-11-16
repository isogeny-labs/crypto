# Mixed Prym carrier: a proper polarized construction

This is a written deduction from the checked free cyclic10 curve action and
the standard abelian-variety/Prym results below. It is not a Lean theorem
about schemes, not explicit theta equations, and not the old det19 quotient.
No desired CM-sign projection is asserted.

Let C have genus61, H=C/<T5> genus31, D=C/<T2> genus13 and Q=C/<T> genus7.
The checked fixed-locus calculations and cusp orbits give all these covers
etale; C=H x_Q D. Let P=Prym(C/H), dimension30, and B=Prym(D/Q), dimension6.
The characteristic2 Prym theorem gives both canonical principal polarization
morphisms Xi_P,Xi_B overF2, with restrictions of the Jacobian polarizations
2Xi_P,2Xi_B. This statement does not choose theta functions or a rational
line-bundle representative.

Pullback f*:J(D)->J(C) for f:C->D restricts to i:B->P. Its kernel is the
character group scheme of the cyclic5 torsor. Because the whole deck group
is abelian, sigma=T5 fixes that kernel. On B,sigma=-1. An element of ker(i)
therefore is killed by both2 and5, so the kernel is trivial, as a group
scheme. Thus i is a closed embedding, not merely an isogeny onto its image.
The adjoint norm n:P->B satisfies ni=[5]. Pullback of Jacobian polarizations
and the displayed factors2 give i*Xi_P=5Xi_B; cancellation is in the
torsion-free Hom group, not division on geometric points in characteristic2.
Hence n is the adjoint of i for the principal polarizations. Dualizing the
abelian quotient P/i(B) shows ker(n) is connected; call it K.

K has dimension24 and is a closed abelian subvariety overF2. Orthogonality
gives K intersect i(B)=i(B[5]). For the restricted polarization Xi_K its
kernel is exactly that intersection, of geometric type (Z/5)^12. Therefore
Xi_K has polarization type (1 repeated18,5 repeated6), degree5^12. One can
also obtain the degree by the addition isogeny i(B)xK->P with kernelB[5];
its pullback polarization is 5Xi_B external-product Xi_K. No2-primary
kernel is hidden here, since the displayed intersection has odd order.

The actual integral source correspondence
 E=(1-T5)*(5-(1+T2+T4+T6+T8))
has image K. On P it is2*(5-i n), on K it acts10, and E^2=10E. Thus10 is
invertible on11torsion. The actual nonzero cusp class constructed in ACTION01
has T=6,so sigma=-1 and sum_j T^(2j)=0 on it. Connectedness of ker(n) then
places that class in K[11]. This gives a nonzero11class in this proper
polarized carrier. It still does not determine its projection onto either
of the two residual-congruent CM sign factors.

References checked before this deduction:
- Achter--Casalaina-Martin, Putting the P back in Prym, Theorems A,5.12,6.4:
  https://www.math.colostate.edu/~achter/math/prym1224.pdf
  (arbitrary-base/characteristic2 principal Prym polarization).
- Milne, Abelian Varieties, sections11--12, especially Proposition12.1:
  https://www.jmilne.org/math/xnotes/AVs.pdf
  (duality, polarization orthogonal complement).
The norm/pullback and character-kernel interpretation use the actual
cyclic5 torsor, not the bare genus numbers. Proper curve, Picard, duality
and polarization arguments above remain written obligations, separately
from native identities and the Lean additive-group lemmas.
