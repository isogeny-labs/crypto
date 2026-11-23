import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! A rootless residue polynomial excludes every integer root. -/

namespace XedniResidue10K3

def evalInt : List ℤ → ℤ → ℤ
  | [], _ => 0
  | c :: cs, t => c + t * evalInt cs t

def evalResidue (n : ℕ) : List ℤ → ZMod n → ZMod n
  | [], _ => 0
  | c :: cs, z => (c : ZMod n) + z * evalResidue n cs z

theorem evalResidue_cast (n : ℕ) (cs : List ℤ) (t : ℤ) :
    evalResidue n cs (t : ZMod n) = (evalInt cs t : ZMod n) := by
  induction cs with
  | nil => simp [evalInt, evalResidue]
  | cons c cs ih => simp [evalInt, evalResidue, ih]

theorem no_integer_root_of_rootless_mod (n : ℕ) (cs : List ℤ)
    (hrootless : ∀ z : ZMod n, evalResidue n cs z ≠ 0) (t : ℤ) :
    evalInt cs t ≠ 0 := by
  intro hzero
  have h := hrootless (t : ZMod n)
  rw [evalResidue_cast] at h
  exact h (by simp [hzero])

#print axioms evalResidue_cast
#print axioms no_integer_root_of_rootless_mod

end XedniResidue10K3
