import PairwiseOutput
import Mathlib.GroupTheory.OrderOfElement

namespace MumfordVerifiedFamilyGroupAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilySuccessorAudit MumfordFamilyCycleAudit
  MumfordIndexArithmeticAudit MumfordPairwiseOutputAudit

def familyIndex (t : ℕ) (D : OrbitPair t) : OrbitIndex t := (orbitEquiv t).symm D

/-- Unique normalized polynomial output, with closure proved by the all-pair
certificates, before introducing the group structure below. -/
def cantorAdd (t : ℕ) (D E : OrbitPair t) : OrbitPair t :=
  ⟨pairwiseOutput t (familyIndex t D) (familyIndex t E),by
    rw [pairwiseOutput_eq]
    exact ⟨combinedIndex t (familyIndex t D) (familyIndex t E),rfl⟩⟩

theorem cantorAdd_runs (t : ℕ) (D E : OrbitPair t) :
    SelectedRun t (familyIndex t D) (familyIndex t E) (cantorAdd t D E).val :=
  pairwiseOutput_runs t _ _

theorem cantorAdd_equiv (t : ℕ) (i j : OrbitIndex t) :
    cantorAdd t ((orbitEquiv t) i) ((orbitEquiv t) j)=
      (orbitEquiv t) (combinedIndex t i j) := by
  apply Subtype.ext
  change pairwiseOutput t ((orbitEquiv t).symm ((orbitEquiv t) i))
    ((orbitEquiv t).symm ((orbitEquiv t) j))=orbitPair t (combinedIndex t i j)
  rw [Equiv.symm_apply_apply,Equiv.symm_apply_apply,pairwiseOutput_eq]

def scalarEquiv (t : ℕ) : OrbitPair t≃ZMod (exponent t) :=
  (orbitEquiv t).symm.trans (residueEquiv t)

theorem scalar_pair (t : ℕ) (i : OrbitIndex t) :
    scalarEquiv t ((orbitEquiv t) i)=residue t i := by
  change residue t ((orbitEquiv t).symm ((orbitEquiv t) i))=residue t i
  rw [Equiv.symm_apply_apply]

theorem scalar_cantorAdd (t : ℕ) (D E : OrbitPair t) :
    scalarEquiv t (cantorAdd t D E)=scalarEquiv t D+scalarEquiv t E := by
  obtain ⟨i,rfl⟩:=(orbitEquiv t).surjective D
  obtain ⟨j,rfl⟩:=(orbitEquiv t).surjective E
  rw [cantorAdd_equiv,scalar_pair,scalar_pair,scalar_pair]
  exact combinedIndex_residue t i j

instance familyAddCommGroup (t : ℕ) : AddCommGroup (OrbitPair t) :=
  (scalarEquiv t).addCommGroup

theorem cantorAdd_eq_add (t : ℕ) (D E : OrbitPair t) : cantorAdd t D E=D+E := by
  apply (scalarEquiv t).injective
  rw [scalar_cantorAdd]
  change (scalarEquiv t).addEquiv D+(scalarEquiv t).addEquiv E=
    (scalarEquiv t).addEquiv (D+E)
  exact (map_add _ _ _).symm

theorem actual_sum_runs (t : ℕ) (D E : OrbitPair t) :
    SelectedRun t (familyIndex t D) (familyIndex t E) (D+E).val := by
  rw [←cantorAdd_eq_add]
  exact cantorAdd_runs t D E

theorem zero_pair (t : ℕ) : (orbitEquiv t) none=0 := by
  apply (scalarEquiv t).injective
  change scalarEquiv t ((orbitEquiv t) none)=(scalarEquiv t).addEquiv 0
  rw [map_zero,scalar_pair]
  rfl

theorem zero_coordinates (t : ℕ) : (0 : OrbitPair t).val.u=1 ∧ (0 : OrbitPair t).val.v=0 := by
  rw [←zero_pair]
  exact ⟨rfl,rfl⟩

def familyGenerator (t : ℕ) : OrbitPair t :=
  (orbitEquiv t) (some (false,⟨0,by unfold genus;omega⟩))

theorem generator_scalar (t : ℕ) : scalarEquiv t (familyGenerator t)=1 := by
  rw [familyGenerator,scalar_pair]
  simp [residue]

theorem generator_coordinates (t : ℕ) :
    (familyGenerator t).val.u=X ∧ (familyGenerator t).val.v=1 := by
  change (X:K[X])^1=X ∧ C (1:K)=1
  simp

theorem generator_order (t : ℕ) : addOrderOf (familyGenerator t)=exponent t := by
  have h:=(scalarEquiv t).addEquiv.addOrderOf_eq (familyGenerator t)
  change addOrderOf (scalarEquiv t (familyGenerator t))=addOrderOf (familyGenerator t) at h
  rw [generator_scalar,ZMod.addOrderOf_one] at h
  exact h.symm

theorem generator_nsmul_zero_iff (t j : ℕ) :
    j • familyGenerator t=0 ↔ exponent t∣j := by
  rw [←(scalarEquiv t).injective.eq_iff]
  change (scalarEquiv t).addEquiv (j • familyGenerator t)=(scalarEquiv t).addEquiv 0 ↔ _
  rw [map_nsmul,map_zero]
  change j • scalarEquiv t (familyGenerator t)=0 ↔ _
  rw [generator_scalar,nsmul_one]
  exact ZMod.natCast_eq_zero_iff j (exponent t)

theorem all_elements_generated (t : ℕ) (D : OrbitPair t) :
    ∃ j : ℕ,j<exponent t ∧ j • familyGenerator t=D := by
  refine ⟨(scalarEquiv t D).val,?_,?_⟩
  · exact ZMod.val_lt _
  · apply (scalarEquiv t).injective
    change (scalarEquiv t).addEquiv ((scalarEquiv t D).val • familyGenerator t)=scalarEquiv t D
    rw [map_nsmul]
    change (scalarEquiv t D).val • scalarEquiv t (familyGenerator t)=scalarEquiv t D
    rw [generator_scalar,nsmul_one,ZMod.natCast_zmod_val]

theorem full_family_group_certificate (t : ℕ) :
    Nat.card (OrbitPair t)=exponent t ∧
    addOrderOf (familyGenerator t)=exponent t ∧
    (∀ D E : OrbitPair t,SelectedRun t (familyIndex t D) (familyIndex t E) (D+E).val) ∧
    (∀ D : OrbitPair t,∃ j : ℕ,j<exponent t ∧ j • familyGenerator t=D) ∧
    (∀ D : OrbitPair t,Smooth D.val) ∧
    (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=1 ∧
    (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
      Nat.card (ReducedPair (genus t) (curvePolynomial t))<1 :=
  ⟨orbit_cardinality t,generator_order t,actual_sum_runs t,all_elements_generated t,
    all_orbit_smooth t,smooth_orbit_fraction_one t,smooth_ambient_fraction_lt_one t⟩

#print axioms scalar_cantorAdd
#print axioms actual_sum_runs
#print axioms generator_order
#print axioms full_family_group_certificate
end
end MumfordVerifiedFamilyGroupAudit
