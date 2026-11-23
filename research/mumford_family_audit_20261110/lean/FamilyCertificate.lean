import Cycle

namespace MumfordFamilyCertificateAudit
noncomputable section
open MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilyTransitionsAudit MumfordFamilySuccessorAudit
  MumfordFamilyCycleAudit

theorem genus_unbounded (G : ℕ) : ∃ t : ℕ,G<genus t := by
  refine ⟨G,?_⟩
  unfold genus
  omega

theorem period_unbounded (N : ℕ) : ∃ t : ℕ,N<exponent t := by
  refine ⟨N,?_⟩
  unfold exponent
  omega

/-- A direct negation of uniform equality for these actual finite reduced-pair
sets and their polynomial-defined addition-by-(X,1) cycles. The statement
does not identify these sets with a geometric Picard group. -/
theorem universal_cycle_ambient_equality_false :
    ¬∀ t : ℕ,
      (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=
      (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
        Nat.card (ReducedPair (genus t) (curvePolynomial t)) := by
  intro h
  exact orbit_ambient_fractions_differ 0 (h 0)

theorem growing_family_arithmetic_certificate :
    (∀ G : ℕ,∃ t : ℕ,G<genus t) ∧
    (∀ N : ℕ,∃ t : ℕ,N<exponent t) ∧
    (∀ t : ℕ,
      (curvePolynomial t).Monic ∧
      (curvePolynomial t).Separable ∧
      (curvePolynomial t).natDegree=2*genus t+1 ∧
      Finite (ReducedPair (genus t) (curvePolynomial t)) ∧
      Nat.card (OrbitPair t)=exponent t ∧
      (∀ i : OrbitIndex t,∃! R : Pair t,
        OneStepRun (coeffA t i) (coeffB t i) (coeffC t i)
          (generator t) (orbitPair t i) R) ∧
      (∀ j : ℕ,∀ D : OrbitPair t,(actualStep t)^[j] D=D ↔ exponent t∣j) ∧
      (Nat.card {D : OrbitPair t // Smooth D.val}:ℚ)/Nat.card (OrbitPair t)=1 ∧
      (Nat.card {D : ReducedPair (genus t) (curvePolynomial t) // Smooth D}:ℚ)/
        Nat.card (ReducedPair (genus t) (curvePolynomial t))<1 ∧
      badPair t∉Set.range (orbitPair t)) := by
  refine ⟨genus_unbounded,period_unbounded,?_⟩
  intro t
  exact ⟨curve_monic t,curve_separable t,(curve_degree t).trans (exponent_genus t),
    inferInstance,orbit_cardinality t,run_output_exists_unique t,
    actualStep_period_iff t,smooth_orbit_fraction_one t,
    smooth_ambient_fraction_lt_one t,bad_outside_orbit t⟩

#print axioms universal_cycle_ambient_equality_false
#print axioms growing_family_arithmetic_certificate
end
end MumfordFamilyCertificateAudit
