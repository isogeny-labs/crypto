# The second trace-26 Frobenius orbit has an explicit genus-five cover route

16 November 2026. **Finding:** the five trace-`+26` source models that had no hit in the saved degree-seven quadratic-square cover search are degree-two isogenous, pair by pair, to the five models with a verified degree-seven cover. Their negative quadratic twists have the same connection. One of the second-orbit negative models now has a checked order-`185302017921931` subgroup map, an exact degree-fourteen cover, and a same-target transported full GHS-versus-rho cost. The native rho run on that second-orbit curve is in progress. All claims await independent PCC admission. [pcc:unverified]

## Exact curves and maps

Set `K=F3[a]/(a⁵+2a+1)` and `L=F_(3^35)` using the exact modulus and embedding saved in [`construct.json`](construct.json). The two normalized negative curves are

```text
E30⁻/K:  v²=u³+u²+2(a+a³),
E167⁻/K: V²=U³+U²+(a⁴+2a+1).
```

Both have `#E(K)=270`; over L both have `#E(L)=270·185302017921931`. Their j-invariants differ, so these are distinct K-isomorphism classes. The positive twists, with order 218 over K, are `E30⁺:v²=u³+2u²+a+a³` and `E167⁺:V²=U³+2U²+2a⁴+a+2`. The latter is the **mask-167 equation already enumerated** in the earlier ten-model trace-26 census, not a newly discovered equation.

Sage constructed the unique K-rational degree-two isogeny from E30⁻ to an unnormalized E2 and a reverse degree-two isogeny. Translating the E2 x-coordinate by `s=2a³+2a²+1` gives E167⁻. The fixed reverse rational map `ψ:E167⁻→E30⁻` can be evaluated without isogeny enumeration: set `W=U+s` and `d=−a³−a²−1`, then

```text
ψx = (W²+dW+a³+a²)/(W+d) − 1,
ψy = −V·(W²+(a³+a²+1)W+a⁴+a³−a²+a−1)
        /(W²+(a³+a²+1)W+a⁴−a³+a−1).
```

The independent [`audit_manual.sage`](audit_manual.sage) checks the rational-function identities on both elliptic curves, E167⁻ group order over K, the coordinate shift, and the two point images without calling Sage's isogeny constructor. The saved `P167,Q167` have exact prime order `r=185302017921931`, satisfy `Q167=12345P167`, and map to the **exact** `P,Q` in the earlier GHS and rho receipts. The saved rho collision coefficients also give an equality directly on `P167,Q167`. The forward and reverse maps compose to `[2]` up to the checked sign; since r is odd, transfer is injective on the selected subgroup. [`Isogenous2Arithmetic.lean`](Isogenous2Arithmetic.lean) checks the finite order/coprimality arithmetic and a generic composition-injectivity lemma, with 10 theorems and zero trusted exceptions. The elliptic geometry is verified by Sage identities, not formalized in Lean.

The original negative carrier is

`Y⁻/F3: y²=2(x¹¹+x⁹+2x⁸+x⁷+2x⁵+x³+x²+x+1)`.

Its saved degree-seven map `Y⁻_K→E30⁻` has `u=2A/B, v=yC/B²`, with exact A,B,C in the [mask-30 cover receipt](../ghs_odd5_carrier_20261115/degree7_quadratic_square_30.json). Composing it with the forward degree-two isogeny and the x-translation gives an **explicit separable degree-fourteen map** `Y⁻_K→E167⁻`. [`compose_cover.json`](compose_cover.json) contains the reduced numerator/denominator coefficient vectors; [`audit_cover.sage`](audit_cover.sage) independently reconstructs those vectors and verifies the full identity `2f(x)·v(x)²=u(x)³+u(x)²+(a⁴+2a+1)` and x-map degree fourteen. Pullback plus the established five-term trace transfers the selected source subgroup into `J(Y⁻)(F_(3^7))` through the evaluated reverse isogeny.

## Complete paired-orbit certificate and bounds

The five pairings are:

| Covered orbit | New isogeny-assisted orbit |
|---:|---:|
| 30 | 167 |
| 194 | 182 |
| 103 | 230 |
| 139 | 3 |
| 233 | 27 |

