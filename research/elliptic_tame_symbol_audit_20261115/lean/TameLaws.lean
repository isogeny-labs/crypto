import TameLocal

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K]

theorem unit_order_mul (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : pointOrder W (f*g : W.FunctionFieldˣ) P=
      pointOrder W f P+pointOrder W g P :=
  point_order_mul W f.ne_zero g.ne_zero P

theorem balanced_swap (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : balancedUnit W P g f=(balancedUnit W P f g)⁻¹ := by
  simp only [balancedUnit,inv_div]

theorem balanced_mul_left (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g h : W.FunctionFieldˣ) :
    balancedUnit W P (f*g) h=balancedUnit W P f h*balancedUnit W P g h := by
  simp only [balancedUnit,unit_order_mul,mul_zpow,zpow_add,div_eq_mul_inv,mul_inv_rev]
  ac_rfl

theorem sign_inverse (n : ℤ) : ((-1 : Kˣ)^n)⁻¹=(-1 : Kˣ)^n := by
  rw [←inv_zpow]
  congr 1

theorem tame_skew (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : tameSymbol W P g f=(tameSymbol W P f g)⁻¹ := by
  unfold tameSymbol
  have hb : balancedLocalUnit W P g f=(balancedLocalUnit W P f g)⁻¹ :=
    Subtype.ext (balanced_swap W P f g)
  rw [hb,map_inv,mul_comm (pointOrder W g P),mul_inv_rev,sign_inverse]
  exact mul_comm _ _

theorem tame_mul_left (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g h : W.FunctionFieldˣ) :
    tameSymbol W P (f*g) h=tameSymbol W P f h*tameSymbol W P g h := by
  unfold tameSymbol
  have hb : balancedLocalUnit W P (f*g) h=
      balancedLocalUnit W P f h*balancedLocalUnit W P g h :=
    Subtype.ext (balanced_mul_left W P f g h)
  rw [unit_order_mul,add_mul,zpow_add,hb,map_mul]
  ac_rfl

theorem tame_mul_right (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g h : W.FunctionFieldˣ) :
    tameSymbol W P f (g*h)=tameSymbol W P f g*tameSymbol W P f h := by
  rw [tame_skew,tame_mul_left,mul_inv_rev,←tame_skew,←tame_skew,mul_comm]

theorem tame_diagonal (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) : tameSymbol W P f f=(-1 : Kˣ)^pointOrder W f P := by
  have hb : balancedLocalUnit W P f f=1 := by
    apply Subtype.ext
    simp [balancedLocalUnit,balancedUnit]
  simp only [tameSymbol,hb,map_one,mul_one]
  simp only [neg_one_zpow_eq_ite,Int.even_mul,or_self]

/-- The actual local symbol is a multiplicative character in its first argument. -/
def tameLeft (W : WeierstrassCurve.Affine K) (P : W.Point)
    (g : W.FunctionFieldˣ) : W.FunctionFieldˣ →* Kˣ where
  toFun f := tameSymbol W P f g
  map_one' := by
    have h := tame_mul_left W P 1 1 g
    simpa only [one_mul,mul_eq_left] using h.symm
  map_mul' f h := tame_mul_left W P f h g

/-- Two nonzero units have symbol1; poles and zeros remain allowed in tameLeft. -/
theorem tame_zpow_left (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (n : ℤ) : tameSymbol W P (f^n) g=(tameSymbol W P f g)^n :=
  (tameLeft W P g).map_zpow f n

#print axioms tame_mul_left
#print axioms tame_mul_right
#print axioms tame_skew
#print axioms tame_diagonal
end
end EllipticTameSymbolAudit
