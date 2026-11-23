import P1FiniteGenerators

namespace P1TameReciprocityAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem p1_infinity_constant_tame (c : Kˣ) (g : (RatFunc K)ˣ) :
    p1InfinityTame (p1ConstantUnit c) g=c^baseInfinityOrder (g : RatFunc K) := by
  have hb : baseBalancedLocalUnit (p1ConstantUnit c) g=
      (p1InfinityConstantLocal c)^baseInfinityOrder (g : RatFunc K) := by
    apply Subtype.ext
    simp [baseBalancedLocalUnit,baseBalancedUnit,p1_infinity_constant_order,p1InfinityConstantLocal]
  rw [p1InfinityTame,baseTameSymbol,p1_infinity_constant_order,zero_mul,zpow_zero,one_mul,hb,map_zpow]
  change (p1InfinityUnitResidue (p1InfinityConstantLocal c))^baseInfinityOrder (g : RatFunc K)=_
  rw [p1_infinity_constant_value]

theorem p1_infinity_constants (c d : Kˣ) :
    p1InfinityTame (p1ConstantUnit c) (p1ConstantUnit d)=1 := by
  rw [p1_infinity_constant_tame,p1_infinity_constant_order,zpow_zero]

theorem p1_infinity_constant_linear (c : Kˣ) (a : K) :
    p1InfinityTame (p1ConstantUnit c) (baseVerticalUnit a)=c⁻¹ := by
  rw [p1_infinity_constant_tame,p1_infinity_linear_order,zpow_neg_one]

def p1InfinityLinearRatio (a b : K) : (BaseInfinityRing K).unitGroup :=
  ⟨baseVerticalUnit a/baseVerticalUnit b,(mem_base_units_iff _).mpr (by
    rw [base_order_div,p1_infinity_linear_order,p1_infinity_linear_order,sub_self])⟩

/-- The actual proper residue of two monic linear factors of equal degree is one. -/
theorem p1_infinity_linear_ratio_value (a b : K) :
    p1InfinityUnitResidue (p1InfinityLinearRatio a b)=1 := by
  let r : BaseInfinityRing K := (BaseInfinityRing K).unitGroupMulEquiv (p1InfinityLinearRatio a b)
  have he : ((r-baseConstant 1 : BaseInfinityRing K) : RatFunc K)=
      algebraMap K[X] (RatFunc K) (C (b-a))/algebraMap K[X] (RatFunc K) (X-C b) := by
    change ((baseVerticalUnit a/baseVerticalUnit b : (RatFunc K)ˣ) : RatFunc K)-
      algebraMap K (RatFunc K) 1=_
    rw [Units.val_div_eq_div_val,map_one]
    change algebraMap K[X] (RatFunc K) (X-C a)/algebraMap K[X] (RatFunc K) (X-C b)-1=_
    rw [div_sub_one (base_vertical_polynomial_nonzero b),←map_sub]
    congr 2
    simp only [map_sub]
    ring
  have hv : baseInfinityValuation K (algebraMap K[X] (RatFunc K) (X-C b))=WithZero.exp (1 : ℤ) := by
    rw [base_infinity_polynomial,degree_X_sub_C]
    rfl
  have hc : baseInfinityValuation K (algebraMap K[X] (RatFunc K) (C (b-a)))≤1 :=
    base_constant_integral (b-a)
  have hsmall : baseInfinityValuation K ((r-baseConstant 1 : BaseInfinityRing K) : RatFunc K)<1 := by
    rw [he,map_div₀,hv]
    calc
      _ ≤ 1/WithZero.exp (1 : ℤ) := by
        gcongr
        exact zero_le
      _ < 1 := by
        rw [one_div,←WithZero.exp_neg,WithZero.exp_lt_one_iff]
        norm_num
  have hz : baseEvaluation (auxiliaryModel (0 : K)) (r-baseConstant 1)=0 := by
    have hm : r-baseConstant 1∈IsLocalRing.maximalIdeal (BaseInfinityRing K) :=
      (Valuation.mem_maximalIdeal_iff (RatFunc K) (baseInfinityValuation K)).mpr hsmall
    rw [←base_evaluation_kernel (auxiliaryModel (0 : K))] at hm
    exact hm
  rw [map_sub,base_eval_constant,sub_eq_zero] at hz
  apply Units.ext
  change baseEvaluation (auxiliaryModel (0 : K)) r=1
  exact hz

theorem p1_infinity_linear_balanced (a b : K) :
    baseBalancedLocalUnit (baseVerticalUnit a) (baseVerticalUnit b)=p1InfinityLinearRatio b a := by
  apply Subtype.ext
  change (baseVerticalUnit a)^baseInfinityOrder (baseVerticalUnit b : RatFunc K)/
    (baseVerticalUnit b)^baseInfinityOrder (baseVerticalUnit a : RatFunc K)=baseVerticalUnit b/baseVerticalUnit a
  rw [p1_infinity_linear_order,p1_infinity_linear_order,zpow_neg_one,zpow_neg_one]
  simp only [div_eq_mul_inv,inv_inv]
  exact mul_comm _ _

theorem p1_infinity_linear_pair (a b : K) :
    p1InfinityTame (baseVerticalUnit a) (baseVerticalUnit b)= -1 := by
  rw [p1InfinityTame,baseTameSymbol,p1_infinity_linear_order,p1_infinity_linear_order,
    show (-1 : ℤ)*(-1)=1 by norm_num,zpow_one,p1_infinity_linear_balanced]
  change (-1 : Kˣ)*p1InfinityUnitResidue (p1InfinityLinearRatio b a)= -1
  rw [p1_infinity_linear_ratio_value,mul_one]

#print axioms p1_infinity_constant_tame
#print axioms p1_infinity_constants
#print axioms p1_infinity_constant_linear
#print axioms p1_infinity_linear_ratio_value
#print axioms p1_infinity_linear_pair
end
end P1TameReciprocityAudit
