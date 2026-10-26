# Projected query and verified-relation plumbing

This is implementation work following the cofactor audit, not a complete IC
experiment, new speed result or new measured crossover. It does not change any
frozen HKY instrument, original producer cost or historical subgroup receipt.

## Sources checked before implementation

- The study's `COFACTOR_PROJECTION_PROTOCOL.md`, `PROTOCOL.md` and README distinguish
  strict original-point membership from relations after cofactor projection.
- The lead atlas was searched with `cofactor projection subgroup query randomization
  kernel`. `LEAD-ICB-cofactor-torsion-quotients` records the existing obstruction to
  treating small cofactor labels as a new general speedup; it does not price this
  proposed complete pipeline. Projection is a known mechanism, with no novelty claim.
- [SEC 1 v2](https://www.secg.org/sec1-v2.pdf), sections 3.1.2, 3.1.2.2 and 3.3.2,
  was read for the binary-curve order/cofactor convention, prime-order generator
  checks and cofactor multiplication. Download SHA256:
  `54687189af3de645756a30a801b596fd14c6566992c13bfd58eede399f324a89`.
  SEC 1 supplies those definitions; it does not prescribe this IC query protocol.
- Gaudry's 2009 paper was located through the author's current publication page
  and the official HAL API: DOI `10.1016/j.jsc.2008.08.005`, HAL `inria-00337631`.
  Its document endpoint returned HTTP403, so this additional review does not claim
  to have read that paper. The older LIX publication URL returned HTTP404. These
  access failures are not negative mathematical evidence. No novelty or asymptotic
  conclusion relies on an unread source.

Conductor is unavailable on this host; no reservation is claimed. New source is
confined to `examples/ic_projected_query_law.rs` and its `core.rs` support file,
plus this note and source-check receipts. The actual 248-child HKY replay remains
in its separate pinned worktree.

## Conditional algebra and the corrected query law

Assume a correct finite abelian curve group E of cardinality h*r, a point G of
prime order r, and gcd(h,r)=1. The ambient cardinality must have its own valid
certificate; checking the order of G alone does not certify it.

Let H=<G> and K=ker[h]. Multiplication by h is invertible on H. Since r divides
the cardinality exactly once, the r-primary subgroup is H; consequently [h]E=H
and |K|=h. An element of H intersect K is killed by both r and h, so Bezout's
identity makes it zero. Addition therefore gives a bijection H x K -> E.

For a target T=[d]G, sample independent uniform k in Z/r and U in K, and query

    R = T + [k]G + U.

Translation preserves the bijection, so R is uniform on E under those sampling
assumptions. This is a conditional algebraic derivation, not an empirical
randomness test. The source helper accepts indices; it provides no random sampler.

The earlier prospective draft used U=O. That still gives a sound projection
identity but only samples H, which can miss cofactor cosets and cannot justify
full-curve uniformity or useful relation yield. Kernel randomization corrects
that design before any new projected-query experiment.

For an actually verified signed full-curve relation sum eps_i P_i=R, let
L_i=log_G([h]P_i), with every supplied log independently checked. Then

    sum eps_i L_i = h*(d+k) mod r,
    d = h^(-1) * sum eps_i L_i - k mod r.

For setup queries T=O the right-hand side is the known scalar h*k. For R=O,
the direct-sum decomposition forces U=O and d=-k; the direct hit must still be
verified and charged. Neither identity computes factor-base logs or finds a
decomposition.

Five algebraic dependencies in `proofs/ProjectedQueryLaw.lean` pass the fresh
repository proof gate with zero trusted exceptions. The complete finite-group,
cardinality and sampling bridge is not formalized. Finite source tests do not
constitute that general proof.

| Readable dependency | Lean declaration | Exact scope |
|---|---|---|
| Project away a kernel translation | `kernel_translation_projects_away` | explicit additive projection, zero and kernel hypotheses |
| Solve the projected scalar equation | `recover_scalar_from_projected_sum` | commutative ring, supplied inverse and log-sum equation |
| Known setup right-hand side | `known_setup_right_hand_side` | commutative ring, supplied inverse and setup equation |
| Scalar at an identity hit | `projected_identity_hit_scalar` | commutative ring, supplied inverse and zero projected shift |
| Coordinates imply unique decomposition | `certified_coordinates_give_unique_decomposition` | supplied left/right coordinate identities; does not construct them |

The first and last declarations use no axioms; the ring declarations use only
`propext`, `Classical.choice` and `Quot.sound`. No sorry, custom axiom or
`native_decide` exception. Receipt: `pipeline-receipts/projected-query-law-lean-20261026/`.

## Implemented checks and limits

The native core verifies an odd prime r<=2^32-1 by exact trial division,
1<=h<=4096, coprimality, ambient-order metadata consistency, nonzero order-r G,
and an explicitly supplied cyclic kernel generator. It enumerates h distinct
kernel elements and verifies their annihilation by h. A noncyclic kernel is
rejected; that implementation limit does not invalidate the mathematical law
for a noncyclic kernel with a correct general enumeration.

Certificates retain the complete arithmetic-domain parameters, and query
construction rejects a different domain, invalid subgroup targets or out-of-range indices.
The binary binding also rejects disagreement between metadata and its actual
field/curve backend.
Recovery rechecks the query context, every point and projected logarithm, the
signed full-curve relation, and finally [d]G=T. Zero projections, repeated points,
negative signs and verified identity hits stay explicit. There is no unverified
log anchor, hidden use of the target scalar, relation discovery or LA solver.

The binary adapter counts its group calls, including validation calls. Binary
field operations, membership tests, construction and all other phases lack a
complete price here. Its group ledger is not an end-to-end cost or speed metric.
The backend must supply a correct certified ambient order; general certification
of the qualified n22/n25 curve records is not replaced by this interface.

## Source verification and remaining work

Seven standalone native tests pass. They check bijective coverage on four small
cyclic groups for every target, the missing-kernel control, signed recovery with
independently looked-up logs, invalid relations/logs/ranges, direct identity hits,
and rejection of a noncyclic kernel, incompatible order data or reuse with a
different arithmetic domain. The first six-test receipt is retained with its
original source hash; the seventh was added after static domain-binding review.

The Cargo source gate adds an independently enumerated small binary curve,
reference polynomial-basis projections/logs and full query/recovery checks. Its
first attempt failed before compilation because sparse checkout omitted the
existing `isogeny_algos` path dependency. That exact dependency was materialized;
offline locked metadata now passes, with Cargo.lock unchanged. The second gate
is queued, not passed. All raw failures and final statuses are retained in
`pipeline-receipts/projected-query-law-source-checks-20261026/`.

| Requirement | Status |
|---|---|
| Checked abstract query/relation helper | implemented; seven standalone tests pass |
| Binary backend binding | implemented; native Cargo gate pending |
| Full-curve order/kernel receipts for actual qualified cells | no new run; original receipts unchanged |
| Uniform bounded sampler and frozen projected-IC protocol | not implemented/frozen |
| Factor-base projection, signed deduplication, cold log collection and mod-r LA | incomplete |
| Complete relation oracle, target solve and every phase's price | incomplete |
| Same-instance current IC baseline, strongest rho, genuine A/A and external solver comparison | pending full instrument/protocol |
| General finite-group Lean specialization and cardinality/uniformity bridge | open |

Next: complete source checks, then build the missing cold pipeline before
freezing an actual projected-IC comparison. Use the existing qualified public
models with explicit curve-order provenance, preserve every construction/query
failure and count all setup, sampling, lifting, projection, relation, LA and
verification work. Differences in a whole method's query law must remain in its
variable ledger; they do not isolate an individual stage gain. No projected
experiment or additional VM has launched. The total additional cloud-compute
ceiling remains USD100 including replay.
