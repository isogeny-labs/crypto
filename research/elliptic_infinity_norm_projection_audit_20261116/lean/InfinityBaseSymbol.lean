import InfinityBase

namespace EllipticInfinityNormProjectionAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem base_order_mul (a b : (RatFunc K)ˣ) :
    baseInfinityOrder ((a*b : (RatFunc K)ˣ) : RatFunc K)=baseInfinityOrder (a : RatFunc K)+baseInfinityOrder (b : RatFunc K) :=
  valuation_order_mul (baseInfinityValuation K) a.ne_zero b.ne_zero

theorem base_order_div (a b : (RatFunc K)ˣ) :
    baseInfinityOrder ((a/b : (RatFunc K)ˣ) : RatFunc K)=baseInfinityOrder (a : RatFunc K)-baseInfinityOrder (b : RatFunc K) := by
  rw [Units.val_div_eq_div_val]
  exact valuation_order_div (baseInfinityValuation K) a.ne_zero b.ne_zero

theorem base_order_zpow (a : (RatFunc K)ˣ) (n : ℤ) :
    baseInfinityOrder ((a^n : (RatFunc K)ˣ) : RatFunc K)=n*baseInfinityOrder (a : RatFunc K) := by
  rw [Units.val_zpow_eq_zpow_val,baseInfinityOrder,baseInfinityOrder,map_zpow₀,WithZero.log_zpow]
  simp only [smul_eq_mul]
  ring

theorem base_order_zero_iff (a : (RatFunc K)ˣ) :
    baseInfinityOrder (a : RatFunc K)=0 ↔ baseInfinityValuation K (a : RatFunc K)=1 := by
  have hv : baseInfinityValuation K (a : RatFunc K)≠0 := (Valuation.ne_zero_iff _).mpr a.ne_zero
  rw [baseInfinityOrder,neg_eq_zero]
  constructor
  · intro hz
    rw [←WithZero.exp_log hv,hz,WithZero.exp_zero]
  · intro he
    rw [he,WithZero.log_one]

theorem mem_base_units_iff (a : (RatFunc K)ˣ) :
    a∈(BaseInfinityRing K).unitGroup ↔ baseInfinityOrder (a : RatFunc K)=0 := by
  rw [Valuation.mem_unitGroup_iff]
  exact (base_order_zero_iff a).symm

def baseBalancedUnit (a b : (RatFunc K)ˣ) : (RatFunc K)ˣ :=
  a^baseInfinityOrder (b : RatFunc K)/b^baseInfinityOrder (a : RatFunc K)

theorem base_balanced_order (a b : (RatFunc K)ˣ) :
    baseInfinityOrder (baseBalancedUnit a b : RatFunc K)=0 := by
  unfold baseBalancedUnit
  rw [base_order_div,base_order_zpow,base_order_zpow]
  ring

def baseBalancedLocalUnit (a b : (RatFunc K)ˣ) : (BaseInfinityRing K).unitGroup :=
  ⟨baseBalancedUnit a b,(mem_base_units_iff _).mpr (base_balanced_order a b)⟩

/-- Actual classical base tame symbol, evaluated only on the proved balanced local unit. -/
def baseTameSymbol (W : WeierstrassCurve.Affine K) (a b : (RatFunc K)ˣ) : Kˣ :=
  (-1 : Kˣ)^(baseInfinityOrder (a : RatFunc K)*baseInfinityOrder (b : RatFunc K))*
    baseUnitEvaluation W (baseBalancedLocalUnit a b)

def unsignedBaseTame (W : WeierstrassCurve.Affine K) (a b : (RatFunc K)ˣ) : Kˣ :=
  baseUnitEvaluation W (baseBalancedLocalUnit a b)

theorem base_balanced_self (a : (RatFunc K)ˣ) : baseBalancedLocalUnit a a=1 := by
  apply Subtype.ext
  exact div_self' _

theorem actual_base_parameter_diagonal (W : WeierstrassCurve.Affine K)
    (a : (RatFunc K)ˣ) (ha : baseInfinityOrder (a : RatFunc K)=1) :
    baseTameSymbol W a a= -1 ∧ unsignedBaseTame W a a=1 := by
  constructor
  · rw [baseTameSymbol,ha,mul_one,zpow_one,base_balanced_self,map_one,mul_one]
  · rw [unsignedBaseTame,base_balanced_self,map_one]

#print axioms base_order_zero_iff
#print axioms base_balanced_order
#print axioms actual_base_parameter_diagonal
end
end EllipticInfinityNormProjectionAudit
