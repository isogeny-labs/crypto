# Actual C7 elliptic-cover Jacobian basis

This is a written global argument, with explicit standard Jacobian/Rosati
dependencies. It is unsigned, has no compiled generic Mathlib proof, and
must not be inferred merely from the finite rational-point test below.
[pcc:unverified]

Use the actual smooth C7/F7 and E7:y²=x³+2x+6 qualified in source
3916f563ce5717f50ee1716ac10a9c979e5f656c, result
62daac1e0c57cf04009fb57952dd6975339facd5 (merged PR5026).
Let T=diag(1,1,-1) and R=[[1,5,2],[0,5,0],[2,1,1]] act projectively.
Let f0:C7→E7 be the actual degree2 deck quotient, f1=f0∘R, and
f3:C7→Q→E7 the actual degree3 quotient by <R> with its qualified
elliptic model conversion. All are separable F7 morphisms. [pcc:unverified]

For J=Jac(C7), put φ0=f0*, φ1=f1*, φ3=f3*:E7→J, where * here denotes
pullback on Pic0. Define B=(φ0,φ1,φ3):E7³→J and F=B†:J→E7³ using
the canonical principal polarizations. Thus F has the three actual
pushforwards as its components. Standard functoriality gives
fi_* fi*=[deg fi], and a quotient by a finite automorphism group has
fi* fi_*=the group trace. These are dependencies about actual Jacobians,
not a definition of a synthetic group on curve points. [pcc:unverified]

In normal coordinates [X:Y:Z]=[x+y+z:5x:y-z], T swaps y,z and
C7:x4+3x²yz+2xy³+2xz³+2y²z²=0. Set u=yz/x², v=y³/x³ and
H(u)=1+3u+2u². The quotient relation is 2v²+H(u)v+2u³=0,
and w=4v+H(u). Swapping y,z sends v to u³/v, hence w to -w.
The induced involution on the elliptic Q is hyperelliptic; on Pic0 it
is [-1] even though its action on the selected elliptic curve origin
may include a translation. Therefore T*φ3=-φ3. Since f0 is T-invariant,
f0_*φ3=-f0_*φ3, and Hom(E7,E7) is torsion free, so f0_*φ3=0.
Since f3 is R-invariant, f1_*φ3=f0_* R_*φ3=0 as well. Duality gives
the opposite cross terms zero. [pcc:unverified]

The degree3 trace now gives (1+R*+(R*)²)φ0=φ3 f3_*φ0=0.
Let h=φ0† R*φ0, so h+h†=[-2]. The positive semidefinite Gram matrix
of φ0 and R*φ0 has diagonal2; Rosati Cauchy-Schwarz gives N(h)≤4.
The independently qualified trace of E7 is -3 and its Frobenius
discriminant is -19, fundamental. Standard ordinary elliptic endomorphism
theory gives End(E7)=Z[π]=O_{-19}. Its Rosati involution is conjugation.
Writing ω=π+2, N(a+bω)=a²+ab+5b²=(a+b/2)²+19b²/4. If b≠0 this
integer norm is at least5, so N(h)≤4 forces h an integer. The trace
identity forces h=-1. This avoids any rank3 Hermitian classification
or choice of the previously abstract CM19 α. [pcc:unverified]

Consequently, as GLOBAL endomorphisms of E7³,

    F B = G = [[2,-1,0],[-1,2,0],[0,0,3]], det(G)=9.

The positive definite G is an isogeny; hence B and F are isogenies.
For a matrix over the ordinary quadratic endomorphism order, the degree
is N(det G)=81. Dual isogenies have equal degree, so deg(B)=deg(F)=9.
They are separable (7∤9), and injective on 2-power and 11-primary torsion.
The already qualified #J(F7)=11³=#E7(F7)³ implies F is a bijection on
rational points. This supplies an actual correspondence-based isogeny,
independently of the abstract principal-polarization classification.
[pcc:unverified]

For every P on E7, B(P,0,0)=f0*([P]-[O]) and its F-image is
(2P,-P,0). The finite implementation will construct the actual geometric
fibres fi^-1(P) with scheme multiplicities and evaluate
Σ f_k(Q) over fi^-1(P) minus Σ f_k(Q) over fi^-1(O). It does not assign
formal divisor classes by scalar labels. Tests on E7(F7) corroborate this
cohort and cannot replace the preceding global argument. [pcc:unverified]

The G-pairing on the cover basis is not the identity polarization. The
kernel/gluing at3 and its integral relation to CM19 H remain to be
constructed; transporting an arbitrary E³ matrix through B does not
automatically give an integral J endomorphism. No actual nontrivial
degree64 S/J4 map, independent proof replay, prime-field descent,
generic Lean theorem or ECDLP computational advantage is claimed.
In particular the displayed original-subgroup transfer remains directly
solvable by the same elliptic DLP; a promising new cost mechanism is
still required before an advantage benchmark. [pcc:unverified]
