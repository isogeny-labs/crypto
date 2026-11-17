import TameSteinberg

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open scoped WithZero
variable {K : Type*} [Field K]

def infinityParameterUnit (W : WeierstrassCurve.Affine K) : W.FunctionFieldˣ :=
  Units.mk0 (parameter W) (parameter_is_uniformizer W).ne_zero

theorem infinity_parameter_order (W : WeierstrassCurve.Affine K) :
    pointOrder W (infinityParameterUnit W) 0=1 := by
  rw [point_order_formula]
  change -WithZero.log (functionValuation W (parameter W))=1
  rw [function_valuation_parameter,WithZero.log_exp]
  rfl

/-- Actual leading local unit with respect to any actual order1 parameter. -/
def leadingLocalUnit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (f : W.FunctionFieldˣ) :
    (RegularRing W P).unitGroup :=
  ⟨f/π^pointOrder W f P,by
    apply (mem_point_units_iff W P _).mpr
    rw [Units.val_div_eq_div_val,point_order_div W f.ne_zero (Units.ne_zero _),order_zpow,hπ,mul_one,sub_self]⟩

theorem leading_balance {G : Type*} [CommGroup G] (f g π : G) (a b : ℤ) :
    (f/π^a)^b/(g/π^b)^a=f^b/g^a := by
  rw [div_zpow,div_zpow,←zpow_mul,←zpow_mul,mul_comm b a,div_div_div_cancel_right]

/-- Parameter formula is derived from the actual parameter-free balanced unit. -/
theorem balanced_leading (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (f g : W.FunctionFieldˣ) :
    balancedLocalUnit W P f g=
      (leadingLocalUnit W P π hπ f)^pointOrder W g P/
        (leadingLocalUnit W P π hπ g)^pointOrder W f P := by
  apply Subtype.ext
  simp only [Subgroup.coe_div,Subgroup.coe_zpow]
  exact (leading_balance f g π (pointOrder W f P) (pointOrder W g P)).symm

def parameterSymbol (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (f g : W.FunctionFieldˣ) : Kˣ :=
  (-1 : Kˣ)^(pointOrder W f P*pointOrder W g P)*
    (pointUnitEvaluation W P (leadingLocalUnit W P π hπ f))^pointOrder W g P/
      (pointUnitEvaluation W P (leadingLocalUnit W P π hπ g))^pointOrder W f P

theorem tame_parameter_formula (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (f g : W.FunctionFieldˣ) :
    tameSymbol W P f g=parameterSymbol W P π hπ f g := by
  rw [tameSymbol,balanced_leading W P π hπ,map_div,map_zpow,map_zpow,parameterSymbol,mul_div_assoc]

/-- No compatibility between parameters is assumed; both formulas equal the actual symbol. -/
theorem parameter_independence (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π ρ : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (hρ : pointOrder W ρ P=1)
    (f g : W.FunctionFieldˣ) :
    parameterSymbol W P π hπ f g=parameterSymbol W P ρ hρ f g := by
  rw [←tame_parameter_formula,←tame_parameter_formula]

/-- Classical convention on (parameter,local unit) is inverse residue. -/
theorem tame_parameter_unit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π u : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (hu : pointOrder W u P=0) :
    tameSymbol W P π u=(localUnitValue W P u hu)⁻¹ := by
  have hb : balancedLocalUnit W P π u=
      (⟨u,(mem_point_units_iff W P u).mpr hu⟩ : (RegularRing W P).unitGroup)⁻¹ := by
    apply Subtype.ext
    simp only [balancedLocalUnit,balancedUnit,hu,hπ,zpow_zero,zpow_one,one_div,Subgroup.coe_inv]
  rw [tameSymbol,hπ,hu,mul_zero,zpow_zero,one_mul,hb,map_inv]
  rfl

/-- The author-erratum's inverse repairs the actual parameter/unit basis normalization. -/
theorem milnor_parameter_unit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (π u : W.FunctionFieldˣ) (hπ : pointOrder W π P=1) (hu : pointOrder W u P=0) :
    milnorSymbol W P π u=localUnitValue W P u hu := by
  rw [milnorSymbol,tame_parameter_unit W P π u hπ hu,inv_inv]

variable [DecidableEq K]

/-- Translation of the actual X/Y at infinity constructs a parameter at every actual point. -/
def pointParameterUnit (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) :
    W.FunctionFieldˣ := generalTranslatedUnit W hΔ (-P) (infinityParameterUnit W)

theorem point_parameter_order (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (P : W.Point) :
    pointOrder W (pointParameterUnit W hΔ P) P=1 := by
  rw [pointParameterUnit,general_translated_unit_coe,general_translation_order,add_neg_cancel]
  exact infinity_parameter_order W

#print axioms tame_parameter_formula
#print axioms parameter_independence
#print axioms milnor_parameter_unit
#print axioms point_parameter_order
end
end EllipticTameSymbolAudit
