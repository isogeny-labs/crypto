module

public import LabelCoefficient
public import ActualProbability

@[expose] public section
namespace TwoStepCollisionAudit
open scoped BigOperators ENNReal

noncomputable def uniformLabel (_ : Label) : ℝ := 1 / 2

theorem uniform_label_moments :
    (∑ i : Label, uniformLabel i) = 1 ∧ secondMoment uniformLabel = 1 / 2 ∧
      fourthMoment uniformLabel = 1 / 8 := by
  norm_num [uniformLabel, secondMoment, fourthMoment, Label, Fin.sum_univ_two]

theorem uniform_label_coefficient : labelCoefficient uniformLabel = 3 / 8 := by
  rw [degree_two_label_coefficient uniformLabel uniform_label_moments.1,
    uniform_label_moments.2.1, uniform_label_moments.2.2]
  norm_num

theorem good_actual_matches_degree_two_coefficient :
    sampleLaw.toOuterMeasure (twoSamples goodIncrements : Set Sample) =
      ENNReal.ofReal (labelCoefficient uniformLabel) / 7 := by
  rw [good_two_probability, uniform_label_coefficient]
  apply (ENNReal.toReal_eq_toReal_iff' (by finiteness) (by finiteness)).mp
  norm_num

theorem coefficient_formula_without_genericity_false :
    ¬∀ g : Label → G, Function.Injective g → (∀ i, g i ≠ 0) →
      sampleLaw.toOuterMeasure (twoSamples g : Set Sample) =
        ENNReal.ofReal (labelCoefficient uniformLabel) / 7 := by
  intro he
  have hc := he badIncrements bad_still_distinct_nonzero.1 bad_still_distinct_nonzero.2
  rw [bad_two_probability, uniform_label_coefficient] at hc
  have hd := congrArg ENNReal.toReal hc
  norm_num at hd

theorem degree_two_collision_certificate :
    (∀ (I : Type*) [Fintype I] [DecidableEq I] (p : I → ℝ), (∑ i, p i) = 1 →
      labelCoefficient p = 1 - secondMoment p - (secondMoment p)^2 + fourthMoment p) ∧
    (Nat.Prime 7 ∧ Fintype.card Hash = 128 ∧ Fintype.card Sample = 6272 ∧
      DegreeTwoGeneric goodIncrements ∧
      (oneSamples goodIncrements).card = 448 ∧
      (twoSamples goodIncrements).card = 336 ∧
      (∀ z : Sample, sampleLaw z =
        (∏ _ : G, (2 : ENNReal)⁻¹) * (7 : ENNReal)⁻¹ * (7 : ENNReal)⁻¹) ∧
      sampleLaw.toOuterMeasure (oneSamples goodIncrements : Set Sample) = (1 : ENNReal) / 14 ∧
      sampleLaw.toOuterMeasure (twoSamples goodIncrements : Set Sample) = (3 : ENNReal) / 56 ∧
      labelCoefficient uniformLabel = 3 / 8 ∧
      sampleLaw.toOuterMeasure (twoSamples goodIncrements : Set Sample) =
        ENNReal.ofReal (labelCoefficient uniformLabel) / 7 ∧
      sampleLaw.toOuterMeasure (twoSamples goodIncrements : Set Sample) ≠
        sampleLaw.toOuterMeasure (oneSamples goodIncrements : Set Sample)) ∧
    (Function.Injective badIncrements ∧ (∀ i, badIncrements i ≠ 0) ∧
      ¬DegreeTwoGeneric badIncrements ∧ (twoSamples badIncrements).card = 112 ∧
      sampleLaw.toOuterMeasure (twoSamples badIncrements : Set Sample) = (1 : ENNReal) / 56 ∧
      (¬∀ g : Label → G, Function.Injective g → (∀ i, g i ≠ 0) →
        sampleLaw.toOuterMeasure (twoSamples g : Set Sample) =
          ENNReal.ofReal (labelCoefficient uniformLabel) / 7)) := by
  refine ⟨?_, ?_, bad_still_distinct_nonzero.1, bad_still_distinct_nonzero.2,
    bad_not_generic, bad_two_count, bad_two_probability, coefficient_formula_without_genericity_false⟩
  · intro I _ _ p hp
    exact degree_two_label_coefficient p hp
  · refine ⟨by decide, ?_, sample_card, good_generic, good_one_count, good_two_count,
      sampleLaw_product_mass, good_one_probability, good_two_probability,
      uniform_label_coefficient, good_actual_matches_degree_two_coefficient,
      actual_two_rate_not_degree_one⟩
    norm_num [Hash, G, Label, Fintype.card_fun]

#print axioms degree_two_collision_certificate
end TwoStepCollisionAudit
