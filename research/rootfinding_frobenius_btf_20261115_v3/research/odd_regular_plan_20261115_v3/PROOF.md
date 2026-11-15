# Odd-characteristic regular doubled-source chart

UNEXECUTED derivation and next experiment, 2026-11-15. No algorithm, oracle,
compiler or benchmark was executed for this note. It is a conditional norm
chart, not a complete original S3 solver or a novelty claim. [pcc:unverified]

Let K be a finite field of odd characteristic, including characteristic three.
Let C(X)=X^3+c2*X^2+c1*X+c0 be squarefree, so E:Y^2=C(X) is nonsingular.
Fix a rational target (z,yT), h in K, and a rational point (a,ya) with a!=z.
Put G=hX+k, P=X^3+rX^2+sX+t and

    t=-z^3-r*z^2-s*z-(h*z+k)*yT.

Then P(z)+G(z)yT=0 and the literal norm N=P^2-CG^2 is divisible by X-z.
Its quotient H is monic of degree five. We restrict to the REGULAR branch
G(a)!=0 and to the selected rational lift P(a)=-G(a)ya.

## Necessary and sufficient regular double-root equations

Since a!=z, (X-a)^2 divides H iff N(a)=N'(a)=0. The selected lift gives
N(a)=0. Literal differentiation gives

    N'=2P*P'-C'*G^2-2C*G*h.

If ya!=0, substitute P(a)=-G(a)ya and C(a)=ya^2, and divide by the nonzero
2*G(a)*ya. The exact derivative condition is

    P'(a)=-C'(a)*G(a)/(2ya)-h*ya.

Writing d=a-z, the point and derivative equations in r,s are

    (a^2-z^2)*r+(a-z)*s = v0+v1*k,
    2a*r+s = w0+w1*k,

where

    v0=-h*a*ya+z^3-a^3+h*z*yT,  v1=yT-ya,
    w0=-C'(a)*h*a/(2ya)-h*ya-3a^2,
    w1=-C'(a)/(2ya).

Their determinant is -(a-z)^2, nonzero in every allowed characteristic.
For j=0,1 set rj=(d*wj-vj)/d^2 and sj=wj-2a*rj. Thus

    r=r0+r1*k,  s=s0+s1*k,
    t=t0+t1*k,
    t0=-z^3-r0*z^2-s0*z-h*z*yT,
    t1=-r1*z^2-s1*z-yT.

Conversely these identities imply the target, selected point and derivative
conditions, hence exact division H=(X-a)^2 Q for a monic cubic Q. All
denominators are fixed nonzero; no division by three or by a occurs. Therefore
characteristic three and a=0 with ya!=0 are included without a new formula.

If ya=0, squarefreeness implies C'(a)!=0. At the selected norm root P(a)=0,

    N'(a)=-C'(a)*G(a)^2 !=0.

Consequently the regular doubled-root branch at a ramified rational point is
EMPTY. This conclusion does not cover G(a)=0: that common-factor branch must
be recovered independently or explicitly remain UNKNOWN. Singular C, a=z,
other h/a/ordinate choices and original nonrational/auxiliary branches are not
discarded by this chart.

## Quadratic field structure, with an important binary contrast

Write P=P0+kP1, G=hX+k and D=(X-z)(X-a)^2. Then

    N=N0+k*N1+k^2*N2,
    N0=P0^2-C*(hX)^2,
    N1=2P0*P1-2C*hX,
    N2=P1^2-C.

The target/point/derivative identities hold over K[k], so each Ni is exactly
divisible by D as a formal polynomial identity, not merely by checking finitely
many field values. N0/D is monic cubic, N1/D has degree<=2 and N2/D degree<=1.
Write Q=X^3-S(k)X^2+T(k)X-U(k). Then T,U are scalar quadratics in the SAME k.
Crucially the cubic sum is not constant:

    S(k)=h^2-z-2a-2r0-2r1*k.

If r1!=0, S uniquely determines k; substitution gives T and U as quadratics
in S, just as the project's prior odd-prime anchored-parabola charts. This
overlap is explicit; the present coincident-anchor/Hermite guard does not by
itself establish new coverage, a faster algorithm or a new mathematical idea.
If r1=0, the constant-S fiber must be retained rather than divided away.

