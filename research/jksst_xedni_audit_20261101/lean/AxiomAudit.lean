import JKSSTAll
import Lean.Util.CollectAxioms
import Lean.Elab.Command

/-!
Fail-closed representative axiom audit for the JKSST suite. The repository
proof gate separately inspects every theorem in every built module; this file
keeps a small, directly reproducible trust check beside the study.
-/

open Lean Elab Command

run_cmd do
  let allowed : Array Name := #[`propext, `Classical.choice, `Quot.sound]
  for name in #[
      `JKSSTAsymptotic.bigO_remainder_does_not_imply_main_upper_bound,
      `JKSSTAudit.useful_relation_extract,
      `JKSSTAudit.bounded_relation_of_exact_norm_ball,
      `JKSSTAudit.independent_cubic_conditions_le_nine,
      `JKSSTActualHeightCounterexample.no_uniform_bound_by_actual_model_discriminant,
      `WeierstrassCurve.Affine.Point.neronTateHeight_nsmul,
      `WeierstrassCurve.Affine.Point.neronTateHeight_eq_zero_iff_mem_torsion,
      `JKSSTCoefficientBox.card_nonzeroCoefficientBox,
      `JKSSTCoefficientProbability.bounded_coefficient_success_probability,
      `JKSSTCoefficientProbability.bounded_coefficient_success_over_base_with_size_factor,
      `JKSSTCompositeModulus.relation_probability_is_one_half,
      `JKSSTConditionalHeightCore.bounded_relation_of_certified_height_ratio,
      `JKSSTConcreteCubic.fullRowRank_points_on_nonzero_cubic_le_nine,
      `JKSSTCountNonDeterminacy.same_minimum_different_radius_one_count,
      `JKSSTCountNonDeterminacy.same_euclidean_minimum_different_radius_one_count,
      `JKSSTCyclicECDLP.extract_log_of_group_relation,
      `JKSSTEllipticCyclic.extract_log_on_curve,
      `JKSSTExpectedTrials.inverse_order_hazard_gives_linear_wait,
      `JKSSTHeightDiscriminant.no_universal_fixed_discriminant_bound,
      `JKSSTHeightDiscriminant.uniform_height_ratio_bound,
      `JKSSTHasseCofactor.bounded_coefficient_success_from_hasse,
      `JKSSTLang.no_uniform_lower_bound_from_quadraticity,
      `JKSSTMathlibECFacts.RationalHeight.finite_rational_torsion,
      `JKSSTMathlibECFacts.OverRat.finite_torsion,
      `JKSSTModelDiscriminant.scaled_logAbs_discriminant_unbounded,
      `JKSSTOmegaRuntime.inverse_order_recurrence_isOmega,
      `JKSSTOmegaRuntime.inverse_order_hazard_isOmega,
      `JKSSTPacking.card_le_fiberBound_mul_nat_pow,
      `JKSSTProbability.card_usefulEvent,
      `JKSSTProbability.fixed_positive_probability_not_bigO_inverse,
      `JKSSTProbability.fixed_positive_real_not_bigO_inverse,
      `JKSSTProbability.fixed_positive_probability_not_bigO_inverse_nat,
      `JKSSTSuccessComposition.exact_success_bound_of_group_relation_certificates,
      `JKSSTSuccessComposition.ordinary_success_bound_of_group_relation_certificates,
      `JKSSTSuccessComposition.no_useful_reduction_at_zero_sample,
      `JKSSTWeierstrassScaling.scaled_shortModel_discriminant] do
    let axioms ← Lean.collectAxioms name
    let forbidden := axioms.filter fun ax => !allowed.contains ax
    unless forbidden.isEmpty do
      throwError "Forbidden proof dependency in {name}: {forbidden}"
    logInfo m!"AXIOM_AUDIT|{name}|{axioms}"

#print axioms JKSSTAsymptotic.bigO_remainder_does_not_imply_main_upper_bound
#print axioms JKSSTAudit.useful_relation_extract
#print axioms JKSSTActualHeightCounterexample.no_uniform_bound_by_actual_model_discriminant
#print axioms WeierstrassCurve.Affine.Point.neronTateHeight_eq_zero_iff_mem_torsion
#print axioms JKSSTCoefficientProbability.bounded_coefficient_success_probability
#print axioms JKSSTConditionalHeightCore.bounded_relation_of_certified_height_ratio
#print axioms JKSSTCyclicECDLP.extract_log_of_group_relation
#print axioms JKSSTExpectedTrials.inverse_order_hazard_gives_linear_wait
#print axioms JKSSTProbability.card_usefulEvent
#print axioms JKSSTProbability.fixed_positive_probability_not_bigO_inverse_nat
#print axioms JKSSTSuccessComposition.exact_success_bound_of_group_relation_certificates
