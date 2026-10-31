# A degree-bounded isogeny screen for residual cubic GHS classes

27 October 2026. The [Sage certificate](certificate.json), [independent exhaustive audit](audit.py), and [Lean checks](CubicDivisor.lean) establish a **necessary condition** for a particular genus-three transfer: one rational `(ell,ell,ell)` isogeny from `Res_{GF(4096)/GF(16)} E` to a genus-three Jacobian, for odd primes `ell` in `{3,5,7,11,13}`. This is a new exact finite classification inside the project's frozen census. It is an exclusion of a specified route, not a global classification of cover vulnerability.

## The bound

Let `k=GF(16)`, `K=GF(4096)`, and let `E/K` have `N=#E(K)`. Its cubic Weil restriction `A/k` has Frobenius characteristic polynomial

`F_N(T)=T^6-(4097-N)T^3+4096`.

For odd `ell`, an `F_16`-rational `(ell,ell,ell)` isogeny with maximal isotropic kernel requires a Frobenius-stable rank-three subgroup of `A[ell]`. The subgroup's three-dimensional `F_ell` vector space has a cubic characteristic polynomial, which must divide `F_N(T) mod ell`. Therefore **absence of any monic cubic divisor excludes every one-step rational `(ell,ell,ell)` isogeny**, regardless of the chosen target genus-three curve or polarization. Presence of a divisor says only that this elementary obstruction is absent; it gives neither an isotropic kernel nor an explicit isogeny.

The theorem uses the standard finite-field Tate module description of rational isogeny kernels. It applies to the specified separable odd-prime kernel route. It does not exclude higher prime degrees, composite kernels, inseparable maps, general correspondences, or GHS covers of genus at least four.

There is also a closed-form **infinite-family obstruction at `ell=3`**. For any cubic extension `F_{q³}/F_q` with `3∤q`, write `t=q³+1-#E(F_{q³})`. In `F_3[T]`, `F_E(T)=(T²-tT+q)^3`. A degree-three divisor exists exactly when the quadratic has a linear factor, equivalently when `t²-4q` is a square in `F_3` (zero included). In particular, for the binary subfields `q=4^m`, `q≡1 mod 3`, so **`t≡0 mod 3` excludes every rational one-step `(3,3,3)` isogeny from the cubic restriction**. For `q=16`, this is the simple order criterion `#E(GF(4096))≡2 mod 3`, explaining all 18 excluded classes in the first row of the table. This is a theorem about the direct 3-isogeny route, not a vulnerability claim or a classification of arbitrary covers.

Among the **44** simple source-order classes outside the previously certified genus-three Hess closure that have a catalog genus-three Jacobian in their isogeny class, the exact results are:

| `ell` | Classes excluded | Source models excluded | Classes passing the necessary condition |
|---:|---:|---:|---:|
| 3 | 18 | 888 | 26 |
| 5 | 21 | 1,059 | 23 |
| 7 | 22 | 1,062 | 22 |
| 11 | 23 | 1,155 | 21 |
| 13 | 22 | 1,224 | 22 |

Exactly **two** classes, covering **72** normalized source equations, fail the cubic-divisor test for *all five* screened primes:

| `#E(K)` | Trace `4097-N` | Source models | Largest prime factor | Catalog genus-three isogeny class | One exact source model |
|---:|---:|---:|---:|---|---|
| 4076 | 21 | 36 | 1019 | `3.16.a_a_av` | `y²+xy=x³+106` |
| 4118 | −21 | 36 | 71 | `3.16.a_a_v` | `y²+xy=x³+32x²+106` |

Codes `32` and `106` are polynomial-basis elements of `K=GF(2)[w]/(w¹²+w⁷+w⁶+w⁵+w³+w+1)`: code `32` means `w⁵`, while `106` means `w⁶+w⁵+w³+w`. The two exact equations above are representatives from the [frozen source census](../ghs_cubic_hess_closure_q16_20261027/certificate.json), not claims that these equations were absent from prior literature. The [LMFDB classes](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_av) and [second class](https://www.lmfdb.org/Variety/Abelian/Fq/3/16/a_a_v) are known isogeny classes; a catalog model is not an explicit transfer from either source elliptic curve.

For the four exact Hess genus-four controls, this screen excludes `ell=5` for `4006`, `4078`, and `4126`, and `ell=11` for `4054`. Other small primes pass the necessary condition. Their already constructed degree-four elliptic quotients and the nonzero source-to-Hess maps are [separate certificates](../ghs_q16_hess_elliptic_quotient_20261027/REPORT.md); this screen does not supply a bridge to a genus-three quartic.

If a degree-`ell³` isogeny is eventually constructed and the named source subgroup has prime order `r≠ell`, its restriction to that subgroup is automatically injective: its kernel has `ell`-power order and cannot meet an `r`-group nontrivially. For the four Hess controls, `ell=3` passes this *necessary* factor test and each named prime order `2003,2027,2039,2063` is coprime to three. Computing an isotropic rational kernel, a genus-three target, and the evaluation map remains necessary.

## Prior-work boundary and candidate ledger

The [per-class ledger](prior_work.json) records all 44 class labels, the checked catalog presence, and the level of publication checking. Published characteristic-two cubic cover work predates this screen: Shimura–Momose–Chao presented [part I in 2010](https://www.u-tokai.ac.jp/uploads/2021/02/about_pdf_annual-report2009_mokuroku-1.pdf) and [part II in 2011](https://scis.jp/2011/program/). [Murai–Shimura–Iijima–Chao (2022)](https://www.iwsec.org/scis/2022/_abst/1A3-2.pdf) describes a complete even-characteristic classification **under the isogeny condition**. A [2025 research poster](https://iss.iisec.ac.jp/sympo24/posters/M2_212poster24.pdf) says the classification without that condition was still partial. Those scope statements do not establish that any equation here is unpublished or vulnerable.

[Tian's prime-order cubic cover attack](https://arxiv.org/pdf/2012.07173) already develops rational `(ell,ell,ell)` isogenies from Weil restrictions to genus-three Jacobians and evaluates image points. Its displayed reconstruction starts from `y²=x³+a₄x+a₆`, three nonzero geometric 2-torsion points, and a six-element symplectic 2-torsion basis for the restriction. An ordinary elliptic curve in characteristic two has only one nonzero geometric 2-torsion point, so that **published 2-torsion reconstruction cannot be instantiated verbatim** for the present binary-field candidates. This does not rule out a characteristic-two adaptation using odd-level theta data. The present test checks which small odd-prime kernel degrees could even qualify for such an adaptation.

## Validation and remaining attack gate

`sage -python screen.py` factors each polynomial in `GF(ell)[T]`. `python3 audit.py` independently tests every monic cubic `T³+aT²+bT+c` by direct remainder arithmetic without Sage or Sympy, across all `44×5` class/prime cases. `sage -python verify_models.py` independently counts the two displayed source curves. `lean CubicDivisor.lean` kernel-checks the ten finite claims that orders `4076` and `4118` have no cubic divisor for the five primes. Lean does **not** formalize the algebraic theorem linking invariant kernels to polynomial factors; that mathematical argument is above.

The new result bounds one proposed route. To identify a new vulnerable family, a surviving candidate still needs a computed rational isotropic kernel or another explicit correspondence, an evaluable nonzero map on its named source subgroup, complete target-dependent decomposition and linear algebra, and a measured total cost below matched rho on that same subgroup. None of these follow from a cubic divisor or from matching Weil polynomials.
