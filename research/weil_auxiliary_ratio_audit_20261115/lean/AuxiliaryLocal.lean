import AuxiliaryDivisor

namespace WeilAuxiliaryRatioAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit
variable {K : Type*} [Field K]

theorem local_negation_value (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionField) :
    localFunctionValuation W P (fieldNeg W f)=localFunctionValuation W (-P) f := by
  cases P with
  | zero => exact field_neg_norm_value W f
  | some x y h => exact finite_pullback_valuation W h f

theorem negation_constant (W : WeierstrassCurve.Affine K) (c : K) :
    fieldNeg W (constantHom W c)=constantHom W c := by
  simpa only [polynomial_function_constant] using field_neg_polynomial W (Polynomial.C c)

def regularNegation (W : WeierstrassCurve.Affine K) (P : W.Point) :
    RegularRing W (-P) →+* RegularRing W P where
  toFun r := ⟨fieldNeg W r,by
    change localFunctionValuation W P (fieldNeg W r)≤1
    rw [local_negation_value];exact r.property⟩
  map_zero' := Subtype.ext (map_zero (fieldNeg W))
  map_one' := Subtype.ext (map_one (fieldNeg W))
  map_add' a b := Subtype.ext (map_add (fieldNeg W) (a : W.FunctionField) (b : W.FunctionField))
  map_mul' a b := Subtype.ext (map_mul (fieldNeg W) (a : W.FunctionField) (b : W.FunctionField))

/-- Actual residue equality is the actual maximal-ideal condition. -/
theorem regular_evaluation_difference (W : WeierstrassCurve.Affine K) (P : W.Point)
    (r : RegularRing W P) (c : K) :
    regularEvaluation W P r=c ↔
      localFunctionValuation W P ((r : W.FunctionField)-constantHom W c)<1 := by
  let d : RegularRing W P := r-regularConstant W P c
  have he : regularEvaluation W P d=0 ↔ d∈IsLocalRing.maximalIdeal (RegularRing W P) := by
    change d∈RingHom.ker (regularEvaluation W P) ↔ _
    rw [regular_evaluation_kernel]
  have hv : d∈IsLocalRing.maximalIdeal (RegularRing W P) ↔
      localFunctionValuation W P ((r : W.FunctionField)-constantHom W c)<1 :=
    Valuation.mem_maximalIdeal_iff W.FunctionField (localFunctionValuation W P)
  rw [←hv,←he]
  change regularEvaluation W P r=c ↔ regularEvaluation W P (r-regularConstant W P c)=0
  rw [map_sub,regular_eval_constant,sub_eq_zero]

/-- Negation residue naturality, derived from its actual normalized local maps. -/
theorem regular_negation_residue (W : WeierstrassCurve.Affine K) (P : W.Point)
    (r : RegularRing W (-P)) :
    regularEvaluation W P (regularNegation W P r)=regularEvaluation W (-P) r := by
  apply (regular_evaluation_difference W P _ _).mpr
  change localFunctionValuation W P
    (fieldNeg W r-constantHom W (regularEvaluation W (-P) r))<1
  rw [←negation_constant,←map_sub,local_negation_value]
  exact (regular_evaluation_difference W (-P) r _).mp rfl

def negatedUnit (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ →* W.FunctionFieldˣ :=
  Units.map (fieldNeg W).toMonoidHom

def pointUnitNegation (W : WeierstrassCurve.Affine K) (P : W.Point) :
    (RegularRing W (-P)).unitGroup →* (RegularRing W P).unitGroup where
  toFun f := ⟨negatedUnit W f.val,by
    apply (mem_point_units_iff W P _).mpr
    change pointOrder W (fieldNeg W (f.val : W.FunctionField)) P=0
    rw [point_order_pullback]
    exact (mem_point_units_iff W (-P) f.val).mp f.property⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul (negatedUnit W) a.val b.val)

theorem point_unit_negation_value (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : (RegularRing W (-P)).unitGroup) :
    pointUnitEvaluation W P (pointUnitNegation W P f)=pointUnitEvaluation W (-P) f := by
  apply Units.ext
  have he : ((RegularRing W P).unitGroupMulEquiv (pointUnitNegation W P f) : RegularRing W P)=
      regularNegation W P ((RegularRing W (-P)).unitGroupMulEquiv f : RegularRing W (-P)) :=
    Subtype.ext rfl
  change regularEvaluation W P
    ((RegularRing W P).unitGroupMulEquiv (pointUnitNegation W P f) : RegularRing W P)=
    regularEvaluation W (-P) ((RegularRing W (-P)).unitGroupMulEquiv f : RegularRing W (-P))
  rw [he,regular_negation_residue]

/-- A genuinely defined local unit value, never a total meromorphic evaluation. -/
def localUnitValue (W : WeierstrassCurve.Affine K) (P : W.Point) (f : W.FunctionFieldˣ)
    (h : pointOrder W f P=0) : Kˣ :=
  pointUnitEvaluation W P ⟨f,(mem_point_units_iff W P f).mpr h⟩

theorem local_unit_value_mul (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hf : pointOrder W f P=0) (hg : pointOrder W g P=0) :
    localUnitValue W P (f*g) (by rw [Units.val_mul,point_order_mul W f.ne_zero g.ne_zero,hf,hg];rfl)=
      localUnitValue W P f hf*localUnitValue W P g hg :=
  (pointUnitEvaluation W P).map_mul
    ⟨f,(mem_point_units_iff W P f).mpr hf⟩ ⟨g,(mem_point_units_iff W P g).mpr hg⟩

theorem local_unit_value_negation (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f (-P)=0) :
    localUnitValue W P (negatedUnit W f) (by
      change pointOrder W (fieldNeg W (f : W.FunctionField)) P=0
      rw [point_order_pullback];exact hf)=localUnitValue W (-P) f hf :=
  point_unit_negation_value W P ⟨f,(mem_point_units_iff W (-P) f).mpr hf⟩

variable [DecidableEq K]

theorem local_unit_value_translation (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f : W.FunctionFieldˣ) (hf : pointOrder W f (P+Q)=0) :
    localUnitValue W P (generalTranslatedUnit W hΔ Q f) (by
      rw [general_translated_unit_coe,general_translation_order];exact hf)=
      localUnitValue W (P+Q) f hf :=
  general_translation_point_unit_evaluation W hΔ P Q ⟨f,(mem_point_units_iff W (P+Q) f).mpr hf⟩

#print axioms regular_evaluation_difference
#print axioms regular_negation_residue
#print axioms point_unit_negation_value
#print axioms local_unit_value_translation
end
end WeilAuxiliaryRatioAudit
