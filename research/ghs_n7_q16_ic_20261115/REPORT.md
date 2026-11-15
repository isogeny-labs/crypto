# Exact q16 genus-eight GHS transfer and complete attack cost

15 November 2026. The [order screen](SCREEN_PROTOCOL.md), [selected-subgroup map gate](MAP_PROTOCOL.md), and [attack protocol](ATTACK_PROTOCOL.md) were committed before their respective measurements. [Attempt 1](receipts/attempt1.json) is preserved as an output-truncated run; [the amendment](AMENDMENT.md) explains its output-only correction. [Attempts 2 and 3](receipts/attempt2.json) complete the same exact source attack, with [attempt 3](receipts/attempt3.json) as a repeat. This instance lies inside Teske's published n7 selector. [pcc:unverified]

## Exact equation, subgroup and target

Let `K=F2[z]/(z²⁸+z³+1)`. The source is `E:y²+xy=x³+a x²+b`, with `a` polynomial mask `65,712,646` and `b` mask `406,166`. Their full polynomial forms, and the source point coordinates, are in [map.magma](map.magma). Sage screened the first 128 full-field coefficients under the frozen rule and selected this one: `#E(K)=268,413,186=6·44,735,531`, with 44,735,531 prime. The point `P` has coordinate masks `(179,462,209, 218,074,333)` and exact order 44,735,531. The public validation target is `Q=1,234,567P`. Bounded KB and web exact-parameter searches found no publication match, which leaves exact-equation priority unproved. [pcc:unverified]

Magma 2.30-1 constructed the genus-eight hyperelliptic target over `F16`, with subfield generator `κ=z²⁷+z²⁰+z¹⁵+z¹⁰+z⁸+z⁷+z⁵+z³` embedded in K. In the final attack receipts its equation is

`C: y²+(x⁸+κ³x⁴+κx²+κ¹²x)y = κ²x¹⁷+κ¹⁴x¹⁶+κ⁸x⁹+κ¹⁰x⁸+κ⁴x⁵+κ⁹x⁴+κ¹¹x³+κ¹⁰x²+κ¹²x`.

The [map receipt](receipts/map1.json) and both final attack receipts verify `G=h(P)≠0`, `rG=0`, and `h(Q)=1,234,567G`; the exact Mumford image is printed there. Magma computed `#Jac(C)(F16)=4,831,437,348=108·r`. Every one of 143 factor-base classes was checked through `108Bᵢ=(108Lᵢ)G`, so the solved `Lᵢ` are selected-subgroup projected coordinates even though none of the factor-base classes itself lies in `<G>`. [pcc:unverified]

## Complete relation, descent and matched cost

The degree-two Mumford factor base yielded 208 reconstructed relations from 8,650 nonzero subgroup multiples, with zero reconstruction failures. The modular relation matrix has full rank 143 on 143 classes and a unique consistent solution. The nonzero target descent used 35 offsets and only already solved classes; it returned scalar 1,234,567, checked on E and the Jacobian. The [independent Python auditor](audit_receipt.py) replays all 208 modular equations, rank, projected coordinates, and descent arithmetic for both final receipts. Magma checks divisor reconstruction and the 143 exact projection identities. [pcc:unverified]

| Final receipt | Setup CPU | Relations CPU | Linear algebra CPU | Descent CPU | Total GHS CPU | Rho mean CPU, 100 verified solves | GHS / rho |
|:--|--:|--:|--:|--:|--:|--:|--:|
| [Attempt 2](receipts/attempt2.json) | 3.780 s | 0.780 s | 0.010 s | 0.010 s | 5.200 s | 0.2321 s | 22.4× |
| [Attempt 3](receipts/attempt3.json) | 3.800 s | 0.790 s | 0.010 s | <0.010 s | 5.210 s | 0.2344 s | 22.2× |

The total also charges about 0.61–0.62 second outside the four displayed phase timers, chiefly the 143 exact projection assertions and diagnostic output. The same-process rho uses a deterministic three-way Floyd walk on the same P and Q: 31,392 walk steps and one restart. Its 100-run batch was timed before GHS. The 28.8–29.0-second public service time includes that batch and is not the one-target GHS cost. A [q4 predecessor](../ghs_n7_q4_ic_20261115/REPORT.md) measured a roughly 206–209-fold GHS penalty on its own smaller subgroup; the q16 penalty is smaller, but two sizes do not locate a crossover. This exact q16 run still costs more than rho, so it does not establish a vulnerable new curve or a new eligible family. [pcc:unverified]

## Rigorous genus floor and scope

For this E, the F16 Weil-restriction Frobenius polynomial is `X¹⁴−22,271X⁷+268,435,456`. The [Rabin certificate](genus_floor.json) reduces it modulo 13 to `f=X¹⁴+11X⁷+3`, checks `X^(13¹⁴)≡X mod f`, and verifies `gcd(f,X^(13²)−X)=gcd(f,X^(13⁷)−X)=1`. Hence f is irreducible over F13 and the integer polynomial is irreducible. The seven-dimensional Weil restriction is simple; any nonzero algebraic homomorphism from it to a Jacobian over F16 has image dimension seven, forcing target genus at least seven. This is a geometric floor for this exact isogeny class, not a bound on every curve susceptible to every generalized cover or a cost lower bound. [pcc:unverified]

Primary family and transfer sources: [Teske](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf), [Galbraith–Hess–Smart](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf), and [Magma's Weil-descent API](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html). The exact equation, map and benchmark are reproducible from the committed scripts and immutable receipts; no publication-priority proof or family-wide vulnerability claim is made. [pcc:unverified]
