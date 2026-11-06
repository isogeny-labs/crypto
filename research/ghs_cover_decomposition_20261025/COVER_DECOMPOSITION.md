# Cover decompositions and transfer-factor gate

25 October 2026. This is an exact follow-up to the ten end-to-end binary GHS
receipts in `../ghs_generalized_20261024/results/`. The calculation is new;
the attack construction is the prior-known GHS/isogeny family. No additional
target or elapsed-time sample was selected here.

## Two meanings of decomposition

An **isogeny decomposition** splits a Jacobian into lower-dimensional abelian
factors. It helps a transferred discrete logarithm only if an explicit
projection sends the *selected source subgroup* injectively into a factor
whose DLP is cheaper. A factored group order, cover equation, or genus alone
does not establish that property. An irreducible degree-four Frobenius
polynomial rules out an elliptic isogeny factor over the base field.

A **divisor decomposition** expresses Jacobian elements as sums of
factor-base classes to build index-calculus relations. It does not require a
split Jacobian. The existing quadratic attack executable uses this second
meaning: it tests whether reduced degree-at-most-two Mumford divisors split
over the smaller field, reconstructs each accepted relation, solves a matrix,
and descends the same source target. Joux and Vitse's structured genus-three
cover uses a different, Nagao-type six-variable decomposition; it is prior
art and already has a separate repository ledger.

## Exact point-count method

For every executed edge, `audit_frobenius.sage` reads the exact source field
polynomial, the descended `h,f` arrays, and the **subfield embedding basis**
from its frozen JSON receipt. The cover coefficient words are coordinates in
that smaller-field basis, so treating them as source-field polynomial-basis
words would give the wrong curve. The script independently constructs
`K=GF(2^n)`, enumerates `k={x in K : x^(2^(n/2))=x}`, and counts points of
`C: W^2+h(u)W=f(u)` over both `k` and `K`.

For `h(u)=0`, every `u` has one ordinate. Otherwise there are two ordinates
exactly when the absolute binary trace of `f(u)/h(u)^2` is zero. The odd
degree-five model has one rational point at infinity. At degrees 6 and 8,
the script separately enumerates every `(u,W)` and verifies the trace count
for all four edges. It then derives the genus-two Weil polynomial from
`N_1=#C(k)` and `N_2=#C(K)`: with `q=|k|`,

```text
t_1 = q+1-N_1,
t_2 = q^2+1-N_2,
a_2 = (t_1^2-t_2)/2,
P_C(X) = X^4 - t_1 X^3 + a_2 X^2 - q t_1 X + q^2.
```

It factors `P_C` over the integers in Sage and independently recomputes the
source elliptic order with Sage. The ten results are in `results.json`, with
the SHA-256 of every input receipt.

| Source field `K` | `q` | `#C(k)` | `#C(K)` | `P_C(X)` | `#Jac(C)(k)` | source `r` | integer square obstruction `2q-a_2` |
|:--|--:|--:|--:|:--|--:|--:|--:|
| GF(2^6), modulus `0x43` | 8 | 9 | 87 | `X^4+11X^2+64` | 76 | 19 | 5 |
| GF(2^8), modulus `0x11b` | 16 | 17 | 279 | `X^4+11X^2+256` | 268 | 67 | 21 |
| GF(2^10), modulus `0x409` | 32 | 33 | 1039 | `X^4+7X^2+1024` | 1032 | 43 | 57 |
| GF(2^12), modulus `0x1009` | 64 | 65 | 3863 | `X^4-117X^2+4096` | 3980 | 199 | 245 |

All ten polynomials are irreducible over `Q`; direct, degree-2, degree-3,
and degree-5 edges at a given source degree have identical polynomials.
The table's square-obstruction values are all nonsquares. The script also
records the other integer-factor cases and performs the full Sage
factorization rather than relying on this shortcut alone. Every
`#Jac(C)(k)` equals the independently checked `#E(K)`, and each selected
prime `r` divides it.

Writing `t_E=q^2+1-#E(K)`, the checked equality is

