# A wider cubic Jacobian isogeny-class family over GF(16)

26 October 2026. Conductor T-5382. This report uses the frozen
[protocol](PROTOCOL.md), [catalog snapshot](catalog_snapshot.json),
[Sage audit](audit.py), [certificate](certificate.json), and
[Lean arithmetic checks](CatalogQ16.lean).

## Finding and scope

For `k=GF(16)` and `K=GF(4096)`, the complete normalized ordinary elliptic
cohort has 8190 `K`-isomorphism classes and 128 distinct group orders.
The LMFDB finite-field abelian-variety catalog records a genus-three
Jacobian in **126 of the 128 corresponding cubic Weil-restriction isogeny
classes**. These 126 classes contain **8142** of the 8190 normalized
elliptic sources. Of those, **5534** sources belong to orders with no
direct binary cubic genus-three GHS cover, even after arbitrary `K`-isogeny
walks. For **5302** of these 5534, the largest prime subgroup passes the
simple-root sufficient gate for an isogeny that is nonzero on that subgroup.

This is an exact finite **catalog-backed isogeny eligibility** bound, not
a new attack or a newly vulnerable family. The Jacobian models are already
in the LMFDB, the direct cubic cover and isogeny-walk ideas are in prior
work, and the isogeny furnished by Tate is not explicitly evaluated. The
new contribution within this project is the exhaustive cross-index join,
the exact partition, and a replayed class outside the earlier 300-source
quartic screen. Publication novelty has not been established.

**Outside the published cubic weak-set congruence.**
[Menezes--Teske, Section 7](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
define their non-subfield isogeny-walk set `X` by source order congruent to
`0` or `2` modulo `8`. Among our catalog-positive models with order
congruent to `4` or `6` modulo `8` are **4032 class-C non-subfield
models**; **3852** of them also pass the largest-prime simple-root gate.
There are another 16 class-A models in these residues, 12 of which pass
the gate. This residue partition was noticed after the initial frozen
catalog join and independently recomputed in the final audit. It gives
a finite, exact family beyond that published *direct-cover walk set*;
it does not show that the older authors claimed impossibility for other
transfers. Nor does it imply an advantage over rho.

| Category | normalized source models |
| :--- | ---: |
| Catalog Jacobian and direct-B order, simple-root gate | 2156 |
| Catalog Jacobian and direct-B order, gate fails for largest prime | 452 |
| Catalog Jacobian outside direct-B order, simple-root gate | **5302** |
| Catalog Jacobian outside direct-B order, gate fails for largest prime | 232 |
| Catalog status uncertain outside direct-B order | 48 |
| **Total** | **8190** |

The two catalog-uncertain orders are `3972` and `4222`, with 24 normalized
models each. LMFDB status `0` means **unknown**, so the 48 are not excluded
from having a genus-three Jacobian. The existence count is therefore at
least 8142, and at most 8190 within this specific genus-three
isogeny-class criterion. The exact count of curves for which a complete
transfer attack beats rho remains bounded only by `0..8190`; the table is
not an attack-cost measurement.

## Exact class outside the direct-cover closure

Take `K=GF(2^12)` in the Conway polynomial basis and let `w` be its
generator. The source is

```text
E/K : y^2 + x*y = x^3 + K.from_integer(32)*x^2 + K.from_integer(306).
#E(K) = 3974 = 2*1987; 1987 is prime.
```

An exact order-1987 source point has encoded coordinates
`P=(K.from_integer(307), K.from_integer(2825))`. Sage checked
`P != O` and `[1987]P=O`; primality makes its order exactly 1987.

The nonzero `b=K.from_integer(306)` has nonzero relative trace to `k`,
and the complete source census has no class-B model of order 3974.
Consequently this source is outside the entire direct genus-three
hyperelliptic-cover isogeny closure. Its group order is `6` modulo `8`,
also outside the Menezes--Teske set `X`. Its selected subgroup has order
`r=1987`; `t=4097-3974=123`, `P(X)=X^6-123X^3+4096`, and
`gcd(r,P'(1))=gcd(1987,-363)=1`.

The [LMFDB class 3.16.a_a_aet](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_aet)
already supplies the following hyperelliptic model over `k=GF(16)`;
`a` satisfies `a^4+a+1=0`:

```text
y^2 + (a^2*x^4 + a*x)*y
  = x^8 + x^3 + a*x^2 + (a^2+a+1)*x + a^3+a+1.
```

Sage independently gives genus 3 and point counts `(17,257,3728)`
over `(k,k^2,k^3)`. These determine exactly `P(X)` above. Tate's
finite-field isogeny theorem gives a `k`-isogeny
`Res_{K/k}(E) -> Jac(C)`. The simple-root proposition in the
[earlier report](../ghs_cubic_tate_bridge_20261026/REPORT.md)
shows that **some** such isogeny is nonzero on the rational order-1987
subgroup. This is an existence result. An explicit correspondence and
subgroup image have not been constructed.

This class contributes **12** normalized elliptic models, separate from
the three earlier highlighted classes of 96, 96, and 108 models. The
equation of the Jacobian curve is cataloged prior art. The particular
elliptic equation has not been checked exhaustively for prior appearance;
its status is a new **exact source in this project**, not a claimed new
eligible family or method.

## Rigorous bound and reproducibility

For any genus-three `C/k` with the three displayed point-count
conditions, Newton identities force the Weil polynomial
`X^6-tX^3+q^3`. Thus its Jacobian is `k`-isogenous to the cubic Weil
restriction of every `E/K` with that trace. For a prime `r | #E(K)`
with `gcd(r,3(q^3-1))=1`, the cited integral Tate/Hensel argument
guarantees a subgroup-surviving isogeny. This is a sufficient gate and
does not bound the degree or computation time of that isogeny.

The prior complete source census and independent Walsh replay determine
the 128 orders and direct-B closure. The frozen LMFDB `av_fq_isog`
extraction, completed 2026-10-19, supplies each class label, polynomial,
and Jacobian status. `sage -python audit.py` joins the 8190 rows,
checks all count identities and the selected prime gates, and recounts
the displayed hyperelliptic curve. `lean CatalogQ16.lean` checks the
integer identities and gcd. It does not formalize Tate's theorem,
catalog correctness, or point counting. The snapshot and certificate
retain their input hashes and all 128 order classifications.

Only the displayed class's curve equation was independently replayed in
Sage. The other 125 positive classes are supported by frozen catalog
status and labels; their equations and point counts were not each
recomputed. A bulk attempt to fetch all 126 live model pages encountered
intermittent incomplete responses and was stopped without changing the
snapshot. This is a reproducibility limit for the wider count, not a
negative result about those models.

The method and its limitations are consistent with
[Gaudry--Hess--Smart](https://www.kantmath.de/publications/papers/weil_descent.pdf),
[Menezes--Teske, Section 7](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf),
and [Hess's generalized GHS](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf).
The [LMFDB catalog](https://www.lmfdb.org/Variety/Abelian/Fq/)
is the source of preexisting Jacobian models. A class-level isogeny
certificate alone gives no evaluated transfer, target descent, relation
collection, linear algebra, or same-target rho comparison.
