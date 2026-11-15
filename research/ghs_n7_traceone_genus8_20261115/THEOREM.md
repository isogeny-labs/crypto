# Exact n=7 trace-one genus-eight coefficient slice

This is an elementary classification of a **published classical GHS profile**, including [Teske's nonzero W0 plus cubic-component genus-eight selection](https://doi.org/10.1007/s00145-004-0328-3). The count and disjointness below are rigorous deductions from finite-field linear algebra; the resulting curve family and isogeny-based GHS extension are prior art. The argument does not bound all cover attacks or certify a lower total-cost attack.

Let `q=2^s`, `k=F_q`, `K=F_(q^7)`, and `sigma(x)=x^q`. Over F2,

`T^7+1=(T+1)p1(T)p2(T)`, where `p1=T³+T+1` and `p2=T³+T²+1`.

Put `Vi=ker pi(sigma)`. Because the extension degree seven is odd and the three factors are distinct, the Frobenius module decomposes as `K=k⊕V1⊕V2` over k. Each Vi has dimension three. Fix the unique absolute-trace-one isomorphism class of `a` and let

`S8={b=v+c: v∈V1×∪V2×, c∈k×}`.

**Claim 1: exact size.** Each summand `Vi×+k×` has `(q³−1)(q−1)` elements by uniqueness of the direct-sum decomposition. The two summands are disjoint: equality `v1+c1=v2+c2` implies `v1=v2=0`, contrary to membership. Hence

`|S8|=2(q³−1)(q−1)` and `|S8|/(q⁷−1)=2(q³−1)(q−1)/(q⁷−1)=O(q⁻³)`

within the trace-one ordinary equations. All b lie outside k, so no source has a model over a proper intermediate field containing k. For `s>1`, a coefficient may nevertheless lie in another proper F2-subfield of K; the finite q=4 sieve identifies those separately.

**Classical low-genus ceiling.** The same direct sum gives the full non-subfield magic-number-four locus `S4=(k+V1×)∪(k+V2×)` of size `2q(q³−1)`. Its relative-trace-zero part is `V1×∪V2×`, of size `2(q³−1)` and classical genus seven in the trace-one branch; its nonzero-trace part is exactly S8 and has genus eight. [Teske's primary presentation](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) states that n=7 classical magic numbers are 1, 4, and 7, with the magic-number-four set equal to S4. Thus among the `q⁷−1` ordinary trace-one classes, **at most** `2q(q³−1)` admit a non-subfield classical GHS descent of genus at most eight, and these are exactly the S4 classes under that construction. This ceiling is for classical GHS; generalized covers and isogeny-class reachability are outside it.

**Claim 2: direct genus eight.** For `b=v+c∈S8`, square root is an automorphism of each Frobenius-stable summand, so `sqrt(b)` has exact sigma-annihilator `(T+1)pi`: its Vi component and k component are both nonzero. The classical GHS choice `gamma1=1`, `gamma2=sqrt(b)` has `s1=1`, `s2=4`, `t=4`. [Menezes–Teske's two-pole formula](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) gives `g=2⁴−2³−2⁰+1=8`; [Maurer–Menezes–Teske's type formula](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/D4B264D082FF67354EE9968D9803102A/S1461157000000723a.pdf/analysis_of_the_ghs_weil_descent_attack_on_the_ecdlp_over_characteristic_two_finite_fields_of_composite_degree.pdf) gives the same classical magic-number profile. Because seven is odd, the trace-one descent condition holds. This constructs a hyperelliptic genus-eight target and its published map. It does not by itself establish that every source subgroup survives the map.

**Claim 3: disjointness from W1.** The relative trace pairing `(u,v)↦Tr_(K/k)(uv)` is nondegenerate, and `sigma` is adjoint to `sigma⁻¹`. The reciprocal polynomial of `p1` is `p2`, and conversely. Since `p2(sigma)` is invertible on V1, applying the adjoint relation to `p1(sigma)u=0` gives `Tr_(K/k)(uv)=0` for all `u,v∈V1`; similarly for V2. Also `Tr_(K/k)(u)=0` for u in either cubic kernel. Thus every published W1 product `b=[u(v+c)]²` with `u,v` in the same Vi and c in k has `Tr_(K/k)(b)=0`. For every b in S8, `Tr_(K/k)(b)=c≠0`. Therefore `S8∩W1=∅` for every q. This separates two known attack constructions by an exact, inexpensive recognition invariant.

The q=2 and q=4 [Sage census](screen.json) checks the formulas on exact fields and adds group-order and full-F2-field-of-definition counts. The [Magma receipt](map8_receipt.json) checks one exact order-71 subgroup map at q=2. Neither finite check formalizes this general proof in Lean or measures total index-calculus cost against rho.
