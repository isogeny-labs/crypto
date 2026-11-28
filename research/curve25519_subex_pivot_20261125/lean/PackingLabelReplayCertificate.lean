module

public import PackingLabelRowReplay
public import PackingLabelColumnReplay
public import PackingLabelSourceGuard
public import PackingLabelParseReceipt

@[expose] public section

/-!
# Joined exact-boundary packing certificate

The independent parser, row, column, and source-guard receipts compose in one
join module through the strict source-guard parser.
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

/-- The exact-boundary projection parses and replays to its named artifact. -/
theorem packingLabelSourceGuardWitness_replayed :
    parseReplayedPackingLabelArtifact packingLabelSourceGuardWitnessNumbers =
      some packingLabelSourceGuardWitness := by
  exact parseReplayedPackingLabelArtifact_of_components
    packingLabelSourceGuardWitness_parsed
    packingLabelSourceGuardWitness_traceMatches

/-- The strict parser accepts the positive exact-boundary native projection. -/
theorem packingLabelSourceGuardWitness_sourceGuardParsed :
    parseSourceGuardPackingLabelArtifact packingLabelSourceGuardWitnessNumbers =
      some packingLabelSourceGuardWitness := by
  exact parseSourceGuardPackingLabelArtifact_of_components
    packingLabelSourceGuardWitness_replayed
    packingLabelSourceGuardWitness_sourceClassGuardMatches

set_option maxRecDepth 100000 in
/-- The positive exact-boundary witness is accepted directly from the
structured named-source projection, not only from its embedded CSV. -/
theorem packingLabelSourceGuardWitnessSource_sourceGuardParsed :
    parseSourceGuardPackingLabelArtifact
        packingLabelSourceGuardWitnessSource.projectionValues =
      some packingLabelSourceGuardWitnessSource.artifact := by
  apply (PackingLabelSource.parseSourceGuardProjection_iff _).mpr
  rw [packingLabelSourceGuardWitnessSource_artifact]
  have hspec := parseSourceGuardPackingLabelArtifact_iff.mp
    packingLabelSourceGuardWitness_sourceGuardParsed
  have hwell : packingLabelSourceGuardWitnessSource.WellFormed :=
    (PackingLabelSource.artifact_wellFormed_iff _).mp (by
      rw [packingLabelSourceGuardWitnessSource_artifact]
      exact hspec.1.1.1)
  have hbounded : packingLabelSourceGuardWitnessSource.Bounded :=
    (PackingLabelSource.artifact_bounded_iff _).mp (by
      rw [packingLabelSourceGuardWitnessSource_artifact]
      exact hspec.1.1.2.1)
  exact ⟨⟨⟨hwell, hbounded⟩, hspec.1.2⟩, hspec.2⟩

end Curve25519PackingLabelArtifact
