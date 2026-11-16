# K600 m6 native model replay amendment — 2026-11-15 UTC

This versioned amendment changes the post-solver verification implementation in
the [frozen protocol](PROTOCOL.md). The historical `replay_model.py` and its
selftest remain as evidence; no Python cryptographic execution is authorized
for new measurements under the repository's current implementation rule.

The Rust `primary-k600-m6-replay` command in
[`primary_adapter.rs`](../../primary_adapter.rs) now takes, in order, the
retained base root, export JSON, original XCNF, solver receipt, solver stdout,
frozen exporter source commit, frozen exporter binary SHA-256, and fresh output
JSON path. It requires the frozen curve, base, policy, source and binary
bindings, a SAT exit receipt, byte-identical XCNF and stdout hashes, a complete
nonduplicated assignment, and satisfaction of every original CNF and signed
XOR row. It imports the exact retained K600 archive, checks target curve and
subgroup membership and the public fixture or constructed witness, lifts all
six x coordinates through at most two archived points each, checks all four
intermediate x coordinates and the final point sum, then verifies the
signed-Frobenius relation row by independent group arithmetic. It writes a
fresh `n83.k600-m6-model-replay/v2` receipt only after all gates pass.

The Rust command is a verifier, not a solver. The frozen 3,600-second proposed
search envelope, host gates, input hashes, arm order, and stopping rules remain
as stated in the protocol. No full K600 solver result is implied by compiling
or testing the verifier. A future execution lock must name the committed
source and native replayer binary before any timed stage, and its report must
retain failed or incomplete SAT models as failures.

Validation on macOS arm64 with Rust 1.99.1: the release example suite passed
24/24 tests. The parser test rejects partial, duplicate, non-SAT, false CNF,
false XOR, wrong-row-count, and unterminated assignments. A constructed
six-point witness on the same N83 curve checks the export's point-object
encoding, each partial sum, and the exact relation-row hash; changed point,
partial sum, row hash, and index inputs are rejected.
The full K600 replay command has not consumed a solver model; that end-to-end
verification remains pending. The exporter serializes constructed witness
points as `{x_decimal, y_decimal}` objects; the historical Python replayer
expects two-element arrays, so it cannot faithfully verify those exports.
The native replayer compares the actual object encoding against the imported
archive and checks the archived relation-row hash.

The still-open [total-runtime selection problem](../../../../problems/n83-mixed-orbit-factor-base-selection.md)
tracks the later positive/natural execution, relation yield, rank, linear
algebra, target descent, and matched rho obligations.
