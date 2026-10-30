import KoblitzRound22Base
import Mathlib.LinearAlgebra.Matrix.Determinant.Basic
namespace KoblitzRound22SS
variable {K : Type*}

-- Algebra behind the nonreduced projections; the original fiber
-- decomposition is separately certified by native ideal lifts and Frobenius.
theorem lambda_fourth_power [CommRing K] [CharP K 2] (l b : K) :
    (l^4+b*l+b^2)^4=l^16+b^4*l^4+b^8 := by
  have fourth (x y : K) : (x+y)^4=x^4+y^4 := by
    calc
      (x+y)^4=((x+y)^2)^2 := by ring
      _=(x^2+y^2)^2 := by rw [KoblitzRound22Base.char_two_square]
      _=(x^2)^2+(y^2)^2 := KoblitzRound22Base.char_two_square _ _
      _=x^4+y^4 := by ring
  rw [fourth,fourth]
  ring

theorem trace_square [CommRing K] [CharP K 2] (z b : K) :
    (z^4+b^2*z+b^3)^2=z^8+b^4*z^2+b^6 := by
  rw [KoblitzRound22Base.char_two_square,KoblitzRound22Base.char_two_square]
  ring

theorem relative_jacobian_determinant [CommRing K] (b : K) :
    Matrix.det (!![b, 0; b^4, b^3] : Matrix (Fin 2) (Fin 2) K)=b^4 := by
  simp only [Matrix.det_fin_two,Matrix.of_apply,Matrix.cons_val_zero,
    Matrix.cons_val_one]
  ring
end KoblitzRound22SS
