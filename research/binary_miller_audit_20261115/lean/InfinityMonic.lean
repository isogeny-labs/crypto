module
public import AllMiller

@[expose] public section
namespace MillerRecursionAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit MillerNormValuationAudit
open MillerInfinityChartAudit
open scoped WithZero
variable {K : Type*} [Field K]

theorem near_one_value {L : Type*} [Field L] (v : Valuation L ℤᵐ⁰) {a : L}
    (ha : v (a - 1) < 1) : v a = 1 := by
  have h := v.map_one_add_of_lt ha
  have he : 1 + (a - 1) = a := by ring
  rwa [he] at h

theorem near_one_mul {L : Type*} [Field L] (v : Valuation L ℤᵐ⁰) {a b : L}
    (ha : v (a - 1) < 1) (hb : v (b - 1) < 1) : v (a * b - 1) < 1 := by
  have hm : v ((a - 1) * (b - 1)) < 1 := by
    rw [map_mul]
    exact (mul_le_mul_of_nonneg_left hb.le zero_le).trans_lt (by simpa only [mul_one] using ha)
  have he : a * b - 1 = (a - 1) * (b - 1) + (a - 1) + (b - 1) := by ring
  rw [he]
  exact v.map_add_lt (v.map_add_lt hm ha) hb

theorem near_one_div {L : Type*} [Field L] (v : Valuation L ℤᵐ⁰) {a b : L}
    (ha : v (a - 1) < 1) (hb : v (b - 1) < 1) : v (a / b - 1) < 1 := by
  have hv := near_one_value v hb
  have hb0 : b ≠ 0 := v.ne_zero_iff.mp (by rw [hv]; exact one_ne_zero)
  have he : a / b - 1 = ((a - 1) - (b - 1)) / b := by field_simp [hb0]; ring
  rw [he, map_div₀, hv, div_one]
  exact v.map_sub_lt ha hb

/-- Scale an actual function by the inverse of its actual infinity order, using source X/Y. -/
def infinityScaled (W : WeierstrassCurve.Affine K) (f : W.FunctionField) : W.FunctionField :=
  parameter W ^ (-pointOrder W f 0) * f

/-- Source normalization: the leading unit has residue one; the entire series need not be one. -/
def InfinityMonic (W : WeierstrassCurve.Affine K) (f : W.FunctionField) : Prop :=
  functionValuation W (infinityScaled W f - 1) < 1

theorem scaled_mul (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) :
    infinityScaled W (f * g) = infinityScaled W f * infinityScaled W g := by
  unfold infinityScaled
  rw [point_order_mul W hf hg, neg_add,
    zpow_add₀ (parameter_is_uniformizer W).ne_zero]
  ring

theorem scaled_div (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) :
    infinityScaled W (f / g) = infinityScaled W f / infinityScaled W g := by
  unfold infinityScaled
  rw [point_order_div W hf hg]
  have he : -(pointOrder W f 0 - pointOrder W g 0) =
      -pointOrder W f 0 - (-pointOrder W g 0) := by ring
  rw [he, zpow_sub₀ (parameter_is_uniformizer W).ne_zero]
  exact div_mul_div_comm _ _ _ _

theorem infinity_monic_mul (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf0 : f ≠ 0) (hg0 : g ≠ 0) (hf : InfinityMonic W f) (hg : InfinityMonic W g) :
    InfinityMonic W (f * g) := by
  unfold InfinityMonic
  rw [scaled_mul W hf0 hg0]
  exact near_one_mul (functionValuation W) hf hg

theorem infinity_monic_div (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf0 : f ≠ 0) (hg0 : g ≠ 0) (hf : InfinityMonic W f) (hg : InfinityMonic W g) :
    InfinityMonic W (f / g) := by
  unfold InfinityMonic
  rw [scaled_div W hf0 hg0]
  exact near_one_div (functionValuation W) hf hg

theorem infinity_monic_one (W : WeierstrassCurve.Affine K) : InfinityMonic W 1 := by
  simp [InfinityMonic, infinityScaled, point_order_one]

