import Reassociation

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit
open ResultantInterchangeAudit RecursiveSemaevEvaluationAudit
noncomputable section
variable {R : Type*} [CommRing R]

def scalarLift : R →+* Polynomial (Polynomial R) := C.comp C

theorem scalarLift_injective : Function.Injective (scalarLift : R →+* Polynomial (Polynomial R)) :=
  C_injective.comp C_injective

theorem pairEval_scalar (x y a : R) :
    pairEval (RingHom.id R) x y (scalarLift a) = a := by
  simp [scalarLift, pairEval]

theorem pairEval_inner (x y : R) :
    pairEval (RingHom.id R) x y (C X) = x := by simp [pairEval]

theorem pairEval_outer (x y : R) :
    pairEval (RingHom.id R) x y X = y := by simp [pairEval]

theorem lift_specialize (p : Polynomial R) (x y : R) :
    (p.map scalarLift).map (pairEval (RingHom.id R) x y) = p := by
  rw [Polynomial.map_map]
  have h : (pairEval (RingHom.id R) x y).comp scalarLift = RingHom.id R := by
    ext a
    exact pairEval_scalar x y a
  rw [h, Polynomial.map_id]

/-- Specialization retains all declared sizes; x=y is included. -/
theorem reassociation_all_parameters [IsDomain R] (p : Polynomial R)
    (b2 b4 b6 b8 x y z : R) :
    leftAssoc p.natDegree p b2 b4 b6 b8 x y z =
      rightAssoc p.natDegree p b2 b4 b6 b8 x y z := by
  have h := reassociation_distinct (p.map scalarLift)
    (scalarLift b2) (scalarLift b4) (scalarLift b6) (scalarLift b8)
    (C (X : Polynomial R)) X (scalarLift z) (X_ne_C (X : Polynomial R)).symm
  rw [natDegree_map_eq_of_injective scalarLift_injective] at h
  have he := congrArg (pairEval (RingHom.id R) x y) h
  rw [leftAssoc_map, rightAssoc_map] at he
  simpa only [lift_specialize, pairEval_scalar, pairEval_inner, pairEval_outer] using he

#print axioms reassociation_all_parameters

end
end SemaevReassociationAudit
