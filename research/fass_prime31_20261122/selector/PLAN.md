# Split-selector resultant: correctness development plan

Written 2026-11-22 before implementation calls. Parent: PR #4921.
Scope: native Rust algebraic two-coordinate relation extractor for
S3(X,Y,t)=0, X^d=1, Y^d=1. This is candidate C of the original FASS
protocol; the full chained-S3 and complete cold ECDLP objectives stay open.

## Exact candidate and predictions

Work in Fp[X]/(X^d-1), without dividing by the quadratic leading
coefficient. Compute the selector resultant from the Lucas/Dickson recurrence
with binary doubling. Extract support with gcd, recover coordinate pairs,
and replay rational signed curve points. Candidate extraction may not call
the exhaustive reference or use a planted answer to restrict its domain.

The predicted result is equality of complete coordinate-pair sets with a
separate exhaustive polynomial oracle and complete signed-point pair sets
with a separate group-addition oracle. Any disagreement fails the check.
Empty sets are retained; an exception/refusal is not an empty solution set.
No speed, operation-cost, relation-yield or asymptotic experiment is admitted
by this development plan.

## Frozen unit-test scope

- Resultant identity: all coefficient triples over F5 and F7 for selector
  degrees 1 through 6, against the existing Sylvester determinant with
  formal degrees, including zero, linear and constant quadratics.
- Polynomial identity: coefficient-polynomial specializations over F17
  and F31, including zero-divisor leading coefficients, against the explicit
  product over all selector roots, then reduce by the selector.
- Small fields: nonsingular curves (p,a,b)=(7,1,0),(13,2,3),(17,2,2),
  (31,2,3), selectors d<=12 dividing p-1, and every target abscissa.
  Compare all ordered coordinate pairs. For each rational target ordinate,
  compare all ordered rational point pairs whose sum is the target.
- Edge fixture: E/F7:y^2=x^3+x, target (0,0), d=3. Coordinates (2,4)
  satisfy S3 and selectors but have no F7 point lifts; they must be rejected
  by signed lifting. Include diagonal/tangent cases and d=1,2 and odd d.
- Prime31: p=2147483647,a=847514846,b=2087444724. Reuse parameters from
  examples/prime31_batched_ic_same_1024q.rs (also used by #4920/#4922).
  Declared r=2147446831 and G=(625037715,2100627251) are checked by
  primality/point/order replay; this test is not an independent point count.
  Selectors d=6,31,1302; construct one positive public target from two
  selector point lifts for each d, compare the complete extracted coordinate
  pairs with the exhaustive selector-pair polynomial oracle, and replay every
  signed result. These are chosen correctness fixtures, not random workloads.
- Refusals: composite/even/out-of-range moduli, nonsplit selectors, zero or
  excessive degree, singular curves, noncanonical coefficients/coordinates,
  invalid target points and nonpositive order constraints.

## Execution and source binding

Use ordinary local development builds/unit tests under AGENTS.md section 9a.
Build the actual production module and reused dependencies in a small native
qualifier. Prefer complete unchanged modules; declare any source-range
extraction if required. The qualifier does not replace the whole-library gate.
Separate build, pre-call and post-call manifests; refuse overwriting pre-call
records. Copy executables to a read-only path and hash before each execution.
Preserve failed attempts, exact commands, toolchain, output and exit status.
Use finite command deadlines, one build/test process at a time. No paid
compute, timed benchmark, cloud fallback or relaxed benchmark gate.

## Sources and coordination

KB unavailable: no exposed MCP and repository wrapper lacks its executable.
Conductor unavailable: command not found; use isolated checkout and disjoint
parent/child edit scopes per docs/CONDUCTOR.md. Read-only audit checked #4920,
#4921,#4922,#4692,#4822,#4812,#4368,#4228 and relevant Rust modules.
Use semaev_prime_poly polynomial/field arithmetic, pkm_smooth_tower point and
root routines, and semaev_projection quadratic roots. Existing
ic_framework/subgroup_semaev.rs owns the same-base F4 comparator.
Primary sources read: Kosters/Yeo arXiv:1503.08001v3 section 2; Mathlib
Polynomial.Dickson documentation. Semaev 2004/031 primary search text was
available; direct ePrint PDF access returned HTTP 403. No novelty claim.

## Next admitted experiment

After correctness, compare original/direct and power-chain F4 with the
resultant on identical factor bases and public targets, charging setup,
polynomial work, extraction, failed targets, signed lifting and useful rank.
Retain the optimized O(d) generic signed-point scan as a separate-track
reference. Performance requires the original protocol, source/image,
checkpoint, live budget/capacity and authenticated EKS admission. Whole cold
ECDLP adds relation collection, filtering, scalar-mod-r linear algebra and
individual recovery; this extractor alone closes none of those cost fields.

## Coverage addendum, before the second test execution

The original 13 tests passed in attempt2, but all three Prime31 fixture builders
selected the same diagonal/doubling target. Preserve those outcomes and add two
targeted d=1302 checks to address the untested Prime31 off-diagonal and fixed-target
paths: (1) choose two rational selector points with distinct x coordinates before
calling the extractor; (2) use the already fixed public generator G as target,
without searching for a successful decomposition. Compare complete coordinate
and signed sets in both cases, retain empties, and leave every previous test
unchanged. Record these as attempt3 with new source bindings. This adds coverage
and changes neither the candidate nor any accepted equality condition.
