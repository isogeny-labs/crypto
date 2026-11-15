# Trace-one n=7 genus-eight GHS complement: exact family, prior-art result

15 November 2026. [Protocol and amendments](PROTOCOL.md), [general count and disjointness proof](THEOREM.md), [finite screen](screen.json), and [public Magma map receipt](map8_receipt.json). This audit reconstructed Teske's published n=7 genus-eight branch, counted its nonzero base-field component exactly, and separated it from the published W1 coefficient slice by relative trace. The result is a precise rediscovery and a reproducible exact-curve witness, **not a new vulnerability family**. [pcc:unverified]

## Family and rigorous boundary

For `k=F_q`, `K=F_(q^7)`, `q=2^s`, and the two cubic Frobenius kernels `Vi=ker pi(sigma)`, fix absolute trace-one `a` in `E(a,b): y²+xy=x³+a x²+b`. The family is `b=v+c`, `v∈V1×∪V2×`, `c∈k×`. Every such coefficient has `Tr_(K/k)(b)=c≠0` and a direct classical hyperelliptic GHS map of genus eight. The exact number of ordinary isomorphism classes in this coefficient slice is

`N8(q)=2(q³−1)(q−1)` out of `q⁷−1` trace-one classes, or density `2(q³−1)(q−1)/(q⁷−1)=O(q⁻³)`.

The [proof](THEOREM.md) shows that every Menezes–Teske W1 coefficient has relative trace zero, so this genus-eight locus is **disjoint** from W1 for every q. Including the known genus-seven zero-trace arm gives exactly `2q(q³−1)` non-subfield classical magic-number-four classes: a rigorous ceiling of `2q(q³−1)/(q⁷−1)` on the share with direct classical genus at most eight. The trace test separates the two construction profiles. This ceiling measures direct eligibility, not the number of curves on which a full attack has lower total cost than rho. Some coefficients may define curves over a smaller absolute F2-subfield even though they lie outside k; those are separately sieved in finite examples.

## Exact examples and subgroup map

| Base q | Explicit field | Genus-eight coefficients | W1 overlap | Full absolute field coefficients | Group orders modulo eight |
|:---:|:---|---:|---:|---:|:---|
| 2 | `F2[z]/(z⁷+z+1)` | 14 | 0 | 14 | all 14 are 6 mod 8 |
| 4 | `F2[z]/(z¹⁴+z⁵+1)` | 378 | 0 | 364 | 126 are 2 mod 8; 252 are 6 mod 8 |

At q=2, `E: y²+xy=x³+x²+(z+1)` has **142=2·71** points. Magma 2.30-1 returned `WeilDescentGenus(E,F2,1)=8`, a hyperelliptic curve, and an evaluated Jacobian point for `P=(z⁷:z¹¹⁴:1)` of exact order 71. It checked that the image of `7P` equals seven times the image of P and recovered the scalar 7 by a bounded scan of the 71-element target subgroup. The full response records 0.35 seconds of Magma service time for this construction and check. It is a functional selected-subgroup transfer; the scalar scan is not an index-calculus solve or a rho comparison.

The q=4 screen independently enumerated the W1 product locus and confirmed zero overlap, then removed 14 coefficients in the proper F128 subfield. Its selected `a` has absolute trace one and mask 5107 in the fixed polynomial basis. The earlier pre-sieve screen is retained as [discovery evidence](screen_before_field_sieve.json).

## Prior work and novelty decision

[Maurer–Menezes–Teske, Theorem 5 and Remark 20](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/D4B264D082FF67354EE9968D9803102A/S1461157000000723a.pdf/analysis_of_the_ghs_weil_descent_attack_on_the_ecdlp_over_characteristic_two_finite_fields_of_composite_degree.pdf) classify classical GHS magic-number-four curves; [Galbraith–Hess–Smart (EUROCRYPT 2002)](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf) explicitly discuss n=7 genus-eight GHS and isogeny extension. [Teske's 2006 trapdoor construction](https://doi.org/10.1007/s00145-004-0328-3) and her [primary presentation](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) state the W0 plus one cubic-component selector; its genus-eight branch is also [implemented in this repository](../../docs/teske161-selector.md). Her approximate `2^93` coefficient count at q=`2^23` agrees with the exact component count here. [Menezes–Teske, Section 5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) define the distinct genus-fourteen W1 family. These sources make the basic family and isogeny-walk idea prior art. The exact equation above was not located in a targeted KB/web search, but absence from search results is not evidence that it has never appeared.

This audit adds an explicit W1 trace-separation argument, the `2(q³−1)(q−1)` count for the nonzero W0 slice, and an independently checked order-71 map for one exact equation. These do not establish priority for the family or its vulnerability. A new attack claim would require a source family outside these published constructions, a verified nonzero subgroup route, and complete measured cost below rho on the same subgroup. A universal bound on all GHS-vulnerable curves remains open. [pcc:unverified]
