module
public import ValuationCertificate
public import Mathlib.RingTheory.FiniteType
public import Mathlib.RingTheory.Localization.AtPrime.Basic
public import Mathlib.RingTheory.DiscreteValuationRing.TFAE

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
open MillerNormValuationAudit
open scoped WithZero
variable {K : Type*} [Field K]

/-- The actual coefficient embedding into the already constructed valuation ring. -/
def coefficientHom (W : WeierstrassCurve.Affine K) : K →+* valuationRing W where
  toFun c := ⟨constantHom W c, function_valuation_constant_le W c⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add _ a b)
  map_mul' a b := Subtype.ext (map_mul _ a b)

instance coefficientAlgebra (W : WeierstrassCurve.Affine K) : Algebra K (valuationRing W) :=
  (coefficientHom W).toAlgebra

def chartT (W : WeierstrassCurve.Affine K) : valuationRing W := integralParameter W

def chartZ (W : WeierstrassCurve.Affine K) : valuationRing W :=
  ⟨(yFunction W)⁻¹, by
    change functionValuation W (yFunction W)⁻¹ ≤ 1
    rw [Valuation.map_inv, function_valuation_y, WithZero.inv_exp]
    norm_num⟩

/-- The actual image algebra of polynomial functions in the Y-chart coordinates X/Y and 1/Y. -/
def chartAlgebra (W : WeierstrassCurve.Affine K) : Subalgebra K (valuationRing W) :=
  Algebra.adjoin K {chartT W, chartZ W}

abbrev ChartRing (W : WeierstrassCurve.Affine K) : Type _ := chartAlgebra W

def tCoord (W : WeierstrassCurve.Affine K) : ChartRing W :=
  ⟨chartT W, Algebra.subset_adjoin (by simp)⟩

def zCoord (W : WeierstrassCurve.Affine K) : ChartRing W :=
  ⟨chartZ W, Algebra.subset_adjoin (by simp)⟩

theorem chart_internal_generation (W : WeierstrassCurve.Affine K) :
    Algebra.adjoin K {tCoord W, zCoord W} = (⊤ : Subalgebra K (ChartRing W)) := by
  apply Subalgebra.map_injective (f := (chartAlgebra W).val) Subtype.val_injective
  rw [AlgHom.map_adjoin, Algebra.map_top, Subalgebra.range_val]
  simp only [Set.image_pair, tCoord, zCoord]
  rfl

instance chartFiniteType (W : WeierstrassCurve.Affine K) : Algebra.FiniteType K (ChartRing W) :=
  Algebra.FiniteType.adjoin_of_finite ((Set.finite_singleton (chartZ W)).insert (chartT W))

instance chartNoetherian (W : WeierstrassCurve.Affine K) : IsNoetherianRing (ChartRing W) :=
  Algebra.FiniteType.isNoetherianRing K (ChartRing W)

def coefficientResidue (W : WeierstrassCurve.Affine K) :
    K →+* IsLocalRing.ResidueField (valuationRing W) :=
  (IsLocalRing.residue (valuationRing W)).comp (coefficientHom W)

theorem residue_chartT (W : WeierstrassCurve.Affine K) :
    IsLocalRing.residue (valuationRing W) (chartT W) = 0 := by
  apply (IsLocalRing.residue_eq_zero_iff _).mpr
  apply (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := functionValuation W)).mpr
  change functionValuation W (parameter W) < 1
  rw [function_valuation_parameter]
  norm_num

theorem residue_chartZ (W : WeierstrassCurve.Affine K) :
    IsLocalRing.residue (valuationRing W) (chartZ W) = 0 := by
  apply (IsLocalRing.residue_eq_zero_iff _).mpr
  apply (Valuation.mem_maximalIdeal_iff (K := W.FunctionField) (v := functionValuation W)).mpr
  change functionValuation W (yFunction W)⁻¹ < 1
  rw [Valuation.map_inv, function_valuation_y, WithZero.inv_exp]
  norm_num

def residueConstantAlgebra (W : WeierstrassCurve.Affine K) : Subalgebra K (valuationRing W) :=
  { ((coefficientResidue W).range.comap (IsLocalRing.residue (valuationRing W))).toSubsemiring with
    algebraMap_mem' c := ⟨c, rfl⟩ }

theorem chart_residue_constant (W : WeierstrassCurve.Affine K) :
    chartAlgebra W ≤ residueConstantAlgebra W := by
  apply Algebra.adjoin_le
  intro a ha
  simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at ha
  rcases ha with rfl | rfl
  · change ∃ c : K, coefficientResidue W c = IsLocalRing.residue (valuationRing W) (chartT W)
    exact ⟨0, by rw [map_zero, residue_chartT]⟩
  · change ∃ c : K, coefficientResidue W c = IsLocalRing.residue (valuationRing W) (chartZ W)
    exact ⟨0, by rw [map_zero, residue_chartZ]⟩

/-- The exact source Y-chart relation, in the actual chart-function image algebra. -/
theorem chart_relation (W : WeierstrassCurve.Affine K) :
    zCoord W * (1 + algebraMap K (ChartRing W) W.a₁ * tCoord W +
      algebraMap K (ChartRing W) W.a₃ * zCoord W -
      algebraMap K (ChartRing W) W.a₂ * tCoord W ^ 2 -
      algebraMap K (ChartRing W) W.a₄ * tCoord W * zCoord W -
      algebraMap K (ChartRing W) W.a₆ * zCoord W ^ 2) = tCoord W ^ 3 := by
  apply Subtype.ext
  apply Subtype.ext
  change (yFunction W)⁻¹ * (1 + constantHom W W.a₁ * parameter W +
    constantHom W W.a₃ * (yFunction W)⁻¹ - constantHom W W.a₂ * parameter W ^ 2 -
    constantHom W W.a₄ * parameter W * (yFunction W)⁻¹ -
    constantHom W W.a₆ * ((yFunction W)⁻¹) ^ 2) = parameter W ^ 3
  rw [parameter]
  field_simp [y_function_nonzero W]
  linear_combination function_curve_relation W

#print axioms chart_relation
#print axioms chart_residue_constant
end
end MillerInfinityChartAudit
