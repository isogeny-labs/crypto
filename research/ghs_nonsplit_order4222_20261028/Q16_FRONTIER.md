# Exact q16 frontier beyond three direct genus-three V4 screens

## Result and scope

The retained replay exhausts all 8,190 normalized ordinary elliptic models
`E_(a,b): Y^2+XY=X^3+aX^2+b` over `K=F_(16^3)`, with `b != 0` and one `a`
from each absolute-trace twist class. Triangle, Hess, and nonsplit quadratic
pole direct-cover screens contain 6,158 models. Their full `K`-isogeny closure
contains 8,124 models: **exactly 66 models in three source-order classes lie
outside that closure**. The finite exception is a sharper bound on these
specified direct-cover routes than the raw 2,032-model screen complement.

The 66 models are *not* proved resistant to GHS or other cover attacks.
LMFDB already stores genus-three hyperelliptic Jacobians in each of their
three isogeny classes. The simple-root criterion from the existing cubic
Tate-bridge report proves that some `k=F_16` isogeny from each Weil
restriction to such a Jacobian preserves the selected prime subgroup. Its
coordinates, evaluation cost, relation decomposition, and same-target rho
comparison remain unknown. The isogeny-transfer method itself is prior work
([Tian](https://arxiv.org/abs/2012.07173)). These are a tightly bounded
**isogeny-only candidate cohort** relative to the three screens, not a newly
demonstrated vulnerable curve family or a globally new geometric family.

## Exact equations and candidate subgroups

Use `K=F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1)`. Every decimal coefficient or
point coordinate is its polynomial-basis mask. The representative equation
in each row is `Y^2+XY=X^3+32X^2+b`. Sage checked its cardinality with the
native elliptic routine independently of the exhaustive Walsh computation.
The script deterministically chose `G` and checked `Q=7G`, `rG=rQ=0`, and
`G,Q != 0`.

| `#E(K)` | `b` | models outside closure | selected `r` | `G` | `Q=7G` | LMFDB class |
|---:|---:|---:|---:|:---|:---|:---|
| 3974 | 306 | 12 | 1987 | `(307,2825)` | `(1678,342)` | [`3.16.a_a_aet`](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aet) |
| 3978 | 11 | 30 | 17 | `(2340,4083)` | `(2181,2262)` | [`3.16.a_a_aep`](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aep) |
| 4178 | 31 | 24 | 2089 | `(3408,895)` | `(1008,2898)` | [`3.16.a_a_dd`](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_dd) |

For each selected `r`, `gcd(r,3(1-16^3))=1`; the simple-root survival
proposition therefore applies to some isogeny. The exact `b` masks of all
66 models are in `frontier_q16.json`. Order 3978 and its hyperelliptic
Jacobian class already appear in the earlier binary cubic isogeny screen.
Its selected prime subgroup has only 17 elements, so the two order-`2r`
classes are the more useful map-construction targets.

## Bound and proof

For each nonzero `b in K`, the triangle condition chooses exactly one twist,
giving `4095` models. Products from `ker Tr_(K/k)` give `2295` Hess-screen
models; norms from `ker Tr_(L/F_(16^2))`, `L=F_(16^6)`, give `2055` nonsplit
models. Their set union has `6158` models, independently agreeing with the
all-`q` intersection proof in the companion nonsplit-family report. The
script enumerates all three sets with exact finite-field arithmetic.

The trace-of-inverse identity gives every source order:

```
#E_(a,b)(K) = 16^3+1 + (-1)^Tr(a)
              * sum_(u in K*) (-1)^Tr(u^(-1)+b*u).
```

An exact trace-dual Walsh transform evaluates those 4,095 sums. Among all
8,190 models there are 128 source orders. The three-screen union attains 125
of them. The missing orders are precisely `3974,3978,4178`, with `12,30,24`
models respectively. By the finite-field elliptic isogeny theorem, two
curves over `K` are `K`-isogenous exactly when their source orders agree.
Consequently every model in the 125 attained orders has an isogeny to one of
the three direct-screen models, and none of the remaining 66 does. This is
an exhaustive finite proof of the stated exclusion, without a bounded walk
radius assumption. The computation saves its full 66-model witness set and
checks one model per missing order with Sage's native cardinality routine.
`FrontierQ16Gate.lean` kernel-checks the cohort arithmetic, source factors,
three simple-root gcd gates, and finite trial division for the selected
primes. It does not formalize the field enumeration or isogeny theorem.

For each missing order `N`, let `t=16^3+1-N`. The Weil restriction has
Frobenius polynomial `X^6-tX^3+16^3`. LMFDB's live `av_fq_isog` rows for
the three corresponding classes have `has_jacobian=1` and respectively
`4,20,36` stored hyperelliptic models. Those existing models establish a
genus-three Jacobian in each class. Combining the matching polynomial with
the simple-root condition proves **existence** of a subgroup-preserving
isogeny. It does not construct or time that isogeny.

## Prior work and next gate

The per-order check is in `prior_work_frontier_q16.json`. The [Murai et al.
2022 abstract](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf) reports a
complete even-characteristic minimal-genus cover classification, but its
full case table was inaccessible in this review. The [Tian
paper](https://arxiv.org/abs/2012.07173) establishes the general
Weil-restriction-to-Jacobian isogeny approach; its displayed coordinates use
odd-characteristic two-torsion formulas that do not directly evaluate these
binary maps. Exact-label web queries found no indexed discussion of this
particular 66-model partition; unsuccessful queries do not establish
publication novelty. LMFDB's positive Jacobian rows are clear prior catalog
entries, and the order-3978 class appears in the earlier repository screen.

The next experiment should build and evaluate a rational subgroup map for
the order-3974 and order-4178 representatives, verify `Phi(G)` has order
`1987` or `2089` and `Phi(Q)=7Phi(G)`, then run a complete hyperelliptic
Jacobian index calculus and matched source-group rho on the same backend.
Until those receipts exist, attack advantage is unmeasured.

## Complete Magma GHS-parameter genus sweep on representatives

The [Magma Weil-descent documentation](https://magma.maths.usyd.edu.au/magma/handbook/text/1561)
defines a standard descent parameter `c in K*` and exposes
`WeilDescentGenus(E,k,c)`. For each of the three exact representative
equations above, `magma_genus_frontier_q16.py` asserted that its Conway root
`w` has order 4,095 and exhausted `c=w^i` for every `0 <= i < 4095` in
Magma 2.30-1. All three representatives gave the same complete histogram:

| Outcome for one source | Parameter count |
|:---|---:|
| Genus 4 | 30 |
| Genus 6 | 450 |
| Genus 7 | 3,585 |
| Intrinsic rejection | 30 |
| Genus 3 | 0 |

The four nonzero counts sum to 4,095 for each source. This eliminates a
genus-three output from **that documented parameterized GHS intrinsic on
those three equations**. The first exploratory receipt was mathematically
complete but the local output parser expected three lines instead of Magma's
six wrapped lines; the parser was corrected and the whole sweep was rerun.
The retained Magma code and final untruncated server receipt are
`magma_genus_frontier_q16.magma` and
`magma_genus_frontier_q16_receipt.json`. The sweep does not quantify all
covers, all 66 source equations, or the separate isogeny-to-Jacobian route.

Reproduce: `sage -python research/ghs_nonsplit_order4222_20261028/frontier_q16.py`
and `lean research/ghs_nonsplit_order4222_20261028/FrontierQ16Gate.lean`.
Then run `python3 research/ghs_nonsplit_order4222_20261028/magma_genus_frontier_q16.py`.
The compact rendered report is `output/pdf/GHS_Q16_DIRECT_SCREEN_FRONTIER.pdf`.
