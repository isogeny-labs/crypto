# Independent signed-extraction source review

Status: static review before the immutable build/qualification call. No
execution outcome is asserted by this record. The parent coordinates source
freezing, execution bounds, receipts and publication in crypto PR #4976.

## Accepted contracts

- Exactly 132 inputs and 42 preselected persisted input/output pairs, as
  independently specified in `native/independent.rs::registry` and `persisted`.
- Both negative-ordinate controls use x=1, selected before execution.
- For Q=O, G is the actual admissible base-abscissa polynomial under both
  left-selector policies. F is either the full selector or actual base x set.
- The output is every ordered signed triple, with repeated points permitted
  and the identity excluded from the base. Ordinary points have two distinct
  lifts; rational two-torsion points have one. Each point belongs to the
  explicitly configured cyclic subgroup or the unrestricted curve group.
- The order-four control retains both finite-prefix witnesses (P,P,P) and
  (-P,-P,P) at the same coordinate pair, and the separate identity-prefix
  witness (-P,P,-P), for target -P.

## Arithmetic and independence boundary

The new independent module uses its own u128 residue arithmetic, left-to-right
powering, extended-Euclidean inverses, square-root logic and affine group law.
It obtains the exact tuple set by enumerating every ordered first pair and
looking up the only possible third point Q-(P1+P2). This is a generic finite
completeness oracle used only for development checking; the producer does
not consume its answers or use it as the algebraic extraction mechanism.

The module independently constructs the full selector, rational lifts and
exact cyclic subgroup. It obtains finite-target right coordinates from roots
of every independently interpolated S3 quadratic. It binds H on the complete
split grid by a direct selector product and checks every left/right witness
bucket. It reuses the predecessor's independently written polynomial identity
checker, under the module name `query_independent`, to check the unchanged
quotient-certificate contract. The certificate data types and repository SHA-256
routine are shared; this is not an independent implementation of hashing, nor
an independently built copy of the entire program.

The new reader binds the fixture registry before any type or curve-dependent
semantic dispatch. Direct typed JSON parsing and a value roundtrip reject
duplicate declared fields, unknown nested fields and omitted explicit fields.
With raw records, both proof and input-sidecar bytes are SHA-256 and size-bound.
Without raw records, the full expected artifact set and mathematical contract
still apply. `READER_CONTROLS.md` fixes 91 required rejections and three positive
checks before execution.

## Static findings

The producer and checker now agree on d<=128, explicit cyclic orders 2..512,
the x=1 negative controls, the identity-target G, ordered base/tuple indexing,
permutation groups and integer-row convention. Integer two-torsion coefficients
are retained as integers; they are not silently reduced modulo two. A row is
not interpreted as independent or useful merely because it is nonzero.

All semantic output counts are independently reconstructed. Arithmetic-call
counters remain producer diagnostics: agreement of their serialization does
not establish complete field-operation pricing or a measured performance gain.
The full tuple set, root and bucket inventories, and every row/permutation
backpointer are checked before accepting those semantic counts.

The separate diagonal-identity program was reviewed as a fixed finite check:
its S3(a,z,a) expression agrees algebraically with
f'(a)^2-4f(a)(z+2a), with total degree at most four in A,B,a,z. The coefficient
mutation recomputes its statement identifier and therefore requires arithmetic
rejection, while the digest mutation separately exercises byte-value binding.
The program explicitly declines a formal-proof or adversarial probability
claim for its public deterministic evaluation points. The exact source/build
and finite execution outcome belong in subsequent receipts. [pcc:unverified]

## Sources checked

Sources checked (KB): the parent's discovery found no research-KB MCP tool,
and `scripts/research-kb status` reported no executable. The scoped fallback
was the predecessor `quotient_query/THEORY.md` section 6 and its frozen input
binding/checker contract, `target_quotient/THEORY.md`, and Petit, Kosters and
Messeng, [Algebraic approaches for the Elliptic Curve Discrete Logarithm Problem
over prime fields](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
sections 2.4 and 3.1 (including the final point-membership and relation step).
These sources motivate separating polynomial coordinate solutions from
actual rational signed relations. No absence-of-prior-work claim is made.

The inherited repository policy and adopted prime-field lessons require
distinct immutable source/binary/pre-call/post-call manifests, preservation of
failures, and unsigned PCC standing until the existing signed verifier changes
it. Local correctness calls do not discharge the separately incomplete full
library gate or authorize an EKS timing comparison.
