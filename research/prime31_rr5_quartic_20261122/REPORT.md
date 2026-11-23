# Prime31 RR5 monic-quartic four-leaf algebraic extraction — development gate

**Status:** implemented source and source-bound development workflow, not yet a qualified native execution or a complete solver. [pcc:unverified]
**Date:** 2026-11-22 Pacific / 2026-11-23 UTC.
**Curve:** unchanged ordinary cofactor-one E/F_2147483647: y²=x³+847514846x+2087444724, #E=2147446831 (reported prime), G=(625037715,2100627251). B4 uses four original-curve generator multiples as an explicitly **known-answer correctness control only**, NOT a qualified factor base or a cost competitor. The production base must be the merged original Rust B677 source from #4901.

## Exact finite-divisor identity

Let Q be a nonidentity public point, R=-Q=(u,v). For four affine, distinct-x points P1,...,P4 whose sum equals Q, R+P1+...+P4 is a degree-five divisor linearly equivalent to 5O. Since L(5O) has basis 1,x,y,x²,xy, there is a function F whose zero divisor contains R and the four Pi. In the regular chart with nonzero xy coefficient, normalize it to one and write

    F(x,y)=(x-u)(alpha+beta*x)+(y-v)(x+t).

The four variables (alpha,beta,t) are actually **three unknown coefficients**. Set U(x)=alpha+beta*x and V(x)=x+t. Eliminating y from F=0 and the curve gives

    D(x) = [v V(x)-(x-u)U(x)]²-(x³+A*x+B)V(x)²
         = -(x-u) M(x;alpha,beta,t),

where M is monic degree four. Because R is on the curve, division by x-u is exact and does not require an inverse. M has coefficients of degree at most two in the three unknowns.

Let h(x)=product_(w in W)(x-w) for the distinct public factor-base abscissae. The four equations

    remainder(h,M)[0..3]=0

are an exact polynomial divisibility condition M | h because M is monic. Their construction uses the coefficients of h and quadratic expressions for M; it enumerates neither factor-base pairs nor possible four-leaf sum tuples. For every algebraic solution, factor M over the original Fp by the repository's counted Frobenius root finder, uniquely recover signed ordinates by

    y_i = v - (x_i-u)(alpha+beta*x_i)/(x_i+t),

then verify (x_i,y_i) lies on the original curve, its x-coordinate indexes the public base, and the signed sum equals Q with original Rust group arithmetic.

The sign is thus recovered from the function, not selected by brute-force group joins. A verified signed relation for T=Q+[k]G becomes

    SUM_i eps_i log(P_i) - log(Q) = k mod 2147446831.

The Rust development gate includes a modular signed-row independence accumulator; this is only an interface step toward the merged authentic-RHS solver, not final scalar recovery.

## Exact scope and exceptional cases

The above chart covers **distinct abscissae**, finite Q, and every solution whose normalized V is nonzero at the four roots. If the xy coefficient vanished, F would have pole order at most four at O and could not have the five distinct zeros R,P1,...,P4; this justifies the monic chart for generic distinct five-point divisors. In the implemented bounded gate, reject x_i=x_R, V(x_i)=0, duplicate roots, nonrational roots, and intermediate infinity instead of inventing a signed lift. Repeated-factor and cancellation relations, points paired under negation, the R=O branch, and base points sharing abscissae need **additional charts** before any completeness claim. Root-deadline and F4-degree exhaustion return **Undetermined**, not UNSAT.

Most importantly, this finite algebraic presentation does **NOT** establish that solving the three-variable system is inexpensive: expanding the four remainder polynomials can create O(B³) distinct monomials for B large; ordinary Lex F4 may have a still larger Macaulay cost. The current source refuses B>8 in this expanded diagnostic. A viable B677 method requires a compact remainder circuit, structured elimination, and a root solver without costly hidden enumeration. No asymptotic speedup follows from the input representation alone.

## Prior work / already explored — do not double count novelty

