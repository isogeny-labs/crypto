# A dihedral quotient family with an elliptic Weil-restriction polynomial

16 November 2026. This is a general arithmetic eligibility theorem for an
established type of dihedral cover. It is **not** a demonstrated GHS
vulnerability or a claim that the geometric family is new. The derivation
below is a paper proof, not a Lean formalization. Claim standing:
[pcc:unverified].

## Exact hypotheses and construction

Let `ell=2n+1` be prime with odd `n>=3` (hence `ell=3 mod 4`). Let
`k=F_q` have odd characteristic `p!=ell`, and assume the multiplicative
order of `q` modulo `ell` is exactly `n`. Choose nonzero
`a,b mod ell` and `lambda in k\{0,1}`. Let `C/k` be the smooth
projective normalization of

`y^ell = f(x) = x^a ((x-1)/(x-lambda))^b`.

Its inertia at `0,1,lambda,infinity` is `(a,b,-b,-a)`, so
Riemann--Hurwitz gives `g(C)=ell-1=2n`. Since `ell` does not divide
`q-1`, there is a unique `delta in k` with
`delta^ell=lambda^(a-b)`. The identity

`f(lambda/x)*f(x)=lambda^(a-b)`

makes `tau(x,y)=(lambda/x,delta/y)` an involution. The two geometric
fixed base coordinates satisfy `x^2=lambda`; neither is a branch
coordinate, and each odd-prime Kummer fiber has exactly one fixed
point. Thus `tau` has two geometric fixed points and the smooth
quotient `Q=C/<tau>` has `g(Q)=n`. [pcc:unverified]

## Quotient Weil polynomial

For `1<=m<n`, the order assumption gives
`gcd(ell,q^m-1)=gcd(ell,q^m+1)=1`. The map `y->y^ell` is bijective on
`F_(q^m)`, so every rational base coordinate has one rational point
above it and `#C(F_(q^m))=q^m+1`. To count the twisted fixed points of
`tau*Frob_(q^m)`, its base coordinates solve
`x^(q^m+1)=lambda`, giving `q^m+1` distinct geometric solutions.
The action on each `ell`-point fiber has multiplier `-q^m mod ell`, so
`gcd(ell,q^m+1)=1` makes exactly one lift fixed. The standard quotient
fixed-orbit formula therefore gives

`#Q(F_(q^m)) = (#C(F_(q^m)) + #Fix(tau*Frob_(q^m)))/2 = q^m+1`

for `1<=m<n`. The twisted fixed-point count is `q^n+1` also at
`m=n`, because `q^n=1 mod ell` and hence `ell` does not divide
`q^n+1`. Newton identities make the first `n-1` non-leading
coefficients of the degree-`2n` Frobenius polynomial vanish; the
functional equation makes their reciprocal partners vanish. Thus

`P_Q(X)=X^(2n)-t X^n+q^n`, where
`t=(q^n+1-#C(F_(q^n)))/(2n)`.

For rational branch coordinates, put `N` equal to the number of
`x in F_(q^n)\{0,1,lambda}` for which `f(x)` is a nonzero
`ell`th power. Then `#C(F_(q^n))=4+ell*N`, giving the search formula

`t=(q^n-3-ell*N)/(ell-1)`.

The quotient itself guarantees that this rational expression is an
integer. The Hasse bound gives `|t|<=2q^(n/2)`.
[pcc:unverified]

## Transfer and genus criteria

If `p` does not divide `t`, the quadratic polynomial
`Z^2-tZ+q^n` is an ordinary elliptic Weil polynomial. By the
Honda--Tate/Waterhouse classification there is an elliptic curve
`E/F_(q^n)` with this trace. The restriction of scalars
`A=Res_(F_(q^n)/k) E` has Frobenius polynomial `P_Q`, so Tate's
finite-field theorem gives `A ~_k J(Q)`. This identifies a
**parameterized isogeny-class carrier family**, not an evaluation
algorithm for a source-point map. [pcc:unverified]

