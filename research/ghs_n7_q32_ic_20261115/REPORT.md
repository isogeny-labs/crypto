# Exact q32 GHS transfer, genus floor, and complete attack cost

15 November 2026. The [order screen](SCREEN_PROTOCOL.md), [point/map gate](MAP_PROTOCOL.md), [full-Euler gate](FACTOR_PROTOCOL.md), and [attack protocol](ATTACK_PROTOCOL.md) were committed before their measurements. The first attack response [truncated during relation output](receipts/attempt1.json); the [output-only amendment](AMENDMENT.md) preceded the successful [compact run](receipts/attempt2.json). This exact instance lies in the published n=7 genus-eight Hess/Teske selector, so the result is an attack-cost measurement and exact-curve check, not a new-family claim. [pcc:unverified]

## Exact source, target, and subgroup map

Put `K=F2[z]/(z^35+z^2+1)`, `k=F32` embedded in K as in the [map receipt](receipts/map1.json), and let `b` have polynomial-basis mask `9698245`, namely

`b=1+z²+z⁶+z⁷+z⁸+z⁹+z¹¹+z¹²+z¹³+z¹⁴+z¹⁵+z¹⁶+z¹⁷+z²⁰+z²³`.

The source is `E/K: y²+xy=x³+x²+b`. Sage screened the frozen first 128 full-field coefficients and selected row 9: `#E(K)=34,359,894,078=6·5,726,649,013`, with prime `r=5,726,649,013`, source trace `−155,709`, and a selected full-degree b in cubic arm 1. The entire fixed-one-pole selector has 65,520 b values of full absolute F2 degree 35 under this representation; this is a coefficient count, not an attack count. The [point receipt](point.json) fixes `P=(25722924589,16457048142)` in polynomial-basis coordinate masks, of exact order r, and `Q=1,234,567P` with masks `(31908057006,10517578624)`. [pcc:unverified]

Magma 2.30-1 constructed the genus-eight hyperelliptic curve `C/k`:

`y²+(κ²⁷x⁸+κ²⁸x⁴+κ¹²x²+κ¹⁸x)y`

`=κ¹⁶x¹⁷+κ²³x¹⁶+κ¹⁸x⁹+κ²³x⁸+κ¹⁷x⁵+κ²⁶x⁴+κ²⁹x³+κ²⁵x²+κ¹⁹x`, where κ is Magma's `k.1` and its embedding into K is printed in the [full map receipt](receipts/map1.json). That receipt checks `G=h(P)≠0`, `rG=0`, `h(Q)=1,234,567G`, and the homomorphism action at 2 and 3. The exact image Mumford classes are preserved there. [pcc:unverified]

## Rigorous factor and genus bounds

The independent [full Euler-factor receipt](receipts/factor1.json) verifies the equality below. [pcc:unverified]

`L_C(T)=(1−11T+32T²)(1+155709T⁷+32⁷T¹⁴)`.

Thus `#J_C(k)=755,917,669,716=132r`. This agrees with the published [Hess L-polynomial theorem](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf): the fixed elliptic factor `E0:y²+xy=x³+x²+1` has `#E0(F32)=22`, and `#J=#E(K)·#E0(k)`. Since `#E=6r`, the exact subgroup projection multiplier is `m=6·22=132`. Every factor-base class B used below was checked by the identity `132B=(132L)G`, even though none of the 470 raw classes lay in `<G>`. [pcc:unverified]

The [Rabin certificate](genus_floor.json) reduces the Weil-restriction polynomial `X¹⁴+155709X⁷+32⁷` modulo 139 to `X¹⁴+29X⁷+50`. It checks `X^(139¹⁴)=X` modulo f and unit gcds with `X^(139²)−X` and `X^(139⁷)−X`. The reduction is irreducible, hence the integer polynomial is irreducible. The degree-seven Weil restriction is simple; any nonzero algebraic map from it into a Jacobian over F32 has image dimension seven, so that target has genus at least seven. This is an exact-isogeny-class geometric floor, not a universal bound on all GHS-vulnerable curves or a lower bound on attack runtime. [pcc:unverified]

\newpage

## Complete selected-target cost

The [successful compact run](receipts/attempt2.json) reconstructed 679 degree-at-most-two-smooth relations from 77,700 nonzero subgroup attempts, with zero reconstruction failures. Its modular matrix has full rank 470 on 470 factor-base classes and a unique consistent solution. Magma checked all 470 cofactor-132 projections and descended the nonzero mapped Q after 27 offsets, returning scalar 1,234,567 and checking it on E and J. The [independent Python auditor](receipts/attempt2-audit.json) decoded all 679 compact rows, replayed the modular equations and rank, reconstructed projected coordinates, and verified the descent arithmetic; Magma performed the divisor-class and Jacobian group checks. [pcc:unverified]

| Component | Measured CPU |
|:--|--:|
| GHS setup, including `#E0(F32)` | 0.180 s |
| GHS relation collection | 5.920 s |
| GHS linear algebra | 0.090 s |
| GHS total, including 470 exact projections and other charged work | **8.420 s** |
| Same-process rho, mean of 20 verified solves of the same P and Q | **3.138 s** |
| Total GHS / rho mean | **2.683** |

The first rho solve used 340,152 three-way Floyd walk steps and one restart; its CPU was 3.140 seconds. The 20-solve rho batch took 62.760 CPU seconds and was timed before GHS in the same process. The 75.05-second client wall time includes that batch and public-service overhead; it is not the one-target GHS cost. q4 and q16 complete exact-instance ratios were 206–209 and 6.77, respectively. These three sizes show a narrowing gap but do not prove a q64 crossover. [pcc:unverified]

## Prior work and formal scope

The selector belongs to published [Teske](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) and [Hess](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) constructions. The q32 order/mask were not matched in bounded KB searches (revision `9141e386254340a9bdba3036f2bb706a`) or three exact web searches; that leaves publication priority of this **exact equation** unproved. The exact equation is therefore a candidate newly recorded by this investigation, while the eligible family and attack method are known. It is not a measured vulnerability because the complete GHS attack costs 2.683 times rho on the same subgroup. [pcc:unverified]

[Q32Ledger.lean](Q32Ledger.lean) was checked by Lean 4.35.0 and proves the exact order/cofactor and candidate-count arithmetic with no axioms. The field and group calculations, mod-139 Rabin certificate, Hess geometry, subgroup map and complete cost are verified by the separate Sage/Magma receipts and cited theorem; they are not formalized by that Lean file. [pcc:unverified]
