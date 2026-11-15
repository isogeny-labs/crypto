# Degree-five Jacobian carrier theorem and its exact scope

15 November 2026. All finite enumeration and algebraic claims in this note are [pcc:unverified] until the repository's independent PCC gate signs them. The scripts and immutable receipts below allow the arithmetic to be checked without relying on this prose.

## Statement

Put `k=F3`, `K=F3[a]/(a^5+2a+1)`, and `S={-26,-22,-14,-10,10,14,22,26}`. For each `t∈S`, there is an ordinary elliptic curve `E_t/K` and a genus-five hyperelliptic curve `Y_t/k` such that

`L_(Y_t/k)(T)=1-tT^5+3^5T^10=L_(Res_(K/k) E_t)(T)`.

Consequently `J(Y_t)` and `Res_(K/k) E_t` are isogenous over `k`. The degree-ten polynomial `X^10-tX^5+3^5` is irreducible over `Q`, so both abelian varieties are `k`-simple of dimension five. Any nonzero algebraic transfer from this restriction into a Jacobian requires genus at least five. These `Y_t` attain that dimension floor **as isogeny-class carriers**. The theorem does not specify an evaluable isogeny or its action on a chosen subgroup.

The full ordinary `K`-isomorphism-class census has `2(3^5−1)=484` classes. Exactly **100** have trace in `S`: counts `10,10,20,10,10,20,10,10` in ascending trace order. Every such class is in one of the eight isogeny classes above, and every source order is `244−t≡2 (mod 4)`. The 2013 Iijima--Momose--Chao direct degree-five genus-five `(2,...,2)` case has four rational branch roots and hence full rational 2-torsion, so its order is divisible by four. Thus the 100 classes are outside that **specific direct-cover family and its same-field isogeny closure**. This is an exact count of carrier-eligible ordinary source classes over `K`, not a count of cryptographically vulnerable classes and not a classification of all GHS routes.

The earlier exhaustive degree-five direct-cover census counted **122** ordinary `K`-classes with order `2 mod 4` and derived a direct `(2,...,2)` cover genus floor of **33** for that torsion type. The 100 classes here are a subset of those 122, so the isogeny-class carrier genus five is strictly below the established direct-cover floor on 100 exact classes. This comparison is between two constructions, not a contradiction or a faster DLP claim. [pcc:unverified]

For every `s≥1` coprime to five, the base changes to `k_s=F_(3^s)` and `K_s=F_(3^(5s))` retain a simple degree-five restriction and a genus-five Jacobian carrier over `k_s`. The source curve has coefficients in the proper subfield `K` when `s>1`; this matters for attack-cost comparisons.

## Exact equations

The coefficient `b` below is encoded as `b=sum_i d_i a^i`, where `d_i` are the base-three digits of its mask. Each source is `E_t:y²=x³+a2*x²+b` over `K`. Each target is `Y_t:y²=f_t(x)` over `k`, where `f_t=x^11+sum_(i=0)^10 c_i x^i` and `c_i` are the base-three digits of the target mask. All target masks are the first retained mask for that trace in the frozen enumeration.

| t | #E_t(K) | a2 | b mask | Y mask | # ordinary K classes | irreducible mod |
|---:|---:|---:|---:|---:|---:|---:|
| -26 | 270 | 1 | 6 | 28967 | 10 | 59 |
| -22 | 266 | 2 | 37 | 28869 | 10 | 59 |
| -14 | 258 | 1 | 11 | 9735 | 20 | 19 |
| -10 | 254 | 2 | 24 | 2801 | 10 | 19 |
| +10 | 234 | 1 | 12 | 2872 | 10 | 19 |
| +14 | 230 | 2 | 19 | 10172 | 20 | 19 |
| +22 | 222 | 1 | 68 | 32280 | 10 | 59 |
| +26 | 218 | 2 | 3 | 35518 | 10 | 59 |

