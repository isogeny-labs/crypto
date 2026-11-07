# Scalar 3-torsion family for cubic binary transfer

27 October 2026. This is a **necessary-condition family** for a specific one-step rational `(3,3,3)` transfer from a cubic Weil restriction to a smooth genus-three Jacobian. It is not a vulnerability classification for GHS in general. The exact quotient, genus-three equation, point evaluation, and target-dependent attack cost have not been constructed for the new candidates.

## Literature and scope checked first

The [earlier Hess closure census](../ghs_cubic_hess_closure_q16_20261027/REPORT.md), [simple-restriction bound](../ghs_q16_simple_restriction_bound_20261027/REPORT.md), and [odd-isogeny screen](../ghs_q16_odd_isogeny_screen_20261027/REPORT.md) fix the source model convention and earlier claims. Crucially, this repository's [prior prime-characteristic cubic-transfer work](../cubic_extension_transfer_20261025/historical/extension-field-experiments/research_lead/README.md) **already derived the scalar-3/Jordan module screen, counted the 16 and 12 kernels, and evaluated 3-isogeny preprocessing**. The present result extends that established screen to the binary `q=16` census and supplies an exact all-`4^m` parameter count; the mechanism itself is a rediscovery. [Hess's generalized GHS paper](https://doi.org/10.1112/S146115700000108X) and [Menezes–Teske's characteristic-two analysis](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) predate this route. [Tian's cubic-cover paper](https://arxiv.org/pdf/2012.07173) develops rational odd-level `(ell,ell,ell)` transfers, but its displayed reconstruction uses the three nonzero geometric points of `E[2]`, which an ordinary characteristic-two elliptic curve does not have. The [2022 even-characteristic cover-classification abstract](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf) asserts classification under its isogeny condition. The [per-class ledger](prior_work.json) links every surviving class to the frozen LMFDB Jacobian record and records the limited exact-equation search.

## A family and an exact count

Let `q=4^m`, `m>=1`, `K=F_(q^3)`, and take each ordinary elliptic curve in normalized form

`E_(a,b): y²+xy=x³+a x²+b`, `b in K*`, `a in {0,a_tr1}`,

where `Tr_(K/F2)(a_tr1)=1`. Its 3-division polynomial is `psi_3(X)=X⁴+X³+b`. Define the **scalar-three family** by complete splitting of this polynomial over `K`. Its four roots are the four order-three cyclic subgroups. Frobenius acts on `E[3]` as a scalar `+I` or `-I` exactly when it fixes all four roots, since the kernel of `GL_2(F3) -> PGL_2(F3)` consists of scalars.

For every `q=4^m`, among the `q³-1` nonzero values of `b` the numbers of values with respectively `0,1,4` roots of `X⁴+X³+b` in `K` are

`q³/4`, `2(q³-1)/3`, `(q³-4)/12`.

Thus exactly `(q³-4)/6` of the `2(q³-1)` normalized ordinary models have scalar `E[3]`. For `q=16`, this is **682/8190** source models, and `341/4095` nonzero `b` values. This bounds the *direct one-step degree-27 route* by approximately one twelfth of the normalized models. It does **not** bound all GHS attacks, higher odd-degree transfers, or sources that first move through an elliptic isogeny.

### Exact bound after at most one elliptic 3-isogeny

Write `N=#E(K)` and `Q=|K|=q³`. Because `q=4^m`, `Q=1 mod 9`. A `K`-isogenous curve with scalar `+I` on 3-torsion has all nine 3-torsion points rational, so `N=0 mod 9`. Scalar `-I` means the quadratic twist has all nine points rational; its order is `2(Q+1)-N`, so `N=4 mod 9`. These congruences are therefore necessary even after any number of `K`-rational elliptic isogenies.

They are also **sufficient after at most one degree-three step**. If `N=0 mod 9`, either `E[3]` is already rational or the rational 3-primary group is cyclic and contains a point `P` of order nine. Quotient by `<3P>`. The image of `P` gives one rational 3-line on the quotient; the dual kernel gives a second distinct rational 3-line, because the dual sends the image of `P` to `3P`, which is nonzero. The dual kernel is rational since `mu_3` is split over `K`. Hence the quotient has full rational `E'[3]`. If `N=4 mod 9`, apply the same argument to the quadratic twist, whose order is divisible by nine, then twist the resulting isogeny back. This extends the [repository's earlier prime-characteristic `-I` argument](../cubic_extension_transfer_20261025/historical/extension-field-experiments/research_lead/README.md) to these binary fields and to the composite-order `+I` case. It remains a kernel-eligibility theorem, not a Jacobian or attack theorem.

In the frozen `q=16` census, **28 of all 128** order classes and **2530 of all 8190** normalized models satisfy the one-step congruence. Within the **44** simple outside-Hess catalog classes, precisely **8** satisfy it. They contain **651 normalized source models**; 177 are already scalar and the remaining 474 can reach a scalar model by one rational elliptic 3-isogeny. Thus 651/2643 is the exact size of this *preprocessed degree-27 search pool* within that residual class set, and all other 36 classes are excluded from this specific route.

Here is a self-contained count proof. Put `f(x)=x⁴+x³`. The fiber over zero is `{0,1}`. For two distinct roots `x,y` over the same nonzero `b`, set `z=x+y`. Algebra gives `xy=z²(z+1)`; after dividing the quadratic with roots `x,y` by `z²`, its equation is `T²+T+z+1=0`. Since `[K:F2]` is even, `Tr(1)=0`, so this is solvable exactly when `Tr(z)=0`. The other two roots satisfy, after scaling, `T²+T+z=0`, and hence are also in `K`. Every nonzero fiber therefore has size `0,1,4`. There are `q³/2-2` admissible values `z` after excluding `0,1`, each giving two ordered pairs. A four-root fiber has twelve ordered distinct pairs, giving `(q³-4)/12` such fibers. Counting nonzero input points gives `n_1+4n_4=q³-2`; then `n_1=2(q³-1)/3` and `n_0=q³/4`. An explicit parameterization takes `z not in {0,1}` with `Tr(z)=0`, solves `t²+t=z+1`, and sets `b=(zt)³(zt+1)`.

## Why the condition is necessary for the one-step degree-27 route

Put `A=Res_(K/F_q) E`. Geometrically, `A[3]` is the symplectic direct sum of three two-dimensional spaces. Since `q=4^m` is `1 mod 3`, base Frobenius `T` preserves the Weil pairing. After choosing compatible bases, its action is `(v0,v1,v2) -> (M v2,v0,v1)`, where `M` is `K`-Frobenius on `E[3]` and `M in SL_2(F3)`.

There are three cases, exhaustive in `SL_2(F3)`:

1. `M` has irreducible quadratic characteristic polynomial `X²+1`. The characteristic polynomial of `T` is `(X²+1)³`; every `T`-invariant subspace has even dimension. A rational three-dimensional kernel is impossible.
2. `M` is non-scalar Jordan with eigenvalue `lambda=+/-1`. The block-cyclic `T` has one Jordan block of size six. Its unique invariant three-dimensional subspace is `ker(T³-lambda I)`, the direct sum of the three eigenlines of `M`. This is a product Lagrangian. Quotienting the product principally polarized threefold by that product kernel yields another geometric product, not the principally polarized Jacobian of a smooth genus-three curve.
3. `M=lambda I`. Mixed rational Lagrangians are possible. Exact finite enumeration gives **16** rational Lagrangians, four product and twelve mixed, for either scalar sign. This is an opportunity for a genus-three quotient, not a proof that one of the quotients is a Jacobian or has an efficient evaluable map.

The [Sage enumeration](lagrangians.py) checks all 1120 geometric Lagrangians in `F3^6` for each representative matrix. The [Lean file](ScalarKernel.lean) kernel-checks one explicit mixed scalar-minus kernel: rank-three pivots, isotropy, Frobenius stability, trivial intersection with each separate elliptic factor, and the finite count arithmetic. Lean does not formalize the general module classification or the quotient-polarization theorem above.

## Exact `F_4096` outcome

The [Sage certificate](certificate.json) enumerates every nonzero `b`, joins the frozen 8190-model order census, checks the 3-torsion Frobenius matrices, and evaluates named elliptic 3-isogenies. The [independent bit-field audit](audit.py) recomputes the `0/1/4` fiber histogram, all named subgroup identities, and the eight image points using characteristic-two Vélu formulas over an explicit quadratic extension when required. Among the **44 simple outside-Hess classes with a cataloged genus-three Jacobian**, eight contain 177 direct scalar-three models and 651 models after at most one elliptic 3-isogeny:

| Source order | Direct / after one step | Largest prime | Direct representative `b` (`a=w⁵`) | Existing catalog class |
|---:|---:|---:|---:|---|
| 3978 | 6 / 30 | 17 | 508 | `3.16.a_a_aep` |
| 4014 | 36 / 108 | 223 | 107 | `3.16.a_a_adf` |
| 4018 | 27 / 81 | 41 | 284 | `3.16.a_a_adb` |
| 4054 | 24 / 120 | 2027 | 133 | `3.16.a_a_abr` |
| 4086 | 12 / 60 | 227 | 57 | `3.16.a_a_al` |
| 4126 | 36 / 108 | 2063 | 111 | `3.16.a_a_bd` |
| 4158 | 24 / 96 | 11 | 208 | `3.16.a_a_cj` |
| 4198 | 12 / 48 | 2099 | 116 | `3.16.a_a_dx` |

The most useful new focal equation is

`E: y²+xy=x³+w⁵x²+116` over `K=F2[w]/(w¹²+w⁷+w⁶+w⁵+w³+w+1)`.

Its order is `4198=2*2099`, with prime `2099`. The exact source points in this polynomial basis are `G=(117,3864)` and `Q=(1053,3821)=[7]G`; `[2099]G=0`. Its 3-division polynomial splits and `K`-Frobenius on `E[3]` is `-I`. The class `3.16.a_a_dx` was already cataloged, so this is an exact structural candidate within a known isogeny class, **not** a new vulnerable curve finding.

Two earlier Hess source controls have explicit, evaluated subgroup-preserving elliptic degree-three steps into this scalar family:

| Source `b`, order, prime `r` | Kernel `x` code | Scalar target `b` | Image of `G` | Image of `Q=[7]G` |
|---|---:|---:|---|---|
| `32`, 4054, 2027 | 1552 | 751 | `(1199,1632)` | `(3706,496)` |
| `63`, 4126, 2063 | 732 | 2825 | `(3332,2345)` | `(755,1786)` |

The complete rational maps and codomain normalizations are in the certificate. Because `r` is coprime to three, these degree-three isogenies are injective on the named `r`-subgroups; both image identities were also checked by the independent audit. These are **elliptic** isogenies only. The subsequent degree-27 quotient of `Res(E')`, its genus-three Jacobian model, and an evaluator from the source to it remain to be constructed.

The certificate additionally gives one explicit nonscalar-to-scalar degree-three isogeny, named source and image points, and full rational maps for **each of the eight classes**. The `(source b, kernel x, target b)` triples are `(11,3576,723)`, `(96,2520,3258)`, `(13,489,2130)`, `(32,1552,751)`, `(97,1729,57)`, `(63,732,2825)`, `(112,2570,2166)`, and `(37,2397,4039)` in table order. The independent audit evaluates each named generator and target under Vélu's kernel formula and checks the reported images exactly; Sage supplies a separate rational-map evaluation.

## Claim boundary and cost

The finite family count and the direct-route obstruction are rigorous within the stated model. The module screen and odd-level preprocessing had already been recorded in this repository for prime characteristic. The binary all-`4^m` count and eight-class intersection are new **local evidence** relative to the reviewed files; global publication novelty is unverified. The class labels and published transfer principles are prior work. A bounded query of the eight exact LMFDB class labels found no indexed equation-specific GHS publication; that is not evidence of publication absence. Exact-equation novelty is unresolved for every row. There is no new vulnerable family or attack method established here.

No full GHS cost or matched rho cost exists for these eight degree-27 candidates, so there is no measured speed ratio for them. The four *different* completed direct Hess genus-four transfers previously measured at [512–698 times slower than matched rho](../ghs_q16_simple_restriction_bound_20261027/REPORT.md); those measurements cannot be assigned to the degree-27 route. A vulnerability claim would require an evaluable nonzero source-to-Jacobian map, a complete target-dependent decomposition/relation solve with independent scalar replay, and a lower measured online cost than same-source rho under the same resource envelope.

## Reproduce

```sh
sage -python research/ghs_scalar3_frobenius_gate_20261027/classify.py
python3 research/ghs_scalar3_frobenius_gate_20261027/audit.py
sage -python research/ghs_scalar3_frobenius_gate_20261027/lagrangians.py
lean research/ghs_scalar3_frobenius_gate_20261027/ScalarKernel.lean
python3 research/ghs_scalar3_frobenius_gate_20261027/prior_work.py
```
