import Recoding
import Mathlib.LinearAlgebra.Matrix.Determinant.Basic
noncomputable section
open Matrix
namespace Determinantal
variable {K : Type*} [Field K]

/-- Affine-linear 3x3 pencil, with a fixed identity pivot block. -/
def pencil (a b c x y l0 l1 l2 : K) : Matrix (Fin 3) (Fin 3) K :=
  !![y-b*x-c-l0, x, l1; a*x, 1, 0; l2, 0, 1]

theorem determinant (a b c x y l0 l1 l2 : K) :
    (pencil a b c x y l0 l1 l2).det = y-(a*x^2+b*x+c+l0+l1*l2) := by
  simp [pencil, Matrix.det_fin_three]
  ring

/-- Complete normalized kernel chart; the fixed pivot prevents a missing chart. -/
theorem normalized_kernel_iff (a b c x y l0 l1 l2 : K) :
    (∃ v w : K, (pencil a b c x y l0 l1 l2).mulVec ![1,v,w] = 0) ↔
      y = a*x^2+b*x+c+l0+l1*l2 := by
  constructor
  · rintro ⟨v, w, h⟩
    have h0 := congrFun h 0
    have h1 := congrFun h 1
    have h2 := congrFun h 2
    simp [pencil, Matrix.mulVec, dotProduct, Fin.sum_univ_succ] at h0 h1 h2
    linear_combination h0-x*h1-l1*h2
  · intro h
    refine ⟨-a*x, -l2, ?_⟩
    ext i
    fin_cases i <;> simp [pencil, Matrix.mulVec, dotProduct, Fin.sum_univ_succ]
    linear_combination h

theorem determinant_iff_kernel (a b c x y l0 l1 l2 : K) :
    (pencil a b c x y l0 l1 l2).det = 0 ↔
      ∃ v w : K, (pencil a b c x y l0 l1 l2).mulVec ![1,v,w] = 0 := by
  rw [normalized_kernel_iff, determinant, sub_eq_zero]

/-- This particular determinantal arm reduces exactly to XOR recoding. -/
theorem xor_schur (a b c x y u v : K) :
    (pencil a b c x y (2*u+12*v) (-4*u) v).det = 0 ↔
      y = Recoding.graph a b c x u v := by
  rw [determinant, sub_eq_zero]
  unfold Recoding.graph
  ring

end Determinantal

#print axioms Determinantal.determinant
#print axioms Determinantal.normalized_kernel_iff
#print axioms Determinantal.determinant_iff_kernel
#print axioms Determinantal.xor_schur
