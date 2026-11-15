# Exact q=4 simple-restriction stratum within Teske's genus-eight GHS family

15 November 2026. [Frozen stratum protocol](STRATA_PROTOCOL.md), [trace receipt](strata.json), [frozen map protocol](MAP_Q4_PROTOCOL.md), [Magma receipt](map_q4_receipt.json), [simplicity protocol](SIMPLE_Q4_PROTOCOL.md), and [factorization receipt](simple_q4.json). This is a finite classification and subgroup-map check within the **published** n=7 classical selector. The family and the GHS construction are prior art. [pcc:unverified]

## Exact class and curve

The q=4 trace-one genus-eight coefficient set has `2(4³−1)(4−1)=378` values. Its 26 source-trace strata contain **371 coefficients with irreducible** degree-14 Weil-restriction polynomial `X¹⁴−tX⁷+4⁷`. The remaining seven coefficients have trace 87 and lie in an F4 base-field isogeny class. Of the 371 irreducible cases, **364 have b in no proper absolute F2-subfield** of F_(2^14). These are exact finite counts from the Sage screen, not an asymptotic vulnerability rate. [pcc:unverified]

An exact representative is `K=F2[z]/(z¹⁴+z⁵+1)` and

`E: y²+xy=x³+a x²+b`,

where `a=1+z+z⁴+z⁵+z⁶+z⁷+z⁸+z⁹+z¹²` (mask 5107) and `b=1+z+z⁵+z⁶+z⁷+z⁸` (mask 483). Here a lies in F4 with absolute trace one; b has full absolute field of definition. Sage and public Magma 2.30-1 agree on `#E(K)=16,634=2·8,317`, with 8,317 prime and source trace −249. Its trace differs from the F4-base-change traces 87 and 251. Magma returned a genus-eight hyperelliptic GHS target over F4. The selected exact order-8,317 source point maps to a nonzero Jacobian point annihilated by 8,317; `h(7P)=7h(P)` and a scan of target multiples 0 through 10 returned seven. The Magma construction/check service time was 0.31 seconds. This is a checked subgroup-preserving transfer, not a complete index-calculus solve or a rho comparison.

## Genus floor and what the bound means

For the representative, the Frobenius quadratic discriminant is `t²−4·4⁷=−3535=−5·7·101`. The quadratic field is `L=Q(√−3535)`. Any algebraic integer in L has form `(u+v√−3535)/2` with u,v of the same parity and norm `(u²+3535v²)/4`. Norm four forces v=0 and the integer to be ±2. A Frobenius root π has norm `4⁷`; if π were a seventh power in L, its seventh root would be an algebraic integer of norm four. But `(±2)⁷` has trace ±256, whereas π has trace −249. Thus π is not a seventh power. [Capelli's criterion](https://pub.math.leidenuniv.nl/~stevenhagenp/GaloisTheory.pdf) makes `X¹⁴+249X⁷+16384` irreducible over Q. The Sage factorization independently confirms this polynomial and irreducibility.

The same factorization check finds irreducibility for 25 of the 26 q4 trace strata, totaling 371 coefficients. For each, the seven-dimensional Weil restriction is simple. Hence any nonzero algebraic homomorphism from that restriction to a curve Jacobian has image dimension seven and requires **target genus at least seven**. The direct GHS target has genus eight. This is a rigorous lower bound on algebraic transfer geometry for this finite set, not a bound on the number of all GHS-vulnerable curves or on attack cost. The prior [±9 genus-floor proof](../ghs_n7_base_change_20261115/GENUS_FLOOR.md) uses the same Capelli and simple-variety argument. [pcc:unverified]

The [Lean 4 arithmetic ledger](Q4TraceLedger.lean) checks the two F4 base-change traces, the candidate order factorization, discriminant identity, and separation from both base-change traces. `lean Q4TraceLedger.lean` exits successfully, and `#print axioms` reports no axioms for either theorem. This ledger formalizes these integer equalities only; the Capelli step, the simple-variety implication, and the GHS subgroup map have independent Sage, paper, and Magma evidence rather than Lean proofs. [pcc:unverified]

## Prior work and novelty status

[Teske's primary presentation](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) already gives the W0 plus cubic-component selector and genus-seven/eight GHS target, and [Galbraith–Hess–Smart](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf) already gives the isogeny extension. The exact q4 equation, its trace −249 stratum, and the 371-count finite classification were not located by targeted KB queries for the masks, source order, and trace or by exact-parameter web queries. That bounded search does **not** establish publication priority. This is a potentially new exact equation and finite classification *inside a known family*; it is neither a new eligible family nor a new attack method. A vulnerability claim still needs complete relation collection, linear algebra, target recovery, and matched rho on the same order-8,317 source subgroup. [pcc:unverified]
