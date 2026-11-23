# Semaev decomposition-oracle replacements

The implementation starts with the two routes that can be made exact and
falsifiable without pretending an unbuilt cryptographic-scale solver exists.

| proposed route | status in this PR | decisive evidence |
|---|---|---|
| Parametric decomposition compiler | implemented as a complete target-function compression falsifier | exact interpolant density/degree and prefix-trained recurrence holdout |
| Projection-only Macaulay/Krylov | implemented at the narrower quadratic-link boundary as a no-GB projection join | exact agreement with independent signed group sums; table/query counts |
| FAQ / worst-case-optimal join | partial: the root-key join is the concrete join plan for the balanced Semaev tree | projected relation sizes are measured; no generic FAQ-width claim |
| Wagner-compatible filter | not implemented | remains open; must preserve recall and charge exact verification |
| Incremental SAT across targets | not implemented | remains open; must compare learned-clause reuse with F4 trace replay on identical systems |

The projection join is useful engineering even if the expected disposition is
`SO-TABLE`: it supplies a native, exact, no-Gröbner control and exposes the
actual projected relation sizes. The compiler experiment is deliberately
adversarial. If the target function is dense/high-order, that closes the cheap
interpolation story at toy scale. If it is small, the protocol requires a
disjoint confirmation before any performance claim.

Next experiments, after this frozen run:

1. exact rational-function interpolation of a nonconstant output coordinate,
   only if the dense/recurrence falsifier leaves the lane open;
2. a fingerprint/Bloom prefilter in front of the exact root-key join, reporting
   recall, false positives and verification cost (never accepting an
   approximate relation);
3. clause-learning reuse on the same balanced system and target sequence,
   compared with the committed F4 trace replay arm;
4. end-to-end relation-generation integration only if an oracle-stage arm
   beats the current native route after its build is amortized.
