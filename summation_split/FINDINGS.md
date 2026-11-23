# Semaev-only degree lowering — correctness and negative Gröbner control

**Date:** 2026-11-23. **Standing:** producer-owned measurements; no independent replay, no PCC signed claim ([pcc:unverified]). **Scope:** only S_(m+1)(x1,...,xm,X)=0 over nonsingular odd-prime E: y²=x³+ax+b. No factor-base map or ECDLP performance claim.

## Exact construction and invariant

Keep m−2 intermediate partial-sum abscissae T_i, with homogeneous/projective infinity charts and T_i ∈ P¹(Fp²), as a chain S3(x1,x2,T1), S3(T1,x3,T2), …, S3(T_(m−2),xm,X). If a T_i=∞, it represents the group identity and its neighbor equality is used instead of invalid affine S3 substitution. All S3 chart equations have total degree ≤4; the coordinate field equations u^p−u=v^p−v=0 have degree p and must be charged.

**Invariant:** Every x∈Fp admits two geometric lifts on E(Fp²), since any square root of an Fp element lies in Fp². Their partial sums remain in E(Fp²), including identity. Hence every zero of the true S_(m+1) over Fp admits such an Fp² projective chain, and any consistent chain gives a signed zero sum by choosing intermediate point signs. Thus the projected **sets of Fp input points** agree. This does **not** establish equality of elimination ideals, multiplicities, bounded first-fall degree, or improved F4 complexity.

## Correctness gate

Reproducible seed 12042026, 50 independent deterministic random x-tuples per parameter cell:

| (p,a,b,m) | instances | total projected X-roots | resultant = projective chain = Fp² point-addition |
|---|---:|---:|---|
| (5,0,1,3) | 50 | 46 | pass |
| (7,1,1,3) | 50 | 64 | pass |
| (11,1,1,3) | 50 | 54 | pass |
| (11,1,1,4) | 50 | 68 | pass |
| (19,2,3,4) | 50 | 43 | pass |
| (19,2,3,5) | 50 | 70 | pass |

All 300 of 300 exact root-set gates passed in locally run Python/SymPy tests (not independent verification). For m=3, symbolic S4 rewritten in elementary symmetric e_i with quadratic recurrence ties matched the resultant in all 150 cases for **every X∈Fp**. m=4,5 full symmetric terminals are unimplemented, so those arms have no claim of validation. Invertible difference coordinates reconstructed all tested tuples.

## Gröbner diagnostic (negative)

bench.csv records 48 strategy/sample rows: 42 completed with exact projected GB root sets and 6 explicitly unimplemented S5/S6 symmetrizations. SymPy lex Buchberger, not F4; all x_i fixed, so the monolithic control is only *univariate in X*. Timings (L0 exploratory, only two samples per cell) are **not** a full Eq.(4) benchmark or statistically qualified speed comparison. They were measured only after every 50-case gate passed.

| p | m | median monolithic GB wall (s) | median projective Fp² chain GB wall (s) | chain/monolithic |
|---:|---:|---:|---:|---:|
|5|3|0.0113|0.0227|2.01×|
|7|3|0.0061|0.0222|3.67×|
|11|3|0.0076|0.0227|2.97×|
|11|4|0.0080|0.0971|12.2×|
|19|4|0.0083|0.1030|12.4×|
|19|5|0.0120|2.103|175×|

Chain overhead grows with charts and auxiliary coordinates; it is **slower**, not faster, in this easy specialized control. This does not rule out gains in a positive-dimensional unspecialized system. Plot solving_degree_vs_m.svg displays **only generator degree**: monolithic X-degree 2^(m−1), S3 maximum raw total degree 4. **d_F4 and first-fall degree are unmeasured.** The maximum degree in the *final* reduced Gröbner basis is not an F4 solving-degree measurement.

The peak RSS counters describe subprocess high-water usage (including inherited fork memory), not incremental solver allocation.

## Smallest failing counterexamples

Over F5, E:y²=x³+1 is nonsingular (p=5 is the smallest admissible odd p>3). Baseline S4(0,0,1,1)=0, while no *finite* t∈F5 solves S3(0,0,t)=S3(t,1,1)=0. The required witness is **t=∞**. A different base-field-only counterexample is S4(0,1,0,1)=0 with the common finite intermediate roots t=2±2ω (ω²=2∈F5, nonsquare), lying in F25\F5. Thus an affine Fp-only chain is incorrect even before measuring complexity. Exact resultants must retain fixed Sylvester degree at degenerate leading coefficients to capture these exceptional fibers.

## Negative strategies and priority next proof obligation

- **Naive Fp-affine chaining:** loses valid solutions; Fp² and infinity charts restore the projected set, but increase work. Elimination restores the original zero set, not a proved ideal equality; saturation and chart gluing remain open.
- **Elementary symmetrization:** known in prior art; S4 terminal + quadratic e-recurrences are exact, but no lower-cost GB control. S5/S6 terminal circuits remain to be implemented.
- **Invertible linear recoordinatization:** no change to total polynomial degree; linear auxiliary equations eliminate back to the original ideal. Variable *individual* degrees may change but this alone supplies no asymptotic claim.
- **Balanced tree, chord/tangent group-law lift, nonresultant selectors:** future hypotheses. Price branch counts, all field equations, and elimination explicitly.

**Next decisive run:** over exactly the same unspecialized positive-dimensional S_(m+1) constraint, add a declared independent slicing scheme so all systems have comparable zero-dimensional fibers, then run Sage/Singular F4 and msolve/FGb on paired m=3,4,5 instances. Record actual maximum Macaulay degree, first-fall measured according to a fixed definition, wall, memory, chart totals, and witnesses; preserve 50-instance exactness. Degree-four generators alone cannot justify bounded first-fall or an asymptotic speedup.

**Repository prior work:** research/pdp_split_dreg_20261011/README.md previously found splitting lowered F4 degree in a distinct **binary Weil-descent** experiment but did not always lower runtime. research/notes/index-calculus/RESEARCH_SYMMETRIZED_SEMAEV.md covers known symmetrized formulations. **Literature:** [Complexity bounds on Semaev's naive index calculus method](https://www.degruyterbrill.com/de/document/doi/10.1515/jmc-2019-0029/html) (resultant, symmetry, degrees) and [On the first fall degree of summation polynomials](https://www.degruyterbrill.com/document/doi/10.1515/jmc-2017-0022/html), whose theorem applies to characteristic-two Weil descent, **not** the odd-prime S3 chain here. Research-KB MCP, Sage/F4/FGb/msolve unavailable in this runtime; local SymPy served as explicitly qualified fallback.

## Reproduce

    PYTHONPATH=. python -m summation_split.harness --samples 50 --timeout 4 --out summation_split
    PYTHONPATH=. python -m unittest -q summation_split.test_split
