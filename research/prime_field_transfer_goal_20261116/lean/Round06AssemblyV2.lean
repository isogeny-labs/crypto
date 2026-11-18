import Mathlib.Data.ZMod.Basic
import Mathlib.Data.Matrix.Mul
import Mathlib.Tactic
open scoped Matrix

example (A B : Matrix (Fin 2) (Fin 2) (ZMod 199)) (h : B * A = 1)
    (v : Fin 2 → ZMod 199) : B *ᵥ (A *ᵥ v) = v := by
  rw [Matrix.mulVec_mulVec, h, Matrix.one_mulVec]

example : (!![164, 36; 128, 71] : Matrix (Fin 2) (Fin 2) (ZMod 199)) *
    !![1, 106; 1, 92] = 1 := by
  ext i j
  fin_cases i <;> fin_cases j <;>
    norm_num [Matrix.mul_apply, Fin.sum_univ_succ] <;> decide

example (A B : Matrix (Fin 2) (Fin 2) (ZMod 199)) (h : B * A = 1) :
    Function.Injective A.mulVec := by
  apply Function.LeftInverse.injective (g := B.mulVec)
  intro v
  rw [Matrix.mulVec_mulVec, h, Matrix.one_mulVec]
