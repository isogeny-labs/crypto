# Class-group screen for metacyclic GHS carriers

16 November 2026. This note extends the conditional factor screen in
[`../ghs_metacyclic_factor_multiplicity_20261116/ORDINARY_FACTOR_BOUND.md`](../ghs_metacyclic_factor_multiplicity_20261116/ORDINARY_FACTOR_BOUND.md).
It is a paper argument, not a Lean-checked theorem or a measured attack.
Claim standing: [pcc:unverified].

## Family-level necessary condition

Let `ell=6n+1` be prime, `n>1` odd, `gcd(n,3)=1`, and
`ord_ell(q)=n`. Let `Y/F_q -> P1` be a tame cyclic `ell`-cover
with `3r` fully ramified branch points in `r` orbits of an
order-three base automorphism. Assume its lift `tau` has order three
and conjugates the deck generator by an element of order three
modulo `ell`. Write `Q=Y/<tau>`. The branch and fixed-point
calculation in the linked note gives `g(Q)=n(3r-2)` and

`P_Q(X)=R(X^n),   deg R=2(3r-2)`.

Over `F_(q^n)` the deck group is defined. Its nonidentity
characters each occur `3r-2` times in `H^1(Y)`: a nonidentity deck
map fixes the `3r` branch points, so its Lefschetz trace on `H^1`
is `2-3r`, while the trivial character is absent because the deck
quotient is `P1`. Taking `tau`-invariants gives `3r-2` copies of
the nontrivial double-coset representation on `H^1(Q)`.
Consequently the field

`M=Q(zeta_ell)^(<k>),   [M:Q]=2n`,

