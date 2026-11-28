module

public import PackingLabelArtifact

@[expose] public section

/-!
# Kernel parse receipt for the exact-boundary packing witness

The shared CSV evaluation is isolated from the independent row, column, and
source-guard checks so Lake can schedule those computations in parallel.
-/

namespace Curve25519PackingLabelArtifact

set_option maxRecDepth 100000 in
/-- The positive native source-guard projection parses to its named value. -/
theorem packingLabelSourceGuardWitness_parsed :
    parsePackingLabelArtifact packingLabelSourceGuardWitnessNumbers =
      some packingLabelSourceGuardWitness := by
  decide +kernel +revert

end Curve25519PackingLabelArtifact