```text
P_C(X) = X^4 - t_E X^2 + q^2.
```

This is the Frobenius polynomial of `Res_{K/k}(E)`. By the finite-field
isogeny theorem, the checked genus-two Jacobian is `k`-isogenous to that
Weil restriction. Its irreducible polynomial makes it `k`-simple: none of
these ten specific covers supplies a smaller-field elliptic factor to
project onto. After extending constants to `K`, the Frobenius polynomial
becomes `(X^2-t_E X+q^2)^2`, so the base-changed Jacobian is isogenous to
`E^2` over `K`; its group order there is `#E(K)^2`. This latter splitting
does not itself give a cheaper DLP or an executable projection. The audit
does not compute the geometric isogenies in either direction.

## Placement of a transferred subgroup

For a same-field cover with pullback `pull:E->Jac(C)` and norm
`push:Jac(C)->E`, the geometric identity `push(pull(x))=[d]x` motivates
the complementary operator `Pi=[d]id-pull*push`. The checked Lean theorem
`complementary_kills_pull` proves `Pi(pull(x))=0` for every `x`, conditional
on that identity. Thus the canonical pulled source subgroup has zero image
under this Prym-style complement. This does **not** prove a concrete norm
or pullback implementation, nor does it rule out a different correspondence
whose projection is nonzero. It also must not be applied automatically to
the GHS trace-conorm map across fields, which has different geometry.

The norm-pullback issue is relevant to the existing same-field cubic binary
cover in `src/binary_ecc/cover_transfer.rs`: its degree-three composition
is certified on point-class witnesses, while arbitrary-class pushforward is
implemented only for one fixed GF(8) fixture. The present calculation does
not upgrade that boundary. For any proposed factor route, a usable receipt
still needs an explicit projector, nonzero order-`r` image, target relation,
and complete measured attack against same-target rho.

## Exact cubic-cover Prym decomposition

`audit_cubic_cover.sage` separately checks the repository's complete GF(8)
norm fixture. In the polynomial basis `z^3+z+1=0`, its equations are

```text
E: y^2+xy=x^3+x^2+(z+1),       #E(GF(8))=6,
H: v^2+u^2*v=u^7+u^4+d*u,      d^2=z+1,
phi(u,v)=(u^3,u*v+d),          deg(phi)=3.
```

Direct point counts give `#H(GF(8))=6`, `#H(GF(64))=104`, and
`#H(GF(512))=558`. Each count agrees across all three Frobenius-conjugate
embeddings of the fixture GF(8) into the extension field; the GF(8) count
also agrees with independent equation enumeration. Newton identities give

```text
P_H(X)=X^6-3X^5+24X^4-48X^3+192X^2-192X+512
      =(X^2-3X+8)(X^2+8)^2.
```

Thus the Prym isogeny factor has polynomial `(X^2+8)^2` and order 81;
`#Jac(H)(GF(8))=6*81=486`, agreeing with the independently enumerated
486 reduced classes in the Rust fixture. The factor `X^2+8` is realized
by the elliptic curve `E_0:y^2+y=x^3` over GF(8), independently checked
to have order 9. Tate's finite-field theorem therefore identifies the
isogeny class as `Jac(H) ~ E × E_0^2` over GF(8). This is a genuine
elliptic-factor decomposition, but its complementary factors are not
candidate destinations for the canonical pullback: the checked
norm-pullback identity and the Lean lemma make `[3]id-pull*push` zero on
that source image. `E` and `E_0` have different Frobenius polynomials, so
they are not GF(8)-isogenous and there is no nonzero GF(8)-defined
homomorphism from `E` to either `E_0` factor. Consequently every algebraic
projection of this canonical source image to the Prym is zero; this is
stronger than comparing the factor orders.

The coprime factors have the exact integral identity

```text
3X(X^2-3X+8)+(9-3X)(X^2+8)=72.
```