For example, `E_10:y²=x³+x²+(a²+a)` and `Y_10:y²=x^11+x^7+2x^5+2x^4+x³+x²+1`. The frozen Magma 2.30-1 call checked the whole target Euler factor `1−10T^5+243T^10`, genus five and both group orders 234 (`representative.json`).

The two strongest new catalog witnesses are fully explicit:

- `t=-26`: `E:y²=x³+x²+2a` over `K`, and `Y:y²=x^11+x^9+x^8+x^7+2x^5+x³+2x²+x+2` over `F3`. Both have 270 rational points in the corresponding elliptic/Jacobian group, and `L_Y(T)=1+26T^5+243T^10`.
- `t=+26`: `E:y²=x³+2x²+a` over `K`, and `Y:y²=x^11+x^9+2x^8+x^7+2x^5+x³+x²+x+1` over `F3`. Both groups have order 218, and `L_Y(T)=1−26T^5+243T^10`.

The frozen Magma 2.30-1 `lmfdb_unknown.json` receipt checked smoothness, genus, source orders, **whole Euler factors** and Jacobian orders for both, with success and untruncated output. This independent check supports the point-count derivation for the two labels that were uncertain in the LMFDB snapshot. [pcc:unverified]

## Proof and computational certificates

1. The finite screen enumerates all `3^11=177147` monic degree-eleven polynomials over `F3`. The odd degree gives one rational point at infinity. It checks squarefreeness and the exact character-sum point counts `N_m=3^m+1` for `m=1,2,3,4`, and `N_5=244−5t`. Newton identities set the first five coefficients of the zeta numerator to `0,0,0,0,−t`; the genus-five functional equation forces the remaining five, yielding `1−tT^5+3^5T^10`. `audit_sage.py` independently replayed every polynomial and all 288 retained rows with different finite-field arithmetic (`audit.json`). `floor_cert.json` gives Rabin irreducibility certificates for each `X^10−tX^5+243` by reduction modulo 19 or 59. Tate's finite-field isogeny theorem supplies the abstract isogeny. Irreducibility makes the restriction simple; a nonzero image in `J(C)` then forces `dim J(C)≥5`.
2. In characteristic three, an ordinary elliptic curve has nonzero `x²` coefficient after completing the square and translating `x` to remove the linear term. Scaling `x=u²x'`, `y=u³y'` puts that coefficient in one of the two square classes `1,2`; the remaining constant is nonzero. The residual normal-form automorphisms do not identify different `(a2,b)` pairs, yielding exactly 484 classes. `class_count.py` enumerated every pair with Sage elliptic cardinality. Frozen `replay_class_count.py` independently counted all 484 orders by 117612 quadratic-residue evaluations and matched every row, trace histogram and selected row (`class_replay.json`). The eight selected source orders were also directly recounted in the original census.
3. The direct-cover exclusion uses only the published family's quartic branch-point form: four roots in `K` give all three nonzero rational 2-torsion points, so `4 | #E(K)`. Our eight group orders are 2 modulo four. Same-field isogenies preserve group order. This excludes that published **direct** family, without excluding every possible cover or correspondence.
4. Let `π` be a root of `U²−tU+3^5` in its ordinary imaginary quadratic field `F`. The degree-ten irreducibility at `s=1` implies `π` is not a fifth power in `F`. If `gcd(s,5)=1` and `π^s=z^5`, choose integers `u,v` with `us+5v=1`; then `π=(z^uπ^v)^5`, contradiction. Ordinarity implies `π/πbar` is not a root of unity, so `Q(π^s)=F`; the prime-degree binomial criterion makes `X^5−π^s` irreducible over `F`, hence `X^10−t_sX^5+3^(5s)` irreducible over `Q`, where `t_s=π^s+πbar^s`. Finite-field linear disjointness for `gcd(s,5)=1` identifies the base change of `Res_(K/k) E` with `Res_(K_s/k_s) E_s`. Base changing the Tate isogeny gives the genus-five carrier. `base_change_check.json` tests all 80 pairs of the eight traces with `s=1..10`; it is a check, not the all-s proof.