embeds in `End^0_(F_(q^n))(JQ)`. This uses the group-algebra
correspondence of [Carocca--Lange--Rodriguez, Section 3](https://arxiv.org/pdf/0906.4185),
with the representation multiplicity above for arbitrary `r`.
[pcc:unverified]

Suppose an ordinary elliptic Weil polynomial

`g_t(Z)=Z^2-tZ+q^n`

occurs with multiplicity `e` in `R(Z)`. Over `F_(q^n)`, the
corresponding isotypic component of `JQ` is `E^(ne)` and has
endomorphism algebra `M_(ne)(L)`, where
`L=Q(sqrt(t^2-4q^n))`. The `M`-action preserves this component.
If `L` is not contained in `M`, then `ML/L` has degree `2n`;
its action on `L^(ne)` forces `2n | ne`, so **`e` is even**.
An odd-multiplicity ordinary elliptic factor therefore requires

`L=L0=Q(sqrt(-ell)),   t^2+ell*u^2=4q^n`

for an integer `u`. Here `L0` is the unique quadratic subfield of
`M`, since `ell=6n+1` is `3 mod 4`. These are necessary conditions;
neither factor occurrence nor a source-to-carrier map follows.
[pcc:unverified]

## Class-group exclusion of a new extension-field isogeny class

There is a second restriction on the odd-multiplicity exception.
Let `h=h(L0)` and assume `gcd(n,h)=1`. If an ordinary `E/F_(q^n)`
has endomorphism field `L0`, its Frobenius ideal is a power of one
prime above the characteristic: `(pi)=P^(sn)` for `q=p^s` (or the
conjugate prime). Thus `[P^s]^n=1` in `Cl(L0)`. Coprimality gives
`[P^s]=1`, so `P^s=(alpha)` with `Norm(alpha)=q`. The algebraic
integer `alpha` is an ordinary `q`-Weil number. Waterhouse's
classification supplies `E0/F_q` with Frobenius `alpha`.
Since `pi` and `alpha^n` generate the same ideal, they differ by
a unit; for `ell>3` the only units are `+/-1`. Because `n` is odd,
replacing `alpha` by `-alpha` absorbs this sign. Therefore `E` is
`F_(q^n)`-isogenous to the base change of an elliptic curve over
`F_q`. [pcc:unverified]

Hence, **when `gcd(n,h(-ell))=1`, every ordinary elliptic factor
of `R` that represents an extension-field isogeny class absent
over `F_q` has even multiplicity**. For two branch orbits,
`deg R=8`, so such a factor occupies at least degree four in `R`
and dimension `2n` in `JQ`. This is a lower bound on carrier
dimension and multiplicity, not an attack-cost bound.
[pcc:unverified]

For `ell=31,n=5`, the reduced primitive positive binary forms of
discriminant `-31` are `(1,1,8)` and `(2,+/-1,4)`, so `h(-31)=3`
(also recorded by [LMFDB, field 2.0.31.1](https://www.lmfdb.org/NumberField/2.0.31.1)).
Thus the coprimality condition holds for **every** base field
`F_q` with `ord_31(q)=5`, including the `F4` example. A simple
ordinary elliptic factor, if present for any such six-branch
quotient, is in a subfield isogeny class. A genuinely new
extension-field class must enter `R` with multiplicity at least
two. [pcc:unverified]

## Exact `F47` two-orbit screen

The norm exception is actually admissible at `q=47`:
`ord_31(47)=5` and `4+sqrt(-31)` has norm `47` and trace `8`.
Its fifth-power trace is `808`, by the recurrence
`s_0=2`, `s_1=8`, `s_j=8s_(j-1)-47s_(j-2)`.
Thus the only ordinary `L0` traces over `F_(47^5)` are `+/-808`.
These are subfield isogeny classes by the preceding theorem.
[pcc:unverified]

An exact six-branch model over `F47` uses
`phi(x)=1/(1-x)` with rational three-cycles
`(0,1,infinity)` and `(2,-1,24)`. Define the smooth
normalization of

`Y47: y^31 = f(x) = x*(x-1)^5*(x-2)*(x+1)^5*(x-24)^25`.

The exponent at infinity is `-37=25 mod 31`; the six branch
points have inertia exponents `(1,5,25)` on each orbit.
The explicit lift is

`tau(x,y) = (1/(1-x), 6*y^5/((x-1)^2*(x-24)^4))`.

Indeed `f(phi(x))/f(x)^5 = 24/((x-1)^62*(x-24)^124)` and
`6^31=24 mod 47`. Its cube fixes `x` and is an `F47`-rational
deck map; because `mu_31(F47)={1}`, the cube is the identity.
The quotient `Q47=Y47/<tau>` has genus `20` and
`P_Q47(X)=R47(X^5)`, `deg R47=8`. These equations give an
explicit cover-to-quotient map in function-field form. They do
not give an elliptic-source-to-quotient map. `R47` has not been
computed; no elliptic factor is asserted. [pcc:unverified]

## Prior work and next falsifier

[Carocca--Lange--Rodriguez](https://arxiv.org/pdf/0906.4185)
establish the metacyclic double-coset field and analyze their
three-branch CM case. Their proof does not compute `R47` or an
elliptic transfer for this six-branch model.
[Iijima--Momose--Chao](https://eprint.iacr.org/2013/487.pdf)
classify specified weak-cover families under an isogeny condition;
the exact relation of this genus-20 quotient to all their families
requires a separate comparison. [Waterhouse](https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf)
supports the elliptic existence step. The class-group combination
and `F47` equation are deductions here; absence of an exact hit in
the checked sources does not establish global publication priority.

The shared KB was checked at revision
`9141e386254340a9bdba3036f2bb706a` (last refresh
14 November 2026). Searches covered six-branch metacyclic
elliptic factors, factor multiplicity, and GHS isogeny transfers.
Sources checked (KB): `source:d83cb619b3afdb7b9d999946161057c7`
(`crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-weil-restriction-jacobian-class.yaml`)
records the abstract carrier route, not this class-group bound.
The KB predates this calculation. [pcc:unverified]

The next decisive test is to compute `R47`, factor it over the
integers, and apply the even-multiplicity and `+/-808` gates.
Any surviving non-subfield factor needs an explicit nonzero map
on the selected source subgroup, complete transfer and
decomposition costs, and matched rho measurements. The shared
session log is reserved by Conductor T-6720; this note records
the derivation while new computational experiments remain deferred.
[pcc:unverified]