The resultant 72 is divisible by both primes in the rational source
order 6, so dividing this scaled factor separation does not construct
an integral projector on either source primary subgroup. The degree-three
norm-pullback certificate alone establishes injectivity only for the
order-two subgroup; it does not certify injectivity for order three.
The result is a structural counterexample to treating a split cover
Jacobian as an automatic lower-cost transfer. No new factor attack or
timed attack run follows from this decomposition.
Lombardo, Lorenzo García, Ritzenthaler, and Sijsling previously studied
Prym decompositions for degree-three genus-three covers of elliptic curves
with two total-ramification points. Their treatment is in characteristic
zero and does not certify this characteristic-two fixture or its exact
GF(8) Frobenius polynomial; it does establish that the broad cover/Prym
strategy is prior work.

## Divisor-decomposition evidence already measured

The ten frozen attack receipts exhaust the nonzero selected subgroup during
relation collection, so these counts describe that finite subgroup walk,
not a random sample from the full Jacobian. At each source degree, the
direct and small-isogeny edges have the same accepted-divisor count:

| `n` | factor-base classes | decomposable nonzero subgroup elements | relation attempts | direct matrix rank |
|--:|--:|--:|--:|--:|
| 6 | 8 | 10 | 18 | 7 |
| 8 | 16 | 50 | 66 | 16 |
| 10 | 32 | 26 | 42 | 22 |
| 12 | 64 | 132 | 198 | 56 |

The degree-3 edge at `n=10` has rank 18 rather than 22. The other listed
isogeny edges retain the direct rank. These numbers are a complete local
decomposition census for the selected subgroup walks; they are not an
asymptotic smoothness estimate or a speedup claim. The exact precomputation,
matrix, target-descent, and rho costs remain in the original ten receipts
and `../ghs_generalized_20261024/REPORT.md`.

## Next cover-attack gate

These ten quadratic covers are closed as **base-field elliptic-factor
leads**. A new cover or correspondence should pass, in order: exact field
and curve identity; geometric map; subgroup-preserving pullback/isogeny;
Frobenius-factor and projector computation if factor splitting is proposed;
factor-base decomposition with verified relations; full target descent;
independent replay; and same-target rho accounting. A split polynomial
without an explicit nonzero subgroup projection remains a structural lead.
The structured cubic-extension constructions of Joux--Vitse and Tian are
separate prior-known routes, with different cover and isogeny hypotheses;
the present binary quadratic audit does not test them.

Reproduce from this directory:

```sh
sage -python audit_frobenius.sage
sage -python audit_cubic_cover.sage
LEAN_PATH=../ghs_generalized_20261024/lean lean -o CoverProjection.olean CoverProjection.lean
LEAN_PATH=.:../ghs_generalized_20261024/lean lean --run ../../tools/lean/AuditAxioms.lean CoverProjection
```

The two Sage audits passed on 25 October 2026. The Lean checker reported
one theorem, zero failing and no extra axioms for the complementary
operator. From the repository root, the focused
`cargo test fixed_f8_arbitrary_pushforward_is_complete_and_homomorphic --lib`
passed (one test, 4,746 filtered out, 190.27 seconds of test execution).
That fixture test checks all 486 arbitrary-class norm images and ordered
pair homomorphism cases, and the norm-pullback composition on all six
rational points of `E`. Its time is validation time, not a DLP attack cost.

Primary references checked before selecting this audit:

1. [Magma, Weil Descent in Characteristic Two](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html): parameter, map, and divisor-class scope.
2. [Joux and Vitse, *Cover and Decomposition Index Calculus on Elliptic Curves Made Practical*](https://eprint.iacr.org/2011/020.pdf): structured genus-three cover and relation decomposition.
3. [Tian, *Cover Attacks for Elliptic Curves with Prime Order*](https://arxiv.org/abs/2012.07173): cubic-extension Weil-restriction isogeny route.
4. [Tate, *Endomorphisms of Abelian Varieties over Finite Fields*](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf): finite-field isogeny classification by Frobenius.
5. [Lombardo et al., *Decomposing Jacobians via Galois Covers*](https://arxiv.org/abs/2003.07774): prior degree-three genus-three to genus-one Prym decomposition in characteristic zero; not an exact finite-field certificate for this fixture.
