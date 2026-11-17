# A new GF(8) cubic quartic Jacobian witness and five additional source classes

26 October 2026. The [frozen protocol](PROTOCOL.md), [deterministic search](search.py),
[raw witnesses](search_results.json), [independent replay](verify.py),
[verification certificate](verification.json), [independent smoothness audit](smoothness_audit.py),
[smoothness certificate](smoothness_certificate.json), [direct-route bound audit](audit_bounds.py),
[bound certificate](bounds_certificate.json), [catalog and literature check](prior_work.json),
and [Lean arithmetic](Counts.lean) extend the earlier
[GF(8) cubic search](../ghs_binary_cubic_isogeny_20261026/REPORT.md).

## New structural finding at source order 548

Set `k=GF(8)=GF(2)[a]/(a³+a+1)` and
`K=GF(512)=GF(2)[w]/(w⁹+w⁴+1)` in Sage Conway models. The following
smooth plane quartic over `k` was found at deterministic draw 12,638:

```text
C: x^4 + (a+1)x^3y + (a²+1)x²y² + axy³ + ax²yz
   + a²xy²z + ay³z + (a+1)x²z² + (a²+a)y²z²
   + (a²+a+1)xz³ + a²z⁴ = 0.
```

Its 15 coefficients in the repository's fixed quartic monomial order are
`[1,3,0,5,2,3,2,4,0,7,0,2,6,0,4]`. Independent Sage projective-point
enumeration gives `#C(k)=9`, `#C(GF(64))=65`, and `#C(K)=618`; Sage also
rechecks geometric smoothness over all three fields. Newton identities
and the genus-three functional equation determine the complete Weil
polynomial

```text
P_C(X) = X⁶ + 35X³ + 512.
```

Consequently `#Jac(C)(k)=P_C(1)=548=4*137`, independently confirmed by
Sage's plane-quartic Jacobian group. A separate Gröbner-basis audit
finds dimension zero for the homogeneous singular-cone ideal
`(F,F_X,F_Y,F_Z)`, proving projective smoothness over the algebraic
closure. The source family consists of **nine
distinct normalized ordinary `K`-isomorphism classes**

```text
E_b: v²+uv = u³+b,  b in {155,211,224,314,372,382,391,418,444}
```

where integers are Conway encodings. Each has order 548 and belongs to
class C (`Tr_{K/k}(b) ≠ 0` and `b ∉ k`). All 1,022 normalized ordinary
sources were enumerated in the parent census; **none** of order 548 is in
the known direct cubic class B. Since a `K`-isogeny preserves elliptic
order, these nine cannot reach the known direct class-B cubic cover via
*any* `K`-isogeny. Their Jacobian isogeny class is the cataloged
[`3.8.a_a_bj`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bj).

The live LMFDB row for this exact Weil polynomial has
`has_jacobian=0`, `hyp_count=0`, and no listed curve equation. In the
[LMFDB status convention](https://github.com/LMFDB/lmfdb-inventory/blob/master/db-abvar.md),
**zero means uncertain**. The explicit smooth quartic and independent
point counts settle Jacobian existence positively for this isogeny class.
That is a new certificate in this project and a candidate database
addition. It does not establish that the equation or result has never
appeared in a publication; [prior_work.json](prior_work.json) records
the bounded exact-label and equation search.

By [Tate's theorem](https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf),
`Res_{K/k}(E_b)` and `Jac(C)` are `k`-isogenous for each of the nine
sources. For the selected prime `r=137`, `P_C'(1)=111` and
`gcd(137,111)=1`. The previously proved
[simple-root proposition](../ghs_cubic_tate_bridge_20261026/REPORT.md)
therefore guarantees *some* `k`-isogeny nonzero on each order-137 source
subgroup. The replay gives a selected source `b=155`, point
`G=(478,460)` of order 137, and `Q=[7]G=(328,58)`, plus a nonzero
order-137 rational-point class in `Jac(C)(k)`. This is an existence and
factor-base certificate, not coordinates for an evaluated source map.

## Four other exact plane-quartic witnesses

The same frozen search finds four more smooth quartics in source-order
classes absent from the prior 1,024-draw sample. Exact equations and
source models are in [search_results.json](search_results.json); the
independent replay checks each equation, all source orders, the full
Weil polynomial, Jacobian group order, prime-subgroup reach from
`C(k)`, and the absence of a direct-B source at the same order.

| `#E(K)` | Prime/cofactor | Normalized source classes | Quartic `#C(K)` | LMFDB status before this search |
| ---: | :--- | ---: | ---: | :--- |
| 482 | `241,2` | 9 | 420 | [`3.8.a_a_abf`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_abf): Jacobian known, 15 hyperelliptic equations |
| 524 | `131,4` | 18 | 546 | [`3.8.a_a_l`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_l): Jacobian known, 15 hyperelliptic equations |
| 548 | `137,4` | 9 | 618 | [`3.8.a_a_bj`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bj): Jacobian uncertain, no listed equation |
| 554 | `277,2` | 9 | 636 | [`3.8.a_a_bp`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bp): Jacobian known, three hyperelliptic equations |
| 556 | `139,4` | 9 | 642 | [`3.8.a_a_br`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_br): Jacobian known, three hyperelliptic equations |

The five orders cover **54** normalized source isomorphism classes.
Their five exact plane-quartic equations are absent from the currently
listed LMFDB `curves` arrays. For the four catalog-positive classes,
every listed equation is hyperelliptic, so these smooth plane quartics
are also **not isomorphic to any currently listed curve in those classes**.
They could still have been published or cataloged elsewhere. The
deterministic 20,000-draw search did not find a witness at order 478;
that is an observed miss, not an exclusion.

## Rigorous direct-route bound over GF(512)

The complete 1,022-model normalized ordinary census has 126 direct-B
models occupying 12 group orders. Exactly 324 normalized models have
one of these orders: 126 direct B and 198 class C. Thus at most
**324/1,022** models can enter the known direct binary cubic genus-three
cover family through an arbitrary `K`-isogeny. The other **698/1,022**
are rigorously outside that route. These counts bound *structural
eligibility*, not the number for which a complete attack beats rho.
All 54 models in the five new quartic-witness order classes are among
the 698 outside the direct-B closure. The plane-quartic route is a
different isogeny mechanism and remains open.

## Attack and novelty boundary

[GHS](https://www.kantmath.de/publications/papers/weil_descent.pdf) and
[Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
cover the direct binary route and its isogeny closure.
[Tian](https://arxiv.org/pdf/2012.07173) provides an evaluated cubic
Weil-restriction isogeny construction whose displayed two-torsion
formulas do not directly apply to ordinary characteristic two;
[Milio](https://www.ams.org/mcom/2020-89-323/S0025-5718-2019-03486-5/)
computes Jacobian-to-Jacobian isogenies given a kernel. None of these
sources supplies an evaluated map for the nine order-548 sources to
the quartic above.

The order-548 finding is a **new eligible family within this project**,
with an exact quartic absent from the current catalog and a newly positive Jacobian
certificate for a catalog-uncertain class. It is **not yet a new
vulnerable family or attack method**. Before that claim, construct and
verify an efficiently evaluable subgroup-preserving isogeny, solve a
public target through the complete plane-quartic relation and
decomposition pipeline, and measure all target-dependent cost against
rho on that same `(G,Q)` pair. Current full GHS/rho cost is unknown.

`lean Counts.lean` checks the finite arithmetic, direct-route partition,
and `gcd(137,111)=1`. It does not formalize Sage point counts, Tate's
theorem, or the source-to-Jacobian map.
