import GeneratorAgreement
import Mathlib.NumberTheory.LSeries.PrimesInAP

namespace MumfordPrimeFamilyAudit
noncomputable section
open MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilySuccessorAudit MumfordPairwiseOutputAudit MumfordVerifiedFamilyGroupAudit
  MumfordGeneratorAgreementAudit

theorem prime_exponent_unbounded (B : ℕ) :
    ∃ t : ℕ,B<t ∧ Nat.Prime (exponent t) := by
  obtain ⟨p,hp,hprime,hmod⟩:=Nat.forall_exists_prime_gt_and_modEq
    (24*B+5) (q:=24) (a:=5) (by decide) (by decide)
  have hm : p%24=5 := by simpa only [Nat.ModEq] using hmod
  have he:=Nat.mod_add_div p 24
  refine ⟨p/24,by omega,?_⟩
  have hx : exponent (p/24)=p := by unfold exponent;omega
  exact hx.symm ▸ hprime

theorem prime_cardinality_and_order (B : ℕ) :
    ∃ t : ℕ,B<genus t ∧ B<exponent t ∧ Nat.Prime (Nat.card (OrbitPair t)) ∧
      addOrderOf (familyGenerator t)=exponent t := by
  obtain ⟨t,ht,hp⟩:=prime_exponent_unbounded B
  refine ⟨t,?_,?_,?_,generator_order t⟩
  · unfold genus
    omega
  · unfold exponent
    omega
  · rw [orbit_cardinality]
    exact hp

/-- An unbounded-prime-order arithmetic family, with all actual pairwise
normalized polynomial sums. Geometric ambient/Picard correspondence is not
assumed, and genus grows with the subgroup order. -/
theorem unbounded_prime_family_certificate :
    ∀ B : ℕ,∃ t : ℕ,
      B<genus t ∧ B<exponent t ∧ Nat.Prime (exponent t) ∧
      Nat.card (OrbitPair t)=exponent t ∧
      addOrderOf (familyGenerator t)=exponent t ∧
      (∀ D E : OrbitPair t,SelectedRun t (familyIndex t D) (familyIndex t E) (D+E).val) ∧
      (∀ D : OrbitPair t,∃ j : ℕ,j<exponent t ∧ j • familyGenerator t=D) ∧
      (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=1 ∧
      (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
        Nat.card (ReducedPair (genus t) (curvePolynomial t))<1 := by
  intro B
  obtain ⟨t,ht,hp⟩:=prime_exponent_unbounded B
  refine ⟨t,?_,?_,hp,orbit_cardinality t,generator_order t,actual_sum_runs t,
    all_elements_generated t,smooth_orbit_fraction_one t,smooth_ambient_fraction_lt_one t⟩
  · unfold genus
    omega
  · unfold exponent
    omega

theorem unbounded_prime_smoothness_counterexamples (B : ℕ) :
    ∃ t : ℕ,B<exponent t ∧ Nat.Prime (exponent t) ∧
      (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)≠
      (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
        Nat.card (ReducedPair (genus t) (curvePolynomial t)) := by
  obtain ⟨t,ht,hp⟩:=prime_exponent_unbounded B
  exact ⟨t,by unfold exponent;omega,hp,orbit_ambient_fractions_differ t⟩

theorem universal_prime_family_uniformity_false :
    ¬∀ t : ℕ,Nat.Prime (exponent t) →
      (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=
      (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
        Nat.card (ReducedPair (genus t) (curvePolynomial t)) := by
  intro h
  obtain ⟨t,_,hp,hneq⟩:=unbounded_prime_smoothness_counterexamples 0
  exact hneq (h t hp)

#print axioms prime_exponent_unbounded
#print axioms unbounded_prime_family_certificate
#print axioms universal_prime_family_uniformity_false
end
end MumfordPrimeFamilyAudit
