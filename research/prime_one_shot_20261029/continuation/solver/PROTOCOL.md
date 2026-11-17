# Unknown-coefficient solver development contract

This is a bounded native **correctness test**, not a performance experiment or
asymptotic claim. Freeze this file before executing the checks. No EKS scientific
run, timing, degree-growth fit, DLP solve or speedup is part of this contract.

The candidate receives only `(p,a,b,t,c,x_R,y_R)`, never source summands,
generation scalars, a norm-function witness or a precomputed decomposition table.
Its fixed arity is three. It writes every rational selected-chart decomposition.

For `A=X^2+uX+v`, `C=w`, the target equation eliminates
`v=y_R*w-x_R^2-u*x_R`. The candidate visits all `(u,w)` in
`F_p x F_p^*`, forms the norm quotient `U=(A^2-f*w^2)/(X-x_R)`, checks squarefree
`U`, target separation and `U | X^(2^t)-c` by repeated remainder squaring, and
recovers rational points as `(x,-A(x)/w)`. This is explicitly a `p*(p-1)`
coefficient search plus all other work. It is exponential in the bit length and
is a correctness baseline, not the proposed sub-exponential solver.

An independent reference executable enumerates signed on-curve point triples,
requires distinct abscissas different from the target abscissa, and sums them by
its own affine group-law implementation. It does not use the norm formulation.
The two complete canonical model sets must agree exactly, not just in count.

Fixed unit fixtures: `E/F17: y^2=x^3+2x+2`, every affine target (18 targets), for
five split membership domains `(t,c)=(1,1),(2,1),(3,1),(3,16),(4,1)`. These give
90 fully specified model-set checks on one small curve, not independent-size
samples. The small domains are deliberate empty-chart controls; the `t=3,c=1`
domain has both natural successful and unsuccessful targets. No target is chosen
from a source decomposition supplied to the candidate.

Both programs reject singular/non-prime-order/anomalous curve inputs, nonprime or
out-of-range field parameters, nonsplit/zero cosets, off-curve targets, infinity
and malformed argument lists. Negative CLI checks retain nonzero exit statuses.
Candidate answers additionally recheck membership and rational on-curve recovery.
No full exceptional-chart coverage is asserted.

Retain all canonical models, per-case counts, compiler/source identities, command
and exit records, and exact repeat comparisons. Every mismatch fails the suite.
No failure is discarded or reclassified as a speed result. Public prototype
source and the PKM 2016 primary text sections 3.1-3.3 were read before this design;
their membership framework and known unknown solver complexity remain prior art.

## Direct mixed-target IC correctness addendum (frozen before its checks)

Extend the fixed F17 development fixture into a complete DLP correctness path.
The solver receives `Q` as an on-curve point and a deterministic development RNG
seed derived only from that point's coordinates. It never receives `k`. Fixture
generation of `Q=[k]P` occurs outside the solver, and a returned scalar is checked
both internally by `[k_returned]P=Q` and externally against the fixture answer.
The fixed generator is `P=(5,1)` of order19; the fixed domain is `(t,c)=(3,1)`.

Each cold solve constructs the three canonical signed-base orbits. For every
attempt sample `a,b` in Z/19 using rejection-mapped native RNG words, form
`R=[a]P+[b]Q`, and pass **only R and the public domain parameters** to the unknown
coefficient PDP. Identity R is an explicitly counted miss. Pick the
lexicographically first canonical PDP model using only its point model set.
Collect B+1=4 accepted rows, construct C, then choose a nonzero left-null vector
using only C. No rank-B admission rule or known factor-base logarithms exist.

Recover `k=-(w.a)/(w.b)` if the denominator is nonzero. A zero denominator is
retained as a failed batch and restarts with fresh samples; it must also give
zero numerator under the verified group relations. Bound development execution
at4096 attempts and64 batches per target; hitting either cap is a failing
correctness case, not a censored success. These limits do not define an
asymptotic algorithm or claimed scientific resource budget.

Run every18 nonzero Q on the fixture. Test the C-only left-null routine separately
on rank0,1,2,3 matrices and test denominator-zero rejection. Retain cold base
construction, all attempts/misses/PDP calls/coefficient pairs/root scans, group
addition calls by phase, accepted rows, matrix dimensions/ranks, batch retries
and recovered/expected scalars. They are explicit diagnostic counters, not a
unified field-operation price or a timing/speedup claim. No rho/BSGS, target-log
scan, precomputed factor-base logs or cross-target retained solver setup is used.
