module

public import PackingLabelArtifact

@[expose] public section

/-!
# Kernel row replay for the exact-boundary packing witness

This module isolates the independent row-phase computation so Lake can build
it in parallel with the column-phase certificate.
-/

namespace Curve25519PackingLabelArtifact

open Curve25519BucketStateReplay

set_option maxHeartbeats 2000000 in
set_option maxRecDepth 100000 in
/-- The exact-boundary witness passes the single-pass row replay. -/
theorem packingLabelSourceGuardWitness_rowReplay :
    replayRowState (q := packingLabelSourceGuardWitness.labelCount) (by decide)
        (packingLabelSourceGuardWitness.leftEndpoint (by decide))
        (packingLabelSourceGuardWitness.rightEndpoint (by decide))
        (packingLabelSourceGuardWitness.rowLabel (by decide)) = true := by
  decide +kernel +revert

end Curve25519PackingLabelArtifact
