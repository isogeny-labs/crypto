import RamifiedRatio

namespace EllipticRamifiedNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem finite_base_order_mul (x : K) (a b : (RatFunc K)ˣ) :
    finiteBaseOrder x ((a*b : (RatFunc K)ˣ) : RatFunc K)=
      finiteBaseOrder x (a : RatFunc K)+finiteBaseOrder x (b : RatFunc K) :=
  valuation_order_mul (finiteBaseValuation x) a.ne_zero b.ne_zero

theorem finite_base_order_div (x : K) (a b : (RatFunc K)ˣ) :
    finiteBaseOrder x ((a/b : (RatFunc K)ˣ) : RatFunc K)=
      finiteBaseOrder x (a : RatFunc K)-finiteBaseOrder x (b : RatFunc K) := by
  rw [Units.val_div_eq_div_val]
  exact valuation_order_div (finiteBaseValuation x) a.ne_zero b.ne_zero

theorem finite_base_order_zpow (x : K) (a : (RatFunc K)ˣ) (n : ℤ) :
    finiteBaseOrder x ((a^n : (RatFunc K)ˣ) : RatFunc K)=n*finiteBaseOrder x (a : RatFunc K) := by
  rw [Units.val_zpow_eq_zpow_val,finiteBaseOrder,finiteBaseOrder,map_zpow₀,WithZero.log_zpow]
  simp only [smul_eq_mul]
  ring

theorem finite_base_order_zero_iff (x : K) (a : (RatFunc K)ˣ) :
    finiteBaseOrder x (a : RatFunc K)=0 ↔ finiteBaseValuation x (a : RatFunc K)=1 := by
  have hv : finiteBaseValuation x (a : RatFunc K)≠0 := (Valuation.ne_zero_iff _).mpr a.ne_zero
  rw [finiteBaseOrder,neg_eq_zero]
  constructor
  · intro hz
    rw [←WithZero.exp_log hv,hz,WithZero.exp_zero]
  · intro he
    rw [he,WithZero.log_one]

theorem mem_finite_base_units_iff (x : K) (a : (RatFunc K)ˣ) :
    a∈(FiniteBaseRing x).unitGroup ↔ finiteBaseOrder x (a : RatFunc K)=0 := by
  rw [Valuation.mem_unitGroup_iff]
  exact (finite_base_order_zero_iff x a).symm

def finiteBaseBalancedUnit (x : K) (a b : (RatFunc K)ˣ) : (RatFunc K)ˣ :=
  a^finiteBaseOrder x (b : RatFunc K)/b^finiteBaseOrder x (a : RatFunc K)

theorem finite_base_balanced_order (x : K) (a b : (RatFunc K)ˣ) :
    finiteBaseOrder x (finiteBaseBalancedUnit x a b : RatFunc K)=0 := by
  unfold finiteBaseBalancedUnit
  rw [finite_base_order_div,finite_base_order_zpow,finite_base_order_zpow]
  ring

def finiteBaseBalancedLocalUnit (x : K) (a b : (RatFunc K)ˣ) : (FiniteBaseRing x).unitGroup :=
  ⟨finiteBaseBalancedUnit x a b,(mem_finite_base_units_iff x _).mpr (finite_base_balanced_order x a b)⟩

def finiteBaseTameSymbol (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h)
    (a b : (RatFunc K)ˣ) : Kˣ :=
  (-1 : Kˣ)^(finiteBaseOrder x (a : RatFunc K)*finiteBaseOrder x (b : RatFunc K))*
    finiteBaseUnitEvaluation W h hP (finiteBaseBalancedLocalUnit x a b)

theorem finite_base_balanced_self (x : K) (a : (RatFunc K)ˣ) : finiteBaseBalancedLocalUnit x a a=1 := by
  apply Subtype.ext
  change a^finiteBaseOrder x (a : RatFunc K)/a^finiteBaseOrder x (a : RatFunc K)=1
  exact div_self' _

theorem finite_base_tame_self (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (hP : (Point.some x y h : W.Point)= -Point.some x y h) (a : (RatFunc K)ˣ) :
    finiteBaseTameSymbol W h hP a a=(-1 : Kˣ)^(finiteBaseOrder x (a : RatFunc K)*finiteBaseOrder x (a : RatFunc K)) := by
  rw [finiteBaseTameSymbol,finite_base_balanced_self,map_one,mul_one]

theorem finite_base_unit_model_independent (W₁ W₂ : WeierstrassCurve.Affine K) {x y₁ y₂ : K}
    (h₁ : W₁.Nonsingular x y₁) (hP₁ : (Point.some x y₁ h₁ : W₁.Point)= -Point.some x y₁ h₁)
    (h₂ : W₂.Nonsingular x y₂) (hP₂ : (Point.some x y₂ h₂ : W₂.Point)= -Point.some x y₂ h₂)
    (u : (FiniteBaseRing x).unitGroup) : finiteBaseUnitEvaluation W₁ h₁ hP₁ u=finiteBaseUnitEvaluation W₂ h₂ hP₂ u := by
  apply Units.ext
  exact finite_base_eval_model_independent W₁ W₂ h₁ hP₁ h₂ hP₂ _

theorem finite_base_tame_model_independent (W₁ W₂ : WeierstrassCurve.Affine K) {x y₁ y₂ : K}
    (h₁ : W₁.Nonsingular x y₁) (hP₁ : (Point.some x y₁ h₁ : W₁.Point)= -Point.some x y₁ h₁)
    (h₂ : W₂.Nonsingular x y₂) (hP₂ : (Point.some x y₂ h₂ : W₂.Point)= -Point.some x y₂ h₂)
    (a b : (RatFunc K)ˣ) : finiteBaseTameSymbol W₁ h₁ hP₁ a b=finiteBaseTameSymbol W₂ h₂ hP₂ a b := by
  rw [finiteBaseTameSymbol,finiteBaseTameSymbol,finite_base_unit_model_independent W₁ W₂ h₁ hP₁ h₂ hP₂]

#print axioms finite_base_balanced_order
#print axioms finite_base_tame_self
#print axioms finite_base_tame_model_independent
end
end EllipticRamifiedNormProjectionAudit
