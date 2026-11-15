# Runtime source comparison: independent review

T-1963. Source inspection and symbolic derivation only; no runtime, compiler or scientific measurement.

## Identified change

Twenty selected old/new Git blobs match the producer bindings. pencil.rs is identical between3c592af and29861c9. Candidate555a leaves every production byte before cfg(test) unchanged and adds the Some/None known-answer test. The transitive solver is different: both dispatched routes replace Horner evaluation with finite differences. Some/None tests therefore compare two new routes, not the old and new algorithm.

## Local derivation

For quadratic u, initialize value=u0, delta=u1+u2, second_difference=2u2. Advance value by delta and delta by second_difference. Linear v advances by v1. This gives the same polynomial values over the field. Canonical residues and0<p<=2^63 bound each addition by2p-2<2^64. Active degree is at most2; missing coefficients are zero and inactive tails excluded.

Let k=2d+1 count active coefficients in u and v. The original zero-initialized Horner folds charge k multiplications, k additions and k entry visits for each of p points. New setup charges2 additions and k visits; every iteration charges3 additions, including the final advance whose value is unused. New-minus-old local deltas are M=-kp; A=3p+2-kp; visits=-k(p-1). CSV records degree-specific symbolic formulas, not measured costs. For degree0 additions increase by2p+2; degree1 adds2; degree2 changes by2-2p.

## Instrument classification

CLOCK_MONOTONIC nanosecond bookkeeping replaces Instant/Duration on64bitLinux. Native tracing/capture, callable applicability and other selected sources change too. Consequently the new candidate must be classified as an algorithm and measurement-instrument change. Historical diagnostic-only/algorithm-unchanged wording cannot describe it. This observation does not authorize a new executable/image or scientific run.

Retained component reports state6398orderedpairs and associated sequence/negative controls; the retained independent review reports parsed counterpairs. They were read, not rerun here. Their scope excludes complete root integration. Original failures and old qualifiedruntime remain intact. Next bind changed source/operation mapping and separately qualify the intended executable/image; byte-identical helpers may be reused where identity is established.

Sources: runtime-source-comparison-20261109/bindings.json and fullselecteddiff; exact Git blobs; candidate-pencil.rs; point-sieve-progress-route-integration/result.json and root-postreview.json. KB unavailable; exact primary repository sources used.
