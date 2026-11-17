# Component regularity derivation

This is an unsigned mathematical derivation supported by exact native
coefficient checks, not a Lean proof or a complete relation-system bound.
Notation D(J)=reg(J); reg(S/J)=D(J)-1, as frozen in PROTOCOL.md.

## Auxiliary membership

Let K be the P-256 prime field, d the existing tower depth, H the common
homogenizing variable, and c the terminal abscissa. In
S=K[X_0,...,X_(d-1),H], negate the d tower quadrics so that they are monic.
Their leading monomials in lex X_0>...>X_(d-1)>H are X_i^2: each later
coordinate occurs only after X_i; the terminal coordinate is cH. Distinct
leading squares are coprime. Buchberger's product criterion gives a
Gröbner basis; the native checker additionally reduces every S-pair and
checks each coefficient reconstruction identity.

The initial ideal is (X_0^2,...,X_(d-1)^2). Its height is d. Since S is
Cohen–Macaulay and there are d generators of height d, the quadrics form
a regular sequence. The minimal Koszul resolution has one shift2i in
homological degree i for each i-subset of the d quadrics. Consequently

    Hilb(S/J,t) = (1+t)^d/(1-t),
    reg(S/J) = d,             D(J) = d+1.

H is regular: normal monomials X_0^e0...X_(d-1)^ed with each e_i in{0,1}
form a free K[H]-basis. The Artinian section H=0 has top nonzero degree d,
so its first zero degree is exactly d+1. This conclusion concerns the
membership block only. All d coordinates and H are counted.

## Dense control and exact fibre semantics

Compose each rational abscissa map using numerator u and denominator v:

    u_next = u^2-r*u*v+delta*v^2,
    v_next = v*(u-r*v),        (u,v) initially (X,1).

The terminal numerator F=u-cv is monic of degree2^d. The native fixture
check enumerates the already-defined coset, verifies distinct abscissae,
nonzero denominators, and exact equality F=product(X-alpha). Thus F has
precisely those roots, including over the algebraic closure. Delta!=0
excludes every affine pole in the tower. Affine intermediate coordinates
are uniquely recovered by the maps, giving an isomorphism with the dense
full-fibre algebra. Its one homogeneous generator has ideal regularity
2^d. This does not assert a projective isomorphism of the two embeddings.

The **actual retained FB1 abscissae** give a different dense polynomial
F_kept. Native checks verify F=F_kept*F_excluded and verify the target
quadratic character at every full-fibre root. An excluded root is not a
target Fp point, but it has target ordinates in an extension. Therefore
the full and retained target ideals agree on Fp points but need not agree
over the algebraic closure. Do not silently substitute the full fibre for
the retained base when claiming a matched algebraic reduction.

## Appending the target cubic

Add Y and C=Y^2*H-X_0^3+3*X_0*H^2-B*H^3. In (S/J)[Y], the leading
coefficient of C as a polynomial in Y is H, which is regular. Hence C
is regular, and the extended ideal is a complete intersection of degrees
(2,...,2,3). Its quotient regularity is d+2 and ideal regularity d+3.
For a dense squarefree abscissa polynomial of degree N, H remains regular;
the analogous degrees(N,3) give quotient regularity N+1 and ideal
regularity N+2. These are unsaturated generator homogenizations.

To use a first-zero Hilbert degree, take the section Y=0, **not H=0**.
At infinity the tower forces all X_i=0. With Y=0 there is no projective
point at infinity. On H=1, native checks verify that the target cubic
right-hand side is nonzero at every full-fibre root, so again there is no
point. Since the complete intersection is Cohen–Macaulay of dimension1,
Y is a parameter and is regular; its Artinian section has the indicated
Hilbert numerator and first zero degree D. The dense case is the same.

The homogeneous curve blocks contain infinity contributions; saturating
with respect to H is a separate operation. These values do not describe
the saturated affine closure or F4 maximum matrix degree.

## Why this cannot close the owner's goal

Adding the complete group-law equations, chart selectors, inverses and a
fixed target changes the ideal. Regularity of a constraint block does not
bound the regularity of that new ideal. Prove/measure the full homogeneous
ideal separately on matched nontrivial m=2–7 cells. Retained-base controls,
extension-field differences, preprocessing and all added variables remain
mandatory. The native output deliberately leaves full_relation_regularity
null and goal_achieved false; no relation row is reported by this verifier.
