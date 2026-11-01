# Cubic trace-zero genus-two transfer gate at q=16

Research status: 27 October 2026, Conductor T-5676. The executable
[`audit.py`](audit.py) and [`certificate.json`](certificate.json) give an exact
census of all 8,190 normalized ordinary elliptic curves over `K=F_(16^3)`.
[`TraceZeroGate.lean`](TraceZeroGate.lean) checks the stated finite arithmetic.
This is a structural transfer result. A full genus-two Jacobian map and an
attack-versus-rho measurement have not been constructed for this route.

## General factor and subgroup criterion

Let `k=F_q`, `K=F_(q^3)`, and let `E0/k` have Frobenius trace `a`. Any
`E/K` that is `K`-isogenous to the base change of `E0` has

```
t_K = a^3 - 3qa,
P_Res(E)(X) = X^6 - t_K X^3 + q^3
            = (X^2-aX+q)
              (X^4+aX^3+(a^2-q)X^2+qaX+q^2).
```

The first factor is `E0`. The second is the trace-zero abelian surface
`B = ker(Res_(K/k)(E0) -> E0)^0`. This decomposition and trace-zero varieties
are established theory, not a new attack method. For a point `P in E0(K)`
of prime order `r` with `r` not dividing `#E0(k)`, the evaluated map

```
                     P |-> P - Frob_q(P)
```

lands in `B(k)`. It is injective on the order-`r` subgroup: if the image
vanishes, `P` is fixed by Frobenius and hence lies in `E0(k)`. The trace
`P+Frob_q(P)+Frob_q^2(P)` also vanishes because it lies in `E0(k)[r]`.
For a source that is only `K`-isogenous to `E0`, an explicit source-to-`E0`
isogeny is additionally required. Existence of an isogeny alone does not
give its evaluation cost or guarantee that an arbitrary chosen isogeny
preserves `r`.

