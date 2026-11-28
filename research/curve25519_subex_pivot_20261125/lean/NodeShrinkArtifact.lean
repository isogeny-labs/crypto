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

/-- A valid compact summary yields eightfold shrink only after its source
dimensions and maximum class are identified with a strictly parsed packing
source. -/
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
    8 * artifact.maximumChildEdges ≤ artifact.edgeCount := by
  rcases hvalid with
    ⟨_, _, hpacket, hpadded, hlength, hchild⟩
  have hlengthSource :
      artifact.packetLength *
          (source.artifact.labelCount * source.artifact.labelCount) ≤
        source.artifact.edgeCount := by
    calc
      artifact.packetLength *
          (source.artifact.labelCount * source.artifact.labelCount) =
          artifact.packetLength * artifact.labelClasses := by rw [hlabels]
      _ ≤ artifact.edgeCount := hlength
      _ = source.artifact.edgeCount := hedges
  have hchildSource :
      artifact.maximumChildEdges * 64 ≤ 2 *
        ((source.artifact.maximumClassSize ⌈/⌉ artifact.packetLength) *
          artifact.packetLength) *
        (source.artifact.labelCount * source.artifact.labelCount) := by
    calc
      artifact.maximumChildEdges * 64 ≤
          2 * artifact.paddedHeight * artifact.labelClasses := hchild
      _ = 2 *
          ((source.artifact.maximumClassSize ⌈/⌉ artifact.packetLength) *
            artifact.packetLength) *
          (source.artifact.labelCount * source.artifact.labelCount) := by
            rw [hpadded, hmaximum, hlabels]
  have hshrink :=
    source.historyChildStrictShrinkOfRoundedMaximumNat
      hparse artifact.packetLength artifact.maximumChildEdges
      hlengthSource hchildSource
  simpa only [← hedges] using hshrink

end Curve25519NodeShrinkArtifact
