# Preserving polynomial coefficients during construction

The first frozen `--check` call failed at equality with the production S3
evaluator. Its separately frozen example-test binary then reported one pass
and four failures. Both attempts and their original constructor source are
retained as `correctness-01-*`, `example-tests-01-*`, and
`constructor-before-monic-fix.rs`. These are unsigned development records.
[pcc:unverified]

Cause: the builder called `f4_fp::normalise` after each intermediate polynomial
sum and product. That production function explicitly makes a nonzero result
monic. Rescaling a completed equation preserves its zero set, but rescaling
an expression before it is combined with another expression changes the
equation, including the Dickson selector's constant term.

Correction: merge equal monomials with the same production Fp32 arithmetic,
discard zero coefficients, and use the production grevlex comparator without
rescaling. The completed equations are handed unchanged to the production
solver, which may normalize its own equations. The original numerical
identity, horizontal-line, M31 assignment and complete tiny-solve checks are
retained without weakening their assertions. A new build/call attempt must
use new manifests and immutable binary copies; the failed records stay intact.
