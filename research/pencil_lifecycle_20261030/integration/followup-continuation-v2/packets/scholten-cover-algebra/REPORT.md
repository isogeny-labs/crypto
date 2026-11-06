# Scholten cover algebra and exceptional fibres

T-1127, 2026-10-31. Producer algebra proof packet; independent source-fidelity review pending. No scientific workload or performance claim.

## Source and checked identities

Bound source: src/cryptanalysis/three_leads.rs:1094-1247 in the current source tree. The source uses i^2=-1, A(t)=(1+i)t+i, B(t)=(1-i)t-i, R=33+56i, S=159+56i and w=-384. Its quintic is the expansion of

w F(t)=R A^6+S A^4 B^2+conj(S) A^2 B^4+conj(R) B^6.

On B!=0 let z=A/B and Y=y/B^3. Then wY^2=R z^6+S z^4+conj(S)z^2+conj(R), and the implemented cover is f(z,Y)=(z^2,Y). The second quotient is g(z,Y)=(z^-2,Y/z^3), on z!=0, and lands on the conjugate elliptic curve. Conjugation exchanges A and B when t is fixed, agreeing with these reciprocal expressions.

CoverAlgebra.lean checks twelve field identities: determinant -2i, the quadratic fibre coefficients, leading coefficient2i(1+x), its nonvanishing when2,i,1+x are nonzero, the exceptional x=-1 polynomial -4t-2, reciprocal product, sextic chart normalization, conjugation of A and B, opposite-fibre signs, reciprocal-fibre signs, and the two-torsion RHS cancellation. Hypotheses are explicit in the declarations. Four fixed control propositions check a rational chart value, the correct and incorrect ordinate signs, and the exceptional root t=-1/2. Readback lists transitive axioms separately. This is algebra over abstract fields, not compiled Rust correspondence or a formal Picard-group theorem.

## Geometric route: derived argument, not yet formalized

Assume the source formulas define the asserted smooth proper genus-two curve C and smooth elliptic curve E over the quadratic field, and use the standard extension of these nonconstant rational maps to degree-two covers. The determinant -2i is nonzero in the declared odd-characteristic domain, so z is a valid rational coordinate change.

The deck involution of f is (z,Y)->(-z,Y). The deck involution of g is (z,Y)->(-z,-Y). The latter leaves g fixed and sends f to its elliptic inverse. Therefore the two points of a generic g-fibre have opposite f-images. This gives zero cross-composition f_* g^* on degree-zero divisor classes; the analogous composition g_* f^* is zero. Extending the generic computation to all classes uses the induced morphisms of Jacobians, not a division at a pole. The diagonal compositions are multiplication by2 because the covers have degree2.

Consequently F=(f^*,g^*):E x conjugate(E)->Jac(C) has a left composition equal to[2]x[2]. Its kernel is finite, so equality of dimensions makes F an isogeny. Conjugation exchanges the two summands, giving the descent to Res(E)->Jac(C) when the stated models/maps are bound. Isogeny over F_p then preserves the Frobenius characteristic polynomial and hence rational point count: #Jac(C)(F_p)=#E(F_p^2).

For P in E(F_p^2), the source's intended tau is f^*(P-O)+conj(f^*(P-O)). The cross-composition above gives f_*tau(P)=[2]P. Thus a nonzero point of odd prime order has nonzero transfer. These conclusions remain a written geometric derivation conditional on the source/class identifications below; the twelve Lean identities alone do not certify them.

## Exceptional cases and literal source obligations

- Source point O: tau returns the identity directly. This must agree with pullback of the zero divisor class.
- x=-1: the quadratic leading term vanishes and the fibre polynomial is -4t-2. The omitted second point is infinity on the quintic. With T=(-1,0), the finite root is W=(-1/2,0). The source's degree-one divisor represents W-infinity. Binding this representative is essential; dropping the infinity contribution without the class convention is incorrect.
- Quintic infinity: using ord_infinity(t)=-2 and ord_infinity(y)=-5, z tends to i and Y tends to0; f(infinity)=T. Since S-R=126 and conj(S)-conj(R)=126, the elliptic RHS at x=-1 vanishes. The order-two property requires the smooth elliptic model.
- Norm basepoint: a degree-d Mumford representative represents sum(Q_j)-d*infinity. Pushforward is sum(f(Q_j))-d*T. The implementation subtracts T for degree1 and nothing for degree2, where2T=O. This explains the source correction; the class-level identification still needs formal binding.
- B=0: the formula has a pole and cover returns O. The projective extension must justify that branch. Its fibre is a hyperelliptic pair, linearly equivalent to2*infinity; this connects tau's Mumford representative to f^*(P-O).
- A=0 / z=0: the first map can ramify. Repeated roots must retain divisor multiplicity. Source norm pushes both roots even when equal; proving this agrees with multiplicities is separate from field normalization.
- Cantor reduction, polynomial monic normalization, conjugation, and finite-word field operations must preserve those divisor classes. Existing assertions of validity/base-field coefficients do not by themselves prove these semantic correspondences.

These obligations specify the next proof work without replacing the full1,260-target scientific evaluation. The previous metadata audit establishes supplied cofactor arithmetic only.

## Literature checked before implementation

Claus Diem, Families of elliptic curves with genus2 covers of degree2, arXiv math/0312413v3, Section2: complementary symmetric pairs and Proposition2.14 identify the product-to-Jacobian map and its finite kernel. The base assumes2 invertible. This supplies the classical context; no novelty claim is made. https://arxiv.org/pdf/math/0312413 . Both author-site paths returned404; the arXiv full text was accessible.

J.S. Milne, Abelian Varieties v2.0, I Proposition10.20 and II Theorem1.1: Tate-module characteristic polynomials and finite-field point counts. These explain the final implication once a base-field isogeny is certified. https://www.jmilne.org/math/CourseNotes/AV.pdf . No point counts or isogeny certificates were generated by consulting these sources.

## Custody and limitations

The first algebra compilation failed because a tactic was applied after all goals were solved; attempt2 corrected the proof script. All attempts remain recorded. Four controls and separate axiom readback have their own receipts. The strict launcher propagates native failures and checks input bindings; attempt2 additionally binds the actual Rust source and prospective plan. Dependency binaries retain the existing Lean trust boundary; no independent binary/kernel rechecker was run. Independent reviewer agents remain unavailable due usage limits. No cost ratio, runtime qualification, or campaign authorization is inferred.
