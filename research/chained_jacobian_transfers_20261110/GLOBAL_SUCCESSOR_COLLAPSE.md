# Global elliptic shortcut for the fixed c=4 successor

Standing: this is an unsigned producer mathematical argument, supported by
kernel-checked cleared algebra and finite development controls. The full
projective/Picard construction is not formalized in Lean. [pcc:unverified]

Let K be any field of characteristic different from2,3,5. All fractions below
are interpreted in K. Put

    C_s: y²=(x²−1)(x²+x+4)(x²−x+4), W=(1,0);
    C_t: w²=(z⁶−102z⁴+393z²−4)/500, W'=(2,0).

These are the same integral models used in the finite tests, reduced at good
characteristics. Use the ordered source factors G1=x²−1,G2=x²+x+4,
G3=x²−x+4, determinant−10. Their normalized brackets are

    H1=(z²−4)/5,
    H2=−(z²−10z+1)/10,
    H3=−(z²+10z+1)/10.

The cited Richelot correspondence is
G1(x)H1(z)+G2(x)H2(z)=0, yw=G1(x)H1(z)(x−z). Let R be its induced
Jacobian map, with the image of W subtracted. The source quotient is
f(x,y)=(x²,y) on E_s:v²=u³+6u²+9u−16. The target quotient is
n:J(C_t)→E_t, with X=z²/500,V=w/500, where

    E_t: V²=X³−(102/500)X²+(393/500²)X−4/500³.

After U=u−1, E_s is v²=U³+9U²+24U. Its standard degree-two quotient
by(U,v)=(0,0) is

    phi(U,v)=(U+9+24/U, v(1−24/U²)),
    E': V²=X³−18X²−15X.

Complete this map at its kernel and infinity. The codomain isomorphism is

    tau(X,V)=(X/100+1/125, −V/1000).

The claim is the identity of elliptic-valued homomorphisms

    n R f* = tau phi [2].

This concerns the first elliptic projection of this fixed successor. It does
not identify the later Jacobian-valued two-hop map with an elliptic map.
[pcc:unverified]

## The base image is essential

The correspondence image of W is represented by(H2,0). Its two branch
points project to the two nonzero2-torsion points of E_t other than
T=(1/125,0). Their sum is T. Thus evaluating R on[P−W] requires subtracting
T after projecting the correspondence fiber. Since T has order2, this is
translation by T. Omitting it gives a different function. This computation
can be made over an algebraic closure and descends to K. [pcc:unverified]

## Projected fiber on a dense open set

For P=(x,y), write

    a=x²−1, b=x²+x+4, c=x²−x+4, A=x²−x−6,
    D=9x²+x−4,
    M=2x³+19x²+7x+74, N=8x³+x²−47x−4.

Then y²=abc. The quadratic correspondence fiber satisfies
A z²+10b z−D=0. On the open set where the denominators are nonzero,
write S=−10b/A and Q=D/A, so z²=Sz+Q. Reducing w modulo this quadratic
shows that the two projected elliptic points lie on the line V=mX+n with

    m=aM/(10yA), n=−aN/(5000yA).

The ordinary elliptic line-intersection formula gives their sum(X0,V0).
The following exact polynomial identities simplify its coordinates:

    5aM²+bc(98A²−100b²−2DA)+75aA²=0,
    aM−b²c=−A(a²−24), N−4M=−75b.

Consequently

    X0=1/125−3a/(20bc),
    V0=−3y(a²−24)/(200b²c²).

The identities are proved over an arbitrary commutative ring in
`lean/SuccessorCollapse.lean`; no finite-field interpolation is used.
[pcc:unverified]

Shift E_t by T. Its coefficients become A_t=−9/50,B_t=−3/2000. Addition
of its2-torsion point(0,0) sends(q,v) to(B_t/q,−B_t v/q²). Applying
this to(X0−1/125,V0), then shifting back, gives

    X=1/125+bc/(100a), V=−y(a²−24)/(1000a²).

Since bc=a²+9a+24, these are exactly tau phi(f(P)). Hence
n R([P−W])=tau phi(f(P)) on the dense open set. The cleared target and
standard2-isogeny identities are also proved in the same Lean file.
[pcc:unverified]

## Completion and whole fibers

The correspondence-induced map on Picard groups and the quotient norm are
actual group homomorphisms by Kunzweiler Propositions7–8 and Galbraith
Theorem8.3.8/Corollary8.3.10. Their restrictions along P↦[P−W] are morphisms
of smooth projective curves. Equality on the dense open set therefore gives
equality everywhere, including the branch points, poles and degree drops.
This standard geometric completion is part of this written argument; it is
not established by the ring-only Lean file. [pcc:unverified]

The fiber above the elliptic origin is equivalent to2W, so
f*(Q−O)=[sum(P in f^-1(Q)) P−2W], with multiplicities. Apply the just-derived
identity to the two fiber points and use homomorphism laws. Both have
f(P)=Q, so n R f*(Q−O)=2 tau phi(Q)=tau phi([2]Q). This argument also
handles nonsplit and ramified fibers over K. [pcc:unverified]

## Evidence and limits

- Seven cleared ring identities compile with the standard axiom gate; claim
  PCC-20261111-ec195a remains unsigned. They do not construct Jacobians.
- Native controls compare n R([P−W]) with tau phi(f(P)) on every rational
  affine curve point and both original infinities at p13,29,31,101. They
  explicitly check n(psi(W))=T. Every elliptic whole fiber also agrees with
  tau phi[2], and the second target norm is zero. Raw receipt:
  `evidence/native-global-shortcut-01.txt`. [pcc:unverified]
- Field implementation support remains p≤4096, with exhaustive enumeration
  capped at101. The algebraic argument has a wider mathematical scope than
  that implementation. No field-size or performance extrapolation follows.
- The direct shortcut must be included in any comparison of this elliptic
  endpoint. This is a functional deduplication, not a measured speed ratio,
  a universal obstruction to transfers, or a result about the final
  Jacobian-valued two-hop chain. [pcc:unverified]

Sources checked: the already-read primary Kunzweiler DCC2024 §§3.1–3.2 and
Galbraith corrected ch.8 §8.3, particularly norm/pullback degree and linearity.
The existing native elliptic group law and historical2-isogeny formula supply
the same line-intersection conventions. Retrieval gaps and exact primary PDF
digests remain in `SOURCES_20261111.md`; no novelty claim is made.
