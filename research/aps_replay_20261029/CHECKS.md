# Replay acceptance and verification boundary

The implementation is
[`replay.rs`](../../src/cryptanalysis/aps_single_relation/replay.rs), nested in
the existing APS module. Its runtime EC path calls existing
[`Point::add`/`double`](../../src/ecc/point.rs) and
[`FieldElement`](../../src/ecc/field.rs), with a simple affine known-scalar
double-and-add loop. It does not call FastCurve, F4, the producer sampler,
its sign recovery, or a discrete-log search. Producer calls in this source
file are confined to test fixture construction under `cfg(test)`.

The mathematical specification applies the established prime-field group law
and point validation in SEC 1 v2.0 sections 2.2.1 and 3.2.2.1. The accepted
domain is the producer's three existing fixed public fixtures. The existing
field/arity/sample/attempt limits are not enlarged.

| Input claim | Independent check | What is established |
| --- | --- | --- |
| Known fixture | Exact model, G, Q, expected public answer and ICV1; canonical on-curve coordinates and nG=nQ=O | Binding to the named public instance |
| Labeled base | Regenerate seed+i draws with separate arithmetic; retain rejection/deduplication and partition order; then directly check each R=uG+vQ | The recorded sampling/label equations |
| Sign representative | Recompute y times 2^64 modulo p with BigUint before applying the producer's half-field rule | Actual Montgomery sign convention, not an assumed affine convention |
| Orbit base | Recompute geometric orbit expansion, propagated labels and every resulting point-label equation | The fixed fixture's recorded orbit basis |
| Coordinate tuple | Strict distinct sorted coordinates, arm membership, all sign classes and actual point sum | Every emitted tuple is an admissible relation |
| Complete output | Enumerate every allowed distinct coordinate tuple independently and require exact set equality | Mathematical relation-set completeness for that recorded base/arm |
| Partial output | Validate only the emitted subset and preserve incomplete/early-stop/timeout semantics | No enumeration-completeness assertion |
| Useful witness | Bind to an emitted tuple; recompute signs/A/B, reject B=0, divide modulo n, check dG=Q | The recovered public fixture logarithm follows from an informative relation |
| Retry/status metadata | Reproduce each successive seed; permit recovery only on the final attempt; require the full attempt cap for unsuccessful outcomes | Consistent logical sampling and stopping description |
| Replay receipt | SHA-256 of the exact parsed input bytes | Receipt/input binding, without a signature or execution attestation |

Global sign negation preserves the zero-sum condition and changes both A and B
to their negatives, so fixing the first sign positive covers all coordinate
decompositions and preserves the recovered quotient. Distinct abscissae make
the remaining signs the only lift choices. For the pruned three-point arm,
horizontal signed triples are excluded in the independent enumeration too.
These are applications of the existing group-law specification, not new
algebraic identities or a new mathematical theorem.

The controls include separate sampling checks on all three fixtures with both
label policies, zero and wrapping seeds, complete/no-relation and partial
outputs, omitted-root corruption, wrong labels/points/model/identity, false
status/truncated retries, invalid sign/scalar witnesses, repeated coordinates,
duplicate/aliased JSON keys and input caps. A horizontal B=0 orbit with the
correct supplied public answer is explicitly rejected. The CLI additionally
checks all twelve arms in exhaustive and first-informative modes.

Shared serialization, RNG and identity conventions remain dependencies. The
verifier does not attest whether a producer physically executed the claimed
work, respected its declared deadline, or honestly counted F4 operations.
Counter tampering cannot turn the receipt into a cost certificate:
`costs_attested` and `execution_history_attested` are always false.
An accepted `incomplete` replay is not a recovered answer or complete search.

The producer retains its original JSON if replay rejects it, and exits with an
error. A raw producer status field is not a replay certificate. The standalone
`replay` command must succeed on those exact bytes, and consumers must inspect
the receipt outcome and input hash.

This is a native finite-fixture correctness verifier, not a Lean proof of Rust.
No new formalization or kernel-validation claim is made. The earlier
[partial proof map](../aps_followup_20261028/PROOF_MAP.md) remains unchanged;
the actual EC/Rust correspondence is still open. Likewise this does not cover
aborts before a transcript is emitted, arbitrary input curves, the paper's
larger ladder, complete cost accounting or benchmark admission.
