# Independent source-review request

Review exact packet manifest and the HEAD-bound sources without compiling,
materializing, or executing a target.

Required findings:

1. Verify `overlay-manifest.json`, reproduce `git apply -p1 --check` for
   `integration.patch`, then compare every proposed managed trace event and
   `relation_capture` field against
   donor `native_solver.rs` and `relation_stream.rs` at the bound hashes.
2. Confirm trace-disabled managed behavior retains statement order and ordinary
   `GraphSolver::relation` behavior.
3. Confirm progress errors remain nonfatal and trace errors do not fall back.
4. Confirm online measurement closes on every post-begin return.
5. Check the typed attempt join for contradictory complete/partial/error states,
   duplicate identities, retry substitution, arm cardinalities, null preservation,
   and unauthenticated custody wording.
6. Review `pencil_relation_sink.rs`, the `record`/TLS/solver integration, and the
   retained-attempt module and CLI for exact create-new, flush/sync, hash, complete
   versus prefix, plan/pair/cardinality, null, and custody semantics.
7. Confirm that the only true external input added is the exact 40-hex source
   revision in each new frozen launch document; reject derivation from a manifest
   SHA-256.
8. State explicitly that `managed-stream.patch` is the historical semantic sketch,
   `integration.patch` is the applyable artifact, and runtime parity remains
   unchecked.

The review must be saved as a new companion file; do not modify or overwrite this
packet's source candidates.