* 2014: Faugère–Huot–Joux–Renault–Vitse, Symmetrized Summation Polynomials, EUROCRYPT; DOI 10.1007/978-3-642-55220-5_3. Symmetric Semaev coordinates are **not** new.
* Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1: prime-field auxiliary rational-map index calculus.
* Amadori–Pintore–Sala, Finite Fields Appl. 51 (2018), DOI 10.1016/j.ffa.2018.01.009: one-factor-base-relation stopping comparator.
* Internal Nov 15 m4 support-resultant: research/prime31_ic_20261115/native/M4_PAIR_SUPPORT_RESULTANT_20261115.md. Algebraic signed B4 positive existed already, B8-to-B16 naive quotient rose ~60x; the RR5 monic chart is an **alternative representation**, not a first four-sum extractor.
* Prior radical pair image: research/radical_pair_image_20260921/RESULTS.md; merged #4924 includes degree-B radical moment / Newton circuit and native B677 two-leaf gate.
* #4692, #4812, #4822 are existing nonenumerative coefficient/RR9/F3 lanes; #4920 owns chained S3; #4922 owns target-independent shared elimination. Avoid rewriting their backends.
* Merged #4901 contains real authentic-RHS signed-graph recovery, with an explicitly **generic** 916,469-key pair-dictionary calibration. Run72 was approximately 4.83–5.40x slower than same-process shared BSGS on 32 actual logs, not a measured best-IC or optimized-rho advantage.
* The project research KB endpoint was not available in this connector environment. This source/PR/literature audit is a bounded fallback, **not** an exhaustive novelty check.

## Development checks and falsification gate

Source: examples/prime31_rr5_quartic.rs; workflow: .github/workflows/prime31-rr5-quartic-gate.yml.

* First run native original-Rust B4 exact-witness test: construct Q=G+2G+3G+4G, independently recover alpha,beta,t from the four public signed points **after** producing equations, check M equals the product of the four x-roots, check all remainders vanish, derive each ordinate, verify full signed group relation. The fixed generating scalars only define the known-answer fixture; no coefficient or log is supplied to F4.
* Poison Q to 17G, check the public equations genuinely differ and the known certificate no longer matches; this does NOT prove that 17G is a no-relation target, since other signs/tuples may represent it.
* Run original F4 at a fixed degree cap 12 and five-second pilot budget. Feed only public polynomial equations. Root isolation is restricted to triangular univariates and uses original native finite-field factoring; **no** p-scan fallback. Outcomes are ExactRows, ExactEmpty, or Undetermined (incomplete F4/triangular/deadline), with F4 mulmods and root-factoring mulmods separately recorded. All finished candidates require exact input-system and point-law replay.
* Compare the resulting signed coefficient rows in modular rank against existing homogeneous base relations; duplicates contribute zero rank. This requires importing the genuine B677 base and complete target-quotient stopping from merged #4901 in a next implementation step; **do not** treat known-answer B4 ranks as a complete DLP solve.
* Source-bound CI is a development correctness attempt, **not** a preregistered isolated performance benchmark. A workflow push may trigger before this document was committed; that attempt is explicitly inadmissible as a speed claim. Retain failed compiler/runner logs.

## Frozen next full-cost experiment — not yet executed

The accepted benchmark must use the unchanged Prime31 source curve and genuine original B677 factor base; fresh public blind targets plus negative/exception/censored controls, independent seeds, and at least two additional ordinary prime-order curves. Compare on identical target(s) in the same isolated resource envelope:

| Arm | Discovery type | Mandatory costs |
| --- | --- | --- |
| RR5 monic quartic / compact quotient F4 | candidate algebraic | factor-base construction and h product, symbolic/circuit setup, every failed and successful F4 call, rational roots, signed lifts, rank update, linear algebra, individual recovery, verification, memory |
| November-15 m4 support resultant | algebraic but O(B²) pair-coordinate construction | entire K product tree, target norm/GCD, factorization, signs/rank/recovery |
| #4924 balanced two-leaf / radical DRL | algebraic extraction component | all cached/precomputed work and the same complete rank/target pipeline |
| best previously measured IC recipe | incumbent IC, not just naive S3 | frozen original library and matching public targets |
| optimized Prime31 negation-map rho (prime_fast tuned), plus shared BSGS diagnostic | generic control | full single-target cold cost, correct negation fold, hash/canonicalization, memory |

Measure cold one-target S=total counted group-addition equivalents/sqrt(n), plus same-host wall, w-band 0/0.1/1, calibrated bitops for field work, and a separately labeled amortized 32/128/1024-target interval. Include whole-score confidence intervals, resource/isolation tier, A/A, external msolve/Magma F4 when available, independent replay of every [d]G=Q and validation ledger. Do not claim cheaper complete solver until the measured candidate beats the strongest applicable complete IC and the matched optimized rho in the appropriate comparison regime.

**This run's truthful milestone:** a concrete exact public-only four-summand polynomial system, bounded F4/triangular root path, and signed-row/rank interface have been checked into GitHub. Native correctness and any B677 solver advantage are **not yet established**. [pcc:unverified]
