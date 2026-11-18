import Std.Tactic

namespace CuspCover95

inductive Cusp where
  | O | A | C | D | B
  deriving DecidableEq, Repr

open Cusp

def diamond : Cusp → Cusp
  | O => A | A => C | C => D | D => B | B => O

def inverseDiamond : Cusp → Cusp
  | O => B | A => O | C => A | D => C | B => D

theorem diamond_inverse (c : Cusp) : inverseDiamond (diamond c) = c := by
  cases c <;> rfl

theorem inverse_diamond (c : Cusp) : diamond (inverseDiamond c) = c := by
  cases c <;> rfl

theorem diamond_fifth (c : Cusp) :
    diamond (diamond (diamond (diamond (diamond c)))) = c := by
  cases c <;> rfl

-- The ramification index e is supplied by the source fiber evidence.
def ordU (c : Cusp) (e : Int) : Int :=
  match c with
  | O => -2 * e
  | A => e
  | C => 0
  | D => 0
  | B => e

def ordH (c : Cusp) (e : Int) : Int := ordU (diamond c) e
def ordK (c : Cusp) (e : Int) : Int := ordU (inverseDiamond c) e

theorem h_orders (e : Int) :
    ordH O e = e ∧ ordH A e = 0 ∧ ordH C e = 0 ∧
    ordH D e = e ∧ ordH B e = -2 * e := by
  simp [ordH, ordU, diamond]

theorem k_orders (e : Int) :
    ordK O e = e ∧ ordK A e = -2 * e ∧ ordK C e = e ∧
    ordK D e = 0 ∧ ordK B e = 0 := by
  simp [ordK, ordU, inverseDiamond]

def excludedU0 : Cusp → Bool
  | A | B => true
  | _ => false

def excludedU1 : Cusp → Bool
  | O | D => true
  | _ => false

theorem excluded_disjoint (c : Cusp) :
    ¬ (excludedU0 c = true ∧ excludedU1 c = true) := by
  cases c <;> decide

theorem h_zero_in_U0 (c : Cusp) :
    (excludedU0 c = false ∧ (c = O ∨ c = D)) ↔
    (c = O ∨ c = D) := by
  cases c <;> decide

end CuspCover95

#print axioms CuspCover95.diamond_inverse
#print axioms CuspCover95.inverse_diamond
#print axioms CuspCover95.diamond_fifth
#print axioms CuspCover95.h_orders
#print axioms CuspCover95.k_orders
#print axioms CuspCover95.excluded_disjoint
#print axioms CuspCover95.h_zero_in_U0
