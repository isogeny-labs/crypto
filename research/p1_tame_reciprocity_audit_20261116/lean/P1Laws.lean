import P1Base

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

theorem p1_finite_tame_embedding (a : K) (f g : (RatFunc K)ˣ) :
    p1FiniteTame a f g=tameSymbol (auxiliaryModel a)
      (Point.some a 0 (auxiliary_nonsingular a))
      (baseFieldUnit (auxiliaryModel a) f) (baseFieldUnit (auxiliaryModel a) g) :=
  finite_place_tame_split_embedding _ (auxiliary_nonsingular a) (auxiliary_split a) f g

theorem p1_finite_mul_left (a : K) (f g h : (RatFunc K)ˣ) :
    p1FiniteTame a (f*g) h=p1FiniteTame a f h*p1FiniteTame a g h := by
  simp only [p1_finite_tame_embedding,map_mul,tame_mul_left]

theorem p1_finite_mul_right (a : K) (f g h : (RatFunc K)ˣ) :
    p1FiniteTame a f (g*h)=p1FiniteTame a f g*p1FiniteTame a f h := by
  simp only [p1_finite_tame_embedding,map_mul,tame_mul_right]

theorem p1_finite_skew (a : K) (f g : (RatFunc K)ˣ) :
    p1FiniteTame a g f=(p1FiniteTame a f g)⁻¹ := by
  rw [p1_finite_tame_embedding,p1_finite_tame_embedding]
  exact tame_skew _ _ _ _

theorem p1_finite_units (a : K) (f g : (RatFunc K)ˣ)
    (hf : finiteBaseOrder a (f : RatFunc K)=0) (hg : finiteBaseOrder a (g : RatFunc K)=0) :
    p1FiniteTame a f g=1 := by
  rw [p1_finite_tame_embedding]
  apply tame_units
  · rw [finite_place_split_field_unit_order _ (auxiliary_nonsingular a) (auxiliary_split a),hf]
  · rw [finite_place_split_field_unit_order _ (auxiliary_nonsingular a) (auxiliary_split a),hg]

theorem p1_finite_diagonal (a : K) (f : (RatFunc K)ˣ) :
    p1FiniteTame a f f=(-1 : Kˣ)^finiteBaseOrder a (f : RatFunc K) := by
  rw [p1_finite_tame_embedding,tame_diagonal,
    finite_place_split_field_unit_order _ (auxiliary_nonsingular a) (auxiliary_split a)]

theorem p1_infinity_mul_left (f g h : (RatFunc K)ˣ) :
    p1InfinityTame (f*g) h=p1InfinityTame f h*p1InfinityTame g h := by
  have hb : baseBalancedLocalUnit (f*g) h=baseBalancedLocalUnit f h*baseBalancedLocalUnit g h := by
    apply Subtype.ext
    simp only [baseBalancedLocalUnit,baseBalancedUnit,base_order_mul,mul_zpow,zpow_add,
      div_eq_mul_inv,mul_inv_rev,Subgroup.coe_mul]
    ac_rfl
  unfold p1InfinityTame baseTameSymbol
  rw [base_order_mul,add_mul,zpow_add,hb,map_mul]
  exact mul_mul_mul_comm _ _ _ _

theorem p1_infinity_skew (f g : (RatFunc K)ˣ) :
    p1InfinityTame g f=(p1InfinityTame f g)⁻¹ := by
  have hb : baseBalancedLocalUnit g f=(baseBalancedLocalUnit f g)⁻¹ := by
    apply Subtype.ext
    change g^baseInfinityOrder (f : RatFunc K)/f^baseInfinityOrder (g : RatFunc K)=
      (f^baseInfinityOrder (g : RatFunc K)/g^baseInfinityOrder (f : RatFunc K))⁻¹
    exact (inv_div _ _).symm
  unfold p1InfinityTame baseTameSymbol
  rw [hb,map_inv,mul_comm (baseInfinityOrder (g : RatFunc K)),mul_inv_rev,sign_inverse]
  exact mul_comm _ _

theorem p1_infinity_mul_right (f g h : (RatFunc K)ˣ) :
    p1InfinityTame f (g*h)=p1InfinityTame f g*p1InfinityTame f h := by
  rw [p1_infinity_skew,p1_infinity_mul_left,mul_inv_rev,←p1_infinity_skew,←p1_infinity_skew,mul_comm]

theorem p1_infinity_units (f g : (RatFunc K)ˣ)
    (hf : baseInfinityOrder (f : RatFunc K)=0) (hg : baseInfinityOrder (g : RatFunc K)=0) :
    p1InfinityTame f g=1 := by
  have hb : baseBalancedLocalUnit f g=1 := by
    apply Subtype.ext
    simp [baseBalancedLocalUnit,baseBalancedUnit,hf,hg]
  simp only [p1InfinityTame,baseTameSymbol,hf,hg,zero_mul,zpow_zero,one_mul,hb,map_one]

theorem p1_infinity_diagonal (f : (RatFunc K)ˣ) :
    p1InfinityTame f f=(-1 : Kˣ)^baseInfinityOrder (f : RatFunc K) := by
  simp only [p1InfinityTame,baseTameSymbol,base_balanced_self,map_one,mul_one]
  simp only [neg_one_zpow_eq_ite,Int.even_mul,or_self]

def p1FiniteLeft (a : K) (g : (RatFunc K)ˣ) : (RatFunc K)ˣ →* Kˣ where
  toFun f:=p1FiniteTame a f g
  map_one':=by
    have h:=p1_finite_mul_left a 1 1 g
    simpa only [one_mul,mul_eq_left] using h.symm
  map_mul':=fun f h=>p1_finite_mul_left a f h g

def p1InfinityLeft (g : (RatFunc K)ˣ) : (RatFunc K)ˣ →* Kˣ where
  toFun f:=p1InfinityTame f g
  map_one':=by
    have h:=p1_infinity_mul_left 1 1 g
    simpa only [one_mul,mul_eq_left] using h.symm
  map_mul':=fun f h=>p1_infinity_mul_left f h g

#print axioms p1_finite_mul_left
#print axioms p1_finite_mul_right
#print axioms p1_finite_skew
#print axioms p1_finite_units
#print axioms p1_finite_diagonal
#print axioms p1_infinity_mul_left
#print axioms p1_infinity_skew
#print axioms p1_infinity_mul_right
#print axioms p1_infinity_units
#print axioms p1_infinity_diagonal
end
end P1TameReciprocityAudit
