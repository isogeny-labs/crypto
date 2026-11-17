import JVSchemeCharts

namespace JVSchemeStalkAudit
noncomputable section
open JVHilbertAudit JVLocalMultiplicityAudit AlgebraicGeometry

/-- Actual Spec stalk comparison and chart controls; no general cycle theorem. -/
theorem jv_scheme_stalk_certificate :
    Nonempty (OriginLocalization ≃ₐ[A] OriginStalk) ∧
    (∀ a : A, canonicalStalkEquiv a = actualGermMap a) ∧
    Function.Bijective (algebraMap A OriginStalk) ∧
    Module.length OriginStalk OriginStalk = 8 ∧
    IsLocalRing OriginStalk ∧
    Nonempty (ActualStalkResidue ≃+* F) ∧
    ¬ AlgebraicGeometry.IsReduced coneScheme ∧
    (∀ p : coneScheme, p = schemeOrigin) ∧
    IsOpen ({schemeOrigin} : Set coneScheme) ∧
    Nat.card coneScheme = 1 ∧
    IsEmpty firstChartScheme ∧ IsEmpty firstOpenScheme ∧
    Nonempty secondChartScheme ∧ Nonempty secondOpenScheme ∧
    (firstBasicOpen : Set coneScheme) = ∅ ∧
    (secondBasicOpen : Set secondConeScheme).Nonempty ∧
    Module.length OriginStalk OriginStalk ≠ Nat.card coneScheme := by
  exact ⟨⟨localizationStalkEquiv⟩,
    fun a => (canonical_coefficient a).trans (coefficient_is_actual_germ a),
    canonical_map_bijective, actual_stalk_length, actual_stalk_local, ⟨stalkResidueEquiv⟩,
    actual_scheme_not_reduced, actual_scheme_unique, actual_scheme_origin_isolated,
    cone_scheme_card, first_chart_scheme_empty, first_open_scheme_empty,
    second_chart_scheme_nonempty, second_open_scheme_nonempty, first_basic_open_empty,
    second_basic_open_nonempty, stalk_length_not_point_count⟩

#print axioms jv_scheme_stalk_certificate
end
end JVSchemeStalkAudit
