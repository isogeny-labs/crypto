import ConductorOrthogonality
import HasseTransferCeiling
import ScalarFrobeniusOracle
import FloorObstructionAndDensity
import ConductorGapEndpoints
import Lean.Util.CollectAxioms
import Lean.Elab.Command

/-!
Axiom audit for the conductor prime-gap bounds project.
Run `lake build && lake env lean AxiomAudit.lean` in this directory.
The `run_cmd` block fails if any listed declaration depends on an axiom
outside `propext`, `Classical.choice` and `Quot.sound`.
-/

open Lean Elab Command

run_cmd do
  let allowed : Array Name := #[`propext, `Classical.choice, `Quot.sound]
  for name in #[
      `ConductorOrthogonality.quadraticOrderSubmodule_le_iff,
      `ConductorOrthogonality.smul_quadraticOrderSubmodule_le_iff,
      `ConductorOrthogonality.step_padicValNat_bounds,
      `ConductorOrthogonality.coprime_step_preserves_padicValNat,
      `ConductorOrthogonality.smooth_walk_preserves_padicValNat,
      `ConductorOrthogonality.small_degree_walk_preserves_padicValNat,
      `ConductorOrthogonality.conductor_gap_forces_prime_power_dvd_prod,
      `ConductorOrthogonality.ascending_gap_forces_prime_power_dvd_prod,
      `ConductorOrthogonality.conductor_gap_forces_large_step,
      `HasseTransferCeiling.fundamental_discriminant_le_neg_three,
      `HasseTransferCeiling.three_mul_conductor_sq_le_four_q,
      `HasseTransferCeiling.conductor_sq_lt_four_q,
      `HasseTransferCeiling.conductor_divisor_bounds,
      `HasseTransferCeiling.conductor_le_sqrt_q_of_le_neg_four,
      `HasseTransferCeiling.chain_solver_transfer,
      `HasseTransferCeiling.vertical_prime_gap_dlp_ceiling,
      `HasseTransferCeiling.linear_velu_gap_is_rho_scale,
      `ScalarFrobeniusOracle.projLine_injective,
      `ScalarFrobeniusOracle.card_projLines,
      `ScalarFrobeniusOracle.span_singleton_eq_projLine,
      `ScalarFrobeniusOracle.projLine_map_scalarFrob,
      `ScalarFrobeniusOracle.centralizer_transitive_on_projLines,
      `ScalarFrobeniusOracle.LineSearchTree.correctSet_card_le,
      `ScalarFrobeniusOracle.scalar_frob_oracle_worst_case_lower_bound,
      `ScalarFrobeniusOracle.scalar_frob_oracle_half_success_lower_bound,
      `ScalarFrobeniusOracle.scalar_frob_rational_iff,
      `ScalarFrobeniusOracle.rational_torsion_curve_order_sq_dvd,
      `ScalarFrobeniusOracle.scalar_frob_extension_degree_bounds,
      `ScalarFrobeniusOracle.exists_scalar_frob_attaining_max_degree,
      `FloorObstructionAndDensity.floor_endomorphism_sq_dvd_norm,
      `FloorObstructionAndDensity.no_coprime_cofactor_floor_endomorphism,
      `FloorObstructionAndDensity.no_kani_two_power_floor_endomorphism,
      `FloorObstructionAndDensity.sq_congruence_dichotomy,
      `FloorObstructionAndDensity.card_sq_congruence_window_le_two,
      `FloorObstructionAndDensity.card_sq_congruence_interval_bound,
      `FloorObstructionAndDensity.hasse_trace_conductor_prime_thinness,
      `ConductorGapEndpoints.prime_ecc2k130_big,
      `ConductorGapEndpoints.prime_263,
      `ConductorGapEndpoints.prime_631409,
      `ConductorGapEndpoints.checkVolcanoGapCert_sound,
      `ConductorGapEndpoints.ecc2k130_big_gap_bounds,
      `ConductorGapEndpoints.ecc2k130_big_not_basefield_rational,
      `ConductorGapEndpoints.ecc2k130_small_gap_bounds,
      `ConductorGapEndpoints.deep_volcano_gap_bounds,
      `ConductorGapEndpoints.p256_crater_no_vertical_gaps] do
    let axioms ← Lean.collectAxioms name
    let forbidden := axioms.filter fun ax => !allowed.contains ax
    unless forbidden.isEmpty do
      throwError "Forbidden proof dependency in {name}: {forbidden}"
    logInfo m!"AXIOM_AUDIT|{name}|{axioms}"
