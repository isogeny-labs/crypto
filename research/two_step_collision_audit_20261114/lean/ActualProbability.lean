module

public import FiniteGraphs

@[expose] public section
namespace TwoStepCollisionAudit
open scoped BigOperators ENNReal

noncomputable def sampleLaw : PMF Sample :=
  PMF.ofFintype (fun _ => (Fintype.card Sample : ℝ≥0∞)⁻¹) (by
    simp only [Finset.sum_const, Finset.card_univ, nsmul_eq_mul]
    exact ENNReal.mul_inv_cancel (by exact_mod_cast Fintype.card_ne_zero) (by simp))

@[simp] theorem sampleLaw_mass (z : Sample) :
    sampleLaw z = (Fintype.card Sample : ℝ≥0∞)⁻¹ := rfl

/-- Actual uniform random static labels and independent uniform starts:
seven vertex-label factors and two independent starting-point factors. -/
theorem sampleLaw_product_mass (z : Sample) :
    sampleLaw z = (∏ _ : G, (2 : ℝ≥0∞)⁻¹) * (7 : ℝ≥0∞)⁻¹ * (7 : ℝ≥0∞)⁻¹ := by
  rw [sampleLaw_mass, sample_card]
  simp only [Finset.prod_const, Finset.card_univ, G, ZMod.card]
  apply (ENNReal.toReal_eq_toReal_iff' (by finiteness) (by finiteness)).mp
  norm_num

theorem sampleLaw_event_mass (s : Finset Sample) :
    sampleLaw.toOuterMeasure (s : Set Sample) = (s.card : ℝ≥0∞) / 6272 := by
  rw [PMF.toOuterMeasure_apply_finset]
  simp only [sampleLaw_mass, sample_card, Finset.sum_const, nsmul_eq_mul, div_eq_mul_inv]
  norm_num

theorem good_one_probability :
    sampleLaw.toOuterMeasure (oneSamples goodIncrements : Set Sample) = (1 : ℝ≥0∞) / 14 := by
  rw [sampleLaw_event_mass, good_one_count]
  apply (ENNReal.div_eq_div_iff (by norm_num) (by norm_num) (by norm_num) (by norm_num)).mpr
  norm_num

theorem good_two_probability :
    sampleLaw.toOuterMeasure (twoSamples goodIncrements : Set Sample) = (3 : ℝ≥0∞) / 56 := by
  rw [sampleLaw_event_mass, good_two_count]
  apply (ENNReal.div_eq_div_iff (by norm_num) (by norm_num) (by norm_num) (by norm_num)).mpr
  norm_num

theorem bad_two_probability :
    sampleLaw.toOuterMeasure (twoSamples badIncrements : Set Sample) = (1 : ℝ≥0∞) / 56 := by
  rw [sampleLaw_event_mass, bad_two_count]
  apply (ENNReal.div_eq_div_iff (by norm_num) (by norm_num) (by norm_num) (by norm_num)).mpr
  norm_num

theorem actual_two_rate_not_degree_one :
    sampleLaw.toOuterMeasure (twoSamples goodIncrements : Set Sample) ≠
      sampleLaw.toOuterMeasure (oneSamples goodIncrements : Set Sample) := by
  rw [good_two_probability, good_one_probability]
  intro he
  have hc := congrArg ENNReal.toReal he
  norm_num at hc

theorem degree_two_genericity_cannot_be_dropped :
    ¬∀ g : Label → G, Function.Injective g → (∀ i, g i ≠ 0) →
      sampleLaw.toOuterMeasure (twoSamples g : Set Sample) = (3 : ℝ≥0∞) / 56 := by
  intro h
  have he := h badIncrements bad_still_distinct_nonzero.1 bad_still_distinct_nonzero.2
  rw [bad_two_probability] at he
  have hc := congrArg ENNReal.toReal he
  norm_num at hc

#print axioms sampleLaw_product_mass
#print axioms degree_two_genericity_cannot_be_dropped
end TwoStepCollisionAudit
