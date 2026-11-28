import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.Tactic

namespace VerheulDistortionAudit

variable {K : Type*} [Field K]

def model (a : K) : WeierstrassCurve.Affine K := ⟨0,0,0,0,a⟩

theorem equation_iff (a x y : K) : (model a).Equation x y ↔ y^2=x^3+a := by
  simp [WeierstrassCurve.Affine.equation_iff,model]

theorem cube_root_ne_zero {w : K} (hw : w^3=1) : w≠0 := by
  intro h
  simp [h] at hw

theorem cube_root_square_ne_one {w : K} (hw : w^3=1) (h1 : w≠1) : w^2≠1 := by
  intro h2
  apply h1
  calc
    w = w*w^2 := by rw [h2,mul_one]
    _ = 1 := by simpa only [← pow_succ'] using hw

/-- The printed y-scaling formula fails at every point with nonzero ordinate. -/
theorem printed_map_fails (a x y w : K) (hw : w^3=1) (h1 : w≠1)
    (hy : y≠0) (hP : (model a).Equation x y) :
    ¬ (model a).Equation x (w*y) := by
  rw [equation_iff] at hP ⊢
  intro h
  have h2 : w^2=1 := by
    apply mul_right_cancel₀ (pow_ne_zero 2 hy)
    rw [one_mul,← mul_pow,h,hP]
  exact cube_root_square_ne_one hw h1 h2

/-- Scaling x instead gives the exact equation-preserving formula. -/
theorem corrected_equation (a x y w : K) (hw : w^3=1) :
    (model a).Equation (w*x) y ↔ (model a).Equation x y := by
  simp only [equation_iff,mul_pow,hw,one_mul]

theorem corrected_nonsingular (a w : K) (hΔ : (model a).Δ≠0) (hw : w^3=1)
    {x y : K} (hP : (model a).Nonsingular x y) : (model a).Nonsingular (w*x) y := by
  apply (WeierstrassCurve.Affine.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp
  exact (corrected_equation a x y w hw).mpr hP.1

end VerheulDistortionAudit
