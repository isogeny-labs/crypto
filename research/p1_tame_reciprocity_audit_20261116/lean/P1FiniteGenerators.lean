import P1Evaluation

namespace P1TameReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem p1_embedded_constant (W : WeierstrassCurve.Affine K) (c : Kˣ) :
    baseFieldUnit W (p1ConstantUnit c)=coefficientUnit W c := by
  apply Units.ext
  exact base_coefficient_map W c

theorem p1_finite_constant_tame (a : K) (c : Kˣ) (g : (RatFunc K)ˣ) :
    p1FiniteTame a (p1ConstantUnit c) g=c^finiteBaseOrder a (g : RatFunc K) := by
  rw [p1_finite_tame_embedding,p1_embedded_constant,tame_constant,
    finite_place_split_field_unit_order _ (auxiliary_nonsingular a) (auxiliary_split a)]

theorem p1_finite_constants (a : K) (c d : Kˣ) :
    p1FiniteTame a (p1ConstantUnit c) (p1ConstantUnit d)=1 := by
  rw [p1_finite_constant_tame,p1_finite_constant_order,zpow_zero]

theorem p1_finite_constant_linear_self (a : K) (c : Kˣ) :
    p1FiniteTame a (p1ConstantUnit c) (baseVerticalUnit a)=c := by
  rw [p1_finite_constant_tame,p1_finite_linear_order_self,zpow_one]

theorem p1_finite_constant_linear_other (a b : K) (h : a≠b) (c : Kˣ) :
    p1FiniteTame a (p1ConstantUnit c) (baseVerticalUnit b)=1 := by
  rw [p1_finite_constant_tame,p1_finite_linear_order_other a b h,zpow_zero]

theorem p1_finite_linear_diagonal_self (a : K) :
    p1FiniteTame a (baseVerticalUnit a) (baseVerticalUnit a)= -1 := by
  rw [p1_finite_diagonal,p1_finite_linear_order_self,zpow_one]

theorem p1_finite_linear_pair_left (a b : K) (h : a≠b) :
    (p1FiniteTame a (baseVerticalUnit a) (baseVerticalUnit b) : K)=(a-b)⁻¹ := by
  have hb : finiteBaseBalancedLocalUnit a (baseVerticalUnit a) (baseVerticalUnit b)=
      (p1FiniteLinearLocal a b h)⁻¹ := by
    apply Subtype.ext
    change (baseVerticalUnit a)^finiteBaseOrder a (baseVerticalUnit b : RatFunc K)/
      (baseVerticalUnit b)^finiteBaseOrder a (baseVerticalUnit a : RatFunc K)=(baseVerticalUnit b)⁻¹
    rw [p1_finite_linear_order_self,p1_finite_linear_order_other a b h,zpow_zero,zpow_one,one_div]
  have he : p1FiniteTame a (baseVerticalUnit a) (baseVerticalUnit b)=
      (-1 : Kˣ)^(finiteBaseOrder a (baseVerticalUnit a : RatFunc K)*
        finiteBaseOrder a (baseVerticalUnit b : RatFunc K))*
        p1FiniteUnitResidue a (finiteBaseBalancedLocalUnit a (baseVerticalUnit a) (baseVerticalUnit b)) := rfl
  rw [he,p1_finite_linear_order_self,p1_finite_linear_order_other a b h,mul_zero,zpow_zero,one_mul,hb,map_inv,
    Units.val_inv_eq_inv_val,p1_finite_linear_value]

theorem p1_finite_linear_pair_right (a b : K) (h : a≠b) :
    (p1FiniteTame b (baseVerticalUnit a) (baseVerticalUnit b) : K)=b-a := by
  rw [p1_finite_skew,Units.val_inv_eq_inv_val,p1_finite_linear_pair_left b a h.symm,inv_inv]

theorem p1_finite_linear_pair_other (a b z : K) (ha : z≠a) (hb : z≠b) :
    p1FiniteTame z (baseVerticalUnit a) (baseVerticalUnit b)=1 :=
  p1_finite_units z _ _ (p1_finite_linear_order_other z a ha) (p1_finite_linear_order_other z b hb)

#print axioms p1_finite_constant_tame
#print axioms p1_finite_constants
#print axioms p1_finite_constant_linear_self
#print axioms p1_finite_constant_linear_other
#print axioms p1_finite_linear_diagonal_self
#print axioms p1_finite_linear_pair_left
#print axioms p1_finite_linear_pair_right
#print axioms p1_finite_linear_pair_other
end
end P1TameReciprocityAudit
