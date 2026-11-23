# Prime31 RR5 monic-quartic four-leaf algebraic extraction — development gate

**Status:** bounded native correctness checks completed for the public B4 example and the follow-up compact B8 extractor against declared production-source closures. These are unsigned development results; full root-library qualification and a complete B677 solver remain open. [pcc:unverified]
**Date:** 2026-11-22 Pacific / 2026-11-23 UTC.
**Curve:** unchanged ordinary cofactor-one E/F_2147483647: y²=x³+847514846x+2087444724, #E=2147446831 (reported prime), G=(625037715,2100627251). B4 uses four original-curve generator multiples as an explicitly **known-answer correctness control only**, NOT a qualified factor base or a cost competitor. The production base must be the merged original Rust B677 source from #4901.

**Continuation:** the [four-summand correctness report](../prime31_four_summand_20261122/REPORT.md) and [publication branch](https://github.com/isogeny-labs/crypto/tree/lane/prime31-rr5-complete-controls-20261122) carry the native extractor, authentic-RHS adapter, compact B8 presentation, and source-bound execution evidence after tracking-only [PR #4949](https://github.com/isogeny-labs/crypto/pull/4949) merged. This report preserves the original algebraic argument and full-cost protocol; the executed checkpoint below records the narrower results now available.

## Exact finite-divisor identity

Let Q be a nonidentity public point, R=-Q=(u,v). For four affine, distinct-x points P1,...,P4 whose sum equals Q, R+P1+...+P4 is a degree-five divisor linearly equivalent to 5O. Since L(5O) has basis 1,x,y,x²,xy, there is a function F whose zero divisor contains R and the four Pi. In the regular chart with nonzero xy coefficient, normalize it to one and write

    F(x,y)=(x-u)(alpha+beta*x)+(y-v)(x+t).

There are **three unknown coefficients** (alpha,beta,t). Set U(x)=alpha+beta*x and V(x)=x+t. Eliminating y from F=0 and the curve gives

    D(x) = [v V(x)-(x-u)U(x)]²-(x³+A*x+B)V(x)²
         = -(x-u) M(x;alpha,beta,t),

where M is monic degree four. Because R is on the curve, division by x-u is exact and does not require an inverse. M has coefficients of degree at most two in the three unknowns.

Let h(x)=product_(w in W)(x-w) for the distinct public factor-base abscissae. The four equations

    remainder(h,M)[0..3]=0

are an exact polynomial divisibility condition M | h because M is monic. Their construction uses the coefficients of h and quadratic expressions for M; it enumerates neither factor-base pairs nor possible four-leaf sum tuples. For every algebraic solution, factor M over the original Fp by the repository's counted Frobenius root finder, uniquely recover signed ordinates by

    y_i = v - (x_i-u)(alpha+beta*x_i)/(x_i+t),

then verify (x_i,y_i) lies on the original curve, its x-coordinate indexes the public base, and the signed sum equals Q with original Rust group arithmetic.

The sign is thus recovered from the function, not selected by brute-force group joins. A verified signed relation for T=Q+[k]G becomes [pcc:unverified]

    SUM_i eps_i log(P_i) - log(Q) = k mod 2147446831.

The Rust development gate includes a modular signed-row independence accumulator. It now binds the actual public base and target and verifies SUM_i eps_i P_i = Q+[k]G before changing its pivots. False signs, a false RHS, or repeated indices are rejected without rank mutation; duplicate valid rows add no rank. This authenticates the claimed equation before rank reduction. The public example still exposes a rank interface rather than final scalar recovery; the follow-up adapter and its bounded recovery checks are documented in the [continuation report](../prime31_four_summand_20261122/REPORT.md). [pcc:unverified]

## Exact scope and exceptional cases

The above chart covers **distinct abscissae**, finite Q, and every solution whose normalized V is nonzero at the four roots. If the xy coefficient vanished, F would have pole order at most four at O and could not have the five distinct zeros R,P1,...,P4; this justifies the monic chart for generic distinct five-point divisors. The implemented bounded gate explicitly excludes targets sharing a base abscissa, including x_i=x_R. It rejects invalid or duplicate base inputs and refuses a lift with V(x_i)=0, a missing rational root, a root outside the public base, or a failed signed point-law replay. An intermediate infinity during group addition is valid and is handled by the original group law; it is not itself a rejection condition. Repeated-factor and cancellation relations, points paired under negation, the R=O branch, and base points sharing abscissae need **additional charts** before a completeness claim covering those cases.

A failed coefficient or signed-lift replay makes the outcome inconclusive: the public example reports `undetermined` with `algebraic_complete=false`, and the native extractor reports `Inconclusive`. Failed assignments are not silently dropped to manufacture an exact empty answer. Root-deadline, F4-degree, and output-cap exhaustion also remain incomplete outcomes, never UNSAT. The [eight public-example checks](../prime31_four_summand_20261122/evidence/public-example-01/stdout.txt) cover these status boundaries, excluded inputs, and authentic-RHS rank mutations. [pcc:unverified]

This finite algebraic presentation does **NOT** establish that solving the three-variable system is inexpensive: expanding the four remainder polynomials can create O(B³) distinct monomials for B large; ordinary Lex F4 may have a still larger Macaulay cost. The public example refuses B>8 in this expanded diagnostic. The follow-up native extractor uses a compact quotient-product presentation for B5 through B8, described below; its bounded B8 success does not establish B677 feasibility. A viable B677 method still requires compact construction, structured elimination, and a root solver without costly hidden enumeration. No asymptotic speedup follows from the input representation alone. [pcc:unverified]

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

The following bullets retain the original development protocol. Actual source-bound executions are recorded in the subsequent checkpoint; these protocol statements alone are not execution receipts.

* First run native original-Rust B4 exact-witness test: construct Q=G+2G+3G+4G, independently recover alpha,beta,t from the four public signed points **after** producing equations, check M equals the product of the four x-roots, check all remainders vanish, derive each ordinate, verify full signed group relation. The fixed generating scalars only define the known-answer fixture; no coefficient or log is supplied to F4. [pcc:unverified]
* Poison Q to 17G, check the public equations genuinely differ and the known certificate no longer matches; this does NOT prove that 17G is a no-relation target, since other signs/tuples may represent it.
* Run original F4 at a fixed degree cap 12 and five-second pilot budget. Feed only public polynomial equations. Root isolation is restricted to triangular univariates and uses original native finite-field factoring; **no** p-scan fallback. Outcomes are ExactRows, ExactEmpty, or Undetermined (incomplete F4/triangular/deadline), with F4 mulmods and root-factoring mulmods separately recorded. All finished candidates require exact input-system and point-law replay.
* Compare the resulting signed coefficient rows in modular rank against existing homogeneous base relations; duplicates contribute zero rank. This requires importing the genuine B677 base and complete target-quotient stopping from merged #4901 in a next implementation step; **do not** treat known-answer B4 ranks as a complete DLP solve.
* Source-bound CI is a development correctness attempt, **not** a preregistered isolated performance benchmark. A workflow push may trigger before this document was committed; that attempt is explicitly inadmissible as a speed claim. Retain failed compiler/runner logs.

## Executed native correctness checkpoint — 2026-11-23

The public example completed **8 tests with exit status zero**, including actual public-input B4 extraction, signed-scalar oracle comparison, chart exclusions, limit handling, and RHS/rank rejection controls. This run compiled the example against its declared exact production-source closure; it did **not** compile or test the full root library. Its [output](../prime31_four_summand_20261122/evidence/public-example-01/stdout.txt), [exit status](../prime31_four_summand_20261122/evidence/public-example-01/exit-status.txt), and [pre-call source inventory](../prime31_four_summand_20261122/evidence/public-example-01/pre-call-sources.sha256) bind that observation. [pcc:unverified]

The separate native extractor retains the three-variable expanded B4 path. For B8 it introduces a monic quartic cofactor C and enforces M*C=h: all eight lower coefficients of M*C-h vanish, and the leading monic coefficient is tautological. This gives **eight equations of degree at most three in seven unknowns**, ordered with Grevlex. Because division by monic M has a unique quotient, the four auxiliary coefficients introduce no extra projected solutions. The native visitor replays every equation on the full assignment before retaining the three RR5 parameters for signed lifting. The [quotient-product addendum](../prime31_four_summand_20261122/QUOTIENT_PRODUCT_ADDENDUM.md) records this change separately from the earlier expanded attempts. [pcc:unverified]

| Executed gate | Observed outcome | Source-bound evidence |
| --- | --- | --- |
| Native regression, construction, and authentic-RHS checks | 17 tests passed; the two B8 gates were filtered here and run separately | [qualification-06 output](../prime31_four_summand_20261122/evidence/qualification-06/stdout.txt), [exit status](../prime31_four_summand_20261122/evidence/qualification-06/exit-status.txt) |
| Natural-x B8 positive target | `Complete`; one signed row, exactly equal to the independent expected row set | [b8-positive-02 output](../prime31_four_summand_20261122/evidence/b8-positive-02/stdout.txt), [exit status](../prime31_four_summand_20261122/evidence/b8-positive-02/exit-status.txt) |
| Natural-x B8 generator target | `Empty`; no signed rows, exactly equal to the independent empty set | [b8-empty-02 output](../prime31_four_summand_20261122/evidence/b8-empty-02/stdout.txt), [exit status](../prime31_four_summand_20261122/evidence/b8-empty-02/exit-status.txt) |

These are **19 native checks across three separately recorded invocations**, plus the separate eight-test public example. Each B8 gate compares against an independent test-only oracle examining all 70 distinct four-subsets and 16 sign patterns, **1,120 signed tuples per target**. The candidate receives only the public curve, base, target, and limits; it receives neither oracle rows nor a decomposition witness. Both B8 traversals completed with zero full-field branches, zero domain evaluations, and no rejected signed assignments. [pcc:unverified]

The unchanged B8 limits were degree 10, a 30-second candidate deadline, 20,000 terms per constructed polynomial, and 4,096 assignments. Each process had one Rayon/test worker, a 180-second outer timeout, and a 2 GiB address-space ceiling; the latter also covers F4 matrix allocation. These are bounded correctness observations, with complete phase accounting still false. They establish neither complete cold cost nor a B677 extraction or recovery result. Failed earlier attempts, their source snapshots, and their logs remain retained in the [continuation study](../prime31_four_summand_20261122/REPORT.md). [pcc:unverified]

## Frozen next full-cost experiment — not yet executed

The accepted benchmark must use the unchanged Prime31 source curve and genuine original B677 factor base; fresh public blind targets plus negative/exception/censored controls, independent seeds, and at least two additional ordinary prime-order curves. Compare on identical target(s) in the same isolated resource envelope:

| Arm | Discovery type | Mandatory costs |
| --- | --- | --- |
| RR5 monic quartic / compact quotient F4 | candidate algebraic | factor-base construction and h product, symbolic/circuit setup, every failed and successful F4 call, rational roots, signed lifts, rank update, linear algebra, individual recovery, verification, memory |
| November-15 m4 support resultant | algebraic but O(B²) pair-coordinate construction | entire K product tree, target norm/GCD, factorization, signs/rank/recovery |
| #4924 balanced two-leaf / radical DRL | algebraic extraction component | all cached/precomputed work and the same complete rank/target pipeline |
| best previously measured IC recipe | incumbent IC, not just naive S3 | frozen original library and matching public targets |
| optimized Prime31 negation-map rho (prime_fast tuned), plus shared BSGS diagnostic | generic control | full single-target cold cost, correct negation fold, hash/canonicalization, memory |

Measure cold one-target S=total counted group-addition equivalents/sqrt(n), plus same-host wall, w-band 0/0.1/1, calibrated bitops for field work, and a separately labeled amortized 32/128/1024-target interval. Include whole-score confidence intervals, resource/isolation tier, A/A, external msolve/Magma F4 when available, independent replay of every [d]G=Q and validation ledger. Do not claim cheaper complete solver until the measured candidate beats the strongest applicable complete IC and the matched optimized rho in the appropriate comparison regime. [pcc:unverified]

**Current milestone:** the public example's eight source-bound checks passed, and the native compact B8 positive and empty controls completed and matched independent exhaustive signed-row expectations under the frozen limits. Authentic-RHS and rank checks accompany that bounded extraction result. The next full-cost requirement remains genuine B677 relations, complete recovery, and the matched comparisons specified above. [pcc:unverified]
