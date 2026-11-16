module
public import GeneralLocalMap

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerNormValuationAudit MillerFinitePointAudit MillerBasechangeAudit
open FinitePointEvaluationAudit LocalTranslationAudit
open scoped WithZero
variable {K : Type*} [Field K]

local instance valuationPointMonoid (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Equation b c) : Monoid (PointLocalRing W b c h) :=
  (inferInstance : CommRing (PointLocalRing W b c h)).toMonoid

local instance pointFractionRing (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Equation b c) : IsFractionRing (PointLocalRing W b c h) W.FunctionField :=
  pointFunctionFractionRing W h

theorem point_u_field (W : WeierstrassCurve.Affine K) {b c : K} (h : W.Equation b c) :
    pointToField W h (localU W h)=shiftX W b := by
  rw [localU,point_to_field_map]
  exact vertical_formula W b

theorem point_v_field (W : WeierstrassCurve.Affine K) {b c : K} (h : W.Equation b c) :
    pointToField W h (localV W h)=shiftY W c := by
  simp only [localV,point_to_field_map,YClass,yFunction,shiftY,constantHom,
    RingHom.comp_apply,map_sub,map_zero,sub_zero]

variable {L : Type*} [Field L] [Algebra K L]

theorem derivative_y_basechange (W : WeierstrassCurve.Affine K) (b c : K) :
    derivativeY (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)=
      algebraMap K L (derivativeY W b c) := by
  simp only [derivativeY,map_add,map_mul,map_ofNat]
  rfl

/-- The actual chosen point uniformizer remains the actual chosen uniformizer after constants extend. -/
theorem point_parameter_basechange (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) :
    fieldBaseChange (L := L) W (pointToField W h.1 (pointParameter W h))=
      pointToField (extendedCurve (L := L) W) (base_nonsingular W h).1
        (pointParameter (extendedCurve (L := L) W) (base_nonsingular W h)) := by
  classical
  by_cases ht : derivativeY W b c≠0
  · have ht' : derivativeY (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)≠0 := by
      rw [derivative_y_basechange]
      intro hz
      exact ht ((algebraMap K L).injective (hz.trans (map_zero _).symm))
    simp only [pointParameter,ite_eq_left ht,ite_eq_left ht']
    rw [point_u_field,point_u_field]
    simp only [shiftX,map_sub,field_base_x,field_base_constant]
  · have ht' : ¬derivativeY (extendedCurve (L := L) W) (algebraMap K L b) (algebraMap K L c)≠0 := by
      apply not_ne_iff.mpr
      rw [derivative_y_basechange,not_ne_iff.mp ht,map_zero]
    simp only [pointParameter,ite_eq_right ht,ite_eq_right ht']
    rw [point_v_field,point_v_field]
    simp only [shiftY,map_sub,field_base_y,field_base_constant]

theorem point_parameter_basechange_value (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) :
    finiteAdic (extendedCurve (L := L) W) (base_nonsingular W h)
      (fieldBaseChange (L := L) W (pointToField W h.1 (pointParameter W h)))=
      WithZero.exp (-1 : ℤ) := by
  rw [point_parameter_basechange,finite_adic_parameter]

theorem point_unit_basechange_value (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (u : (PointLocalRing W b c h.1)ˣ) :
    finiteAdic (extendedCurve (L := L) W) (base_nonsingular W h)
      (fieldBaseChange (L := L) W (pointToField W h.1 u))=1 := by
  have hv := finite_adic_unit (extendedCurve (L := L) W) (base_nonsingular W h)
    (Units.map (pointLocalBasechange (L := L) W h).toMonoidHom u)
  change finiteAdic (extendedCurve (L := L) W) (base_nonsingular W h)
    (pointToField (extendedCurve (L := L) W) (base_nonsingular W h).1
      (pointLocalBasechange (L := L) W h u))=1 at hv
  rw [point_local_basechange_field] at hv
  exact hv

/-- Actual normalized finite-point valuation is unchanged, derived on every actual fraction. -/
theorem finite_valuation_basechange (W : WeierstrassCurve.Affine K) {b c : K}
    (h : W.Nonsingular b c) (f : W.FunctionField) :
    finiteAdic (extendedCurve (L := L) W) (base_nonsingular W h)
      (fieldBaseChange (L := L) W f)=finiteAdic W h f := by
  let := pointLocalDVR W h
  by_cases hf : f=0
  · rw [hf,map_zero,map_zero,map_zero]
  have hπ : Irreducible (pointParameter W h) :=
    (IsDiscreteValuationRing.irreducible_iff_uniformizer _).mpr (point_parameter_maximal W h)
  obtain ⟨n,u,hu⟩ := IsDiscreteValuationRing.exists_units_eq_smul_zpow_of_irreducible hπ hf
  have he : f=pointToField W h.1 u*pointToField W h.1 (pointParameter W h)^n := by
    simpa only [Units.smul_def,Algebra.smul_def,NegationPullbackAudit.point_field_map] using hu
  rw [he]
  simp only [map_mul,map_zpow₀,point_parameter_basechange_value,
    finite_adic_unit,finite_adic_parameter]
  rw [point_unit_basechange_value W h u]

variable [DecidableEq K] [DecidableEq L]

/-- Actual point-image normalized local basechange, including the actual infinity chart. -/
theorem local_valuation_basechange (W : WeierstrassCurve.Affine K) (P : W.Point) (f : W.FunctionField) :
    localFunctionValuation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (fieldBaseChange (L := L) W f)=localFunctionValuation W P f := by
  cases P with
  | zero =>
    have hp : pointBaseChange (L := L) W Point.zero=
        (Point.zero : (extendedCurve (L := L) W).Point) := rfl
    rw [hp]
    exact function_valuation_basechange W f
  | some b c h =>
    rw [point_base_some]
    exact finite_valuation_basechange W h f

theorem local_order_basechange (W : WeierstrassCurve.Affine K) (P : W.Point) (f : W.FunctionField) :
    pointOrder (extendedCurve (L := L) W) (fieldBaseChange (L := L) W f) (pointBaseChange (L := L) W P)=
      pointOrder W f P := by
  rw [point_order_formula,point_order_formula,local_valuation_basechange]

#print axioms point_parameter_basechange
#print axioms finite_valuation_basechange
#print axioms local_order_basechange
end
end GeneralTranslationAudit
