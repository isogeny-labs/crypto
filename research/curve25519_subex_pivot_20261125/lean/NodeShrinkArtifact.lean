module

public import PackingLabelArtifact

@[expose] public section

/-!
# Compact node history-shrink artifact

This module mirrors the native eight-field natural-number projection emitted
from a successfully verified node summary.  Its arithmetic checker is not a
packing source-guard proof: the final theorem also requires alignment with a
strictly parsed `PackingLabelSource`.
-/

namespace Curve25519NodeShrinkArtifact

open Curve25519PackingLabelArtifact

/-- Exact natural-number inputs exported by the native node verifier. -/
structure NodeShrinkArtifact where
  schema : ℕ
  radix : ℕ
  edgeCount : ℕ
  packetLength : ℕ
  labelClasses : ℕ
  maximumClass : ℕ
  paddedHeight : ℕ
  maximumChildEdges : ℕ
  deriving DecidableEq, Inhabited, Repr

/-- Arithmetic accepted by the compact projection.  Strict packing-source
acceptance remains a separate premise of the history theorem below. -/
def NodeShrinkArtifact.Valid (artifact : NodeShrinkArtifact) : Prop :=
  artifact.schema = 1 ∧
    artifact.radix = 64 ∧
    0 < artifact.packetLength ∧
    artifact.paddedHeight =
      (artifact.maximumClass ⌈/⌉ artifact.packetLength) *
        artifact.packetLength ∧
    artifact.packetLength * artifact.labelClasses ≤ artifact.edgeCount ∧
    artifact.maximumChildEdges * 64 ≤
      2 * artifact.paddedHeight * artifact.labelClasses

/-- Executable arithmetic predicate matching the native projection checker. -/
def NodeShrinkArtifact.valid (artifact : NodeShrinkArtifact) : Bool :=
  artifact.schema == 1 &&
    artifact.radix == 64 &&
    decide (0 < artifact.packetLength) &&
    artifact.paddedHeight ==
      (artifact.maximumClass ⌈/⌉ artifact.packetLength) *
        artifact.packetLength &&
    decide (artifact.packetLength * artifact.labelClasses ≤
      artifact.edgeCount) &&
    decide (artifact.maximumChildEdges * 64 ≤
      2 * artifact.paddedHeight * artifact.labelClasses)

/-- The executable checker is exactly the named arithmetic proposition. -/
theorem NodeShrinkArtifact.valid_eq_true_iff
    (artifact : NodeShrinkArtifact) :
    artifact.valid = true ↔ artifact.Valid := by
  simp [NodeShrinkArtifact.valid, NodeShrinkArtifact.Valid, and_assoc]

/-- Fixed native field order: schema, radix, parent edges, packet length,
label classes, maximum class, padded height, and maximum child edges. -/
def NodeShrinkArtifact.projectionValues
    (artifact : NodeShrinkArtifact) : List ℕ :=
  [artifact.schema, artifact.radix, artifact.edgeCount,
    artifact.packetLength, artifact.labelClasses, artifact.maximumClass,
    artifact.paddedHeight, artifact.maximumChildEdges]

/-- Parse exactly eight fields; trailing values are rejected. -/
def parseNodeShrinkArtifactUnchecked : List ℕ → Option NodeShrinkArtifact
  | [schema, radix, edgeCount, packetLength, labelClasses, maximumClass,
      paddedHeight, maximumChildEdges] =>
      some {
        schema, radix, edgeCount, packetLength, labelClasses, maximumClass,
        paddedHeight, maximumChildEdges
      }
  | _ => none

/-- Parse and independently check the compact arithmetic projection. -/
def parseNodeShrinkArtifact (values : List ℕ) : Option NodeShrinkArtifact := do
  let artifact ← parseNodeShrinkArtifactUnchecked values
  if artifact.valid then some artifact else none

/-- The exact native field order round-trips through the unchecked parser. -/
theorem parseNodeShrinkArtifactUnchecked_projectionValues
    (artifact : NodeShrinkArtifact) :
    parseNodeShrinkArtifactUnchecked artifact.projectionValues =
      some artifact := by
  rfl

/-- Every arithmetically valid projection round-trips through the checked
parser. -/
theorem parseNodeShrinkArtifact_projectionValues
    (artifact : NodeShrinkArtifact) (hvalid : artifact.Valid) :
    parseNodeShrinkArtifact artifact.projectionValues = some artifact := by
  simp [parseNodeShrinkArtifact,
    parseNodeShrinkArtifactUnchecked_projectionValues,
    NodeShrinkArtifact.valid_eq_true_iff, hvalid]

