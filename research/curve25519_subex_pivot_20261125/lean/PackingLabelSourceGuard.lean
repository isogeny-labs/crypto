module

public import PackingLabelArtifact

@[expose] public section

/-!
# Kernel source-guard replay for the exact-boundary packing witness

This independent guard computation can build in parallel with both label
replay phases.
-/

namespace Curve25519PackingLabelArtifact

/-- The degree-one matching meets the source-class guard at equality. -/
theorem packingLabelSourceGuardWitness_sourceClassGuardMatches :
    packingLabelSourceGuardWitness.sourceClassGuardMatches = true := by
  decide +kernel +revert

end Curve25519PackingLabelArtifact
