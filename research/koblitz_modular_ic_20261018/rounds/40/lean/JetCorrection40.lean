import Mathlib.Tactic.Ring

/-! Universal two-variable linear-correction identities used by the jet
solver. No native arrays, Newton convergence, local source geometry or
function-field identification is imported. -/
namespace JetCorrection40
variable {A : Type*} [CommRing A]

theorem adjugate_first (a b c d e f : A) :
    a*(d*e-b*f)+b*(a*f-c*e) = (a*d-b*c)*e := by ring

theorem adjugate_second (a b c d e f : A) :
    c*(d*e-b*f)+d*(a*f-c*e) = (a*d-b*c)*f := by ring

theorem inverse_correction_first (a b c d e f q : A)
    (h : (a*d-b*c)*q=1) :
    a*((d*e-b*f)*q)+b*((a*f-c*e)*q)=e := by
  calc
    _ = ((a*d-b*c)*q)*e := by ring
    _ = e := by rw [h, one_mul]

theorem inverse_correction_second (a b c d e f q : A)
    (h : (a*d-b*c)*q=1) :
    c*((d*e-b*f)*q)+d*((a*f-c*e)*q)=f := by
  calc
    _ = ((a*d-b*c)*q)*f := by ring
    _ = f := by rw [h, one_mul]

theorem trivial_kernel_of_unit_determinant (a b c d q x y : A)
    (h : (a*d-b*c)*q=1) (h₁ : a*x+b*y=0) (h₂ : c*x+d*y=0) :
    x=0 ∧ y=0 := by
  have hx : (a*d-b*c)*x=0 := by
    calc
      _ = d*(a*x+b*y)-b*(c*x+d*y) := by ring
      _ = 0 := by rw [h₁, h₂]; ring
  have hy : (a*d-b*c)*y=0 := by
    calc
      _ = a*(c*x+d*y)-c*(a*x+b*y) := by ring
      _ = 0 := by rw [h₁, h₂]; ring
  constructor
  · calc
      x = ((a*d-b*c)*q)*x := by rw [h, one_mul]
      _ = q*((a*d-b*c)*x) := by ring
      _ = 0 := by rw [hx, mul_zero]
  · calc
      y = ((a*d-b*c)*q)*y := by rw [h, one_mul]
      _ = q*((a*d-b*c)*y) := by ring
      _ = 0 := by rw [hy, mul_zero]
end JetCorrection40
