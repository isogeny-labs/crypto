import GaloisLocal

namespace WeilGaloisCovarianceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerBasechangeAudit FinitePointEvaluationAudit GeneralTranslationAudit MillerFinitePointAudit
open WeilAuxiliaryRatioAudit
variable {K L : Type*} [Field K] [Field L] [Algebra K L]
variable [DecidableEq K] [DecidableEq L]

/-- All four proper local domains survive the actual point/function maps. -/
theorem auxiliary_admissible_basechange (W : WeierstrassCurve.Affine K)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) :
    AuxiliaryAdmissible (extendedCurve (L := L) W)
      (pointBaseChange W P) (pointBaseChange W Q)
      (unitBasechange W f) (unitBasechange W g) (pointBaseChange W S) := by
  unfold AuxiliaryAdmissible
  rw [←map_add,←map_neg,←map_sub]
  simpa only [unit_basechange_order,AuxiliaryAdmissible] using h

theorem auxiliary_value_basechange (W : WeierstrassCurve.Affine K)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) :
    auxiliaryValue (extendedCurve (L := L) W) (pointBaseChange W P) (pointBaseChange W Q)
      (unitBasechange W f) (unitBasechange W g) (pointBaseChange W S)
      (auxiliary_admissible_basechange W P Q S f g h)=
        Units.map (algebraMap K L).toMonoidHom (auxiliaryValue W P Q f g S h) := by
  unfold auxiliaryValue
  simp only [←map_add,←map_neg,←map_sub]
  rw [local_unit_basechange_value W (S+Q) f h.1,
    local_unit_basechange_value W (-S) g h.2.1,
    local_unit_basechange_value W S f h.2.2.1,
    local_unit_basechange_value W (P-S) g h.2.2.2,map_div,map_mul,map_mul]

/-- The exact source26.1 orientation is natural, not its inverse convention. -/
theorem source_auxiliary_basechange (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q S : W.Point) (f g : W.FunctionFieldˣ) (h : AuxiliaryAdmissible W P Q f g S) :
    sourceAuxiliaryValue (extendedCurve (L := L) W) (extended_delta_nonzero W hΔ)
      (pointBaseChange W P) (pointBaseChange W Q) (pointBaseChange W S)
      (unitBasechange W f) (unitBasechange W g) (auxiliary_admissible_basechange W P Q S f g h)=
        Units.map (algebraMap K L).toMonoidHom (sourceAuxiliaryValue W hΔ P Q S f g h) := by
  rw [source_auxiliary_inverse,source_auxiliary_inverse,auxiliary_value_basechange,map_inv]

#print axioms auxiliary_admissible_basechange
#print axioms auxiliary_value_basechange
#print axioms source_auxiliary_basechange
end
end WeilGaloisCovarianceAudit