/-- Every successful compact parse satisfies the named arithmetic contract. -/
theorem parseNodeShrinkArtifact_valid
    {values : List ℕ} {artifact : NodeShrinkArtifact}
    (hparse : parseNodeShrinkArtifact values = some artifact) :
    artifact.Valid := by
  unfold parseNodeShrinkArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨candidate, _, hchecked⟩
  split at hchecked
  next hvalid =>
    have hartifact : candidate = artifact := Option.some.inj hchecked
    subst candidate
    exact artifact.valid_eq_true_iff.mp hvalid
  next => simp at hchecked

/-- A valid compact summary yields eightfold shrink only after its dimensions
and maximum class are identified with a strictly parsed packing artifact. -/
theorem NodeShrinkArtifact.historyChildStrictShrinkFromArtifact
    (artifact : NodeShrinkArtifact)
    (source : PackingLabelArtifact)
    (hvalid : artifact.Valid)
    (hparse :
      parseSourceGuardPackingLabelArtifact source.projectionValues = some source)
    (hedges : artifact.edgeCount = source.edgeCount)
    (hlabels : artifact.labelClasses =
      source.labelCount * source.labelCount)
    (hmaximum : artifact.maximumClass =
      source.maximumClassSize) :
    8 * artifact.maximumChildEdges ≤ artifact.edgeCount := by
  rcases hvalid with
    ⟨_, _, hpacket, hpadded, hlength, hchild⟩
  have hlengthSource :
      artifact.packetLength *
          (source.labelCount * source.labelCount) ≤ source.edgeCount := by
    calc
      artifact.packetLength *
          (source.labelCount * source.labelCount) =
          artifact.packetLength * artifact.labelClasses := by rw [hlabels]
      _ ≤ artifact.edgeCount := hlength
      _ = source.edgeCount := hedges
  have hchildSource :
      artifact.maximumChildEdges * 64 ≤ 2 *
        ((source.maximumClassSize ⌈/⌉ artifact.packetLength) *
          artifact.packetLength) *
        (source.labelCount * source.labelCount) := by
    calc
      artifact.maximumChildEdges * 64 ≤
          2 * artifact.paddedHeight * artifact.labelClasses := hchild
      _ = 2 *
          ((source.maximumClassSize ⌈/⌉ artifact.packetLength) *
            artifact.packetLength) *
          (source.labelCount * source.labelCount) := by
            rw [hpadded, hmaximum, hlabels]
  have hshrink :=
    source.historyChildStrictShrinkOfRoundedMaximumNat
      hparse artifact.packetLength artifact.maximumChildEdges
      hlengthSource hchildSource
  simpa only [← hedges] using hshrink

/-- Source-shaped form of the compact-summary implication. -/
theorem NodeShrinkArtifact.historyChildStrictShrink
    (artifact : NodeShrinkArtifact)
    (source : PackingLabelSource)
    (hvalid : artifact.Valid)
    (hparse :
      parseSourceGuardPackingLabelArtifact source.projectionValues =
        some source.artifact)
    (hedges : artifact.edgeCount = source.artifact.edgeCount)
    (hlabels : artifact.labelClasses =
      source.artifact.labelCount * source.artifact.labelCount)
    (hmaximum : artifact.maximumClass =
      source.artifact.maximumClassSize) :
    8 * artifact.maximumChildEdges ≤ artifact.edgeCount :=
  artifact.historyChildStrictShrinkFromArtifact source.artifact hvalid hparse
    hedges hlabels hmaximum

/-- Compact arithmetic plus the complete strict packing artifact. -/
structure NodeShrinkBundle where
  summary : NodeShrinkArtifact
  source : PackingLabelArtifact
  deriving DecidableEq, Inhabited, Repr

/-- The three cross-interface equalities checked by the bundle importer. -/
def NodeShrinkBundle.Aligned (bundle : NodeShrinkBundle) : Prop :=
  bundle.summary.edgeCount = bundle.source.edgeCount ∧
    bundle.summary.labelClasses =
      bundle.source.labelCount * bundle.source.labelCount ∧
    bundle.summary.maximumClass = bundle.source.maximumClassSize

/-- Executable bundle alignment predicate. -/
def NodeShrinkBundle.aligned (bundle : NodeShrinkBundle) : Bool :=
  bundle.summary.edgeCount == bundle.source.edgeCount &&
    bundle.summary.labelClasses ==
      bundle.source.labelCount * bundle.source.labelCount &&
    bundle.summary.maximumClass == bundle.source.maximumClassSize