theorem infinity_monic_line (W : WeierstrassCurve.Affine K) (x y slope : K) :
    InfinityMonic W (lineFunction W x y slope) := by
  have ho : pointOrder W (lineFunction W x y slope) 0 = (-3 : ℤ) := by
    change -WithZero.log (chartAdicValuation W _) = _
    rw [chart_adic_line, WithZero.log_exp]
  have hs : infinityScaled W (lineFunction W x y slope) = parameter W ^ 3 * lineFunction W x y slope := by
    unfold infinityScaled
    rw [ho]
    norm_num
  unfold InfinityMonic
  rw [hs]
  exact line_normalized W x y slope

theorem infinity_monic_vertical (W : WeierstrassCurve.Affine K) (x : K) :
    InfinityMonic W (verticalFunction W x) := by
  have ho : pointOrder W (verticalFunction W x) 0 = (-2 : ℤ) := by
    change -WithZero.log (chartAdicValuation W _) = _
    rw [chart_adic_vertical, WithZero.log_exp]
  have hs : infinityScaled W (verticalFunction W x) = parameter W ^ 2 * verticalFunction W x := by
    unfold infinityScaled
    rw [ho]
    norm_num
  unfold InfinityMonic
  rw [hs]
  exact vertical_normalized W x

variable [DecidableEq K]

theorem step_infinity_monic (W : WeierstrassCurve.Affine K) (P Q : W.Point) :
    InfinityMonic W (stepFunction W P Q) := by
  cases P with
  | zero => exact infinity_monic_one W
  | some x₁ y₁ h₁ =>
    cases Q with
    | zero => exact infinity_monic_one W
    | some x₂ y₂ h₂ =>
      by_cases hxy : x₁ = x₂ ∧ y₁ = W.negY x₂ y₂
      · simp only [stepFunction, ite_eq_left hxy]
        exact infinity_monic_vertical W x₁
      · simp only [stepFunction, ite_eq_right hxy]
        exact infinity_monic_div W (MillerPrincipalIdealAudit.line_function_nonzero W _ _ _)
          (vertical_nonzero W _) (infinity_monic_line W _ _ _) (infinity_monic_vertical W _)

/-- The actual recursively constructed all-n Miller functions satisfy source infinity normalization. -/
theorem miller_infinity_monic (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    InfinityMonic W (millerFunction W P n) := by
  induction n with
  | zero => exact infinity_monic_one W
  | succ n ih =>
    exact infinity_monic_mul W (miller_function_nonzero W P n) (step_function_nonzero W _ _)
      ih (step_infinity_monic W _ _)

/-- Source Miller-function property: actual point divisor and actual leading residue one. -/
def IsMillerFunction (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ)
    (f : W.FunctionField) : Prop :=
  HasPointDivisor W f (millerDivisor W P n) ∧ InfinityMonic W f

theorem constructed_is_miller (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    IsMillerFunction W P n (millerFunction W P n) :=
  ⟨miller_has_point_divisor W P n, miller_infinity_monic W P n⟩

/-- Source addition rule preserves both actual divisor and actual normalization, in all branches. -/
theorem normalized_miller_addition (W : WeierstrassCurve.Affine K) (P : W.Point) (i j : ℕ)
    {f g : W.FunctionField} (hf : IsMillerFunction W P i f) (hg : IsMillerFunction W P j g) :
    IsMillerFunction W P (i + j) (f * g * stepFunction W (i • P) (j • P)) := by
  refine ⟨miller_addition_invariant W P i j hf.1 hg.1, ?_⟩
  exact infinity_monic_mul W (mul_ne_zero hf.1.1 hg.1.1) (step_function_nonzero W _ _)
    (infinity_monic_mul W hf.1.1 hg.1.1 hf.2 hg.2) (step_infinity_monic W _ _)

theorem miller_function_one (W : WeierstrassCurve.Affine K) (P : W.Point) :
    millerFunction W P 1 = 1 := by
  change 1 * stepFunction W (0 • P) P = 1
  rw [zero_nsmul, Point.zero_def]
  simp [stepFunction]

#print axioms miller_infinity_monic
#print axioms normalized_miller_addition
end
end MillerRecursionAudit
