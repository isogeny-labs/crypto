import TameLaws

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open scoped WithZero
variable {K : Type*} [Field K]

/-- Actual residue criterion for the actual local unit character. -/
theorem local_value_eq (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P=0) (c : Kˣ)
    (hc : localFunctionValuation W P ((f : W.FunctionField)-constantHom W c)<1) :
    localUnitValue W P f hf=c := by
  apply Units.ext
  change regularEvaluation W P
    ((RegularRing W P).unitGroupMulEquiv ⟨f,(mem_point_units_iff W P f).mpr hf⟩ : RegularRing W P)=c
  apply (regular_evaluation_difference W P _ c).mpr
  exact hc

theorem tame_small (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hs : (f : W.FunctionField)+(g : W.FunctionField)=1)
    (hf : localFunctionValuation W P f<1) : tameSymbol W P f g=1 := by
  have hg : (g : W.FunctionField)=1-(f : W.FunctionField) := by linear_combination hs
  have hv : localFunctionValuation W P g=1 := by
    rw [hg];exact (localFunctionValuation W P).map_one_sub_of_lt hf
  have ho : pointOrder W g P=0 := (point_order_zero_iff_value_one W g g.ne_zero P).mpr hv
  have he : localUnitValue W P g ho=1 := by
    apply local_value_eq W P g ho 1
    have ha : (g : W.FunctionField)-constantHom W (1 : K)= -(f : W.FunctionField) := by
      rw [map_one,hg];ring
    rw [Units.val_one,ha,Valuation.map_neg];exact hf
  have hb : balancedLocalUnit W P f g=
      (⟨g,(mem_point_units_iff W P g).mpr ho⟩ : (RegularRing W P).unitGroup)^(-pointOrder W f P) := by
    apply Subtype.ext
    rw [Subgroup.coe_zpow]
    simp only [balancedLocalUnit,balancedUnit,ho,zpow_zero,one_div,zpow_neg]
  simp only [tameSymbol,ho,mul_zero,zpow_zero,one_mul,hb,map_zpow]
  change (localUnitValue W P g ho)^(-pointOrder W f P)=1
  rw [he,one_zpow]

theorem tame_large (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hs : (f : W.FunctionField)+(g : W.FunctionField)=1)
    (hf : 1<localFunctionValuation W P f) : tameSymbol W P f g=1 := by
  have hg : (g : W.FunctionField)=1-(f : W.FunctionField) := by linear_combination hs
  have hv : localFunctionValuation W P g=localFunctionValuation W P f := by
    rw [hg];exact (localFunctionValuation W P).map_sub_eq_of_lt_right (by simpa using hf)
  have ho : pointOrder W g P=pointOrder W f P := by rw [point_order_formula,point_order_formula,hv]
  have hr : pointOrder W (f/g : W.FunctionFieldˣ) P=0 := by
    rw [Units.val_div_eq_div_val,point_order_div W f.ne_zero g.ne_zero,ho,sub_self]
  have he : localUnitValue W P (f/g) hr=(-1 : Kˣ) := by
    apply local_value_eq W P (f/g) hr (-1)
    have ha : (↑(f/g) : W.FunctionField)-constantHom W (-1 : K)=(g : W.FunctionField)⁻¹ := by
      rw [Units.val_div_eq_div_val,map_neg,map_one]
      field_simp [g.ne_zero]
      linear_combination hs
    rw [Units.coe_neg_one,ha,map_inv₀]
    have hvg : 1<localFunctionValuation W P g := hv.symm ▸ hf
    exact (inv_lt_one₀ (zero_lt_one.trans hvg)).mpr hvg
  have hb : balancedLocalUnit W P f g=
      (⟨f/g,(mem_point_units_iff W P (f/g)).mpr hr⟩ : (RegularRing W P).unitGroup)^pointOrder W f P := by
    apply Subtype.ext
    simp only [balancedLocalUnit,balancedUnit,ho,div_zpow,Subgroup.coe_zpow]
  rw [tameSymbol,ho,hb,map_zpow]
  change (-1 : Kˣ)^(pointOrder W f P*pointOrder W f P)*
    (localUnitValue W P (f/g) hr)^pointOrder W f P=1
  rw [he,←zpow_add]
  have hp : Even (pointOrder W f P*pointOrder W f P+pointOrder W f P) := by
    convert Int.even_mul_succ_self (pointOrder W f P) using 1
    ring
  exact hp.neg_one_zpow

/-- Actual Steinberg relation, with no restriction on common zeros/poles or characteristic.
Both functions are genuinely nonzero field units and their sum is actually1. -/
theorem tame_steinberg (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hs : (f : W.FunctionField)+(g : W.FunctionField)=1) :
    tameSymbol W P f g=1 := by
  rcases lt_trichotomy (localFunctionValuation W P f) 1 with h|h|h
  · exact tame_small W P f g hs h
  · have ho : pointOrder W f P=0 := (point_order_zero_iff_value_one W f f.ne_zero P).mpr h
    have hg : localFunctionValuation W P g≤1 := by
      have he : (g : W.FunctionField)=1-(f : W.FunctionField) := by linear_combination hs
      rw [he]
      exact (localFunctionValuation W P).map_sub_le (by rw [map_one]) h.le
    rcases hg.eq_or_lt with he|he
    · exact tame_units W P f g ho ((point_order_zero_iff_value_one W g g.ne_zero P).mpr he)
    · rw [tame_skew,tame_small W P g f (by simpa only [add_comm] using hs) he,inv_one]
  · exact tame_large W P f g hs h

#print axioms local_value_eq
#print axioms tame_small
#print axioms tame_large
#print axioms tame_steinberg
end
end EllipticTameSymbolAudit
