# Frozen audit: trace-one n=7 classical genus-eight complement

Frozen 15 November 2026 UTC before the new public Magma call. Scratch Sage screens suggested a trace-one family with b in `(V1+k) ∪ (V2+k)` but outside `V1∪V2∪k`, disjoint from the Menezes–Teske W1 coefficient family at q=2 and q=4. These screens are discovery evidence only; the scripts and independent replay below must establish the finite counts. No private point or scalar is used.

## Literature-first boundary

- [Maurer–Menezes–Teske, Theorem 5 and Remark 20](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/D4B264D082FF67354EE9968D9803102A/S1461157000000723a.pdf/analysis_of_the_ghs_weil_descent_attack_on_the_ecdlp_over_characteristic_two_finite_fields_of_composite_degree.pdf) already give the Frobenius-type count for the classical GHS magic number and analyze small-genus n=7 direct curves. Therefore the basic genus-eight classical transfer is prior art, even if this exact slice is not separately named there.
- [Menezes–Teske, Section 5.2](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) defines the generalized W1 family by cubic/(linear times cubic) factors, genus fourteen, and leaves weakness of all trace-one n=7 curves unsupported. The current hypothesis is a distinct *coefficient locus* and potential isogeny-walk complement, not a new GHS construction.
- [Hess, Theorems 11–15](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) supplies the two-pole genus and homomorphism hypotheses. [Magma's Weil-descent handbook](https://docs.magma-maths.org/ArithmeticGeometry/EllipticCurvesOverFiniteFields/ec_weil_desc.html) documents `WeilDescentGenus` and the hyperelliptic Jacobian map.

## Algebraic predictions

Let `k=F_q`, `K=F_(q^7)`, `q=2^s`, `sigma(x)=x^q`, and `Vi=ker pi(sigma)` for the two irreducible cubic factors of `T^7+1`. Fix absolute `Tr(a)=1`. The proposed family is `b=v+c` with `v∈Vi×` and `c∈k×`. Each arm has `(q^3−1)(q−1)` coefficients and the arms are disjoint because `V1∩V2=0`; hence the union has exactly `2(q^3−1)(q−1)` curve classes. Since `sqrt(b)∈Vi+k` has Frobenius order `(T+1)pi`, the classical GHS presentation `gamma1=1`, `gamma2=sqrt(b)` has genus eight. The coefficient is outside k, so the source is not a proper-subfield curve containing k. The relative trace is `Tr_(K/k)(b)=c≠0`. W1 coefficients have relative trace zero because the trace pairing on one cubic Frobenius kernel is zero, so the two loci are disjoint. These statements are a paper-level proof target; finite code checks do not replace its independent review.

## Finite checks and stopping rules

Run an independent Sage screen at q=2 and q=4 using explicit irreducible field models. Verify cubic-kernel sizes, exact coefficient count, disjointness from the W1 product set, and source orders modulo eight. For q=2 use `K=F2[z]/(z^7+z+1)`, `a=1`, `b=z+1`; expected `#E=142=2·71` from scratch discovery. Stop on any field or count mismatch.

Then submit one public Magma 2.30-1 program with only the exact q=2 curve and a bounded 120-second timeout. Require `#E=142`, `WeilDescentGenus(E,F2,1)=8`, a hyperelliptic target, and the first exact order-71 point to map to a Jacobian class of exact order 71. Preserve the input hash, full response, error, or timeout. This verifies a direct transfer on one selected subgroup; it is not an index-calculus or same-subgroup rho cost measurement. Do not call the family newly vulnerable or a new attack method from this result alone.

## Amendment A: earlier explicit genus-eight isogeny route

Before the Magma response was evaluated, a broader primary-source search found [Galbraith–Hess–Smart, *Extending the GHS Weil Descent Attack* (EUROCRYPT 2002)](https://www.iacr.org/archive/eurocrypt2002/23320028/smart.pdf), which explicitly discusses the n=7 genus-eight GHS family and isogeny-based extension. This closes the proposed isogeny-walk strategy as an attack-method novelty claim. The q2/q4 disjointness and exact count can still serve as a precise classification of the published family; the q2 curve equation remains an exact witness, with absence from prior publications unproven.

## Amendment B: full-field sieve at q=4

The first Sage screen counted coefficient classes relative to `K/k`, not curves of full field of definition over F2. A separate scratch check found 14 of the q=4 coefficients lie in the proper F128 subfield of F_(2^14). Add a finite-field-of-definition count to the frozen screen before drawing any subgroup or attack conclusion for q=4. Predict 364 full-field coefficients out of 378 at q=4; all 14 q=2 coefficients have full field of definition. Preserve the original count and report the subtraction explicitly.

## Amendment C: Teske's exact component construction found in prior work

The post-map KB review located `source:da469498ce3305b539134232c1172760` (`crypto:docs/teske161-selector.md`), which points to [Teske, *An Elliptic Curve Trapdoor System* (2006)](https://doi.org/10.1007/s00145-004-0328-3), Sections 2.1 and 4. Its genus-eight selector combines a nonzero W0 component with one nonzero cubic component over F_(2^161)/F_(2^23). This is the same family shape as S8, so the exact component construction is prior art, not merely the general magic-number-four envelope. The KB was ready at revision `9141e386254340a9bdba3036f2bb706a` (last refresh 2026-11-14T21:44:48Z, before this audit). An independent [Teske presentation](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) states the W0 plus one cubic-component set explicitly. The report now states this stronger priority boundary.
