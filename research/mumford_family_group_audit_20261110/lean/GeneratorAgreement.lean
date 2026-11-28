import VerifiedFamilyGroup

namespace MumfordGeneratorAgreementAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilyTransitionsAudit MumfordFamilySuccessorAudit
  MumfordFamilyCycleAudit MumfordIndexArithmeticAudit MumfordPairwiseOutputAudit
  MumfordVerifiedFamilyGroupAudit

def generatorIndex (t : ℕ) : OrbitIndex t :=
  some (false,⟨0,by unfold genus;omega⟩)

theorem familyIndex_generator (t : ℕ) : familyIndex t (familyGenerator t)=generatorIndex t :=
  (orbitEquiv t).symm_apply_apply _

theorem selectedTriple_generator (t : ℕ) (i : OrbitIndex t) :
    bezoutTriple t (generatorIndex t) i=(coeffA t i,coeffB t i,coeffC t i) := by
  have h : -(1:K[X])=2 := by linear_combination -polynomial_three_zero
  cases i with
  | none => rfl
  | some p =>
    rcases p with ⟨b,d⟩
    cases b <;> simp [bezoutTriple,generatorIndex,sign,coeffA,coeffB,coeffC,h]

theorem family_generator_value (t : ℕ) : (familyGenerator t).val=generator t := by
  apply pair_ext
  · rw [(generator_coordinates t).1]
    simp [generator,powerPair]
  · rw [(generator_coordinates t).2]
    simp [generator,powerPair]

theorem generator_add_run (t : ℕ) (D : OrbitPair t) :
    OneStepRun (coeffA t (familyIndex t D)) (coeffB t (familyIndex t D))
      (coeffC t (familyIndex t D)) (generator t) (orbitPair t (familyIndex t D))
      (familyGenerator t+D).val := by
  have h:=actual_sum_runs t (familyGenerator t) D
  unfold SelectedRun at h
  rw [familyIndex_generator,selectedTriple_generator] at h
  change OneStepRun (coeffA t (familyIndex t D)) (coeffB t (familyIndex t D))
    (coeffC t (familyIndex t D)) (familyGenerator t).val
    (orbitPair t (familyIndex t D)) (familyGenerator t+D).val at h
  rw [family_generator_value] at h
  exact h

theorem actualStep_eq_generator_add (t : ℕ) (D : OrbitPair t) :
    actualStep t D=familyGenerator t+D := by
  apply Subtype.ext
  change stepOutput t (familyIndex t D)=(familyGenerator t+D).val
  exact run_output_unique _ _ _ _ _ _ _ (stepOutput_run t (familyIndex t D))
    (generator_add_run t D)

theorem growing_verified_group_certificate :
    (∀ t : ℕ,
      Nat.card (OrbitPair t)=exponent t ∧
      addOrderOf (familyGenerator t)=exponent t ∧
      (∀ D E : OrbitPair t,SelectedRun t (familyIndex t D) (familyIndex t E) (D+E).val) ∧
      (∀ D : OrbitPair t,actualStep t D=familyGenerator t+D) ∧
      (∀ D : OrbitPair t,∃ j : ℕ,j<exponent t ∧ j • familyGenerator t=D) ∧
      (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=1 ∧
      (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
        Nat.card (ReducedPair (genus t) (curvePolynomial t))<1) ∧
    (∀ G : ℕ,∃ t : ℕ,G<genus t) ∧ (∀ N : ℕ,∃ t : ℕ,N<exponent t) := by
  refine ⟨?_,MumfordFamilyCertificateAudit.genus_unbounded,
    MumfordFamilyCertificateAudit.period_unbounded⟩
  intro t
  exact ⟨orbit_cardinality t,generator_order t,actual_sum_runs t,
    actualStep_eq_generator_add t,all_elements_generated t,smooth_orbit_fraction_one t,
    smooth_ambient_fraction_lt_one t⟩

#print axioms actualStep_eq_generator_add
#print axioms growing_verified_group_certificate
end
end MumfordGeneratorAgreementAudit
