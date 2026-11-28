# Section-coefficient v1: prospective fixed qualification

This component tests a proposed reduction of the supplied-coefficient support
predicate. It does not discover the five coefficients. The original
section-query-v1 and its input closure remain unchanged. All claims are
producer unverified until maintained signed validation. [pcc:unverified]

## Hypotheses and intended system

Use only the original fixed p101 or Curve25519 parameter contracts. h has
exact odd order aq, q and a are the fixed distinct odd primes, 1<=j<=k,
0<=t<q, and 2k<a. Normalize the finite-target chart by c4=1, or the identity
chart by c5=1. The resulting monic degree-six norm S has five free Fp
coefficients; finite two-torsion target charts remain unsupported.

Let H_lambda(T)=product_j(T_q(T)^2-Re(h^(qj))^2). Its degree is D=2qk.
After clearing the invertible map denominator L=2X+A, call its degree-D
pullback E_B. Proposed predicate: E_B mod S=0 and D3 is a unit modulo S,
where (N0,D0)=(X,1), Nnext=(N^2-D^2)^2,
Dnext=4ND(N^2+A ND+D^2), three iterations. The existing fast H_interval
computes a unit multiple of the cleared membership remainder; preserve its
map-inverse guard. This system retains the original distinct-raw-row contract.
It is not a valid replacement for generic RR9 repeated-support rows.

Predictions: E_B splits into distinct Fp roots for the declared window;
raw x values are injective across raw labels, including the two map branches;
squarefree norm support forces nonzero rational curve ordinates; the D3 unit
test excludes exactly identity eightfold images. These implications require
Fp coefficient assignments, not arbitrary algebraic-closure solutions.

Competing hypothesis: removing explicit Frobenius and ordinate tests leaves
an incorrect acceptance, for example repeated support or torsion. Any new
accepted candidate that fails unchanged original extraction falsifies the
proposed equivalence on that control. Exceptions/refusals remain explicit.

## Exact degree audit

Finite normalized S has coefficients of total degree<=2 in the five unknowns.
On the formal pencil c3=T its X5 coefficient has leading term -T^2. Predict
that the X5 coefficient of E_B mod S has exact T-degree 2(D-5), for D>=6.
Test toy q3 and k1..8: expected degrees2,14,26,38,50,62,74,86. Work in
Fp101[T] without reducing T^101-T. This is a degree of this literal generator,
not a lower bound on solving degree, ideal generators, regularity or every
solver. Compute actual integer sizes in Rust; do not expand actual E_B.

## Fixed controls and observables, before code/run

1. Enumerate the24 raw toy columns, both map x branches, and explicit dense
   degree48 E_B. Check split/separable support, map injectivity and eightfold
   denominator classification against direct group multiplication.
2. Use the first eight retained toy raw columns in lexicographic (j,t) order.
   Inspect every six-subset and all64 sign assignments:1792 declared tuples.
   Construct sections from known points outside the query API. Record each
   chart refusal; do not count it as predicate rejection or target absence.
   For every supported chart, query only Spec,R,Q,c. Check the new predicate,
   then unchanged original extraction and signed-row/group replay. Retain
   complete coefficients/labels/signs/targets, extracted original rows and a
   digest of each full original auxiliary witness; retain full witnesses for
   the first and last supported tuples and both actual controls.
3. For the existing known-section recipe j1..6,t0, sign bits0, inspect101
   values of c3 with other normalized coefficients fixed. Use this same
   finite R for the eight formal-degree pencils. Compare fast predicate with dense E_B divisibility and
   D3; every acceptance must extract an original checked row. No assumption
   that a mutation must reject. No useful-yield or target-sampling inference.
4. Repeated copies of one raw point with synthetic distinct construction IDs:
   old H^6 may vanish, while new first-power H must refuse. One six-distinct
   raw support including an identity eightfold image must pass H and fail D3.
5. The eight formal-degree pencils above. Retain exact coefficient vector
   digests and degrees, plus the generic written upper/leading-path argument.
6. Original actual-field q213156431/a60824497 with k31 and k20633: supplied
   known section from j1..6,t0 (same fixed fallback sixth-column policy as v1,
   at most32 attempts, all retained); new predicate plus original full replay.
   Largest k arm last. No q1 replacement or dense full-base enumeration.

Each supported section also exposes a native norm-Jacobian rank diagnostic
if implemented; this is optional before source qualification and never a
solver-complexity or global smoothness theorem. Do not alter primary controls
in response to their results. Every failed arm remains visible; continue
independent arms, and retain incomplete/missing completion separately.

## Resources, qualification and limitations

Single prospective fixed worker call:180second wall limit,2GiB address-space
limit, one thread. Old Engine caps remain unchanged:100000000 scalar work per
engine, n<=65536, quotient degree<=6, p bits<=256. New symbolic pencil caps:
degree(E_B)<=48, coefficient T-degree<=96, at most eight degree controls.
This is bounded development correctness work; not performance measurement.
Counter coverage is partial and carries no GAE, bitop, timing or RSS claim.

Before substantive execution: focused release library tests, fmt, strict
Clippy; retain failed source versions/logs. Commit source, snapshot the exact
new closure as a delta against frozen v1, compare compiled-in hashes, copy
read-only binary and separately copied immutable qualifier, and write distinct
BUILD/PRECALL/POSTCALL files with create-new semantics. Use only a thin shell
launcher. Raw JSONL is create_new, separate from stdout/stderr and flushed per
arm. First check actual transport-only success and collision refusal with the
same launcher. A timeout or cap is incomplete, never an absence certificate.

Readable derivations and a faithful proof map are required. Existing Mathlib
field, polynomial divisibility and squarefree laws should be assembled through
the lean-instance gate. Any incomplete native/formal bridge stays explicitly
open. Classical Chebyshev traces, radical support and Montgomery doubling
are prior art; novelty and useful solving-cost advantage remain unestablished.
