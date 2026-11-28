module

public import GroupFactorization

@[expose] public section

/-!
# Native endpoint-group assignment artifact

This module imports the compact row/column group arrays emitted by the native
factorization certificate.  It binds those arrays to the finite group-fiber
interface used by `complete_group_child_incidence_count`; it does not bind the
child tensor values or the external contraction theorem.
-/

namespace Curve25519GroupAssignmentArtifact

open scoped BigOperators
open Curve25519GroupFactorization

/-- Canonical row/column group arrays in parent-edge order. -/
structure GroupAssignmentArtifact where
  schema : ℕ
  radix : ℕ
  edgeCount : ℕ
  leftCount : ℕ
  rightCount : ℕ
  leftGroups : List ℕ
  rightGroups : List ℕ
  deriving DecidableEq, Inhabited, Repr

/-- Direct shape and range contract for the native group projection. -/
def GroupAssignmentArtifact.WellFormed
    (artifact : GroupAssignmentArtifact) : Prop :=
  artifact.schema = 1 ∧
    0 < artifact.radix ∧
    artifact.leftGroups.length = artifact.leftCount ∧
    artifact.rightGroups.length = artifact.rightCount ∧
    artifact.leftCount = artifact.edgeCount ∧
    artifact.rightCount = artifact.edgeCount ∧
    (∀ group ∈ artifact.leftGroups, group < artifact.radix) ∧
    (∀ group ∈ artifact.rightGroups, group < artifact.radix)

/-- Executable shape and range checker. -/
def GroupAssignmentArtifact.wellFormed
    (artifact : GroupAssignmentArtifact) : Bool :=
  artifact.schema == 1 &&
    decide (0 < artifact.radix) &&
    artifact.leftGroups.length == artifact.leftCount &&
    artifact.rightGroups.length == artifact.rightCount &&
    artifact.leftCount == artifact.edgeCount &&
    artifact.rightCount == artifact.edgeCount &&
    artifact.leftGroups.all (fun group => decide (group < artifact.radix)) &&
    artifact.rightGroups.all (fun group => decide (group < artifact.radix))

/-- The executable checker is exactly the named source contract. -/
theorem GroupAssignmentArtifact.wellFormed_eq_true_iff
    (artifact : GroupAssignmentArtifact) :
    artifact.wellFormed = true ↔ artifact.WellFormed := by
  simp [GroupAssignmentArtifact.wellFormed,
    GroupAssignmentArtifact.WellFormed, List.all_eq_true, and_assoc]

/-- Fixed native field order: five header values, then left and right arrays. -/
def GroupAssignmentArtifact.projectionValues
    (artifact : GroupAssignmentArtifact) : List ℕ :=
  [artifact.schema, artifact.radix, artifact.edgeCount,
    artifact.leftCount, artifact.rightCount] ++
      artifact.leftGroups ++ artifact.rightGroups

/-- Split the flat native projection after its declared left-group count. -/
def parseGroupAssignmentArtifactUnchecked :
    List ℕ → Option GroupAssignmentArtifact
  | schema :: radix :: edgeCount :: leftCount :: rightCount :: groups =>
      let (leftGroups, rightGroups) := groups.splitAt leftCount
      some {
        schema, radix, edgeCount, leftCount, rightCount,
        leftGroups, rightGroups
      }
  | _ => none

/-- Parse and check exact array lengths and group ranges. -/
def parseGroupAssignmentArtifact
    (values : List ℕ) : Option GroupAssignmentArtifact := do
  let artifact ← parseGroupAssignmentArtifactUnchecked values
  if artifact.wellFormed then some artifact else none

/-- Canonical projections round-trip through the unchecked split. -/
theorem parseGroupAssignmentArtifactUnchecked_projectionValues
    (artifact : GroupAssignmentArtifact)
    (hleft : artifact.leftGroups.length = artifact.leftCount) :
    parseGroupAssignmentArtifactUnchecked artifact.projectionValues =
      some artifact := by
  simp [GroupAssignmentArtifact.projectionValues,
    parseGroupAssignmentArtifactUnchecked, hleft]

/-- Every well-formed canonical projection survives checked parsing. -/
theorem parseGroupAssignmentArtifact_projectionValues
    (artifact : GroupAssignmentArtifact) (hvalid : artifact.WellFormed) :
    parseGroupAssignmentArtifact artifact.projectionValues = some artifact := by
  unfold parseGroupAssignmentArtifact
  rw [parseGroupAssignmentArtifactUnchecked_projectionValues artifact
    hvalid.2.2.1]
  simp [GroupAssignmentArtifact.wellFormed_eq_true_iff, hvalid]

