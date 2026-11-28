module

public import GroupAssignmentArtifact

@[expose] public section

/-!
# Native packet-owner assignment artifact

This module imports one native packet owner per parent edge, joins it with the
endpoint-group artifact, and instantiates the generic packet/group child-family
factorization interface.  It does not bind packet entry values, scalar modes,
child tensor values, or the external contraction theorem.
-/

namespace Curve25519PacketAssignmentArtifact

open scoped BigOperators
open Curve25519GroupFactorization
open Curve25519GroupAssignmentArtifact

/-- Canonical packet owner array in parent-edge order. -/
structure PacketAssignmentArtifact where
  schema : ℕ
  packetCount : ℕ
  edgeCount : ℕ
  ownerCount : ℕ
  owners : List ℕ
  deriving DecidableEq, Inhabited, Repr

/-- Direct shape and range contract for the native packet projection. -/
def PacketAssignmentArtifact.WellFormed
    (artifact : PacketAssignmentArtifact) : Prop :=
  artifact.schema = 1 ∧
    0 < artifact.packetCount ∧
    artifact.owners.length = artifact.ownerCount ∧
    artifact.ownerCount = artifact.edgeCount ∧
    ∀ packet ∈ artifact.owners, packet < artifact.packetCount

/-- Executable shape and range checker. -/
def PacketAssignmentArtifact.wellFormed
    (artifact : PacketAssignmentArtifact) : Bool :=
  artifact.schema == 1 &&
    decide (0 < artifact.packetCount) &&
    artifact.owners.length == artifact.ownerCount &&
    artifact.ownerCount == artifact.edgeCount &&
    artifact.owners.all (fun packet => decide (packet < artifact.packetCount))

/-- The executable checker is exactly the named source contract. -/
theorem PacketAssignmentArtifact.wellFormed_eq_true_iff
    (artifact : PacketAssignmentArtifact) :
    artifact.wellFormed = true ↔ artifact.WellFormed := by
  simp [PacketAssignmentArtifact.wellFormed,
    PacketAssignmentArtifact.WellFormed, List.all_eq_true, and_assoc]

/-- Fixed native field order: four header values followed by the owner array. -/
def PacketAssignmentArtifact.projectionValues
    (artifact : PacketAssignmentArtifact) : List ℕ :=
  [artifact.schema, artifact.packetCount, artifact.edgeCount,
    artifact.ownerCount] ++ artifact.owners

/-- Parse the complete flat owner projection. -/
def parsePacketAssignmentArtifactUnchecked :
    List ℕ → Option PacketAssignmentArtifact
  | schema :: packetCount :: edgeCount :: ownerCount :: owners =>
      some { schema, packetCount, edgeCount, ownerCount, owners }
  | _ => none

/-- Parse and check exact coverage and packet-owner ranges. -/
def parsePacketAssignmentArtifact
    (values : List ℕ) : Option PacketAssignmentArtifact := do
  let artifact ← parsePacketAssignmentArtifactUnchecked values
  if artifact.wellFormed then some artifact else none

/-- Canonical owner projections round-trip through the unchecked parser. -/
theorem parsePacketAssignmentArtifactUnchecked_projectionValues
    (artifact : PacketAssignmentArtifact) :
    parsePacketAssignmentArtifactUnchecked artifact.projectionValues =
      some artifact := by
  rfl

/-- Every well-formed canonical projection survives checked parsing. -/
theorem parsePacketAssignmentArtifact_projectionValues
    (artifact : PacketAssignmentArtifact) (hvalid : artifact.WellFormed) :
    parsePacketAssignmentArtifact artifact.projectionValues = some artifact := by
  simp [parsePacketAssignmentArtifact,
    parsePacketAssignmentArtifactUnchecked_projectionValues,
    PacketAssignmentArtifact.wellFormed_eq_true_iff, hvalid]

/-- Every successful parse satisfies the direct owner-array contract. -/
theorem parsePacketAssignmentArtifact_wellFormed
    {values : List ℕ} {artifact : PacketAssignmentArtifact}
    (hparse : parsePacketAssignmentArtifact values = some artifact) :
    artifact.WellFormed := by
  unfold parsePacketAssignmentArtifact at hparse
  rcases Option.bind_eq_some_iff.mp hparse with
    ⟨candidate, _, hchecked⟩
  split at hchecked
  next hvalid =>
    have hartifact : candidate = artifact := Option.some.inj hchecked
    subst candidate
    exact artifact.wellFormed_eq_true_iff.mp hvalid
  next => simp at hchecked

/-- Total finite packet lookup; well-formed artifacts make the modulo exact. -/
def packetAt (owners : List ℕ) (packetCount : ℕ) (hpackets : 0 < packetCount)
    (edge : ℕ) : Fin packetCount :=
  ⟨owners.getD edge 0 % packetCount, Nat.mod_lt _ hpackets⟩

