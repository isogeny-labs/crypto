import P1Laws

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

def p1FinitePolynomial (a : K) : K[X] →+* FiniteBaseRing a where
  toFun p:=⟨algebraMap K[X] (RatFunc K) p,(finiteBasePlace a).valuation_le_one p⟩
  map_zero':=Subtype.ext (map_zero _)
  map_one':=Subtype.ext (map_one _)
  map_add' p q:=Subtype.ext (map_add _ p q)
  map_mul' p q:=Subtype.ext (map_mul _ p q)

theorem p1_finite_polynomial_constant (a c : K) :
    p1FinitePolynomial a (C c)=finiteBaseConstant a c := rfl

/-- The proper normalized residue agrees with actual polynomial evaluation. -/
theorem p1_finite_eval_polynomial (a : K) (p : K[X]) :
    p1FiniteResidue a (p1FinitePolynomial a p)=p.eval a := by
  have hz : p1FinitePolynomial a (p-C (p.eval a))∈IsLocalRing.maximalIdeal (FiniteBaseRing a) := by
    apply (Valuation.mem_maximalIdeal_iff (RatFunc K) (finiteBaseValuation a)).mpr
    change (finiteBasePlace a).valuation (RatFunc K)
      (algebraMap K[X] (RatFunc K) (p-C (p.eval a)))<1
    apply ((finiteBasePlace a).valuation_lt_one_iff_mem (K:=RatFunc K) _).mpr
    rw [finite_base_prime_mem]
    simp
  rw [←p1_finite_residue_kernel] at hz
  change p1FiniteResidue a (p1FinitePolynomial a (p-C (p.eval a)))=0 at hz
  rw [map_sub (p1FinitePolynomial a),p1_finite_polynomial_constant,map_sub,
    p1_finite_residue_constant,sub_eq_zero] at hz
  exact hz

def p1ConstantUnit : Kˣ →* (RatFunc K)ˣ := Units.map (algebraMap K (RatFunc K)).toMonoidHom

theorem p1_finite_constant_order (a : K) (c : Kˣ) :
    finiteBaseOrder a (p1ConstantUnit c : RatFunc K)=0 := by
  change -WithZero.log (finiteBaseValuation a (algebraMap K[X] (RatFunc K) (C (c : K))))=0
  rw [finite_base_polynomial_unit a (C (c : K)) (by simp),WithZero.log_one,neg_zero]

theorem p1_infinity_polynomial_order (p : K[X]) (hp : p≠0) :
    baseInfinityOrder (algebraMap K[X] (RatFunc K) p)= -(p.natDegree : ℤ) := by
  classical
  change -WithZero.log (RatFunc.inftyValuationDef K (algebraMap K[X] (RatFunc K) p))=_
  rw [RatFunc.inftyValuation.polynomial K hp,WithZero.log_exp]

theorem p1_infinity_constant_order (c : Kˣ) :
    baseInfinityOrder (p1ConstantUnit c : RatFunc K)=0 := by
  change baseInfinityOrder (algebraMap K[X] (RatFunc K) (C (c : K)))=0
  rw [p1_infinity_polynomial_order _ (C_ne_zero.mpr c.ne_zero),natDegree_C]
  rfl

theorem p1_finite_linear_order_self (a : K) :
    finiteBaseOrder a (baseVerticalUnit a : RatFunc K)=1 := by
  rw [finiteBaseOrder,finite_base_parameter_value,WithZero.log_exp]
  norm_num

theorem p1_finite_linear_order_other (a b : K) (h : a≠b) :
    finiteBaseOrder a (baseVerticalUnit b : RatFunc K)=0 := by
  change -WithZero.log (finiteBaseValuation a (algebraMap K[X] (RatFunc K) (X-C b)))=0
  rw [finite_base_polynomial_unit a (X-C b) (by simpa using sub_ne_zero.mpr h),WithZero.log_one,neg_zero]

theorem p1_infinity_linear_order (a : K) :
    baseInfinityOrder (baseVerticalUnit a : RatFunc K)= -1 := by
  change baseInfinityOrder (algebraMap K[X] (RatFunc K) (X-C a))= -1
  rw [p1_infinity_polynomial_order _ (X_sub_C_ne_zero a),natDegree_X_sub_C]
  rfl

def p1FiniteConstantLocal (a : K) (c : Kˣ) : (FiniteBaseRing a).unitGroup :=
  ⟨p1ConstantUnit c,(mem_finite_base_units_iff a _).mpr (p1_finite_constant_order a c)⟩

theorem p1_finite_constant_value (a : K) (c : Kˣ) :
    p1FiniteUnitResidue a (p1FiniteConstantLocal a c)=c := by
  apply Units.ext
  change p1FiniteResidue a (finiteBaseConstant a (c : K))=(c : K)
  exact p1_finite_residue_constant a c

def p1FiniteLinearLocal (a b : K) (h : a≠b) : (FiniteBaseRing a).unitGroup :=
  ⟨baseVerticalUnit b,(mem_finite_base_units_iff a _).mpr (p1_finite_linear_order_other a b h)⟩

theorem p1_finite_linear_value (a b : K) (h : a≠b) :
    (p1FiniteUnitResidue a (p1FiniteLinearLocal a b h) : K)=a-b := by
  change p1FiniteResidue a (p1FinitePolynomial a (X-C b))=a-b
  rw [p1_finite_eval_polynomial]
  simp

def p1InfinityConstantLocal (c : Kˣ) : (BaseInfinityRing K).unitGroup :=
  ⟨p1ConstantUnit c,(mem_base_units_iff _).mpr (p1_infinity_constant_order c)⟩

theorem p1_infinity_constant_value (c : Kˣ) :
    p1InfinityUnitResidue (p1InfinityConstantLocal c)=c := by
  apply Units.ext
  change baseEvaluation (auxiliaryModel (0 : K)) (baseConstant (c : K))=(c : K)
  exact base_eval_constant (auxiliaryModel (0 : K)) c

#print axioms p1_finite_eval_polynomial
#print axioms p1_finite_constant_order
#print axioms p1_infinity_polynomial_order
#print axioms p1_finite_linear_order_self
#print axioms p1_finite_linear_order_other
#print axioms p1_infinity_linear_order
#print axioms p1_finite_constant_value
#print axioms p1_finite_linear_value
#print axioms p1_infinity_constant_value
end
end P1TameReciprocityAudit
