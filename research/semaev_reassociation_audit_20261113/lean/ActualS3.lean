import DomainInterchange
import RecursiveSemaevEvaluation

namespace SemaevReassociationAudit
open Polynomial S4DegreeAudit ResultantDegreeStepAudit
open ResultantInterchangeAudit RecursiveSemaevEvaluationAudit
noncomputable section
variable {R : Type*} [CommRing R]

/-- Inner variable t, outer elimination variable u: literal S3(z,t,u). -/
def s3Bi (b2 b4 b6 b8 z : R) : Polynomial (Polynomial R) :=
  quad (qaRing (C z) X) (qbRing (C b2) (C b4) (C b6) (C z) X)
    (qcRing (C b4) (C b6) (C b8) (C z) X)

def pairPoly (b2 b4 b6 b8 x y : R) : Polynomial R :=
  quad (qaRing x y) (qbRing b2 b4 b6 x y) (qcRing b4 b6 b8 x y)

theorem s3Bi_source (b2 b4 b6 b8 z : R) :
    s3Bi b2 b4 b6 b8 z = s3Ring (C (C b2)) (C (C b4))
      (C (C b6)) (C (C b8)) (C (C z)) (C X) X := by
  simp [s3Bi, s3Ring, quad, qaRing, qbRing, qcRing, map_ofNat]

theorem s3Bi_swap (b2 b4 b6 b8 z : R) :
    Polynomial.Bivariate.swap (s3Bi b2 b4 b6 b8 z) = s3Bi b2 b4 b6 b8 z := by
  rw [s3Bi_source]
  change Polynomial.Bivariate.swap.toRingHom
    (s3Ring (C (C b2)) (C (C b4)) (C (C b6)) (C (C b8)) (C (C z)) (C X) X) = _
  rw [s3Ring_map (R := Polynomial (Polynomial R))
    ((Polynomial.Bivariate.swap (R := R)).toRingHom)
    (C (C b2)) (C (C b4)) (C (C b6)) (C (C b8)) (C (C z)) (C X) X]
  change s3Ring (Polynomial.Bivariate.swap (C (C b2)))
    (Polynomial.Bivariate.swap (C (C b4))) (Polynomial.Bivariate.swap (C (C b6)))
    (Polynomial.Bivariate.swap (C (C b8))) (Polynomial.Bivariate.swap (C (C z)))
    (Polynomial.Bivariate.swap (C X)) (Polynomial.Bivariate.swap X) = _
  simp only [Polynomial.Bivariate.swap_C_C, Polynomial.Bivariate.swap_X,
    Polynomial.Bivariate.swap_Y]
  unfold s3Ring qaRing qbRing qcRing
  ring

theorem s3Bi_degree_le (b2 b4 b6 b8 z : R) :
    (s3Bi b2 b4 b6 b8 z).natDegree ≤ 2 := quad_natDegree_le ..

theorem s3Bi_bidegrees [IsDomain R] (b2 b4 b6 b8 z : R) :
    (s3Bi b2 b4 b6 b8 z).natDegree = 2 ∧
    (Polynomial.Bivariate.swap (s3Bi b2 b4 b6 b8 z)).natDegree = 2 := by
  have h : (s3Bi b2 b4 b6 b8 z).natDegree = 2 := by
    apply natDegree_eq_of_le_of_coeff_ne_zero (s3Bi_degree_le ..)
    rw [s3Bi, quad_coeff_two]
    exact pow_ne_zero 2 (sub_ne_zero.mpr (X_ne_C z).symm)
  rw [s3Bi_swap, h]
  exact ⟨rfl,rfl⟩

theorem pairPoly_eval (b2 b4 b6 b8 x y t : R) :
    (pairPoly b2 b4 b6 b8 x y).eval t = s3Ring b2 b4 b6 b8 x y t := by
  simp [pairPoly, quad, s3Ring]

theorem pairPoly_degree [IsDomain R] (b2 b4 b6 b8 x y : R) (h : x ≠ y) :
    (pairPoly b2 b4 b6 b8 x y).natDegree = 2 := by
  unfold pairPoly
  apply natDegree_eq_of_le_of_coeff_ne_zero (quad_natDegree_le ..)
  rw [quad_coeff_two]
  exact pow_ne_zero 2 (sub_ne_zero.mpr h)

theorem step_specialization (p : Polynomial R) (D : ℕ) (b2 b4 b6 b8 z : R) :
    pairEval (C : R →+* Polynomial R) (C z) X (resultantStep D p b2 b4 b6 b8) =
      Polynomial.resultant (lift p) (s3Bi b2 b4 b6 b8 z) D 2 := by
  rw [resultantStep_pairEval]
  rfl

end
end SemaevReassociationAudit
