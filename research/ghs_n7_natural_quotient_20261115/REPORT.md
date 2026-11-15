# Natural genus-seven quotient of the published n=7 W1 GHS cover

**15 November 2026 — scoped mathematical result and finite audit.**

For q=2^ell, K=F_(q^7), and either cubic p in T^7+1, let V_p=ker p(sigma) for sigma(z)=z^q. The published W1 genus-14 family uses gamma1 of sigma-order p and gamma2 of sigma-order (T+1)p, with source coefficient b=(gamma1 gamma2)^2. The natural rank-three Artin--Schreier quotient D has genus seven.

The geometric argument in [THEOREM.md](THEOREM.md) gives a **necessary bound**: if D is hyperelliptic, then b belongs to V_p. Every nonzero b in V_p has a symmetric W1 presentation whose D is hyperelliptic, and Menezes--Teske already gives that source a classical genus-seven GHS presentation. Across both cubic arms the hyperelliptic coefficient locus therefore has exactly `2(q^3-1)` nonzero b values, subject to independent review of the geometric descent step. This is a bound on one natural quotient construction. It does not exclude other covers or correspondences.

The same argument extends to any odd extension degree `n` and irreducible factor `p` of degree `s>=2` in `T^n+1`: the corresponding Hess cover has genus `2^(s+1)-2`, its natural quotient has genus `2^s-1`, and at most `q^s-1` nonzero coefficients can give that quotient a hyperelliptic presentation. A symmetric presentation attains the coefficient-locus bound. See [GENERAL_BOUND.md](GENERAL_BOUND.md) for the proof and its assumptions. This generalization is a structural screen, not a new vulnerability family.

The full n=7 genus-14 cover has 15 geometric degree-two deck quotients: seven of genus six and eight of genus seven. Only the natural genus-seven quotient is defined over `k`; [DECK_QUOTIENTS.md](DECK_QUOTIENTS.md) gives the conductor count. Hess Section 8.3 proves that the norm composition into this intermediate quotient is zero. The classical genus-seven GHS presentation uses a separate transfer map.

The q=2 and q=4 native census tested the algebraic pole-swap condition on every exact W1 presentation. It matched the prior independent W1 counts and found the swap only on classical coefficients:

| q | W1 coefficients per arm | Classical/swap coefficients per arm | W1 union | Classical union |
|---:|---:|---:|---:|---:|
| 2 | 42 | 7 | 56 | 14 |
| 4 | 2898 | 63 | 3906 | 126 |

Concrete nonclassical examples in the chosen field representations: q=2, modulus `0x83`, arm 1 `gamma1=2`, `gamma2=5`, `b=68`; arm 2 `gamma1=30`, `gamma2=47`, `b=120`. For q=4, modulus `0x4021`, arm 1 `(89,187,b=4109)` and arm 2 `(77,430,b=440)`. These examples are members of the known W1 eligibility family; they fail this natural hyperelliptic quotient screen. No claim about their attack cost follows.

The first run failed because the enumeration admitted gamma2 in k*, contrary to the exact Frobenius-order condition. [AMENDMENT_1.md](AMENDMENT_1.md) preserves the correction and the raw failed record. The corrected run passed all assertions: [result.txt](reports/run-20261115T0747Z/result.txt), [timing.txt](reports/run-20261115T0747Z/timing.txt), and [source hash](reports/run-20261115T0747Z/source.sha256). Lean 4 checked [NaturalQuotient.lean](NaturalQuotient.lean), an exhaustive finite F8 lemma used in the abstract involution argument; the curve geometry and descent are not Lean-formalized.

## Literature and novelty

[Menezes--Teske (2004)](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf) gives both the classical genus-seven cubic slice and the broader n=7 W1 genus-14 family. [Hess (2004)](https://www.cambridge.org/core/services/aop-cambridge-core/content/view/315278965D8D277A85812D9498236A88/S146115700000108Xa.pdf/generalising-the-ghs-attack-on-the-elliptic-curve-discrete-logarithm-problem.pdf) governs the transfer map and warns that some quotient norms kill the source subgroup. The exact natural-quotient criterion was not found in a targeted KB and web review, but that search is not a priority or novelty proof. The finite examples are new exact equations in this audit, not a new eligible family. The W1 family and classical genus-seven attack are previously published. A new attack method would require a nonzero subgroup map into a target with a complete same-subgroup cost below rho.

The next mathematical target is a different low-degree quotient or correspondence for **nonclassical** W1 coefficients, with a subgroup-preserving map. The natural hyperelliptic quotient route screened here cannot supply it. Independent function-field verification of the geometric descent is still needed before treating this as a formal theorem.
