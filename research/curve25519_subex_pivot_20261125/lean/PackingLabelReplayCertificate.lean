module

public import PackingLabelTraceCertificate
public import PackingLabelSourceGuard
public import PackingLabelParseReceipt

@[expose] public section

/-!
# Exact-boundary packing parser certificate

The joined trace and independent source-guard receipts compose through the
strict source-guard parser.
-/

namespace Curve25519PackingLabelArtifact

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

end Curve25519PackingLabelArtifact
