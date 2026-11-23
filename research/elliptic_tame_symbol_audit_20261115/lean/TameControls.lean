import TameProduct
import GeneralControls

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open MillerPrincipalIdealAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open scoped WithZero
variable {K : Type*} [Field K]

/-- Omitting only the sign; actual balanced-unit evaluation remains well defined. -/
def unsignedSymbol (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) : Kˣ := pointUnitEvaluation W P (balancedLocalUnit W P f g)

theorem unsigned_at_simple_poles (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hs : (f : W.FunctionField)+(g : W.FunctionField)=1)
    (hf : pointOrder W f P= -1) (hg : pointOrder W g P= -1) :
    unsignedSymbol W P f g= -1 := by
  have he := tame_steinberg W P f g hs
  have h : (-1 : Kˣ)*unsignedSymbol W P f g=1 := by
    simpa only [tameSymbol,hf,hg,neg_mul_neg,one_mul,zpow_one,unsignedSymbol] using he
  apply Units.ext
  have hc := congrArg (fun z : Kˣ => (z : K)) h
  simp only [Units.val_mul,Units.coe_neg_one,Units.val_one] at hc ⊢
  linear_combination -hc

theorem binary_sign_collapse [CharP K 2] (W : WeierstrassCurve.Affine K)
    (P : W.Point) (f g : W.FunctionFieldˣ) : tameSymbol W P f g=unsignedSymbol W P f g := by
  have hn : (-1 : Kˣ)=1 := by
    apply Units.ext
    simp only [Units.coe_neg_one,Units.val_one]
    exact CharTwo.neg_one_eq_one
  simp only [tameSymbol,hn,one_zpow,one_mul,unsignedSymbol]

/-- Any actual simple pole yields a nonzero 1−f and a second actual simple pole. -/
theorem simple_pole_partner (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P= -1) :
    ∃ g : W.FunctionFieldˣ,(f : W.FunctionField)+(g : W.FunctionField)=1 ∧ pointOrder W g P= -1 := by
  have hv : 1<localFunctionValuation W P f := by
    have he : localFunctionValuation W P f=WithZero.exp (1 : ℤ) := by
      have hz := WithZero.exp_log ((localFunctionValuation W P).ne_zero_iff.mpr f.ne_zero)
      rw [point_order_formula] at hf
      have hl : WithZero.log (localFunctionValuation W P f)=1 := by omega
      rw [hl] at hz
      exact hz.symm
    rw [he];norm_num
  have hgval : localFunctionValuation W P (1-(f : W.FunctionField))=localFunctionValuation W P f :=
    (localFunctionValuation W P).map_sub_eq_of_lt_right (by simpa using hv)
  have hg : 1-(f : W.FunctionField)≠0 := by
    intro h
    rw [h,map_zero] at hgval
    exact (ne_of_gt (zero_lt_one.trans hv)) hgval.symm
  refine ⟨Units.mk0 (1-(f : W.FunctionField)) hg,by simp,?_⟩
  rw [point_order_formula,Units.val_mk0,hgval,←point_order_formula,hf]

private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev ControlBase := ZMod 5
noncomputable abbrev controlBaseCommRing : CommRing ControlBase := (inferInstance : Field ControlBase).toCommRing
attribute [local instance] controlBaseCommRing
abbrev tameCurve : WeierstrassCurve.Affine ControlBase := MillerPrincipalIdealAudit.controlCurve

def twoUnit : ControlBaseˣ := Units.mk0 2 (by decide +kernel)

theorem f5_convention_counter :
    tameCurve.Δ≠0 ∧
      tameSymbol tameCurve 0 (infinityParameterUnit tameCurve) (coefficientUnit tameCurve twoUnit)≠
        milnorSymbol tameCurve 0 (infinityParameterUnit tameCurve) (coefficientUnit tameCurve twoUnit) ∧
      milnorSymbol tameCurve 0 (infinityParameterUnit tameCurve) (coefficientUnit tameCurve twoUnit)=twoUnit := by
  refine ⟨control_delta,?_,?_⟩
  · rw [tame_parameter_unit tameCurve 0 _ _ (infinity_parameter_order tameCurve)
      (coefficient_order tameCurve twoUnit 0),milnor_parameter_unit tameCurve 0 _ _
      (infinity_parameter_order tameCurve) (coefficient_order tameCurve twoUnit 0)]
    change (localUnitValue tameCurve 0 (Units.map (constantHom tameCurve).toMonoidHom twoUnit) _ )⁻¹≠
      localUnitValue tameCurve 0 (Units.map (constantHom tameCurve).toMonoidHom twoUnit) _
    rw [local_value_constant]
    intro he
    have hv := congrArg (fun z : ControlBaseˣ => (z : ControlBase)) he
    change (2 : ControlBase)⁻¹=2 at hv
    exact (by decide +kernel : (2 : ControlBase)⁻¹≠2) hv
  · rw [milnor_parameter_unit tameCurve 0 _ _ (infinity_parameter_order tameCurve)
      (coefficient_order tameCurve twoUnit 0)]
    exact local_value_constant tameCurve 0 twoUnit