Unlike characteristic two, k^2 is NOT additive or Fp-linear in odd
characteristic. For example (1+1)^2=4 whereas 1^2+1^2=2, with difference2
nonzero. A binary affine-rank argument therefore does not transfer.

For T=alphaT*k^2+betaT*k+gammaT and
U=alphaU*k^2+betaU*k+gammaU, put T'=T-gammaT, U'=U-gammaU and
Delta=alphaU*betaT-alphaT*betaU. Then

    alphaU*T'-alphaT*U'=Delta*k.

The square coefficients cannot both vanish. If r1!=0, alphaT=r1^2!=0.
If r1=0, P1 has degree<=1, so N2=P1^2-C has degree three with leading
coefficient -1; consequently alphaT=0 and alphaU=1 under the -U convention.
For Delta!=0, recover k by the displayed linear identity, retain one original
quadratic whose square coefficient is nonzero, and also enforce S=S(k).
The other quadratic then follows. For Delta=0 retain compatibility plus one
original nonzero-square equation and S=S(k); do not discard or divide the fiber.
For fixed compatible S,T,U that scalar quadratic has at most two K-roots,
including repeated roots. Odd-field squaring is not a bijection: k^2=c can
have zero, one or two roots. Neither Artin–Schreier halftrace nor a unique
binary square-root shortcut is valid here.

## Exact finite-domain output contract

Let Dsrc be any declared finite abscissa domain containing a. Assume z is
outside Dsrc when a source/target-disjoint relation contract is intended.
Let A_D(X)=product_{u in Dsrc}(X-u), a squarefree split polynomial. All roots
of the cubic Q lie in Dsrc, with any multiplicities, iff Q divides A_D^3,
equivalently (A_D mod Q)^3=0 in K[X]/Q. The proof uses the primary factors
of Q: divisibility by a power of a squarefree split domain polynomial forces
every irreducible factor to be linear with root in Dsrc; exponent three
covers every possible cubic multiplicity. This criterion works in every
characteristic, but cubing is not a linear map in general.

After support, factor Q completely, divide out every multiplicity and replay
the cubic product. Append two labeled copies of a and emit every distinct
ordered five-source tuple, together with its k,r,s,t parameter fiber. Keep
different k values even when they yield the same Q. A factor/output/state cap
returns UNKNOWN plus verified lower-bound tuples; it cannot certify emptiness.
An implicit huge Dsrc needs its own exact compact domain polynomial/remainder
oracle. For a prime field there is no nontrivial proper Fp-subspace: integer
interval support uses exact integer/lift bounds, not binary prefix spans.

For each fixed h,a,ya, constructing the six affine parameter scalars and the
constant-degree quadratic norm has bounded field-arithmetic cost. Searching
all k still costs |K| without an additional justified support-search method.
The at-most-two bound above is per SUPPLIED compatible coefficient triple,
not a bound on all supported cubics. Outer h/a/ordinate search and output can
be enormous. No global complexity or genuine F4/F5/F6 gain is inferred.

## Prior overlap and unobserved evidence

Read local prior work: research/odd_norm_cover_20261030_v1/PROOF.md already
supplies signed norms, characteristic-three controls, ramified/common-zero
exceptions and a nonsquare ghost example. The prior prime parabola route in
research/primal_parabola_20261109_v5/PROOF.md and
research/dual_ladder_20261109_v3/checker.py already uses affine k-to-sum
conversion and quadratic T/U recovery for distinct rational anchors. The
current proposal is a coincident-anchor Hermite specialization requiring
separate measured evidence, not an independently established new paradigm.

The previously source-checked review research/advanced_prior_art_v1/REPORT.md
identifies Nagao2007/112 (Riemann–Roch relation systems), Semaev2004/031
(summation polynomials), and Leem/Jacobson/Scheidler2401.16644 (function-field
norm equations). Those primary sources were not fetched anew in this note.
This is inherited prior-art attribution, not a fresh primary-source novelty
clearance. No tests, measurements or exhaustive-oracle counts are claimed.