/-- Native packet owner of one parent edge. -/
def PacketAssignmentArtifact.edgePacket
    (artifact : PacketAssignmentArtifact)
    (hpackets : 0 < artifact.packetCount)
    (edge : Fin artifact.edgeCount) : Fin artifact.packetCount :=
  packetAt artifact.owners artifact.packetCount hpackets edge.val

/-- Typed packet owners retain the exact serialized owner value. -/
theorem PacketAssignmentArtifact.edgePacket_val
    (artifact : PacketAssignmentArtifact) (hvalid : artifact.WellFormed)
    (edge : Fin artifact.edgeCount) :
    (artifact.edgePacket hvalid.2.1 edge).val =
      artifact.owners.getD edge.val 0 := by
  have hindex : edge.val < artifact.owners.length := by
    rw [hvalid.2.2.1, hvalid.2.2.2.1]
    exact edge.isLt
  have hbound : artifact.owners.getD edge.val 0 < artifact.packetCount := by
    rw [List.getD_eq_getElem artifact.owners 0 hindex]
    exact hvalid.2.2.2.2 _ (List.mem_of_getElem rfl)
  exact Nat.mod_eq_of_lt hbound

/-- Joint native group/packet artifact with an explicit group-prefix length. -/
structure IncidenceAssignmentBundle where
  schema : ℕ
  groups : GroupAssignmentArtifact
  packets : PacketAssignmentArtifact
  deriving DecidableEq, Inhabited, Repr

/-- Complete checked contract, including common parent-edge count. -/
def IncidenceAssignmentBundle.Valid
    (bundle : IncidenceAssignmentBundle) : Prop :=
  bundle.schema = 1 ∧
    bundle.groups.WellFormed ∧
    bundle.packets.WellFormed ∧
    bundle.groups.edgeCount = bundle.packets.edgeCount

/-- Executable joint contract. -/
def IncidenceAssignmentBundle.valid
    (bundle : IncidenceAssignmentBundle) : Bool :=
  bundle.schema == 1 &&
    bundle.groups.wellFormed &&
    bundle.packets.wellFormed &&
    bundle.groups.edgeCount == bundle.packets.edgeCount

/-- Executable joint checking is exactly the named contract. -/
theorem IncidenceAssignmentBundle.valid_eq_true_iff
    (bundle : IncidenceAssignmentBundle) :
    bundle.valid = true ↔ bundle.Valid := by
  simp [IncidenceAssignmentBundle.valid, IncidenceAssignmentBundle.Valid,
    GroupAssignmentArtifact.wellFormed_eq_true_iff,
    PacketAssignmentArtifact.wellFormed_eq_true_iff, and_assoc]

/-- Joint native field order: schema and group-prefix length, followed by both
complete projections. -/
def IncidenceAssignmentBundle.projectionValues
    (bundle : IncidenceAssignmentBundle) : List ℕ :=
  [bundle.schema, bundle.groups.projectionValues.length] ++
    bundle.groups.projectionValues ++ bundle.packets.projectionValues

/-- Parse, check, and align both sides of the joint native projection. -/
def parseIncidenceAssignmentBundle :
    List ℕ → Option IncidenceAssignmentBundle
  | schema :: groupLength :: values => do
      let (groupValues, packetValues) := values.splitAt groupLength
      let groups ← parseGroupAssignmentArtifact groupValues
      let packets ← parsePacketAssignmentArtifact packetValues
      let bundle : IncidenceAssignmentBundle := { schema, groups, packets }
      if bundle.valid then some bundle else none
  | _ => none

/-- Every valid joint artifact survives the complete checked importer. -/
theorem parseIncidenceAssignmentBundle_projectionValues
    (bundle : IncidenceAssignmentBundle) (hvalid : bundle.Valid) :
    parseIncidenceAssignmentBundle bundle.projectionValues = some bundle := by
  rcases bundle with ⟨schema, groups, packets⟩
  rcases hvalid with ⟨hschema, hgroups, hpackets, haligned⟩
  dsimp only at hschema hgroups hpackets haligned ⊢
  subst schema
  simp [parseIncidenceAssignmentBundle,
    IncidenceAssignmentBundle.projectionValues,
    parseGroupAssignmentArtifact_projectionValues groups hgroups,
    parsePacketAssignmentArtifact_projectionValues packets hpackets,
    IncidenceAssignmentBundle.valid_eq_true_iff, hgroups, hpackets,
    haligned, IncidenceAssignmentBundle.Valid]

