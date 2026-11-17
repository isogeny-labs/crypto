import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.GroupTheory.Perm.Cycle.Type
import Mathlib.Tactic

namespace BKLinearAudit
noncomputable section
variable {K : Type*} [Field K]

def actualCoordinates (W : WeierstrassCurve.Affine K) : W.Point → Option (K×K)
  | .zero => none
  | .some x y _ => some (x,y)

theorem actualCoordinates_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (actualCoordinates W) := by
  intro P Q h
  cases P <;> cases Q <;> simp_all [actualCoordinates]

theorem actual_rational_point_finite [Finite K] (W : WeierstrassCurve.Affine K) :
    Finite W.Point := Finite.of_injective (actualCoordinates W) (actualCoordinates_injective W)

variable [DecidableEq K]

/-- Actual rational points supply the source prime-order input by additive
Cauchy. No actual elliptic torsion basis or determinant is assumed. -/
theorem actual_rational_prime_torsion [Finite K] (W : WeierstrassCurve.Affine K)
    (r : ℕ) (hr : r.Prime) (hdiv : r∣Nat.card W.Point) :
    ∃ P : W.Point,P≠0 ∧ addOrderOf P=r ∧ r • P=0 := by
  have : Finite W.Point := actual_rational_point_finite W
  have : Fact r.Prime := ⟨hr⟩
  obtain ⟨P,hP⟩ := exists_prime_addOrderOf_dvd_card' (G:=W.Point) r hdiv
  refine ⟨P,?_,hP,?_⟩
  · intro hzero
    have hh : r=1 := by simpa [hzero] using hP.symm
    exact hr.ne_one hh
  · rw [← hP]
    exact addOrderOf_nsmul_eq_zero P

#print axioms actual_rational_prime_torsion
end
end BKLinearAudit
