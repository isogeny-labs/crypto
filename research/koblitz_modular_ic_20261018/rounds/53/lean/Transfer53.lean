import Mathlib.LinearAlgebra.Matrix.Determinant.Basic
import Mathlib.Algebra.Group.Units.Basic
import Mathlib.Tactic.Ring

namespace Transfer53

/-- The full Gram change product implies its discriminant change. Native
trace-pairing construction, actual entries and normalization are external. -/
theorem discriminant_change {R n : Type*} [CommRing R] [Fintype n]
    [DecidableEq n] (M G P : Matrix n n R)
    (h : M.transpose * G * M = P) :
    P.det = M.det ^ 2 * G.det := by
  rw [← h, Matrix.det_mul, Matrix.det_mul, Matrix.det_transpose]
  ring

/-- Composing the torsion membership and coprime-kernel identity gives the
actual alpha inverse numerator, after the three source equations vanish.
The source products and invertibility of delta must be supplied. -/
theorem alpha_product {R : Type*} [CommRing R]
    (G N p3 p7 alpha x y z U V delta : R)
    (ht : p7 = G * x + N * y + alpha * z)
    (hb : p3 * U + p7 * V = delta ^ 16)
    (hG : G = 0) (hN : N = 0) (h3 : p3 = 0) :
    alpha * (V * z) = delta ^ 16 := by
  calc
    alpha * (V * z) = p3 * U + p7 * V := by
      rw [ht, hG, hN, h3]
      ring
    _ = delta ^ 16 := hb

/-- Applies in a quotient/localization or a specialization where the source
equations vanish and delta is a unit. It makes no claim about bad fibers. -/
theorem alpha_isUnit {R : Type*} [CommRing R]
    (G N p3 p7 alpha x y z U V delta : R)
    (ht : p7 = G * x + N * y + alpha * z)
    (hb : p3 * U + p7 * V = delta ^ 16)
    (hG : G = 0) (hN : N = 0) (h3 : p3 = 0)
    (hd : IsUnit delta) : IsUnit alpha := by
  apply isUnit_of_mul_isUnit_left (y := V * z)
  rw [alpha_product G N p3 p7 alpha x y z U V delta ht hb hG hN h3]
  exact hd.pow 16

end Transfer53