/-- Executable alignment is exactly the three named equalities. -/
theorem NodeShrinkBundle.aligned_eq_true_iff (bundle : NodeShrinkBundle) :
    bundle.aligned = true ↔ bundle.Aligned := by
  simp [NodeShrinkBundle.aligned, NodeShrinkBundle.Aligned, and_assoc]

/-- A valid bundle contains accepted arithmetic, a strictly accepted packing
source, and exact cross-interface alignment. -/
def NodeShrinkBundle.Valid (bundle : NodeShrinkBundle) : Prop :=
  bundle.summary.Valid ∧
    parseSourceGuardPackingLabelArtifact bundle.source.projectionValues =
      some bundle.source ∧
    bundle.Aligned

/-- Joint native field order: eight summary fields followed by the complete
packing-label projection. -/
def NodeShrinkBundle.projectionValues (bundle : NodeShrinkBundle) : List ℕ :=
  bundle.summary.projectionValues ++ bundle.source.projectionValues

@[simp] theorem NodeShrinkArtifact.projectionValues_length
    (artifact : NodeShrinkArtifact) : artifact.projectionValues.length = 8 := by
  rfl

/-- Parse, strictly check, and internally align the joint native projection. -/
def parseNodeShrinkBundle (values : List ℕ) : Option NodeShrinkBundle := do
  let summary ← parseNodeShrinkArtifact (values.take 8)
  let source ← parseSourceGuardPackingLabelArtifact (values.drop 8)
  let bundle : NodeShrinkBundle := { summary := summary, source := source }
  if bundle.aligned then some bundle else none

/-- Every valid joint artifact survives the complete checked importer. -/
theorem parseNodeShrinkBundle_projectionValues
    (bundle : NodeShrinkBundle) (hvalid : bundle.Valid) :
    parseNodeShrinkBundle bundle.projectionValues = some bundle := by
  rcases hvalid with ⟨hsummary, hsource, haligned⟩
  simp [parseNodeShrinkBundle, NodeShrinkBundle.projectionValues,
    parseNodeShrinkArtifact_projectionValues bundle.summary hsummary,
    hsource, NodeShrinkBundle.aligned_eq_true_iff, haligned]

/-- Every successful joint parse satisfies arithmetic, strict-source, and
alignment validity on the canonical stored projections. -/
theorem parseNodeShrinkBundle_valid
    {values : List ℕ} {bundle : NodeShrinkBundle}
    (hparse : parseNodeShrinkBundle values = some bundle) : bundle.Valid := by
  unfold parseNodeShrinkBundle at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨summary, hsummary, hafterSummary⟩
  rcases Option.bind_eq_some_iff.mp hafterSummary with
    ⟨source, hsource, hchecked⟩
  dsimp only at hchecked
  split at hchecked
  next haligned =>
    have hbundle :
        ({ summary := summary, source := source } : NodeShrinkBundle) =
          bundle := Option.some.inj hchecked
    subst bundle
    have hsummaryValid := parseNodeShrinkArtifact_valid hsummary
    rcases parseSourceGuardPackingLabelArtifact_iff.mp hsource with
      ⟨⟨⟨hwell, hbounded, _⟩, htrace⟩, hguard⟩
    have hsourceCanonical :
        parseSourceGuardPackingLabelArtifact source.projectionValues =
          some source :=
      parseSourceGuardPackingLabelArtifact_iff.mpr
        ⟨⟨⟨hwell, hbounded, rfl⟩, htrace⟩, hguard⟩
    exact ⟨hsummaryValid, hsourceCanonical,
      (NodeShrinkBundle.aligned_eq_true_iff _).mp haligned⟩
  next => simp at hchecked

/-- Bundle validity discharges all compact-summary/source alignment premises. -/
theorem NodeShrinkBundle.historyChildStrictShrink
    (bundle : NodeShrinkBundle) (hvalid : bundle.Valid) :
    8 * bundle.summary.maximumChildEdges ≤ bundle.summary.edgeCount := by
  rcases hvalid with ⟨hsummary, hsource, haligned⟩
  exact bundle.summary.historyChildStrictShrinkFromArtifact bundle.source
    hsummary hsource haligned.1 haligned.2.1 haligned.2.2

/-- Any successfully imported joint projection carries the eightfold-shrink
conclusion with no caller-supplied alignment premise. -/
theorem parsedNodeShrinkBundle_historyChildStrictShrink
    {values : List ℕ} {bundle : NodeShrinkBundle}
    (hparse : parseNodeShrinkBundle values = some bundle) :
    8 * bundle.summary.maximumChildEdges ≤ bundle.summary.edgeCount :=
  bundle.historyChildStrictShrink (parseNodeShrinkBundle_valid hparse)

end Curve25519NodeShrinkArtifact
