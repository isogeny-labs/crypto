module
public import NegationValuation

@[expose] public section
namespace NegationPullbackAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit MillerInfinityChartAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

theorem point_order_pullback (W : WeierstrassCurve.Affine K) (f : W.FunctionField)
    (P : W.Point) : pointOrder W (fieldNeg W f) P = pointOrder W f (-P) := by
  cases P with
  | zero =>
    change -WithZero.log (chartAdicValuation W (fieldNeg W f)) =
      -WithZero.log (chartAdicValuation W f)
    rw [field_neg_infinity]
  | some x y h =>
    change -WithZero.log (finiteAdic W h (fieldNeg W f)) =
      -WithZero.log (finiteAdic W ((W.nonsingular_neg x y).mpr h) f)
    rw [finite_pullback_valuation]

def pointNegEquiv (W : WeierstrassCurve.Affine K) : W.Point ≃ W.Point where
  toFun := Neg.neg
  invFun := Neg.neg
  left_inv := neg_neg
  right_inv := neg_neg

/-- Actual point-negation permutation of finite-support integer divisors. -/
def divisorNeg (W : WeierstrassCurve.Affine K) : (W.Point →₀ ℤ) ≃+ (W.Point →₀ ℤ) :=
  Finsupp.domCongr (pointNegEquiv W)

theorem divisor_neg_apply (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ) (P : W.Point) :
    divisorNeg W D P = D (-P) := Finsupp.equivMapDomain_apply (pointNegEquiv W) D P

theorem field_neg_nonzero (W : WeierstrassCurve.Affine K) {f : W.FunctionField} (hf : f ≠ 0) :
    fieldNeg W f ≠ 0 := fun he => hf ((fieldNeg W).injective (he.trans (map_zero _).symm))

/-- Pullback of the actual principal divisor, with its finite support derived earlier. -/
theorem function_divisor_pullback (W : WeierstrassCurve.Affine K) (f : W.FunctionField)
    (hf : f ≠ 0) : functionDivisor W (fieldNeg W f) (field_neg_nonzero W hf) =
      divisorNeg W (functionDivisor W f hf) := by
  ext P
  rw [function_divisor_apply, divisor_neg_apply, function_divisor_apply, point_order_pullback]

#print axioms point_order_pullback
#print axioms function_divisor_pullback
end
end NegationPullbackAudit