[`orbit_bridge.sage`](orbit_bridge.sage) checked the unique degree-two isogeny and codomain isomorphism for every pair in both twist signs: **ten successful cells**. Independent [`audit_orbit.sage`](audit_orbit.sage) checked each pair's Frobenius relation, group order, distinct j-invariants and `Φ₂(j_left,j_right)=0` without isogeny enumeration. The five rows in each column are Frobenius conjugates, so the construction has one independent pair per twist sign. The degree-two map and earlier degree-seven cover base-change through every extension of K; the theorem-level subgroup-injection condition is that the chosen odd prime avoids the fixed cover-transfer kernel.

At K, the positive order is `218≡2 (mod 4)` and the negative order is `270≡2 (mod 4)`. Each elliptic curve therefore has exactly one K-rational point of order two, hence exactly one K-rational degree-two isogeny. Because the paired j-invariants differ and both endpoints have that property, the **rational degree-two closure of each listed isomorphism class consists of exactly its two paired classes**. The same conclusion holds for this s=7 negative line, whose full group order is `270r≡2 (mod 4)`. This is a degree-two graph bound for these exact fields, not a bound on all isogeny degrees or all GHS-vulnerable curves.

The prior fixed-carrier screen excludes direct maps of degrees two through six for the ten positive source models; simultaneous twisting carries that bound to the negative pairs. This study constructs degree fourteen for E167⁻, giving a **fixed-pair direct-cover degree window [7,14]**. The earlier positive-orbit degree-seven cover and genus-five dimension floor extend by isogeny to E167; no nonzero Jacobian transfer from its simple five-dimensional Weil restriction can land in a genus below five. These are algebraic bounds, not a runtime bound for all curves in the isogeny class.

## Exact target cost and evidence limit

The attacker-facing [`preprocess.sage`](preprocess.sage) starts with only E167⁻ and the public `P167,Q167`, translates to E2 and evaluates ψ. It took **0.045958 in-process CPU seconds**. The resulting P,Q exactly match the previously audited full GHS target and Frobenius-aware rho target. [`cost.json`](cost.json) binds all receipt hashes and charges that new preprocessing to both complete saved suffixes:

| Same P167,Q167 target route | Stitched CPU seconds | Status |
|---|---:|---|
| Supplied-cover GHS, including reverse-isogeny preprocessing | 231.171089 | New preprocessing measurement plus previously audited complete GHS suffix |
| Frobenius-aware rho after the same preprocessing | 3268.080314 | New preprocessing measurement plus previously audited complete rho suffix |
| Fixed-carrier targeted cover search plus GHS | 420.401089 | Adds 189.23 external user+system CPU seconds; mixed timing conventions |

The supplied-cover ratio is **14.1371** and the search-charged ratio is **7.7737** for this transported exact target. This is a stitched same-target comparison: the full GHS and rho suffixes were measured previously on the exact P,Q reached by ψ and were not rerun after presenting the source as E167⁻. It excludes carrier discovery, other screened curves and failed search arms. The independent native rho run on E167⁻ uses a separate checkpoint and will be reported independently once terminal; it is not pooled with the transported baseline.

## Prior work and novelty

[Joux–Vitse 2012](https://www.iacr.org/archive/eurocrypt2012/72370010/72370010.pdf) already describes isogeny-assisted cover and decomposition attacks. The [Iijima–Momose–Chao 2015](https://eprint.iacr.org/2015/805.pdf) direct `(2,…,2)` construction does not classify every elliptic cover. The finite-field isogeny class of E167⁻ is already catalogued in LMFDB as `1.243.ba` (`q=243`, order 270); the original local ten-model census already contained its positive twist mask 167 and recorded no degree-seven hit in the tested quadratic-square shapes. Focused searches of those primary sources, the local research corpus and the exact equations found no published instance of this **specific paired-orbit degree-two bridge or degree-fourteen map**, but that is a bounded novelty check, not proof of worldwide priority.

The result newly certifies vulnerability of a **previously enumerated exact orbit under the measured, isogeny-assisted cost model**. It expands the covered exact models in this project from one trace-26 Frobenius orbit to both. It does not establish a new attack method, a parameter-dimensional source family, or a claim that every curve in the trace-26 isogeny class is weak. [pcc:unverified]
