# Trace-class intersection with Hess's published m=3 n=7 locus

Frozen 15 November 2026 UTC before the finite screen. This is a prior-work-controlled search for an **isogenous source representative** of the exact ±9 carrier class, not a new eligible family. [pcc:unverified]

## Prior work and criterion

[Hess, *The GHS Attack Revisited*, §4, especially the m=d construction](https://www.iacr.org/archive/eurocrypt2003/26560374/26560374.pdf) states that for n=7, d=ord_7(2)=3, genus `2^3−1=7` occurs for the α=0 Artin–Schreier class with both reciprocal-pole coefficients in the same cubic Frobenius kernel. Menezes–Teske's published n=7 families and the repository's exact W0 product count are in `../ghs_n7_membership_20261114/REPORT.md`. Magma's direct c scan on the chosen −9 source found minimum genus 14 (`../ghs_n7_explicit_map_20261115/`). Equal traces imply isogeny, but do not supply an explicit isogeny path.

## Finite question and method

In `K=F2[z]/(z⁷+z+1)`, form `V_p=ker p(σ)` for both `p=T³+T+1` and `p=T³+T²+1`, `σ(x)=x²`. Enumerate all nonzero products `γβ` with `γ,β∈V_p×`; square to obtain exact elliptic coefficients b. For each distinct b, count `E(0,b): y²+xy=x³+b` by the Artin–Schreier trace criterion. Predict no intersection a priori; the discriminator is whether trace t=−9 or +9 occurs in this published genus-seven locus. Verify all 127 possible b orders independently with a second method if an intersection is found. Preserve exact b masks, kernel witness pairs, trace histogram, source hash and raw output. Stop on any field, kernel-size, order or direct-count discrepancy. The scan is a finite geometric eligibility result; it cannot establish an executable isogeny chain, target-recovery cost or a new attack family.

## Amendment A: family-label correction and direct map test

The two independent counters agree on all 127 nonzero b values. The published m=3 product set has 49 coefficients with trace classes −15 (14), +1 (7), +9 (21), +17 (7); no trace −9. In particular b=8 has the p1 witness γ=z, β=z^64 (masks 2,18), so `E(0,8)` is a direct Hess m=3 genus-seven candidate. This reveals an error in the preceding base-change report: the +9 a₂=0 source is W0, not W1; the −9 a₂=1 source is W1 only when the absolute trace of 1 is odd. For even s, the two twists become isomorphic and have W0 type. Preserve the original outputs and correct the prior theorem/report rather than silently rewriting their interpretation.

Before the next call, predict `WeilDescentGenus(E(0,8),GF(2),c=z)=7`, source order 120, and a hyperelliptic curve/divisor-class map. Submit only this public equation to Magma 2.30-1, retain the full response, and stop on model, genus, or map-type mismatch. If successful, compare the returned target against the exact `Y_+(v,u)=(0x8f,0x802f)` by point counts or isomorphism before identifying them. The m=3 construction is Hess's published family, so a positive result is a known-family transfer control.

## Amendment B: actual curve representation and evidence limit

The prediction that the genus-seven output would be hyperelliptic was false. Magma 2.30-1 returned a genus-seven `CrvPln` and a `DivCrvElt` evaluated at a point of order five. This agrees with Menezes–Teske's W0 nonhyperelliptic classification. The returned plane model is not identified with the separate hyperelliptic `Y_+`; equal Weil polynomials would only give an isogeny class. The corrected interpretation uses Hess's kernel theorem: for `c=z`, `r=sqrt(b)=z^65`, and `r/c=z^64`, verify their Frobenius annihilators directly before asserting that the five-primary subgroup survives the map. The printed divisor is an evaluated image, not a computed Jacobian-class order. Preserve the first response unchanged.

## Amendment C: stronger published isogeny barrier

A subsequent primary-source check of [Hess, *Generalising the GHS Attack*, Section 6](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) found the general congruence: absolute trace-zero binary elliptic curves have order 0 modulo four; trace-one curves have order 2 modulo four. Thus W1-to-W0 isogenies over the same field are impossible. The finite q2 product screen remains a reproducible positive control and exact coefficient census, but its missing trace −9 is already explained by this published obstruction. Do not describe that absence as a newly discovered exclusion theorem.

## Amendment D: correct the Hess kernel exponent

The first report gave `2^m−1=7` for m=3. Hess's Theorems 7 and 14 give `2^(m−1)=4` torsion under the non-subfield and rational-base hypotheses. Here `j=z^124∉F2`, so the degree-seven source is not a subfield curve, and gcd(5,4)=1. The subgroup preservation conclusion remains valid. The corrected report and kernel receipt replace the bound, while the original prediction remains in Amendment B.
