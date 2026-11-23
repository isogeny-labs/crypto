import Mathlib.Data.ZMod.Basic
import Mathlib.Data.Matrix.Mul
import Mathlib.Tactic
open scoped Matrix

namespace SplitPrime199V1
abbrev F := ZMod 199
def projection : Matrix (Fin 2) (Fin 2) F := !![1, 106; 1, 92]
def decoder : Matrix (Fin 2) (Fin 2) F := !![164, 36; 128, 71]

theorem root : (106 : F) ^ 2 + 106 + 1 = 0 := by decide
theorem conjugate_root : (92 : F) ^ 2 + 92 + 1 = 0 := by decide

theorem left_certificate : decoder * projection = 1 := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    norm_num [decoder, projection, Matrix.mul_apply, Fin.sum_univ_succ] <;> decide

theorem right_certificate : projection * decoder = 1 := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    norm_num [decoder, projection, Matrix.mul_apply, Fin.sum_univ_succ] <;> decide

theorem roundtrip (v : Fin 2 → F) : decoder *ᵥ (projection *ᵥ v) = v := by
  rw [Matrix.mulVec_mulVec, left_certificate, Matrix.one_mulVec]

theorem reverse_roundtrip (v : Fin 2 → F) : projection *ᵥ (decoder *ᵥ v) = v := by
  rw [Matrix.mulVec_mulVec, right_certificate, Matrix.one_mulVec]

theorem projection_injective : Function.Injective projection.mulVec :=
  Function.LeftInverse.injective roundtrip

theorem projection_surjective : Function.Surjective projection.mulVec :=
  Function.RightInverse.surjective reverse_roundtrip

theorem left_row_zero (a b : ℤ) :
    164 * (a + 106 * b) + 36 * (a + 92 * b) - a = 199 * (a + 104 * b) := by ring

theorem left_row_one (a b : ℤ) :
    128 * (a + 106 * b) + 71 * (a + 92 * b) - b = 199 * (a + 101 * b) := by ring

theorem right_row_zero (a b : ℤ) :
    (164 * a + 36 * b) + 106 * (128 * a + 71 * b) - a =
      199 * (69 * a + 38 * b) := by ring

theorem right_row_one (a b : ℤ) :
    (164 * a + 36 * b) + 92 * (128 * a + 71 * b) - b =
      199 * (60 * a + 33 * b) := by ring
end SplitPrime199V1
