import LawsCertificate
import GaloisImports

namespace WeilGaloisCovarianceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerBasechangeAudit FinitePointEvaluationAudit GeneralTranslationAudit
open MillerFinitePointAudit
open WeilAuxiliaryRatioAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]
variable [DecidableEq K] [DecidableEq L]

/-- Actual function-field units under the actual field embedding. -/
def unitBasechange (W : WeierstrassCurve.Affine K) :
    W.FunctionFieldˣ →* (extendedCurve (L := L) W).FunctionFieldˣ :=
  Units.map (fieldBaseChange (L := L) W).toMonoidHom

theorem unit_basechange_order (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) :
    pointOrder (extendedCurve (L := L) W)
      (unitBasechange (L := L) W f).val
      (pointBaseChange (L := L) W P)=pointOrder W f P :=
  local_order_basechange W P f

/-- Actual regular/nonvanishing subgroup membership is derived from actual orders. -/
def pointUnitBasechange (W : WeierstrassCurve.Affine K) (P : W.Point) :
    (RegularRing W P).unitGroup →*
      (RegularRing (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)).unitGroup where
  toFun f := ⟨unitBasechange W f.val,by
    apply (mem_point_units_iff _ _ _).mpr
    rw [unit_basechange_order]
    exact (mem_point_units_iff W P f.val).mp f.property⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul (unitBasechange W) a.val b.val)

/-- Actual residue values commute with basechange, including infinity. -/
theorem point_unit_basechange_value (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : (RegularRing W P).unitGroup) :
    pointUnitEvaluation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (pointUnitBasechange W P f)=
        Units.map (algebraMap K L).toMonoidHom (pointUnitEvaluation W P f) := by
  apply Units.ext
  have he : ((RegularRing (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)).unitGroupMulEquiv
      (pointUnitBasechange W P f) :
        RegularRing (extendedCurve (L := L) W) (pointBaseChange (L := L) W P))=
      regularBasechange (L := L) W P
        ((RegularRing W P).unitGroupMulEquiv f : RegularRing W P) := Subtype.ext rfl
  change regularEvaluation (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      ((RegularRing (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)).unitGroupMulEquiv
        (pointUnitBasechange (L := L) W P f)).val=
    algebraMap K L (regularEvaluation W P
      ((RegularRing W P).unitGroupMulEquiv f).val)
  rw [he,regular_basechange_residue]

theorem local_unit_basechange_value (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) (hf : pointOrder W f P=0) :
    localUnitValue (extendedCurve (L := L) W) (pointBaseChange (L := L) W P)
      (unitBasechange W f) (by rw [unit_basechange_order];exact hf)=
        Units.map (algebraMap K L).toMonoidHom (localUnitValue W P f hf) :=
  point_unit_basechange_value W P ⟨f,(mem_point_units_iff W P f).mpr hf⟩

theorem chosen_unit_basechange (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    unitBasechange (L := L) W (chosenFunctionUnit W P n)=
      chosenFunctionUnit (extendedCurve (L := L) W) (pointBaseChange (L := L) W P) n :=
  Units.ext (miller_function_basechange W P n)

#print axioms unit_basechange_order
#print axioms point_unit_basechange_value
#print axioms local_unit_basechange_value
#print axioms chosen_unit_basechange
end
end WeilGaloisCovarianceAudit
