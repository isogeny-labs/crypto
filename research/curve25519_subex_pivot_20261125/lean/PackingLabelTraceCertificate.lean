module

public import PackingLabelRowReplay
public import PackingLabelColumnReplay

@[expose] public section

/-!
# Joined exact-boundary packing trace certificate

The independently checked row and column phases imply the complete trace
predicate consumed by the replay parser.
-/

namespace Curve25519PackingLabelArtifact

/-- The exact-boundary native projection passes executable trace replay. -/
theorem packingLabelSourceGuardWitness_traceMatches :
    packingLabelSourceGuardWitness.traceMatches = true := by
  have hlabels : 0 < packingLabelSourceGuardWitness.labelCount := by decide
  have hrows : 0 < packingLabelSourceGuardWitness.rowCount := by decide
  have hcolumns : 0 < packingLabelSourceGuardWitness.columnCount := by decide
  simp only [PackingLabelArtifact.traceMatches, hlabels, hrows, hcolumns,
    ↓reduceDIte, Bool.and_eq_true]
  exact ⟨packingLabelSourceGuardWitness_rowReplay,
    packingLabelSourceGuardWitness_columnReplay⟩

end Curve25519PackingLabelArtifact
