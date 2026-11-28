# FASS: factorized adaptive summation systems over F_(2^31-1)
Status: proposed / not benchmarked / no asymptotic advantage established.
Date: 2026-11-22. Target p=2147483647, ordinary nonsingular short-Weierstrass E/Fp, subgroup order and cofactor to be verified for each instance.

## Exact identity and symmetric coordinates
For E:y²=x³+ax+b, let e1=x1+x2, e2=x1*x2. Then
S3(x1,x2,z)=(e1²-4e2)z²-2[e1(e2+a)+2b]z+(e2-a)²-4be1.
Recover x1,x2 as roots of T²-e1*T+e2, requiring discriminant e1²-4e2 to be square in Fp; require x1,x2 to lie in the registered factor base and check actual curve points and signed group relation.

For four summands, retain auxiliary u,v:
S3(x1,x2,u)=0,
S3(x3,x4,v)=0,
S3(u,v,xR)=0.
The existential system is equivalent over an algebraic closure (with usual Semaev sign semantics); over Fp, rational intermediate points and sign consistency must be checked, since elimination may introduce solutions with intermediate coordinates in extensions.

## Adaptive algorithm candidates
A. Symmetric pair elimination: solve in (e1,e2,u), use quadratic splitting to recover factor-base coordinates. Avoid expansion of S4/S5.
B. Balanced-tree join: precompute verified signed pair sums and indexed buckets for target residuals; compare to standard BSGS and meet-in-the-middle. This can save constants, not the exponent by itself.
C. Hybrid solver: choose direct root-finding, sparse resultant, or Groebner from estimated support, degree and actual profiling; use batch inversions and Mersenne reduction p=2^31-1.
D. Partial elimination + large-prime graph: keep verified partial relations, but count only independent full relations after cycle closure and linear algebra.

## Mathematical guardrails
The identity is known, not novel. FASS is a proposed algorithmic combination; novelty and asymptotic gains are unproved.
With factor base size B, exhaustive pair enumeration is O(B²) field/group operations and does not by itself yield sub-rho ECDLP. Any asymptotic claim must specify growing family, input parameter n=log2(p), factor-base scaling, relation success probability, solver complexity, linear algebra and target descent.
Measure independent relation rank, not raw relation count. Account for signs, duplicated x, torsion, exceptional denominators, curve singularities, subgroup membership and cofactor.

## Experiment protocol
1. Reuse src/cryptanalysis/semaev_prime_poly.rs and existing Rust rootfind/index-calculus infrastructure; inspect AGENTS.md and benchmark conventions first.
2. Freeze a curve over p=2147483647, prime-order subgroup and generator with independently verified point count. Do not choose deliberately weak curves.
3. Differential-test symmetric S3 and chained S3 against direct signed group addition on random and exceptional points.
4. Benchmark matched target samples, factor bases and seeds: expanded S_m, chained S3, symmetric S3, balanced joins, hybrid. Measure wall time, field mul/inversion, memory, successes, independent rank and full solver costs.
5. Sweep smaller primes and increasing bit lengths; fit slopes with confidence intervals. A constant-factor speedup is not an asymptotic breakthrough.
6. Compare against optimized negation-map rho and existing prime-field index-calculus frontier; include amortization over multiple targets.

## Existing work to avoid duplicating
- src/cryptanalysis/semaev_prime_poly.rs
- examples/pkm_tower_pilot.rs
- research/twist_index_calculus_20261020/MIXED_POLYNOMIAL_REPORT.md
- PR #4032 (non-Semaev decompositions)
- PR #4232 (isogeny pullback S3 negative toy result)

## Current outcome
Analytic symmetric rewriting and a falsifiable benchmark protocol. No Rust implementation, measured speedup, independent verification, or asymptotic improvement in this iteration.