/-- Every successful parse satisfies the direct array contract. -/
theorem parseGroupAssignmentArtifact_wellFormed
    {values : List ℕ} {artifact : GroupAssignmentArtifact}
    (hparse : parseGroupAssignmentArtifact values = some artifact) :
    artifact.WellFormed := by
  unfold parseGroupAssignmentArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨candidate, _, hchecked⟩
  split at hchecked
  next hvalid =>
    have hartifact : candidate = artifact := Option.some.inj hchecked
    subst candidate
    exact artifact.wellFormed_eq_true_iff.mp hvalid
  next => simp at hchecked

/-- Total finite group lookup; well-formed artifacts make the modulo exact. -/
def groupAt (groups : List ℕ) (radix : ℕ) (hradix : 0 < radix)
    (index : ℕ) : Fin radix :=
  ⟨groups.getD index 0 % radix, Nat.mod_lt _ hradix⟩

/-- Native row-endpoint group function. -/
def GroupAssignmentArtifact.rowGroup
    (artifact : GroupAssignmentArtifact) (hradix : 0 < artifact.radix)
    (edge : Fin artifact.edgeCount) : Fin artifact.radix :=
  groupAt artifact.leftGroups artifact.radix hradix edge.val

/-- Native column-endpoint group function. -/
def GroupAssignmentArtifact.columnGroup
    (artifact : GroupAssignmentArtifact) (hradix : 0 < artifact.radix)
    (edge : Fin artifact.edgeCount) : Fin artifact.radix :=
  groupAt artifact.rightGroups artifact.radix hradix edge.val

/-- Typed row groups retain the exact serialized group value. -/
theorem GroupAssignmentArtifact.rowGroup_val
    (artifact : GroupAssignmentArtifact) (hvalid : artifact.WellFormed)
    (edge : Fin artifact.edgeCount) :
    (artifact.rowGroup hvalid.2.1 edge).val =
      artifact.leftGroups.getD edge.val 0 := by
  have hindex : edge.val < artifact.leftGroups.length := by
    rw [hvalid.2.2.1, hvalid.2.2.2.2.1]
    exact edge.isLt
  have hbound : artifact.leftGroups.getD edge.val 0 < artifact.radix := by
    rw [List.getD_eq_getElem artifact.leftGroups 0 hindex]
    exact hvalid.2.2.2.2.2.2.1 _ (List.mem_of_getElem rfl)
  exact Nat.mod_eq_of_lt hbound

/-- Typed column groups retain the exact serialized group value. -/
theorem GroupAssignmentArtifact.columnGroup_val
    (artifact : GroupAssignmentArtifact) (hvalid : artifact.WellFormed)
    (edge : Fin artifact.edgeCount) :
    (artifact.columnGroup hvalid.2.1 edge).val =
      artifact.rightGroups.getD edge.val 0 := by
  have hindex : edge.val < artifact.rightGroups.length := by
    rw [hvalid.2.2.2.1, hvalid.2.2.2.2.2.1]
    exact edge.isLt
  have hbound : artifact.rightGroups.getD edge.val 0 < artifact.radix := by
    rw [List.getD_eq_getElem artifact.rightGroups 0 hindex]
    exact hvalid.2.2.2.2.2.2.2 _ (List.mem_of_getElem rfl)
  exact Nat.mod_eq_of_lt hbound

/-- The actual imported row/column arrays instantiate the complete group-fiber
split, whose endpoint mass is exactly twice the parent edge count. -/
theorem GroupAssignmentArtifact.completeEndpointIncidenceCount
    (artifact : GroupAssignmentArtifact) (hvalid : artifact.WellFormed) :
    (∑ target : Fin artifact.radix,
      Fintype.card
        {incidence : EndpointIncidence (Fin artifact.edgeCount) //
          endpointGroup
            (artifact.rowGroup hvalid.2.1)
            (artifact.columnGroup hvalid.2.1) incidence = target}) =
      2 * artifact.edgeCount := by
  simpa using
    complete_group_child_incidence_count
      (artifact.rowGroup hvalid.2.1)
      (artifact.columnGroup hvalid.2.1)

end Curve25519GroupAssignmentArtifact
