# Exact q64 GHS transfer and complete same-target cost

15 November 2026. The [coefficient screen](SCREEN_PROTOCOL.md), [subgroup-map gate](MAP_PROTOCOL.md), [full-Euler gate](FACTOR_PROTOCOL.md), and [attack protocol](ATTACK_PROTOCOL.md) were committed before their measurements. The exact equation belongs to the published Hess/Teske n=7 genus-eight family. This report records an exact-curve test and a genus floor, not a new-family vulnerability. [pcc:unverified]

## Source, target, and subgroup map

Set `K=F2[z]/(z^42+z^7+1)` and `k=F64`. The selected source is

`E/K: y²+xy=x³+z^35 x²+(1+z³+z⁶+z¹²)`.

The frozen first-128 screen chose row 20, arm 1, from 523,152 full-degree coefficients in the two cubic-kernel arms. Sage verified `#E(K)=4,398,047,737,114=2r`, where `r=2,199,023,868,557` is prime. The first eligible point under the frozen x/y-mask rule is `P=(4398046509767,4135367847495)` in polynomial-basis masks, and `Q=1,234,567P=(3349320360189,3597932644297)`. Both have exact order r. The source trace is −1,226,009. [pcc:unverified]

Magma 2.30-1 constructed the genus-eight hyperelliptic target `C/k`:

`y²+(κ²⁹x⁸+κ⁴⁴x⁴+κ⁸x²+κ³¹x)y`

`=κ¹³x¹⁷+κ⁴⁵x¹⁶+κ⁴³x⁹+κ¹⁰x⁸+κ³⁴x⁵+κ¹⁰x⁴+κ¹⁷x³+κ⁶¹x²+κ⁶¹x`, where `κ=k.1` and its embedding in K is `z²⁸+z²¹+z¹⁴+z⁷`. The [full map receipt](receipts/map1.json) prints the exact Mumford classes and verifies `G=h(P)≠0`, `rG=0`, `h(Q)=1,234,567G`, and the homomorphism at 2 and 3. The map took 0.130 CPU seconds in its separate validation call. [pcc:unverified]

## Exact factor and geometric bound

A separate Magma call computed the entire degree-16 Euler factor rather than only the group order. The [factor receipt](receipts/factor1.json) verifies the following equality. [pcc:unverified]

`L_C(T)=(1+9T+64T²)(1+1226009T⁷+64⁷T¹⁴)`

and `#J_C(k)=325,455,532,546,436=148r=#E(K)·#E0(k)`, with `#E0(F64)=74`. This is Hess's published seven-orbit plus fixed-point decomposition for this source. Its costed use of `148` projects each factor-base class into the selected r-primary subgroup. The independent full-Euler validation took 9.280 CPU seconds; that validation cost is not included in the one-target attack because the costed setup computes `#E0` and `#E`, not `EulerFactor(J)`. [pcc:unverified]

The [Rabin certificate](genus_floor.json) reduces the Weil-restriction polynomial `X¹⁴+1226009X⁷+64⁷` modulo 83 to `X¹⁴+16X⁷+81`; it verifies `X^(83¹⁴)=X` modulo f and unit gcds with `X^(83²)−X` and `X^(83⁷)−X`. Thus the integer polynomial is irreducible. The seven-dimensional Weil restriction is simple, and any nonzero algebraic map from it to a Jacobian has image dimension seven, forcing target genus at least seven for this exact source isogeny class. This is a geometric floor, not an attack-runtime lower bound or a universal classification of GHS-susceptible curves. [pcc:unverified]

For fixed trace-one a, the full fixed-one-pole genus-eight slice has exactly `2(q−1)(q³−1)=33,030,018` distinct nonzero b values at q=64, a fraction `7.51·10⁻⁶` of `q⁷−1` coefficient classes. The frozen c=1, full-absolute-degree screen uses 523,152 of those classes, a fraction `1.19·10⁻⁷`. These are algebraic eligibility counts, not counts of below-rho curves. The general [Karabina–Menezes–Pomerance–Shparlinski bounds](https://math.dartmouth.edu/~carlp/WeilDescent-Product.pdf) show why small-genus descent cannot be presumed for almost all elliptic curves; this exact q64 count is narrower than their asymptotic setting. [pcc:unverified]

\newpage

## Complete attack and matched rho

The [untruncated public attack receipt](receipts/attack1.json) has a matching frozen source hash. It records 20,573 reconstructed degree-at-most-two relations from 910,000 subgroup attempts, zero reconstruction failures, full matrix rank 2,081 on 2,081 factor-base classes, and a unique consistent solution. Magma checked all 2,081 exact identities `148B_i=(148L_i)G`; none of the raw B_i was itself in `<G>`. The mapped Q descended after 16 offsets and returned scalar 1,234,567, checked on E and J. [pcc:unverified]

| Phase | Measured CPU |
|:--|--:|
| GHS setup | 0.140 s |
| Relation collection | 84.430 s |
| Linear algebra | 10.600 s |
| Descent | 0.010 s |
| Complete GHS, including projections and other charged work | **107.550 s** |
| Deterministic same-process rho, verified P and Q | **9.060 s** |
| Verified same-process rho repeat | **9.180 s** |
| GHS / first rho | **11.871** |

Rho took 944,040 three-way Floyd walk steps and one restart. The second identical solve is a reproducibility check, not a distribution over independent walk seeds. The 126.87-second client time includes both rho solves, GHS, service overhead and output. The public stdout cap led the frozen source to print only the first 500 of 20,573 relation rows. Magma checked every reconstruction, full rank, every cofactor projection and the target equality internally; independent replay of the full matrix is **unfinished**. The measured complete Magma attack is 11.871 times slower than the matched rho run on this exact subgroup. [pcc:unverified]

## Prior work and novelty scope

The coefficient selector and genus-eight construction are in [Teske](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) and [Hess](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf); [Menezes–Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) analyze binary weak-field costs. Bounded exact-order and coefficient searches in KB revision `9141e386254340a9bdba3036f2bb706a` and on the web found no exact match; this does not prove publication priority. The exact equation is a candidate newly recorded in this investigation, while the eligible family and transfer method are known. Its measured attack cost does not support a vulnerability label. [pcc:unverified]

[Q64Ledger.lean](Q64Ledger.lean) passed Lean 4.35.0 with axiom-free proofs of source order, Jacobian cofactor and coefficient-count arithmetic. The geometric factorization, irreducibility certificate, subgroup map and Magma attack are checked by their separate sources and receipts, not formalized in Lean. [pcc:unverified]