/-- Every successful joint parse satisfies both source contracts and their
edge-count alignment. -/
theorem parseIncidenceAssignmentBundle_valid
    {values : List ℕ} {bundle : IncidenceAssignmentBundle}
    (hparse : parseIncidenceAssignmentBundle values = some bundle) :
    bundle.Valid := by
  unfold parseIncidenceAssignmentBundle at hparse
  split at hparse
  next schema groupLength values =>
    rcases Option.bind_eq_some_iff.mp hparse with
      ⟨groups, hgroups, hafterGroups⟩
    rcases Option.bind_eq_some_iff.mp hafterGroups with
      ⟨packets, hpackets, hchecked⟩
    dsimp only at hchecked
    split at hchecked
    next hvalid =>
      have hbundle :
          ({ schema, groups, packets } : IncidenceAssignmentBundle) =
            bundle := Option.some.inj hchecked
      subst bundle
      exact (IncidenceAssignmentBundle.valid_eq_true_iff _).mp hvalid
    next => simp at hchecked
  next => simp at hparse

/-- Concrete endpoint-group function carried by a valid joint artifact. -/
def IncidenceAssignmentBundle.groupOfIncidence
    (bundle : IncidenceAssignmentBundle) (hvalid : bundle.Valid) :
    EndpointIncidence (Fin bundle.groups.edgeCount) → Fin bundle.groups.radix :=
  endpointGroup
    (bundle.groups.rowGroup hvalid.2.1.2.1)
    (bundle.groups.columnGroup hvalid.2.1.2.1)

/-- Both endpoint incidences of one edge inherit its concrete packet owner. -/
def IncidenceAssignmentBundle.packetOfIncidence
    (bundle : IncidenceAssignmentBundle) (hvalid : bundle.Valid) :
    EndpointIncidence (Fin bundle.groups.edgeCount) →
      Fin bundle.packets.packetCount :=
  fun incidence => packetAt bundle.packets.owners bundle.packets.packetCount
    hvalid.2.2.1.2.1 incidence.1.val

/-- The joint packet function retains the exact serialized owner value. -/
theorem IncidenceAssignmentBundle.packetOfIncidence_val
    (bundle : IncidenceAssignmentBundle) (hvalid : bundle.Valid)
    (incidence : EndpointIncidence (Fin bundle.groups.edgeCount)) :
    (bundle.packetOfIncidence hvalid incidence).val =
      bundle.packets.owners.getD incidence.1.val 0 := by
  have hindex : incidence.1.val < bundle.packets.owners.length := by
    rw [hvalid.2.2.1.2.2.1, hvalid.2.2.1.2.2.2.1,
      ← hvalid.2.2.2]
    exact incidence.1.isLt
  have hbound :
      bundle.packets.owners.getD incidence.1.val 0 <
        bundle.packets.packetCount := by
    rw [List.getD_eq_getElem bundle.packets.owners 0 hindex]
    exact hvalid.2.2.1.2.2.2.2 _ (List.mem_of_getElem rfl)
  exact Nat.mod_eq_of_lt hbound

/-- The row and column incidences of one parent edge have the same packet. -/
theorem IncidenceAssignmentBundle.endpointPacket_pair
    (bundle : IncidenceAssignmentBundle) (hvalid : bundle.Valid)
    (edge : Fin bundle.groups.edgeCount) :
    bundle.packetOfIncidence hvalid (edge, false) =
      bundle.packetOfIncidence hvalid (edge, true) := by
  rfl

/-- The imported group and packet functions instantiate the complete finite
child-family factorization for arbitrary old and packet entry terms. -/
theorem IncidenceAssignmentBundle.completeChildFamilyFactorization
    (bundle : IncidenceAssignmentBundle) (hvalid : bundle.Valid)
    {R : Type*} [CommSemiring R]
    (oldTerm : ∀ target : Fin bundle.groups.radix,
      ({incidence // bundle.groupOfIncidence hvalid incidence = target} → Bool) → R)
    (packetEntry : ∀ (target : Fin bundle.groups.radix)
      (selectedPacket : Fin bundle.packets.packetCount),
      ({incidence // bundle.groupOfIncidence hvalid incidence = target ∧
        bundle.packetOfIncidence hvalid incidence = selectedPacket} → Bool) → R) :
    (∑ assignment : EndpointIncidence (Fin bundle.groups.edgeCount) → Bool,
        ∏ target,
          groupChildTerm (bundle.groupOfIncidence hvalid)
            (bundle.packetOfIncidence hvalid) oldTerm packetEntry target
            (fun incidence => assignment incidence.1)) =
      ∏ target, ∑ assignment,
        groupChildTerm (bundle.groupOfIncidence hvalid)
          (bundle.packetOfIncidence hvalid) oldTerm packetEntry target assignment := by
  exact complete_child_family_factorization
    (bundle.groupOfIncidence hvalid) (bundle.packetOfIncidence hvalid)
    oldTerm packetEntry

end Curve25519PacketAssignmentArtifact