If `P_Q` is irreducible over the integers, then `A` and `J(Q)` are
`k`-simple of dimension `n`. Consequently any curve `Z/k` whose
Jacobian receives a nonzero `k`-homomorphism from `A` has genus at
least `n`; this `Q` attains the genus floor. The condition is on the
full polynomial, not merely on the ordinary trace. [pcc:unverified]

For a prime `r` dividing `#E(F_(q^n))=P_Q(1)=q^n+1-t`, an additional
**existence-only** subgroup criterion is

`r does not divide p*n*(t^2-4q^n)`.

Indeed the polynomial discriminant is

`disc(P_Q)=n^(2n)*q^(n(n-1))*(t^2-4q^n)^n`.

When `P_Q` is irreducible and the criterion holds, each `r`-adic Tate
module is free of rank one over the same finite étale order
`Z_r[X]/(P_Q)`. Integral Tate lifts a Frobenius-equivariant lattice
isomorphism to a `k`-isogeny of degree prime to `r`; it is injective
on an order-`r` source subgroup. It does **not** produce an evaluable
isogeny formula. If the criterion fails, subgroup preservation is
undecided, not excluded. [pcc:unverified]

For `ell=11,n=5,q=5`, the previously saved exact carriers have traces
`2` and `24`. The trace-24 source has largest prime subgroup `r=47`;
`24^2-4*5^5=-11924` is nonzero modulo 47, so the existence
criterion applies. At trace 2, `2^2-4*5^5=-12496` is divisible by
71, leaving the largest prime subgroup outside this criterion. The
trace-2 quotient has an independently replayed hyperelliptic model;
the trace-24 quotient has the paper point-count derivation. See
[`../ghs_cyclic11_four_branch_cm_20261116/RESULT.md`](../ghs_cyclic11_four_branch_cm_20261116/RESULT.md)
and its receipts. [pcc:unverified]

## Prior work and claim boundary

The dihedral covering geometry and Jacobian decomposition are prior
art: [Izquierdo--Jimenez--Rojas](https://arxiv.org/abs/1609.01562)
and [Catanese--Lönne--Perroni, *Dihedral covers of algebraic curves*](https://ems.press/content/serial-article-files/29758?nt=1)
study broad dihedral cover families. [Joux--Vitse's cover and
decomposition attack](https://eprint.iacr.org/2011/020.pdf) gives
explicit lower-degree transfers and cites Thériault's 2003 Kummer
descent work. The genus-five
[Fermat quotient and explicit isogeny](https://arxiv.org/abs/2611.08176)
and the [Chebyshev CM family](https://arxiv.org/abs/2511.08422)
are adjacent prior work, with different displayed equations. The
specific `ell=11` equations and quotient replay are in the earlier
study. [Waterhouse's classification](https://numdam.org/item/ASENS_1969_4_2_4_521_0/),
[Tate 1966](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf),
and [Poonen--Rybakov, Lemma 2.2(iii)](https://math.mit.edu/~poonen/papers/tate_modules.pdf)
supply the standard isogeny and integral-lattice steps.

The shared KB was queried first at revision
`9141e386254340a9bdba3036f2bb706a`, last refreshed
14 November 2026. Sources checked (KB):
`source:3cfffa8b1bb8f7b2bae25e9ee62d5d15`
(`crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-odd-prime-ghs-cover-floor.yaml`)
identified the earlier cover-floor idea;
`source:d83cb619b3afdb7b9d999946161057c7`
(`crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-weil-restriction-jacobian-class.yaml`)
already proposed the abstract Jacobian route. The KB predates the
16 November curve work, and an unsuccessful exact-equation search
does not establish publication priority. This note claims a new
general criterion **within this project**, with global novelty
unresolved. The geometric family and isogeny method are established;
no end-to-end attack or below-rho cost has been established by this
theorem. The genus floor applies to this irreducible isogeny class,
not to every GHS-eligible curve or every cover attack.
[pcc:unverified]

The shared `docs/session-logs/2026-11-16.md` was read first, but
Conductor T-6720 reserves it in a verifying branch. This note records
the derivation while that path is unavailable. The next technical
falsifier is an explicit source-to-`J(Q)` evaluation map on the
trace-24 order-47 subgroup, followed by a complete same-target
attack and matched rho run. [pcc:unverified]
