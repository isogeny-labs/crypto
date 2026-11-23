# Prime31 run 53 — exact seven-cubic splitting of signed y^7 factor-base cosets

Date: 2026-11-21 (America/Los_Angeles). **Claim: component-only algebraic reduction, independently reproduced in Python; Rust CI currently pending.** This is not an unknown-target decomposition, a complete ECDLP solve, or a speedup claim against optimized rho/BSGS/IC.

## Public mathematical identity

Let E/F_p be y^2=f(x)=x^3+A x+B, with p=2^31-1, A=847514846, B=2087444724 and 7 | p-1. For nonzero public c=y_0^7, fix public y_0 with y_0^7=c. Let zeta=3^((p-1)/7)=1752599774 mod p, which has order exactly seven. In F_p[X]:

```
f(X)^7-c^2 = PRODUCT_{j=0}^6 ( f(X)-y_0^2*zeta^(2j) ).
```

Proof: the seven constants y_0^2*zeta^(2j) are *all* seventh roots of c^2, because zeta^2 is also primitive of order seven. Apply T^7-c^2=PRODUCT(T-y_0^2*zeta^(2j)) with T=f(X). Every factor is **cubic**.

For a root x in the j-th cubic the signed point is exactly (x, y_0*zeta^j); the sign is uniquely determined by the public y^7=c constraint (and agrees with y=f(x)^4/c from run52). If a cubic has zero/one/three rational roots, enumerate only its actual F_p roots; "perfectly split" is a property of the selected cosets, not a universal guarantee. In this frozen instance each of the **56** cubics has three distinct rational roots. No secret scalar or discrete logarithm enters the reconstruction.

This identity is elementary Kummer/cyclotomic factorization, **not** asserted as a new algorithm or unpublished mathematics.

## Exact controls

The independent Python reference parsed the user-supplied run52 `eight_leaves.tsv` (168 frozen public points), using only eight public pairs (c,y0) to generate candidate roots. It re-used an independently written Cantor–Zassenhaus splitter on both candidate representations and validated every root **after** recovery.

- 8/8 identities match the full degree-21 coefficients;
- 56/56 cubics split into 3 distinct F_p roots (168/168 x-roots), with no cross-coset overlap;
- every recovered x set equals the archived 21 x values for that coset;
- recovered y satisfies both y^2=f(x) and y^7=c, without a sqrt or sign oracle;
- all 56 triples satisfy x1+x2+x3=0 (Vieta), and an independent Python elliptic-curve implementation checked each *signed* horizontal triple sums to the identity. This recovers run46's 56 geometry relations directly from the public cubic factorizations; it does not provide the other 112 relations needed for full logarithm recovery.
- zeta^7=1 and zeta!=1; after seven fibers y returns to y0.

No points from the frozen target decompositions were used in the candidate root solver.

## Paired component timing: local Python only

Same Python modular polynomial arithmetic, same host, both methods using Cantor–Zassenhaus and deterministic independent seeds; each repetition factors all eight cosets to 168 roots. Five successive reps:

| Method | Run times (ms) | Median (ms) |
|---|---|---:|
| Eight generic degree-21 factors | 193.494, 190.326, 196.365, 197.763, 191.444 | **193.494** |
| 56 specialized cubic factors | 39.345, 41.740, 39.160, 43.346, 36.655 | **39.345** |

**Observed ratio = 4.918× for factor-base acquisition in this Python reference only.** A source-linked Rust paired benchmark has been committed as `examples/prime31_coset_cubic_factor.rs`, reusing `cryptanalysis::pkm_smooth_tower::{poly_roots, poly_mul, mulm, powm, Rng}`; no new Rust polynomial solver or elliptic-curve code. Its exact checks are outside the timed region, and it separately times the same Rust root finder on eight 21-degree polynomials vs 56 cubics. The CI workflow `.github/workflows/prime31-f4-field-gate.yml` runs it first, followed by the existing field-width and F4 coset gates.

**Do not transfer the Python 4.918× ratio to Rust.** At this writing, GitHub Actions run is still in progress and no Rust PASS or same-host Rust timing is certified. The 204.55-ms Python degree-21 timing in run52 was an earlier, separate measurement and is not substituted for this paired control.

## Why this is not the needed extraction breakthrough

The new factorization makes **one-time public factor-base construction** cheaper. It does not solve for leaf indices from an arbitrary public Q. Replacing every signed degree-21 constraint in a 7-leaf system by one cubic requires a branching choice of seven fibers **per leaf**; naive full branching means 7^7 = 823,543 branches per seven-leaf attempt, before solving any F3/group-law equations. Therefore it would be unsound to claim that this factorization automatically compresses the blind multivariate F4 system.

The existing Prime31 [#4812](https://github.com/isogeny-labs/crypto/pull/4812) transformed-slot F3/Horner chart (14 variables, max degree 8) and [#4822](https://github.com/isogeny-labs/crypto/pull/4822) four-variable RR9 path are still the more promising extraction admissions; [#4692](https://github.com/isogeny-labs/crypto/pull/4692) owns the registered B8 public-only corpus. Do not duplicate those experiments. [#4793](https://github.com/isogeny-labs/crypto/pull/4793) reported 170.113 ms for 1,024 historical shared BSGS targets on its runner; the old cross-run 0.0931-ms-per-seven-leaf-attempt sensitivity still does not price relation collection, LA, fallback, or same-runner validation.

Published prior work: [Yokoyama–Yasuda–Takahashi–Kogure (2020), DOI 10.1515/jmc-2019-0029](https://doi.org/10.1515/jmc-2019-0029) gives structural lower-bound analysis for naive Semaev prime-field index calculus under statistical assumptions. This factorization is compatible with that caution and is not a counterexample.

## Prioritized go/no-go gates

1. Observe source-specific native Rust CI for the new cubic factorizer and existing univariate F4 path; compare full certified same-runner factor-base setup cost. If the Rust branch does not build, fix source integration before making claims.
2. Use the **existing** #4692/#4812/#4822 public-only blind extractor suites with positive and negative targets, fixed 1-thread CPU/memory/time caps, explicit incomplete-degree/timeout/OOM classifications, measured rows/columns/field operations and actual residual checks.
3. If blind roots appear, integrate them into the already checked rank-112 relation/target-recovery pipeline, charging negative attempts, complete factor-base setup, group arithmetic, linear algebra, exceptional branches and fallback.
4. Only publish an advantage when independently replayed whole-solver Rust time (cold 1 and amortized 32/128/1024/4096 same targets) beats matched optimized negation-BSGS, rho and the best existing prime-field IC.

**Standing:** exact factor-base primitive validated; the target non-generic complete ECDLP advantage is unestablished.