If the quartic factor is simple and `q-a^2>0` has every prime divisor
congruent to `1 mod 3`, [Howe–Nart–Ritzenthaler, Theorem 1.2 and
Table 1.2](https://www.numdam.org/item/10.5802/aif.2430.pdf) imply its
isogeny class contains **no genus-two Jacobian**. Indeed its quartic
coefficient is `b=a^2-q`, so the theorem's exceptional equality
`a^2-b=q` holds identically. This is a general necessary obstruction to
genus-two Jacobian transfer for this subfield-isogenous cubic family, not
an exclusion of genus-three or other attacks.

## Exact q16 classification and bound

Use `K=F_2[w]/(w^12+w^7+w^6+w^5+w^3+w+1)` and the unique `k=F_16`.
Normalized ordinary curves are `E_(u,b): y^2+xy=x^3+u*x^2+b`, with
`b != 0` and `u` in the two absolute-trace twist classes. The census uses
`u=0,32` to count each `K`-isomorphism class once; an independent
trace-dual Walsh transform returns all 128 possible odd traces. It factors
the Weil-restriction polynomial over `Q` for each order. Exactly 7,584
models have an irreducible degree-six polynomial, so their simple
threefold restriction admits no nonzero algebraic map to any Jacobian of
genus at most two over `k`. Exactly 606 models occupy the eight split
orders below. In each split order the quartic factor is irreducible.

For the displayed exact representative equations, `u` and `b` are
12-bit polynomial-basis masks and both lie in `k`. `G` and its nonzero
trace-zero image `H=G-Frob_16(G)` are coordinates in the same `K` basis.
The certificate also stores `Q=7G`, its image, every `(twist,b)` model
in the order class, and checks `rG=rH=0`, `H(Q)=7H(G)`, and trace zero.

| `#E(K)` | models | `r` | `u,b` | `G` | `H` | `#E0(k)` / `#B(k)` | genus-two J? |
|---:|---:|---:|:---|:---|:---|:---|:---|
| 3980 | 28 | 199 | `0,373` | `(2940,3458)` | `(1169,1561)` | `20 / 199` | **no** |
| 3982 | 52 | 181 | `372,372` | `(1197,3477)` | `(936,3980)` | `22 / 181` | yes |
| 4050 | 119 | 5 | `372,1` | `(1939,366)` | `(3074,3962)` | `18 / 225` | yes |
| 4090 | 104 | 409 | `372,3362` | `(2004,472)` | `(2847,2849)` | `10 / 409` | yes |
| 4104 | 104 | 19 | `0,3362` | `(2395,3860)` | `(815,3362)` | `24 / 171` | yes |
| 4144 | 119 | 37 | `0,1` | `(508,579)` | `(971,1344)` | `16 / 259` | yes |
| 4212 | 52 | 13 | `0,372` | `(3385,3363)` | `(1429,4058)` | `12 / 351` | yes |
| 4214 | 28 | 43 | `372,373` | `(1272,361)` | `(1671,365)` | `14 / 301` | **no** |

For the two excluded orders, `a=±3`, `b=a^2-16=-7`, and `7=1 mod 3`.
The HNR theorem independently rules out a genus-two Jacobian in either
quartic isogeny class. The local LMFDB `public.av_fq_isog` snapshot of
19 October 2026 records `has_jacobian=-1` for
[`2.16.ad_ah`](https://www.lmfdb.org/Variety/Abelian/Fq/2/16/ad_ah) and
[`2.16.d_ah`](https://www.lmfdb.org/Variety/Abelian/Fq/2/16/d_ah), and
`has_jacobian=1` for the other six labels in the certificate. The HNR
criterion, rather than a database flag alone, supplies the mathematical
exclusion and existence result. The query used the verified local cache of
the 2,945,722-row snapshot extracted from `public.av_fq_isog` on
19 October 2026 at 16:45 UTC.

Thus, for the **largest prime-order subgroup of each normalized ordinary
source**, at most **550 of 8,190 models** can transfer by an algebraic
homomorphism to a genus-at-most-two Jacobian over `F_16`. The other 7,640
are excluded: 7,584 by simple-threefold dimension and 56 by the genus-two
Jacobian obstruction plus `r` not dividing the elliptic factor order.
This is a rigorous bound on that target-genus route, independent of a
chosen GHS parameter or bounded isogeny walk. It is not a bound on all
GHS vulnerabilities, because genus-three and higher targets remain open.

Of the 550 remaining, 379 models (orders 3982, 4090, 4104, 4144) have
`gcd(r,P_B'(1))=1`. The existing simple-root/Tate argument then proves
that *some* isogeny to a genus-two Jacobian preserves their selected
`r` subgroup. It gives no coordinates or cost. The other 171 models
(orders 4050 and 4212) have a repeated root at one modulo `r`; that
sufficient argument is inconclusive. For a base-defined representative
of every order, the evaluated `P-Frob_16(P)` map already preserves the
subgroup inside the trace-zero surface, including those 171 models.

## Prior work and novelty labels

[Trace-zero varieties and Frobenius maps](https://cacr.uwaterloo.ca/techreports/2009/cacr2009-30.pdf),
[cubic Weil-restriction isogeny transfer](https://arxiv.org/abs/2012.07173),
and the [complete genus-two Jacobian-class criterion](https://www.numdam.org/item/10.5802/aif.2430.pdf)
precede this audit. The eight abelian-surface isogeny classes were already
in LMFDB's 19 October snapshot. This audit adds a project-local exact
8,190-model classification, the 550-model genus-two ceiling, and checked
Frobenius subgroup images. It does not establish a new geometric family,
new attack method, or a vulnerable source curve. Exact-equation publication
novelty has not been established.

The next decisive experiment is to construct an evaluated isogeny from
the trace-zero surface to a named genus-two Jacobian for order 4090
(`r=409`) or 3982 (`r=181`), verify its action on the displayed `G,Q`,
then collect and solve divisor relations with full setup, target descent,
and matched source-group rho costs. No total GHS-versus-rho cost for this
route has yet been measured.

The route has no established asymptotic advantage: the trace-zero group
has order on the scale of `q^2`, so its selected prime subgroup has
`r <= #B(k)` and source rho takes `O(sqrt(r))` group steps, at most on
the scale of `q`. The known genus-two index-calculus bound is
`~O(q^(2-2/2)) = ~O(q)` under its stated hypotheses
([Diem's plane-curve theorem](https://arxiv.org/abs/math/0606607)).
Only a measured constant-factor improvement could make one of these
specific instances a useful attack lead; this audit does not demonstrate it.

Reproduce with `sage -python research/ghs_q16_tracezero_genus2_20261028/audit.py`
and `lean research/ghs_q16_tracezero_genus2_20261028/TraceZeroGate.lean`.
Lean checks the abstract injection of `P-Frob(P)` on an `r`-torsion
subgroup with trivial Frobenius-fixed `r`-torsion, scalar preservation,
the census arithmetic, HNR exception arithmetic, and factor orders. It
does not formalize the external classification, Tate theorem,
finite-field enumeration, or coordinate identities.
