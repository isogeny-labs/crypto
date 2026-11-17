# Shared CPU reduction integration

T-906 released the shared source scope. Conductor admitted the work and T-929
claimed it before edits. The current parent feature branch was merged without
rewriting earlier evidence. Its wider field/hash guards and inverse bound were
preserved, then the tested combined-reduction patch applied cleanly.

The actual shared solver now uses compile-time optional internal profiling and
reuses the generated target during collection. Both original independent
certificate checks remain; the relation checker and verifier region is
byte-identical to the fetched parent. Both new regression tests are integrated.

Validation on the actual checkout:33 release tests pass, all-target strict
Clippy passes. The source hash, parent identity, commands and unchanged-verifier
hash are recorded in `validation.json`. The wide inverse test imported from
the parent also passes. Root crypto/CUDA was not built locally.

The source now differs from the measured round15 snapshot because the parent
also widened the field/hash guards. Earlier snapshot protocols, manifests,
exports and reports remain immutable and correctly labeled. Their timings
must not be attributed to this exact integrated revision. A new frozen
comparison is required before any current-source performance claim.

## Handoff

- Completed: reconciled released ownership; claimed T-929; merged current
  parent; integrated both tested optimizations; validated actual CPU crate.
- Decision: preserve full independent replay and all original timing boundaries;
  retain prior source-pinned evidence without rewriting historical records.
- Remaining: benchmark the integrated revision against the fixed baseline and
  continue reducing arithmetic costs. The further10x goal remains unmet.
  Strict isolation, independent review, external comparisons and larger-scale
  CPU recovery remain outstanding.
- Errors: no merge conflict, patch rejection, release-test or Clippy failure.
