# Independent C0 Cantor correspondence review

Fresh isolated replay passed seven modules, with 25 permitted-axiom readbacks covering twelve producer declarations, six producer controls and seven independent controls. The native-source correspondence holds at the stated polynomial-certificate level. The separate written divisor-class argument is mathematically consistent under its explicit smooth-curve assumptions; it is not a mechanized geometric result.

## Native composition and reduction

The bound three_leads.rs excerpt at 1015–1024 gives norm=-B and s1=C. Thus C=0 bypasses the optimized branch and uses lines 1045–1060. Coprime supports give normalized d1=1 and then d=1, conditional on the native egcd specification. Substituting v_i=ell+a*u_i and f=ell²+U*Q into the literal fallback numerator yields:

c1*e1*u1*v2 + c1*e2*u2*v1 + c2*(v1*v2+f)
= ell + U*(c1*a*(e1+e2)+c2*(a²+Q)).

The proof uses both Bezout identities and allows arbitrary c2; it does not silently choose c2=0. Our added control instantiates c2=1 with nontrivial c1 and verifies the full theorem. Degree ell≤2<4=degree U identifies the initial polynomial remainder. The exact factorization f-ell²=U*(X-x), monicity and degree1 certify the quotient, and the explicit negative-remainder certificate gives -ell(x). This matches one native reduce iteration (990–1003), conditional on actual polynomial arithmetic/trim/egcd correctness. The new proofs do not replay pdiv, prem or native allocation.

## Independent written geometric check

Work on the smooth projective model of y²=f over an algebraic closure in characteristic not2, with squarefree monic quintic f and the stated infinity orders. For each ideal (u_i,y-v_i), the affine quotient is K[X]/(u_i) because u_i divides f-v_i². Its length is2. At nonramified points, y+v_i is a unit on the selected branch and the length counts the multiplicity of the u_i root. At ramified roots, squarefreeness makes the root of f-v_i² simple, so a repeated ramified root in u_i is excluded. Coprimality prevents D1 and D2 overlapping. Since ell-v_i is a multiple of u_i, the same divisors lie in y-ell.

For g=y-ell, at a root alpha with ell(alpha)≠0, y+ell is a unit at the selected point. The identity g*(y+ell)=f-ell² therefore gives the exact root multiplicity. If ell(alpha)=0, f'(alpha)≠0 and f-ell² has a simple root. The local parameter y has order1; x-alpha has order2; ell(x) has order at least2 (or is zero), so g has order1. Repeated roots away from ramification and a residual root coinciding with an input root are counted with multiplicity, not discarded.

At infinity, y has pole order5 while ell has pole order at most4, so cancellation is impossible. Thus div(y-ell)=D1+D2+R-5O. Also div(x_coordinate-x)=R+R'-2O; if R is ramified, R=R' and the zero has order2. In the actual quotient Div0/Prin these two principal-divisor relations give [D1-2O]+[D2-2O]=[R'-O], with R'=(x,-ell(x)). The argument neither assumes the desired group law nor defines an artificial quotient from it. Original-field descent remains subject to the appropriate actual Picard/divisor carrier, as the producer states.

This written proof assumes the usual smooth-projective model, infinity valuation and divisor/Picard constructions. The Lean packet does not construct them. Its characteristic2 polynomial control must not be interpreted as geometry of y²=f in characteristic2. No claim of a concrete mechanized Picard carrier is supported or made.

## Controls, provenance and failures

Seven independent controls cover nonzero second Bezout coefficient, its explicit nonzeroness, a wrong linear quotient, a ramification factorization and simple coefficient, distinct ordinate signs, and repeated residual support. These are kernel-checked algebraic boundary examples, not mechanizations of local valuations. The final 25 readbacks contain only propext, Classical.choice and Quot.sound.

Source modules were extracted from published commit ceecc0b38f9a3e14370781b2275afc6e59b27894 and compared byte-for-byte to the producer originals. The first extraction requested .lean instead of the archive's .lean.txt suffix, leaving empty modules that trivially compiled; extraction-attempt1 retains that invalid attempt. No declaration evidence is attributed to it. Corrected extraction requires nonempty files and byte equality, and the final readback audit requires exactly25 named entries. An independent control initially failed on an ambiguous support name; reviewer-controls-attempt1 retains the failure. Qualified names repaired only reviewer source.

The reviewer read producer PLAN/REPORT/status and native source before proof bodies; this is source-first review, not blind derivation. Exact replay inputs were hash-checked before/after. Retained compiled dependencies and Lean itself remain trust boundaries; there was no independent kernel-binary check. No scientific execution, native polynomial replay, cloud operation or repository edit occurred.
