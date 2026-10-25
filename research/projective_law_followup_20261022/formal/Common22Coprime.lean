import Mathlib.Algebra.Polynomial.Degree.Domain
import Mathlib.RingTheory.Coprime.Basic
import Mathlib.Tactic.Ring

set_option autoImplicit false

namespace Common22Coprime

open Polynomial

variable {K : Type*} [Field K]

noncomputable def firstSupport (q1 r1 : K) : K[X] := X ^ 2 + C q1 * X + C r1
noncomputable def secondSupport (q2 r2 : K) : K[X] := X ^ 2 + C q2 * X + C r2
def determinant (q1 r1 q2 r2 : K) : K :=
  (q1 - q2) * (q2 * r1 - q1 * r2) - (r1 - r2) ^ 2

/-- An exact Bézout identity for the source common-branch determinant. -/
theorem determinant_bezout (q1 r1 q2 r2 : K) :
    let dq := q1 - q2
    let dr := r1 - r2
    let a : K[X] := C dq * X + C (dq * q1 - dr)
    C (determinant q1 r1 q2 r2) =
      (a - C (dq ^ 2)) * firstSupport q1 r1 - a * secondSupport q2 r2 := by
  dsimp [determinant, firstSupport, secondSupport]
  simp only [map_sub, map_mul, map_pow]
  ring

/-- Nonzero source determinant gives coprime quadratic supports. -/
theorem supports_coprime (q1 r1 q2 r2 : K)
    (hb : determinant q1 r1 q2 r2 ≠ 0) :
    IsCoprime (firstSupport q1 r1) (secondSupport q2 r2) := by
  let dq := q1 - q2
  let dr := r1 - r2
  let a : K[X] := C dq * X + C (dq * q1 - dr)
  let b := determinant q1 r1 q2 r2
  have hbez : C b =
      (a - C (dq ^ 2)) * firstSupport q1 r1 - a * secondSupport q2 r2 := by
    simpa [a, dq, dr, b] using determinant_bezout q1 r1 q2 r2
  refine ⟨C (b⁻¹) * (a - C (dq ^ 2)), -(C (b⁻¹) * a), ?_⟩
  calc
    C (b⁻¹) * (a - C (dq ^ 2)) * firstSupport q1 r1 +
        -(C (b⁻¹) * a) * secondSupport q2 r2 =
      C (b⁻¹) * ((a - C (dq ^ 2)) * firstSupport q1 r1 -
        a * secondSupport q2 r2) := by ring
    _ = C (b⁻¹) * C b := by rw [hbez]
    _ = 1 := by simp [← map_mul, b, hb]

end Common22Coprime

#print axioms Common22Coprime.determinant_bezout
#print axioms Common22Coprime.supports_coprime