/-- Concrete actual curve and actual simple poles: the signless Steinberg claim is false. -/
theorem f5_missing_sign_counter :
    tameCurve.Δ≠0 ∧ ∃ f g : tameCurve.FunctionFieldˣ,
      (f : tameCurve.FunctionField)+(g : tameCurve.FunctionField)=1 ∧
      pointOrder tameCurve f 0= -1 ∧ pointOrder tameCurve g 0= -1 ∧
      tameSymbol tameCurve 0 f g=1 ∧ unsignedSymbol tameCurve 0 f g≠1 := by
  let f := (infinityParameterUnit tameCurve)^(-1 : ℤ)
  have hf : pointOrder tameCurve f 0= -1 := by
    change pointOrder tameCurve ((infinityParameterUnit tameCurve)^(-1 : ℤ) : tameCurve.FunctionFieldˣ) 0= -1
    rw [order_zpow,infinity_parameter_order,mul_one]
  obtain ⟨g,hs,hg⟩ := simple_pole_partner tameCurve 0 f hf
  refine ⟨control_delta,f,g,hs,hf,hg,tame_steinberg tameCurve 0 f g hs,?_⟩
  rw [unsigned_at_simple_poles tameCurve 0 f g hs hf hg]
  intro he
  have hv := congrArg (fun z : ControlBaseˣ => (z : ControlBase)) he
  exact (by decide +kernel : (-1 : ControlBase)≠1) hv

/-- The actual x−1 has no zero at any F5-rational finite point on y²=x³+1. -/
theorem f5_no_rational_vertical_zero :
    ∀ x y : ControlBase,tameCurve.Equation x y → x≠1 := by
  have hn : ∀ y : ControlBase,¬tameCurve.Equation 1 y := by
    simp only [WeierstrassCurve.Affine.equation_iff]
    decide +kernel
  intro x y h he
  subst x
  exact hn y h

def missingRootUnit : tameCurve.FunctionFieldˣ := Units.mk0 (verticalFunction tameCurve 1)
  (vertical_nonzero tameCurve 1)

theorem missing_root_divisor :
    functionDivisor tameCurve missingRootUnit missingRootUnit.ne_zero=Finsupp.single 0 (-2) := by
  classical
  ext P
  cases P with
  | zero =>
    rw [function_divisor_apply]
    change -WithZero.log (chartAdicValuation tameCurve (verticalFunction tameCurve 1))=_
    rw [chart_adic_vertical,WithZero.log_exp]
    simp only [←Point.zero_def,Finsupp.single_eq_same]
  | some x y h =>
    have hn : x-1≠0 := sub_ne_zero.mpr (f5_no_rational_vertical_zero x y h.1)
    have hv := coordinate_nonvanishing_value tameCurve h
      (WeierstrassCurve.Affine.CoordinateRing.XClass tameCurve 1)
      (by rw [point_eval_XClass];exact hn)
    rw [function_divisor_apply]
    change -WithZero.log (finiteAdic tameCurve h (verticalFunction tameCurve 1))=_
    change finiteAdic tameCurve h (verticalFunction tameCurve 1)=1 at hv
    rw [hv,WithZero.log_one,neg_zero]
    simp

/-- Omitting geometric points really falsifies a global product1, on an actual curve. -/
theorem f5_rational_only_product_counter :
    tameCurve.Δ≠0 ∧
      (globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit : ControlBase)=4 ∧
      globalTameProduct tameCurve (coefficientUnit tameCurve twoUnit) missingRootUnit≠1 := by
  have hd : divisorDegree tameCurve (functionDivisor tameCurve missingRootUnit missingRootUnit.ne_zero)= -2 := by
    rw [missing_root_divisor,degree_single]
  have hp := global_constant_degree tameCurve twoUnit missingRootUnit
  rw [hd] at hp
  refine ⟨control_delta,?_,?_⟩
  · rw [hp]
    norm_num [twoUnit]
    decide +kernel
  · rw [hp]
    intro he
    have hv := congrArg (fun z : ControlBaseˣ => (z : ControlBase)) he
    norm_num [twoUnit] at hv
    exact (by decide +kernel : (4 : ControlBase)≠1) hv

#print axioms unsigned_at_simple_poles
#print axioms binary_sign_collapse
#print axioms f5_convention_counter
#print axioms f5_missing_sign_counter
#print axioms f5_rational_only_product_counter
end
end EllipticTameSymbolAudit
