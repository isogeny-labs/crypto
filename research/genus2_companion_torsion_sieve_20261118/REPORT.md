# Genus-2 companion-quotient torsion sieve: a toy negative result

Date 2026-11-18. Classification: CONJECTURE (mechanism); EMPIRICAL (toy data); REJECTED at tested sizes. This is an exploratory, not independently verified, result.

## Distinct hypothesis and prior-work boundary

For a split genus-2 curve with a smooth-order second elliptic quotient, the second quotient's torsion order may be a cheap **non-homomorphic sieve** on the first quotient. Hypothesis: for a base of B projected elliptic points, torsion-order selection increases exact two-sum target coverage by >1.20x relative to both uniform size-matched subgroup bases and size-matched cover-image bases, consistently across three toy fields. If it also lowered total oracle cost, that could improve individual descent. No claim of asymptotic advantage is made.

This is **not** the rejected trace-moment Frobenius factor base, isogeny transport, cover-fiber multiplicity, or known GHS/Joux–Vitse weak class. Scope check: crypto research/ic_mega_pass_20261031/brief/BRIEF.md, report/REPORT.md, report/GAPS.md, and GitHub code/PR searches for bielliptic, Prym and companion torsion. The geometry is old; we did not find the exact order predicate in this scoped search. A full KB search is not possible (research-kb MCP missing); novelty remains unverified. Conductor check/start unavailable in this runtime; PR conflict search completed.

## Explicit curves, maps and costs

C: y²=x⁶+a x⁴+b x²+1 over an odd field F_p.
E1: Y²=X³+aX²+bX+1, with pi1(x,y)=(x²,y).
E2: V²=U³+bU²+aU+1, with pi2(x,y)=(x^-2,y/x³), for x != 0.

These are degree-two quotients of a smooth genus-2 curve when the cubic t³+a t²+b t+1 is separable. The experiment checks both rational point coordinates for every enumerated cover point. It does NOT implement a full Jacobian transfer or divisor inverse.

Let r be a prime factor of #E1 and h=#E1/r. Map pi1(P) to T=[h]pi1(P) in the r-subgroup. Accept T when at least one associated Q=pi2(P) has [m]Q=O, with m a divisor of #E2 chosen by cardinality only. In every selected case the largest prime of #E2 is ≤13.

The cheapest exact unordered **exhaustive pair-sum table** requires Q=B(B+1)/2 group additions, with at most Q targets; uniform coverage is heuristically about r(1-exp(-Q/r)). An observed coverage gain gamma can at most lower matched online failed probes by gamma, provided selection and testing costs are unchanged. Preprocessing, verification, sparse linear algebra and descent remain. The generic preprocessing boundary B*D² >= order(r)/|Aut| has not been escaped. Compare an idealized negation-folded Pollard rho baseline sqrt(pi*r/4) elliptic additions (stronger than ordinary rho; an actually measured matched optimized rho was unavailable).

## Fixed toy design and outcomes

Seed 20261118; p=101,211,307; first lexicographic 0≤a,b<11 satisfying squarefreeness, prime factor r>p/2, cofactor h≤4, and #E2 with prime factors ≤13. Pick m minimizing |B-0.67*sqrt(r)| without consulting pair sums. For each arm, 128 independent seeded uniform bases from the whole subgroup and 128 from the cover image; an E1 x-coordinate sorted base is a secondary null. Exact modular group operations and subgroup-order checks throughout.

| p; (a,b) | r, B, m | Sieve / uniform coverage | Sieve / cover-image coverage | Pair-table additions / ideal folded-rho |
| --- | --- | --- | --- | --- |
| 101; (0,8) | 53, 6, 14 | 19/19.172=0.991x | 19/18.859=1.007x | 21/6.452=3.255x |
| 211; (2,9) | 109, 6, 10 | 19/20=0.950x | 19/19.945=0.953x | 21/9.252=2.270x |
| 307; (10,1) | 313, 10, 15 | 51/51.891=0.983x | 51/51.734=0.986x | 55/15.679=3.508x |

**Verdict:** Rejected for the specific torsion-order predicate and these cells: no 1.20x coverage improvement, and the exhaustive-pair-table arm alone is already >2.2x the idealized rho cost BEFORE cover enumeration, selecting m, verifying, matrix building, or recovery. This does not exclude a different companion functional, 3-sums, different distributions, or genus-2 transfer techniques. No end-to-end ECDLP or rank measurement was performed. Admitted-curve search and filter operations in RESULTS.json should not be mistaken for selected-curve setup; they include rejected curve candidates.

Replay with Python stdlib only:

    python3 research/genus2_companion_torsion_sieve_20261118/run.py > /tmp/genus2-sieve.json
    python3 -m py_compile research/genus2_companion_torsion_sieve_20261118/run.py

Code compiled and three exact cases ran locally. Regression replay: cd research/genus2_companion_torsion_sieve_20261118 && python3 test_run.py. It passed the three frozen outcome vectors, 1,536 independently drawn group-law associativity triples across E1/E2, and inverse checks. The run was not cryptographically preregistered before it executed; results need independent replay before marking independently verified.

## Primary literature and known results

- [Mega-pass BRIEF](https://github.com/isogeny-labs/crypto/blob/main/research/ic_mega_pass_20261031/brief/BRIEF.md) and [GAPS](https://github.com/isogeny-labs/crypto/blob/main/research/ic_mega_pass_20261031/report/GAPS.md): homomorphic transport conservation, genus-2 Prym open lever and generic-oracle barrier.
- Kani, [Elliptic subcovers of genus-2 curves, the isogeny defect](https://doi.org/10.1007/s40316-018-0105-6) (2019): bielliptic quotients are established prior art.
- Gaudry, [Hyperelliptic discrete logarithms](https://doi.org/10.1007/3-540-45539-6_2) (EUROCRYPT 2000); Diem, [double-large-prime index calculus](https://arxiv.org/abs/math/0606607) (2006): do not transfer Jacobian complexity claims to an elliptic factor without costing maps.
- Semaev [summation polynomials](https://eprint.iacr.org/2004/031), Faugère–Perret–Petit–Renault [Boolean Weil-descent solver](https://eprint.iacr.org/2012/019), and Petit–Quisquater on Weil descent (ASIACRYPT 2012): established IC machinery, not implemented here.
