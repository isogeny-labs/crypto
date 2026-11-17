import TateMillerEvaluation

namespace TateMillerEvaluationAudit
noncomputable section
open WeierstrassCurve.Affine MillerRecursionAudit MillerBasechangeAudit
open FunctionDivisorFinitenessAudit
open MillerFinitePointAudit WeilAuxiliaryRatioAudit
variable {K : Type*} [Field K] [DecidableEq K]

theorem actual_step_divisor (W : WeierstrassCurve.Affine K) (S Q : W.Point) :
    functionDivisor W (stepFunction W S Q) (step_function_nonzero W S Q)=additionDivisor W S Q := by
  ext R
  exact (step_has_point_divisor W S Q).2 R

theorem actual_shifted_representative (W : WeierstrassCurve.Affine K) (S Q : W.Point) :
    shiftedDivisor W (S+Q) S+
      functionDivisor W (stepFunction W S Q) (step_function_nonzero W S Q)=
        Finsupp.single Q 1-Finsupp.single 0 1 := by
  rw [actual_step_divisor]
  unfold shiftedDivisor additionDivisor
  abel

theorem actual_geometric_shifted_representative
    {L : Type*} [Field L] [Algebra K L] [DecidableEq L]
    (W : WeierstrassCurve.Affine K) (S Q : W.Point) :
    shiftedDivisor (extendedCurve (L:=L) W)
        (pointBaseChange (L:=L) W S+pointBaseChange (L:=L) W Q) (pointBaseChange (L:=L) W S)+
      functionDivisor (extendedCurve (L:=L) W) (fieldBaseChange (L:=L) W (stepFunction W S Q))
        (field_base_nonzero W (step_function_nonzero W S Q))=
      Finsupp.single (pointBaseChange (L:=L) W Q) 1-Finsupp.single 0 1 := by
  simpa only [field_base_step] using
    actual_shifted_representative (extendedCurve (L:=L) W)
      (pointBaseChange (L:=L) W S) (pointBaseChange (L:=L) W Q)

def extendedChosenUnit {L : Type*} [Field L] [Algebra K L]
    (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    (extendedCurve (L:=L) W).FunctionFieldˣ:=
  Units.map (fieldBaseChange (L:=L) W).toMonoidHom (chosenFunctionUnit W P n)

theorem actual_extended_chosen_unit
    {L : Type*} [Field L] [Algebra K L] [DecidableEq L]
    (W : WeierstrassCurve.Affine K) (P R : W.Point) (n : ℕ)
    (hn : n • P=0) (hRP : R≠P) (hR0 : R≠0) :
    pointOrder (extendedCurve (L:=L) W) (extendedChosenUnit (L:=L) W P n)
      (pointBaseChange (L:=L) W R)=0 := by
  change pointOrder (extendedCurve (L:=L) W)
    (fieldBaseChange (L:=L) W (millerFunction W P n)) (pointBaseChange (L:=L) W R)=0
  rw [miller_function_basechange,(miller_has_point_divisor _ _ _).2 _]
  have hk : n • pointBaseChange (L:=L) W P=0 := by rw [←map_nsmul,hn,map_zero]
  have hp : pointBaseChange (L:=L) W R≠pointBaseChange (L:=L) W P :=
    fun h=>hRP (point_base_injective W h)
  have h0 : pointBaseChange (L:=L) W R≠0 :=
    fun h=>hR0 (point_base_injective W (h.trans (map_zero _).symm))
  simp [millerDivisor,hk,hp,h0]

theorem actual_geometric_shifted_support
    {L : Type*} [Field L] [Algebra K L] [DecidableEq L]
    (W : WeierstrassCurve.Affine K) (P A B : W.Point) (n : ℕ)
    (hn : n • P=0) (hAP : A≠P) (hA0 : A≠0) (hBP : B≠P) (hB0 : B≠0) :
    Disjoint (shiftedDivisor (extendedCurve (L:=L) W)
        (pointBaseChange (L:=L) W A) (pointBaseChange (L:=L) W B)).support
      (functionDivisor (extendedCurve (L:=L) W) (extendedChosenUnit (L:=L) W P n)
        (extendedChosenUnit (L:=L) W P n).ne_zero).support :=
  shifted_disjoint _ _ _ _ (actual_extended_chosen_unit W P A n hn hAP hA0)
    (actual_extended_chosen_unit W P B n hn hBP hB0)

#print axioms actual_step_divisor
#print axioms actual_shifted_representative
#print axioms actual_geometric_shifted_representative
#print axioms actual_extended_chosen_unit
#print axioms actual_geometric_shifted_support
end
end TateMillerEvaluationAudit
