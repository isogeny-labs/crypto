# Rigorous size bounds for the published n=7 two-pole descent families

15 November 2026. [Frozen protocol](PROTOCOL.md). This note separates **eligible source equations**, **direct genus-seven maps**, and **Jacobian isogeny-class matches**. It derives elementary counting bounds from the published [Menezes–Teske W0/W1 definitions](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) and [Hess's two-pole genus formula](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf). The bounds do not classify every possible GHS cover or prove an attack faster than rho. [pcc:unverified]

## The general q bounds

Put `k=F_q`, `K=F_(q^7)`, `q=2^s`, and `σ(x)=x^q`. Over F2,

`T^7+1=(T+1)(T³+T+1)(T³+T²+1)`.

Let `V_i=ker p_i(σ)` for the two cubic factors. Each V_i is a three-dimensional k-vector space, so `|V_i|=q³`; their intersection is zero. The ordinary elliptic equations `E(a,b): y²+xy=x³+a x²+b`, with fixed absolute trace of a and `b∈K×`, give `q⁷−1` isomorphism classes in each trace branch.

**W0 direct genus seven.** Here absolute `Tr(a)=0` and `b=(γ₁γ₂)²` with both nonzero factors in one V_i. Multiplication is unchanged by `(γ₁,γ₂)↦(λγ₁,λ⁻¹γ₂)` for `λ∈k×` and by swapping the factors. The number of pair orbits in one arm is

`[(q³−1)²/(q−1)+(q³−1)]/2 = (q³−1)(q²+q+2)/2`.

The second term counts scaled-pair orbits fixed by swapping: exactly those with `γ₂/γ₁∈k×`. Different pair orbits may still produce the same b, and the two arms may overlap. Therefore the number `N_W0(q)` of direct W0 coefficient values satisfies the **rigorous upper bound**

`N_W0(q) ≤ (q³−1)(q²+q+2)`, hence `N_W0(q)/(q⁷−1) ≤ (q³−1)(q²+q+2)/(q⁷−1)`.

This is asymptotic `O(q⁻²)` among trace-zero ordinary curve classes. The same pair-orbit expression appears in Menezes–Teske's W0 counting discussion; the inequality here deliberately makes no product-injectivity assumption.

**W1 cubic/(linear×cubic) family.** Here absolute `Tr(a)=1`, `γ₁∈V_i×`, and `γ₂=v+c` with `v∈V_i×`, `c∈k×`. The ordered pairs number `(q³−1)²(q−1)` per arm. Scaling by k× is free and preserves each product, so further collisions aside,

`N_W1(q) ≤ 2(q³−1)²`, hence `N_W1(q)/(q⁷−1) ≤ 2(q³−1)²/(q⁷−1) = O(q⁻¹)`.

The direct cover for this presentation has genus fourteen. This is an eligibility upper bound for the published family and does not imply each eligible source has a practical attack.

**Separate trace-one direct genus-seven slice.** In the two-pole genus formula, a trace-one a contributes the factor `T+1`. A non-subfield degree-seven source requires one cubic factor. The least possible annihilator degree is four. At that degree, genus seven requires factor-order degrees `(1,3)`; the alternatives `(1,4)`, `(3,4)`, and `(4,4)` give genera 8, 14, and 15, while `(3,3)` violates the trace-one descent condition. Thus a direct genus-seven two-pole presentation exists precisely when `b∈V₁×∪V₂×`. Choose the normalized `a∈k` with absolute trace one. The two kernels meet only at zero, giving **exactly** `2(q³−1)` source classes in this classical trace-one slice and fraction `2(q³−1)/(q⁷−1)` among trace-one ordinary classes. This characterization is for the stated Hess two-pole setup, not for unrelated correspondences.

Finally, [Hess's mod-four congruence](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) separates the two isogeny classes of a: trace-zero curves have order zero modulo four; trace-one curves have order two modulo four. No same-field isogeny can move a W1 source into W0.

## Exact q2 partition and novelty status

The [cross-artifact check](q2_partition.json) reconciles the frozen CSV and W0 result hashes. The two independent q2 counters find 49 actual W0 coefficient values, below the general orbit bound 56. The W1 cubic/(linear×cubic) coefficient census has 56 values, below its general bound 98. The direct genus-seven W1 classical slice has exactly 14 values. In the **56 actual W1 equations** of the paired coefficient census, the trace partition is:

| Source subset | Exact equations | Hyperelliptic genus-seven Jacobian in same isogeny class | Direct genus-seven Hess route |
|:---|---:|:---|:---|
| Classical cubic slice | 14 | 14 | 14 |
| Nonclassical trace `−9` or `−1` | 28 | 28 | 0 in the classical criterion; the exact `−9` source's full c scan has minimum genus 14 |
| Nonclassical trace `−17` or `+7` | 14 | 0 in the complete F2 hyperelliptic census | 0 in the classical criterion |

The 28 additional isogeny-class matches are **geometry leads**. They do not provide explicit maps to the hyperelliptic Jacobians. The 14 exclusions apply only to genus-seven **hyperelliptic** Jacobians over F2; plane and other genus-seven targets remain outside that census. The +9 source from the paired twist control is published W0 and has a direct genus-seven plane descent. These counts use [the CSV](../ghs_n7_prym_trace_20261115/reports/run-20261115T0821Z/curves.csv), [the complete hyperelliptic classification](../ghs_n7_prym_trace_20261115/HYPERELLIPTIC_CLASSIFICATION.md), and [the W0 product result](../ghs_n7_isogenous_w0_20261115/result.json). The general density bounds are deductions from published constructions, not a new vulnerable family. [pcc:unverified]

A genuinely new eligible family must use a construction outside these counted two-pole profiles or construct and verify a new subgroup-preserving route on a source outside their known direct loci. A vulnerability claim further requires complete measured target recovery and a lower total cost than rho on the same source subgroup. The [genus-floor theorem](../ghs_n7_base_change_20261115/GENUS_FLOOR.md) gives genus at least seven for the nonexceptional simple Weil restrictions; that is a geometric lower bound, not an attack-cost bound.
