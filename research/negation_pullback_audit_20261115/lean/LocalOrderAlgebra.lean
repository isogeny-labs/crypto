module
public import FinitePointCertificate

@[expose] public section
namespace MillerRecursionAudit
noncomputable section
open WeierstrassCurve.Affine MillerFinitePointAudit
open scoped WithZero
variable {K : Type*} [Field K]

theorem valuation_order_mul {L : Type*} [Field L] (v : Valuation L ℤᵐ⁰)
    {f g : L} (hf : f ≠ 0) (hg : g ≠ 0) :
    -WithZero.log (v (f * g)) = -WithZero.log (v f) + -WithZero.log (v g) := by
  rw [map_mul, WithZero.log_mul (v.ne_zero_iff.mpr hf) (v.ne_zero_iff.mpr hg)]
  ring

theorem valuation_order_div {L : Type*} [Field L] (v : Valuation L ℤᵐ⁰)
    {f g : L} (hf : f ≠ 0) (hg : g ≠ 0) :
    -WithZero.log (v (f / g)) = -WithZero.log (v f) - -WithZero.log (v g) := by
  rw [map_div₀, WithZero.log_div (v.ne_zero_iff.mpr hf) (v.ne_zero_iff.mpr hg)]
  ring

theorem point_order_mul (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) (R : W.Point) :
    pointOrder W (f * g) R = pointOrder W f R + pointOrder W g R := by
  cases R with
  | zero => exact valuation_order_mul (MillerInfinityChartAudit.chartAdicValuation W) hf hg
  | some _ _ h => exact valuation_order_mul (finiteAdic W h) hf hg

theorem point_order_div (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    (hf : f ≠ 0) (hg : g ≠ 0) (R : W.Point) :
    pointOrder W (f / g) R = pointOrder W f R - pointOrder W g R := by
  cases R with
  | zero => exact valuation_order_div (MillerInfinityChartAudit.chartAdicValuation W) hf hg
  | some _ _ h => exact valuation_order_div (finiteAdic W h) hf hg

theorem point_order_one (W : WeierstrassCurve.Affine K) (R : W.Point) :
    pointOrder W 1 R = 0 := by
  cases R <;> simp [pointOrder]

/-- A nonzero actual rational function with specified actual normalized local orders.
The point orders are the derived local-ring valuations, not abstract supplied multiplicities. -/
def HasPointDivisor (W : WeierstrassCurve.Affine K) (f : W.FunctionField) (D : W.Point →₀ ℤ) : Prop :=
  f ≠ 0 ∧ ∀ R : W.Point, pointOrder W f R = D R

theorem HasPointDivisor.support_finite (W : WeierstrassCurve.Affine K) {f : W.FunctionField}
    {D : W.Point →₀ ℤ} (h : HasPointDivisor W f D) : (Function.support (pointOrder W f)).Finite := by
  have he : pointOrder W f = (D : W.Point → ℤ) := funext h.2
  rw [he]
  exact D.hasFiniteSupport

theorem HasPointDivisor.principal (W : WeierstrassCurve.Affine K) {f : W.FunctionField}
    {D : W.Point →₀ ℤ} (h : HasPointDivisor W f D) :
    principalPointDivisor W f (h.support_finite W) = D := by
  ext R
  exact h.2 R

theorem has_point_divisor_one (W : WeierstrassCurve.Affine K) : HasPointDivisor W 1 0 := by
  refine ⟨one_ne_zero, ?_⟩
  intro R
  exact point_order_one W R

theorem HasPointDivisor.mul (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    {D E : W.Point →₀ ℤ} (hf : HasPointDivisor W f D) (hg : HasPointDivisor W g E) :
    HasPointDivisor W (f * g) (D + E) := by
  refine ⟨mul_ne_zero hf.1 hg.1, ?_⟩
  intro R
  rw [point_order_mul W hf.1 hg.1, hf.2, hg.2, Finsupp.add_apply]

theorem HasPointDivisor.div (W : WeierstrassCurve.Affine K) {f g : W.FunctionField}
    {D E : W.Point →₀ ℤ} (hf : HasPointDivisor W f D) (hg : HasPointDivisor W g E) :
    HasPointDivisor W (f / g) (D - E) := by
  refine ⟨div_ne_zero hf.1 hg.1, ?_⟩
  intro R
  rw [point_order_div W hf.1 hg.1, hf.2, hg.2, Finsupp.sub_apply]

theorem vertical_nonzero (W : WeierstrassCurve.Affine K) (x : K) :
    MillerNormValuationAudit.verticalFunction W x ≠ 0 := by
  apply (MillerNormValuationAudit.functionValuation W).ne_zero_iff.mp
  rw [MillerNormValuationAudit.function_valuation_vertical]
  exact WithZero.exp_ne_zero

variable [DecidableEq K]

theorem has_point_divisor_chord (W : WeierstrassCurve.Affine K) {x₁ y₁ x₂ y₂ : K}
    (h₁ : W.Nonsingular x₁ y₁) (h₂ : W.Nonsingular x₂ y₂)
    (hxy : ¬ (x₁ = x₂ ∧ y₁ = W.negY x₂ y₂)) :
    HasPointDivisor W
      (MillerNormValuationAudit.lineFunction W x₁ y₁ (W.slope x₁ x₂ y₁ y₂))
      (chordDivisor W (Point.some x₁ y₁ h₁) (Point.some x₂ y₂ h₂)) :=
  ⟨MillerPrincipalIdealAudit.line_function_nonzero W _ _ _, chord_point_order W h₁ h₂ hxy⟩

theorem has_point_divisor_vertical (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    HasPointDivisor W (MillerNormValuationAudit.verticalFunction W x)
      (verticalDivisor W (Point.some x y h)) :=
  ⟨vertical_nonzero W x, vertical_point_order W h⟩

#print axioms HasPointDivisor.mul
#print axioms has_point_divisor_chord
end
end MillerRecursionAudit