`Odd5Ledger.lean` checks only the arithmetic equalities (484 classes, sum 100, eight orders modulo four, representative trace) with no axioms. It does **not** formalize the point-count computation, Tate theorem, binomial criterion, or base-change argument.

## Prior work and novelty boundary

- [Iijima--Momose--Chao 2013, Sections 2 and 7](https://eprint.iacr.org/2013/487.pdf) classifies direct `(2,...,2)` covers under an isogeny condition and lists the degree-five minimal-genus elliptic case `(1,4,5)`. Their theorem does not classify all Jacobians isogenous to a Weil restriction. The later [2015 paper](https://eprint.iacr.org/2015/805.pdf) was consulted previously; its mirror returned HTTP 403 during this pass.
- The prior repository [degree-five direct-cover census](../../ghs_cover_census_n5_20261013/README.md) exhaustively checked all 488 elliptic isomorphism classes over `F243`, including 122 of order `2 mod 4`, and derived genus floor 33 for their direct `(2,...,2)` covers. It did not construct a genus-five Jacobian in those restriction isogeny classes. Its existence and bounds materially narrow the novelty statement here.
- The complete local LMFDB `av_fq` isogeny-class snapshot was extracted on 19 October 2026 from `public.av_fq_isog` (2,945,722 rows). It already lists **all eight** Weil-polynomial classes. `has_jacobian=1` for traces `-22,-14,-10,+10,+14,+22` (labels `5.3.a_a_a_a_w`, `5.3.a_a_a_a_o`, `5.3.a_a_a_a_k`, `5.3.a_a_a_a_ak`, `5.3.a_a_a_a_ao`, `5.3.a_a_a_a_aw` respectively). It records **uncertainty** (`has_jacobian=0`) for `t=-26`, label `5.3.a_a_a_a_ba`, and `t=+26`, label `5.3.a_a_a_a_aba`. The [LMFDB field inventory](https://github.com/LMFDB/lmfdb-inventory/blob/master/db-abvar.md) explicitly defines 0 as uncertain, 1 as known yes and -1 as known no. The two explicit curves above resolve those two uncertain snapshot flags mathematically. A connected read-only LMFDB mirror query on 15 November 2026 returned the same zero flags for both exact labels (`lmfdb_connected_mirror.json`), though its refresh timestamp is unavailable; the live site timed out. **Six of eight are catalog-known Jacobian classes; two are potentially new Jacobian identifications relative to the dated snapshot and connected mirror.** Publication priority remains unconfirmed. [pcc:unverified]
- [Tian 2020/2023](https://arxiv.org/abs/2012.07173) gives a distinct explicit isogeny-based cover attack for cubic extensions. It establishes that isogeny-class transfer without a direct cover is already a known attack type.
- The internal KB lead `LEAD-TRF-weil-restriction-jacobian-class` (`source:d83cb619b3afdb7b9d999946161057c7`, `crypto-autoresearcher:knowledge/leads/ecdlp/transfer/LEAD-TRF-weil-restriction-jacobian-class.yaml`) already proposed searching abstract Jacobian carriers. KB revision `9141e386254340a9bdba3036f2bb706a`, refreshed 14 November 2026, also returned `source:1373f49b787295f0ca7f17c2de1a1985` pointing to the prior degree-five census. Targeted web searches found no exact `F3/F243` eight-trace genus-five carrier classification, but absence from that search does not establish publication priority. **The explicit genus-five carriers and strict 33-to-5 separation on these 100 classes are new to this project; global novelty remains unconfirmed.**

## Falsifier for practical vulnerability

Construct an explicit `k`-rational correspondence or isogeny `Res E_t -> J(Y_t)`, evaluate it on a chosen nonzero prime-order subgroup, verify the image order and target scalar, and price its construction, evaluation, relation/decomposition, linear algebra, descent and final verification against a matched rho attack on the same source subgroup. The current receipt contains no such executable map or full attack cost for these odd-five classes. The source orders 218–270 are small, so a cryptographically meaningful cost claim also needs a larger coprime base-change instance and its Frobenius-aware rho baseline.
