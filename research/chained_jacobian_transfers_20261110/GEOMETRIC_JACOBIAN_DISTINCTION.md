# The three fixed Jacobians are geometrically nonisomorphic

Scientific standing unsigned. This extends the base-field-only distinction
in SEVEN_CHAIN_PROOF.md and SUCCESSOR_CHAIN_PROOF_V2.md without changing
those frozen proofs. It applies external CT37's ordinary endomorphism-field
obstruction to the actual certified elliptic embeddings. The geometric
classification, Picard and connected-subvariety arguments are written;
this is not a Lean construction of the varieties. [pcc:unverified]

## Exact elliptic inputs and primary-source review

The fixed short models over F101 are:

| Factor | stage0 | stage1 | stage2 | point count | trace | j-invariants |
|---|---|---|---|---|---|---|
| E | (1,46) | (26,58) | (59,27) | 109 | -7 | 12,87,47 |
| F | (44,21) | (69,50) | (23,7) | 113 | -11 | 58,92,89 |

Native34/5c1a1b uses the existing Rust prime-field elliptic engine to
re-enumerate all six point counts and check smoothness and j-invariants.
It also checks p does not divide either trace and the discriminants
-355/-283. This independently checks the numerical inputs of CT37's
external Python oracle without executing or adopting that oracle. It
does not compute an endomorphism ring or Jacobian isomorphism.
[pcc:unverified]

Sources checked are Galbraith, corrected Chapter9, Theorems9.9.1 and9.11.2
(geometric endomorphism classification and ordinary criterion), Theorem9.10.3
(Frobenius polynomial), and Section9.6/9.7 (nonzero elliptic homomorphisms
and duals), plus Chapter10 Section10.8 on simple isogeny decompositions.
The official Chapter9 PDF is
https://www.math.auckland.ac.nz/~sgal018/crypto-book/ch9.pdf,
SHA256 f9a3272af43e74723accaed2e5e1d19405779141d1ed7f516d822203a40cc495.
The external prior note is CT37_FROBENIUS_ISOTYPY.md at cryptanalysis
commit9d100545300b833e0dc745c07585533907856b00, PR1502. Its Hom
obstruction is standard theory, not a novelty claim here. KB/ecdb unavailable.
[pcc:unverified]

## No cross-factor geometric homomorphisms

Both original factors are ordinary since101 divides neither -7 nor -11.
Their geometric rational endomorphism algebras are imaginary quadratic
fields. Frobenius satisfies respectively

    pi_E² + 7pi_E + 101 = 0,
    pi_F² + 11pi_F + 101 = 0.

Both polynomials are irreducible over Q, so the included Frobenius fields
already have dimension2 and fill those algebras. Thus

    End^0(E_bar)=Q(sqrt(-355)),
    End^0(F_bar)=Q(sqrt(-283)).

These quadratic fields are different: the rational ratio355/283 has
5-adic valuation1, so is not a rational square. No assertion about maximal
orders or conductors is needed. [pcc:unverified]

A nonzero geometric homomorphism between elliptic curves is an isogeny.
Conjugation by that isogeny and its rational inverse would identify the
rational endomorphism algebras, contradicting the different fields.
Therefore Hom(E_bar,F_bar)=Hom(F_bar,E_bar)=0. The selected nonzero
alpha/beta isogenies identify the rational endomorphism algebras within
each family. Consequently Hom(E_i_bar,F_j_bar)=0 for every i,j in0,1,2,
and hence over every finite extension of F101. This uses the ordinary
classification, not a finite extension trace sample. [pcc:unverified]

## The actual elliptic curves survive as embeddings

Write A_i=Jac(T_i), with T0=[31,75,74,54,71,55,1],
T1=[95,26,19,27,94,91,1] and T2=[95,93,43,39,92,17,1]. Let
P_(i,E):E_i→Jac(C_i) be the old split elliptic pullback and R_i the
actual normalized Richelot map. The earlier global constructions and
their geometric2-torsion certificates show that P_(i,E) is an embedding,
and similarly for F_i. Its kernel is contained in elliptic2-torsion by
the matching norm identity[2], and all three nonzero geometric2-points
have distinct nonzero images. Characteristic101 excludes hidden
nonreduced2-primary kernels. [pcc:unverified]

The R0/R1 and R1/R2 intersection controls check that no nonzero embedded
elliptic2-point lies in the corresponding Richelot kernel. Since
R_iHat R_i=[2], Ker R_i is contained in Jac(C_i)[2]. If
R_i P_(i,E)(P)=0, then P lies in E_i[2], and the intersection controls
force P=0. The finite kernel is etale, so the restriction is an embedding
of the actual E_i, not just an isogenous elliptic image. Put

    K_(i,E)=R_i P_(i,E),   K_(i,F)=R_i P_(i,F).

All six are geometric elliptic embeddings. These facts depend explicitly
on the earlier actual Picard/norm constructions and intersection
certificates in SEVEN_JACOBIAN_PROOF.md, SEVEN_CHAIN_PROOF.md and
SUCCESSOR_CHAIN_PROOF_V2.md; equal rational group orders alone would
not supply them. [pcc:unverified]

## Uniqueness of each geometric elliptic subvariety

Let H_(i,F):A_i→F_i be the complementary norm. The old global
norm/pullback identities give H_(i,F)K_(i,F)=[4], so H_(i,F) is
surjective. The identity component B_i of its kernel has dimension1.
The off-diagonal identity H_(i,F)K_(i,E)=0 places the embedded E_i
inside B_i, hence B_i equals its image. [pcc:unverified]

Every geometric elliptic subvariety in A_i isogenous to E has zero
composite with H_(i,F), by the cross-Hom obstruction. It lies in B_i
and therefore equals B_i. The corresponding F-type image is unique
by the symmetric argument. This is uniqueness of the connected
subvariety, not of its parametrizing isogenies or integral projectors.
[pcc:unverified]

Suppose there were an unpolarized geometric abelian-variety isomorphism
u:A_i→A_j, for distinct i,j. The image u(K_(i,E)(E_i)) lies in the
kernel of H_(j,F), because any composite E_i→F_j is zero. It is the
unique E-type connected elliptic subvariety of A_j, namely K_(j,E)(E_j).
Restricting u and using the actual embeddings gives E_i≅E_j. Their
j-invariants12,87,47 are pairwise different, a contradiction. Thus A0,
A1 and A2 are pairwise nonisomorphic over the algebraic closure,
including as unpolarized abelian varieties. In particular, no finite
extension makes any pair isomorphic. [pcc:unverified]

## Retained negative control and scope

The ordinary-End-field hypothesis is essential to this extension of the
older argument. The existing even-degree twist control cd2506 has orders
109 and95, opposite traces, the same quadratic endomorphism field and
geometrically isomorphic elliptic curves. Distinct rational Frobenius
polynomials therefore cannot by themselves establish geometric
nonisomorphism. That counterexample and the earlier conservative
base-field-only statements remain unchanged. [pcc:unverified]

The existing maps retain degrees49 for each hop and2401 for the two-hop
composite, with adjoint norms7 and49. Their explicit prime-field
evaluators and compressed outer norm/pullback formula evaluate transfers
between these genuinely different geometric surfaces. No solver or cost
conclusion is inferred from geometric distinction. The new native input
check and written classification/embedding argument have unsigned
standing; independent whole-isogeny evaluation and full geometric Lean
formalization remain open. [pcc:unverified]
