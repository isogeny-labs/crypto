# R79: bounded local probe of the rejected R50 normalization candidate

The retained R50 finite-`u` normalP output is a **rejected candidate**:
its original producer emitted a coefficient-descent error, and the
later global `norTest` verifier timed out. R79 does not promote it to
an accepted normalization. It loads the exact retained 22-variable,
400-relation candidate, checks characteristic two and its literal
source map `(x,y,p)`, substitutes `p=0`, and computes the two candidate
quotient fibers at `y=0` and `y=1`.

Both finite quotient dimensions are **16 over F2**. The full R78
source-model factors `x^16+x^8+1` and `x^16` reduce to zero in the
respective candidate quotients. Their dimensions sum to 32, matching
the full source degree and the two quadratic-place degree budgets.
This is exact consistency evidence for the candidate at this fiber;
it does not check the generic source-map injection, reducedness,
normality, `norTest`, or the candidate's primitive branch factors.

Attempt `candidate-u0-79-01` printed a marker but emitted Singular
parser errors at a nested `ideal(...)` constructor before computing
dimensions. The strict error-line gate rejected it. The repaired
source/protocol were committed before attempt `candidate-u0-79-02`,
which passed exit 0, exact marker, zero-error and all dimension and
source-containment assertions. Both raw attempts and all 12 historical
source-byte pins are retained; six donor hashes match.

The next focused calculation is the reduced algebra of each
16-dimensional candidate fiber. Any primitive factors found there
remain candidate data until the global source-field and normality
checks or an independent local integral-closure proof pass.
