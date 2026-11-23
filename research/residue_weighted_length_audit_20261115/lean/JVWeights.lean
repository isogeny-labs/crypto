import ProductWeights

namespace ResidueWeightedLengthAudit.JVCalibration
noncomputable section
open JVHilbertAudit JVLocalMultiplicityAudit

instance jvFinite : Module.Finite F A := Module.Finite.of_basis actualBasis

theorem actual_jv_weighted_degree : weightedDegree F A = 8 := by
  rw [← scalar_dimension_eq_weighted_degree F A]
  exact actual_coordinate_algebra_dimension

/-- Derived from the generic weighted identity and credited actual lengths,
without assuming a residue-degree identification. -/
theorem actual_jv_residue_degree : Module.finrank F originIdeal.ResidueField = 1 := by
  let : Module.Finite F OriginLocalization := localization_finite F A originPoint
  have hd : Module.finrank F OriginLocalization = 8 := by
    calc
      Module.finrank F OriginLocalization = Module.finrank F A :=
        (originLocalEquiv.restrictScalars F).toLinearEquiv.finrank_eq.symm
      _ = 8 := actual_coordinate_algebra_dimension
  have he := local_self_dimension F OriginLocalization
  rw [hd, actual_localization_length] at he
  norm_num at he
  omega

instance zeroChartFinite : Module.Finite F FirstChart := by
  let : Subsingleton FirstChart := first_chart_subsingleton
  exact Module.Finite.of_surjective (0 : F →ₗ[F] FirstChart)
    (fun x => ⟨0, Subsingleton.elim _ _⟩)

theorem actual_zero_chart_weighted_degree : weightedDegree F FirstChart = 0 := by
  let : Subsingleton FirstChart := first_chart_subsingleton
  exact zero_algebra_weighted_degree F FirstChart

#print axioms actual_jv_weighted_degree
#print axioms actual_jv_residue_degree
#print axioms actual_zero_chart_weighted_degree
end
end ResidueWeightedLengthAudit.JVCalibration
