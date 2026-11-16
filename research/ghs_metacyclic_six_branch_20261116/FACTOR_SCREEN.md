# Six-branch metacyclic quotient: a new factor screen

16 November 2026. This is a reproducible *candidate search locus*, not a
vulnerable family. The geometry uses established cyclic and metacyclic
cover theory; the finite-field factor criterion below is a deduction
whose publication priority remains open. No source elliptic curve,
subgroup map, factor-base attack or rho comparison is asserted.
Claim standing: [pcc:unverified].

## Branch-orbit theorem

Let `ell=6n+1` be prime, with odd `n>1` and `gcd(n,3)=1`, and work in
characteristic `p` prime to `3*ell`. Suppose a degree-`ell` cyclic
cover `Y -> P1` admits an order-three lift `tau` of a base
automorphism `phi` of order three. Assume `tau` conjugates a deck
generator `sigma` to `sigma^k`, with `k mod ell` of order three.

A base fixed point cannot ramify: if its nonzero Kummer inertia
exponent were `d`, invariance would require `k*d=d mod ell`,
contrary to `k!=1`. Thus the branch set is a union of `r` full
three-point orbits. If every branch point has full inertia, tame
Riemann--Hurwitz gives

`g(Y)=(ell-1)*(3r-2)/2=3n*(3r-2)`.

Each of the two base fixed points is unbranched. On its `ell`-point
fiber, `tau` has exactly one fixed point because `k-1` is invertible
modulo `ell`. A second Riemann--Hurwitz calculation therefore gives

`g(Q)=n*(3r-2)` for `Q=Y/<tau>`.

The established one-orbit metacyclic/Fermat quotient has genus `n`.
The first additional orbit jumps to genus `4n`; there is no
four-branch intermediate case with nontrivial deck action. If a
fourth branch point is fixed and the lift instead centralizes the
deck group (`k=1`), its quotient is rational: after an order-three
coordinate change it is `y^ell=(u-c)^e` with `u=x^3`. This rules
out that proposed low-genus extension. [pcc:unverified]

## Finite-field polynomial and necessary factor test

Let `q=p^s`, `ord_ell(q)=n`, and assume the cover and lift are defined
over `F_q`. For every positive `m` not divisible by `n`, the power
map `y->y^ell` is bijective on `F_(q^m)`; every rational base point
has one rational lift, so `#Y(F_(q^m))=q^m+1`. Each twisted map
`tau^j Frob_(q^m)`, `j=1,2`, fixes `q^m+1` geometric base points.
Its action on each unbranched fiber has deck multiplier `k^j*q^m`;
this differs from one modulo `ell` because the groups of orders
three and `n` have trivial intersection. Thus each fixed base point
has one fixed lift, and the quotient fixed-orbit formula yields

`#Q(F_(q^m))=q^m+1` whenever `n` does not divide `m`.

All Frobenius power sums in these degrees vanish. Newton identities
therefore force the degree-`2g(Q)` polynomial to have the form

`P_Q(X)=R(X^n)`, with `deg R=2*(3r-2)`.

For two branch orbits, `deg R=8`. If an elliptic curve
`E/F_(q^n)` contributes a restriction-of-scalars isogeny factor,
its quadratic Weil polynomial

`Z^2-tZ+q^n`

**must divide** `R(Z)`. A factor with integer `t`, Hasse bound
`|t|<=2q^(n/2)` and Waterhouse's elliptic existence condition
would give an abstract isogeny factor by Tate. It would still need
an explicit map on a chosen subgroup and full attack-cost evidence.
The polynomial `R` has not yet been computed for the exact example
below, so this necessary factor test remains open.
[pcc:unverified]

## Exact six-branch model and order-three lift

Take `ell=31,n=5,q=4` and `F4=F2[zeta]/(zeta^2+zeta+1)`.
Define three quadratics in `F4[x]`:

`H0=x^2+x+zeta`,
`H1=x^2+zeta*x+1`,
`H2=x^2+zeta^2*x+zeta^2`.

The first is irreducible since `Tr_(F4/F2)(zeta)=1`; the other two
are its distinct `x->zeta*x` transforms. Hence the six roots are
distinct and the smooth normalization of

`Y: y^31=H0(x)*H1(x)^5*H2(x)^25`

has six full branch points, no branch at infinity (the right side
has degree 62), and genus 60. Direct substitution gives

`H_j(zeta*x)=zeta^2*H_(j-1)(x)` (indices modulo three), and
`f(zeta*x)=zeta^2*f(x)^5/H2(x)^124`.

Thus the explicit lift

`tau(x,y)=(zeta*x, zeta^2*y^5/H2(x)^4)`

acts on the deck group by the order-three exponent `5 mod 31`.
Its cube fixes `x` and is an `F4`-rational deck map; since
`mu_31(F4)={1}`, the cube is the identity. The smooth quotient has
genus 20 and `P_Q(X)=R(X^5)` with `deg R=8`. A concrete falsifier is
to compute `R` and test every quadratic factor for elliptic
admissibility. The `F4` field and all coordinates above are fixed
before that computation. [pcc:unverified]

## Prior work and experiment boundary

[Carocca--Lange--Rodriguez, Theorem 1 and Proposition 2.1](https://arxiv.org/abs/0906.4185)
establish the metacyclic representation setting and identify its
three-branch CM case. [Kallel--Sjerve](https://arxiv.org/abs/math/0310053)
study prime cyclic cover automorphisms, with complete treatment of
the three-branch setting. Neither paper, as checked here, supplies
the elliptic factor, evaluated transfer or attack cost for this
six-branch model. Absence from this search is not proof of priority.

The shared KB was queried first at revision
`9141e386254340a9bdba3036f2bb706a`, last refreshed 14 November
2026. Searches for six-branch order-three metacyclic elliptic factors
and for genus-`4n` quotient Weil restrictions found no exact record;
`source:d83cb619b3afdb7b9d999946161057c7`
(`crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-weil-restriction-jacobian-class.yaml`)
is the adjacent abstract Jacobian-route lead. The KB predates this
16 November construction. [pcc:unverified]

The shared `docs/session-logs/2026-11-16.md` remains reserved by
Conductor T-6720. This note freezes the source, hypotheses and
factor-screen question before any point-count experiment. The
six-branch factor screen should start only after that log can be
updated or the coordination constraint is resolved. [pcc:unverified]
