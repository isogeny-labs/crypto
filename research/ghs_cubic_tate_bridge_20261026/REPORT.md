# A cubic class-C transfer-eligible family and its exact GF(16) bound

26 October 2026. Conductor T-5377. The [protocol](PROTOCOL.md) preceded
the audited run. [Source](audit.py), [certificate](certificate.json), and
[finite Lean checks](SimpleRootQ16.lean) are in this directory. The exact
plane quartics, field bases, selected subgroup points, and independent
point-count replay are in the [preceding report](../ghs_binary_cubic_isogeny_q16_20261026/REPORT.md).

## Result and novelty boundary

Three smooth plane quartics over `k=GF(16)` identify **three entire
isogeny-class families**, comprising exactly **300 normalized ordinary
`K=GF(4096)` elliptic isomorphism classes**, whose selected prime subgroups
survive under *some* `k`-isogeny
`Res_{K/k}(E) -> Jac(C)`. None of the 300 has the direct binary cubic
genus-three hyperelliptic cover condition `Tr_{K/k}(b)=0`.

This is a new **audited family in this research project**, with a rigorous
finite census and an existence theorem for the subgroup map. Its possible
publication novelty is unresolved. It is not a newly established vulnerable
family: the existence proof gives neither an efficiently evaluable map nor
relation/decomposition and target-descent costs below rho. The overall
method of transferring through a cubic Weil-restriction isogeny is prior work
([Tian](https://arxiv.org/pdf/2012.07173)); direct binary cubic covers are
prior work ([Joux--Vitse, Appendix A.2](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf)).
[Momose--Chao's genus-three weak-cover classification](https://eprint.iacr.org/2009/236.pdf)
assumes odd characteristic, so it does not classify these binary examples.
These comparisons delimit the checked literature; they do not prove that no
other publication identified the same isogeny classes.

## Isogeny-class and subgroup theorem

Let `k=F_q`, `K=F_{q^3}`, and `E/K` be an elliptic curve with trace
`t=q^3+1-#E(K)`. Write `A=Res_{K/k}(E)` and let `C/k` be a smooth genus-three
curve. Frobenius cyclically permutes the three conjugate elliptic factors of
`A` and its third power restricts to the `K`-Frobenius. Therefore

```text
P_A(X) = X^6 - t X^3 + q^3.
```

Newton's identities and the functional equation show that `Jac(C)` has
this same polynomial whenever

```text
#C(k) = q+1,
#C(F_{q^2}) = q^2+1,
#C(K) = q^3+1-3t.
```

The converse holds for a genus-three `C`: these three counts determine its
Weil polynomial. Equality of Frobenius polynomials gives a `k`-isogeny
`A -> Jac(C)` by [Tate's finite-field isogeny theorem](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf).

**Simple-root subgroup-survival proposition.** Suppose `A` and `B` are
`k`-isogenous abelian varieties with common Frobenius polynomial `P`, and
`r != char(k)` is prime with `r | P(1)` and `gcd(r,P'(1))=1`. Then there is
a `k`-isogeny `f:A -> B` that is nonzero on the one-dimensional
`A(k)[r]`. In particular, it injects every order-`r` rational subgroup.

Proof: modulo `r`, the root `X=1` of `P` is simple. Hensel factorization
separates a rank-one Frobenius-stable summand of each `r`-adic Tate module;
Frobenius acts by the same lifted root on both. An equivariant unit map
between those summands, zero on their complements, is an integral Tate-module
homomorphism. [Tate's integral Hom theorem](https://math.stanford.edu/~conrad/mordellsem/Notes/L03.pdf)
identifies it with an element of `Hom_k(A,B) tensor Z_r`. Reducing modulo
`r` gives an actual `k`-homomorphism `f` with the same nonzero action on
`A(k)[r]`. If `f` is not an isogeny, choose any isogeny `g:A -> B` and replace
`f` by `f+r m g`. Its action on `A(k)[r]` is unchanged, while the determinant
on the rational Tate module is a nonzero polynomial in the integer `m`;
some `m` makes it an isogeny. This is an existence argument and supplies
no coordinates or cost bound.

For the cubic polynomial,

```text
P(1)=q^3+1-t=#E(K),
P'(1)=6-3t ≡ 3(1-q^3) (mod r).
```

Thus a prime subgroup `r` outside the prime divisors of `3(q^3-1)` passes
the simple-root gate whenever a matching genus-three Jacobian exists.
For `q=16`, `q^3-1=4095=3^2*5*7*13`, so only
`{3,5,7,13}` can fail this *sufficient* gate among odd primes. Failure of
the gate is not an obstruction to every transfer.

## Exact finite families and bounds

Every ordinary `E/K` has a normalized model
`y^2+xy=x^3+a x^2+b`, with `b != 0` and `a` chosen from the two absolute
trace classes. In the polynomial-basis encoding of the parent report these
representatives are `a=0` and `a=32`. Distinct `b` give distinct
`j=1/b`, so the rows below count distinct `K`-isomorphism classes.

| `#E(K)` | `a` | selected `r` | cofactor | normalized classes | `t` | `P'(1)` | LMFDB class |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | :--- |
| 4036 | 0 | 1009 | 4 | 96 | 61 | -177 | [3.16.a_a_acj](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_acj) |
| 4124 | 0 | 1031 | 4 | 96 | -27 | 87 | [3.16.a_a_bb](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bb) |
| 4126 | 32 | 2063 | 2 | 108 | -29 | 93 | [3.16.a_a_bd](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_bd) |

For each row `r` is prime, `gcd(r,P'(1))=1`, the Jacobian order was
independently replayed, and the complete normalized source census contains
no direct class-B model with that order. The certificate records every
`b` value in each family, not just the counts. Hence exactly 300 normalized
classes satisfy **these three fixed isogeny-class certificates**. Among them,
the number for which a complete attack beats rho is bounded only by
`0 <= vulnerable_count <= 300`; an attack experiment is needed to sharpen it.
Hasse's elliptic bound gives `|t| <= 2*q^(3/2)=128`, so all source orders at
`q=16` lie in `[3969,4225]`; the audit covers every normalized ordinary
model in that finite range, but its quartic witness draw is not exhaustive.

**Exclusion from the known direct-cover isogeny closure.** Every `K`-isogeny
preserves `#E(K)`. The exhaustive normalized ordinary census has zero
trace-zero class-B models at each of the three orders above. Therefore no
curve in these 300 classes can reach a direct binary cubic genus-three
hyperelliptic cover by *any* chain of `K`-isogenies, regardless of walk
length or kernel degrees. This is stronger than a bounded-walk failure.
It excludes only that known direct-cover route; it does not exclude the
different plane-quartic isogeny route certified above or higher-genus GHS
descents.

The full ordinary `K` cohort has `2(q^3-1)=8190` normalized classes.
The direct cubic classes have exact counts

```text
A (b in k*)               = 2(q-1)          = 30,
B (Tr_{K/k}(b)=0, b!=0)  = 2(q^2-1)        = 510,
C (remaining b)           = 2(q-1)(q^2-1) = 7650.
```

Because the cubic relative trace of a nonzero `b in k` equals `b` in
characteristic two, these three sets are disjoint. The direct-B count is
already implicit in the known binary cover classification; the new audited
claim here is the 300-class, class-C-only *isogeny-transfer eligibility*
certificate, not direct-cover eligibility.

### Exact bound for the known direct-cover route, including all isogenies

Two elliptic curves over the same finite field `K` are `K`-isogenous if and
only if they have the same trace, equivalently the same `#E(K)`
([Tate](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf)).
Let `S_B` be the set of source orders attained by at least one normalized
class-B model. Exhausting all 8190 ordinary models gives `|S_B|=36` and

```text
#{E : #E(K) in S_B} = 2608 = 510 direct B + 2088 class C + 10 class A,
#{E : #E(K) not in S_B} = 5582 = 5562 class C + 20 class A.
```

Thus **at most 2608 of the 8190 normalized ordinary GF(4096) classes**
can enter the known direct binary cubic genus-three hyperelliptic-cover
family through an arbitrary `K`-isogeny. The other **5582 are
rigorously outside that entire route**, not just beyond a tested walk
radius. The certificate gives all 36 qualifying orders and the exact
partition. This is a bound on structural eligibility; the number of
curves for which the *complete* attack beats rho is between 0 and 2608.
Other GHS variants, the plane-quartic route above, and higher-genus
descents are outside this exclusion theorem.

## Checked scope and next gate

`sage -python audit.py` independently replayed all 8190 source orders using
a Walsh transform of the trace-of-inverse identity, checked the class
partition, primality and coprimality of all selected `r`, matched the prior
quartic certificates, and wrote `certificate.json`. `lean SimpleRootQ16.lean`
checks the exact integer products and gcd gates. The geometric Tate theorem
is cited and proved at the level above; it is not formalized in Lean here.

The parent `results.json` is managed by the repository artifact store. A
fresh checkout first runs
`python3 tools/artifacts/artifacts.py pull research/ghs_binary_cubic_isogeny_q16_20261026`.
On this host that pull returned S3 `HeadObject` 403 on 26 October 2026. For
the local replay only, the exact bytes were recovered from immutable Git
commit `c9f7029fe12665b3ea6f2312cbf26de6a2877502`, and
`artifacts.py verify` matched the manifest SHA-256. The certificate is
therefore replayed locally, while fresh-checkout artifact retrieval remains
an external access gate.

To move from this family-level existence result to a vulnerability claim,
construct an evaluable `k`-isogeny for each relevant source or a bounded
isogeny route to one, verify its nonzero action on the selected subgroup,
then run complete relation collection, linear algebra, target descent and
independent scalar replay against same-target rho. The general existence
theorem alone supplies no complexity bound for finding or evaluating that
isogeny.
