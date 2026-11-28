# RR5 correctness qualification addendum

Frozen before the B8 candidate calls. Date: 2026-11-22 Pacific. Current tracking
PR: #4942. This extends, and does not replace, the original `PROTOCOL.md` or any
earlier evidence. This is local Rust development correctness work.

## Fixed B8 controls

Use the unchanged ordinary PRIME31 curve and the first eight nonzero-y points
from a deterministic public x scan, choosing the smaller ordinate. The candidate
receives only those public points, the curve and a public target.

1. Positive: the same first-four signed sum as the B4 control, with signs
   `(+1,-1,+1,+1)`. Only the test oracle constructs and retains this witness.
2. Empty control: the fixed public generator. Independently enumerate the
   70 distinct four-index subsets and their 16 signs in test-only integer group
   arithmetic to establish the expected set, before invoking the candidate.

Each ordinary Rust test requires the candidate's complete signed-row set to
equal that independent oracle. The positive must finish as `Complete`; the
generator must have an empty independent oracle and finish as `Empty`.
An inconclusive, degree-limited, output-limited or interrupted call fails this
correctness gate. It is retained as a censored result, never changed into an
emptiness result. No oracle decomposition, scalar or hint enters the extractor.

## Enforced limits

- At most two compiler workers; one Rayon worker and one test worker.
- B8 F4 degree cap 10; one absolute 30-second deadline per candidate call.
- Construction cap 20,000 terms per intermediate polynomial; output cap 4,096
  coefficient assignments.
- Test process: 180-second outer timeout and 2,097,152 KiB `RLIMIT_AS`.
  The address-space ceiling bounds all process allocation, including F4
  matrices. This is not a dimension-aware row/column admission check. Allocation
  failure or process termination is a retained process failure. Native maximum
  observed rows and columns are recorded if the call returns.
- Freeze a copied executable, actual source inventory and toolchain before
  each invocation. Retain exit status, raw output and source/binary rechecks.

The native diagnostic now calls the traversal counter
`forbidden_branch_attempts`: the underlying F4 counter advances before a trace
sink can refuse a child. A nonzero count would not by itself establish executed
field enumeration. Actual domain evaluations are reported separately. Existing
receipts keep their original field names and bytes.

## Qualification boundary

B4 is the special case where the monic quartic equals the degree-four base
polynomial. B8 exercises higher-degree membership remainder equations. Neither
gate qualifies the requested B677 base or the complete-cost experiment.

The public example is additionally compiled against a source-pinned dependency
closure consisting of complete production F4 modules and verbatim selected
production arithmetic/group types. This qualifier is not a substitute for the
root library gate. Each gate has its own retained result. No speed ratio,
asymptotic exponent or calibrated complete-work count is inferred from these
development executions.

## Evaluated identity record

The existing production `identity_certificate` checker and SHA-256 implementation
also issue and recheck the RR5 norm factorization after substituting
`B=s^2-r^3-A*r`. The expression has syntactic total-degree bound 6 in
`X,r,s,A,b,c,d`. Use 32 certificate points, seed `0x5252355f49444331`, and 32
additional points with seed `0x5245504c41595f31`. The v1 certificate evaluates
over `2^61-1`, not PRIME31. A deliberately omitted signed cross term must fail
issuance. Retain the certificate and exact checker output. This is same-team
evaluated evidence; it does not prove the divisor-chart theorem, the identity
over every field, or any complexity bound, and supplies no formal proof pointer.
