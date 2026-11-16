# Order-four metacyclic carrier: exact cubic screen and prior-art collision

16 November 2026. This is a paper construction and an exact finite
screening target. It is not a vulnerability claim or a Lean-checked
theorem. Claim standing: [pcc:unverified].

## Prior work changes the novelty label

[Ellenberg, *Endomorphism algebras of Jacobians*, Section 2,
case 2a](https://people.math.wisc.edu/~ellenberg/EndJacAjour.pdf)
already constructs the metacyclic `G_(ell,4)` quotient with
branch signature `(ell,4,4)` and real multiplication by the
index-four cyclotomic subfield. In particular, the genus-`n`
geometry below is **known**; the exact `F3` equation is a
finite-field specialization, not a new geometric family.
[pcc:unverified]

[Song Tian, *Cover attacks for elliptic curves with prime
order*](https://arxiv.org/pdf/2012.07173) gives a separate
constructive genus-three transfer via `(l,l,l)`-isogenies for
some cubic-extension elliptic curves, including prime-order
groups. That paper does not identify the exact cyclic-13
specialization below in the searched text. Its general method
prevents labeling an abstract genus-three isogeny match here
as a new attack method. The precise overlap of eligible
isogeny classes remains to be checked. [pcc:unverified]

## Conditional finite-field theorem

Let `ell=4n+1` be prime with `n` odd, let `q` be a prime power
prime to `4ell` with `ord_ell(q)=n`, and suppose a tame cyclic
`ell`-cover `Y/P1` over `F_q` has four full-inertia branch
points in one orbit of an order-four base automorphism. Assume
the lift `tau` has order four and normalizes the deck group by
an exponent of order four modulo `ell`. Put `Q=Y/<tau>`.

Riemann--Hurwitz gives `g(Y)=ell-1=4n`. The two fixed points
of the base automorphism cannot be branch points because
their nonzero inertia exponent would have to be fixed by the
nontrivial order-four deck exponent. The lift has one fixed
point above each base fixed point. Riemann--Hurwitz for
`Y->Q` then gives `g(Q)=n`. [pcc:unverified]

For `m` with `n` not dividing `m`, the `ell`th-power map on
`F_(q^m)` is bijective. The same holds for each twisted
Frobenius lift `tau^j Frob_(q^m)`, `j=1,2,3`: its multiplier
on an unbranched fiber has nontrivial order-four-or-`n`
component and cannot be one modulo `ell` because
`gcd(n,4)=1`. Each rational base point therefore has one
fixed lift in every term of the quotient fixed-orbit formula,
so `#Q(F_(q^m))=q^m+1`. Newton identities force

`P_Q(X)=X^(2n)-t*X^n+q^n`

for an integer `t` satisfying `|t|<=2*q^(n/2)`.
If `t` is prime to the characteristic, the quadratic
`Z^2-tZ+q^n` is an ordinary elliptic Weil polynomial;
[Waterhouse](https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf)
and Tate give an elliptic `E/F_(q^n)` with
`JQ ~ Res_(F_(q^n)/F_q) E` over `F_q`. This is an abstract
isogeny-class match. It does not provide an evaluable map on
source points. [pcc:unverified]

## Exact `F3` model

Take `ell=13`, `n=3`, `q=3`, and

`Y3: y^13 = f(x) = x^5*(x-1)^12*(x+1)^8`.

The exponent at infinity is `-25=1 mod 13`, so the four
full-inertia branch points are `(infinity,0,1,2)` with
exponents `(1,5,12,8)`. The order-four base map
`phi(x)=1/(x+1)` cycles them in that order. An explicit lift is

`tau(x,y) = (1/(x+1), y^5/(x*(x-1)^4*(x+1)^5))`.

Direct substitution gives

`f(phi(x))/f(x)^5 = 1/(x^13*(x-1)^52*(x+1)^65)`.

Thus `tau` is a function-field automorphism. Its fourth power
fixes `x` and is an `F3`-rational deck map. Since
`mu_13(F3)={1}`, `tau^4=1`. The smooth quotient
`Q3=Y3/<tau>` has genus three and

`P_Q3(X)=X^6-t*X^3+27`.

The one missing count is `#Q3(F27)`, which determines
`t=(28-#Q3(F27))/3` and `#E(F27)=28-t` if an elliptic
source exists. The factor and source-class checks must
separate subfield-descended classes from extension-only
classes and compare with the cubic-cover classifications of
[Momose--Chao](https://eprint.iacr.org/2009/236.pdf) and
Tian. No value of `t`, elliptic equation, source map,
prime subgroup, or attack cost is asserted here.
[pcc:unverified]

## Search receipt and next test

The KB was queried first at revision
`9141e386254340a9bdba3036f2bb706a`, refreshed
14 November 2026. Searches for order-four metacyclic
four-branch quotients and GHS Weil-restriction factors gave
no exact hit; the primary Ellenberg paper found through a
separate literature search resolves the geometric-priority
question. Sources checked (KB):
`source:4dd85394386cb112e0c92ed6f73967d0`
(`crypto:research/notes/ecc2k130/RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md`)
is a related cyclic-cover obstruction, not this case.
The KB predates this note, and absence from its search does
not establish priority. [pcc:unverified]

The next decisive test is the exact `F27` quotient count,
followed by an independent zeta replay, an eligible source
equation, a subgroup map, and a full same-target cost run.
The shared `docs/session-logs/2026-11-16.md` is reserved
by Conductor T-6720, so new computational experiments are
deferred and this note preserves the exact derivation.
[pcc:unverified]
