import Init.Grind.Module.Basic

/-! Algebra for the explicit F101 descent sign witness and quarter-coordinate
evaluation. No Galois action,regular map,curve or Torelli theorem is defined. -/
set_option autoImplicit false
namespace RichelotDescent
open Lean.Grind

theorem not_neg_fixed_of_bezout {A : Type} [IntModule A]
    (a : A) (n u v : Int) (hb : u*2+v*n=1)
    (hn : n • a=0) (hne : a ≠ 0) : -a ≠ a := by
  intro hneg
  have h2 : (2 : Int) • a=0 := by grind
  have hz : a=0 := by
    calc
      a = (1 : Int) • a := by grind
      _ = (u*2+v*n) • a := by rw [hb]
      _ = u • ((2 : Int) • a) + v • (n • a) := by
        rw [IntModule.add_zsmul,IntModule.mul_zsmul,IntModule.mul_zsmul]
      _ = 0 := by
        rw [h2,hn,IntModule.zsmul_zero,IntModule.zsmul_zero]
        grind
  exact hne hz

theorem sign_ambiguity_removed_by_odd_witness {X A : Type} [IntModule A]
    (f g : X → A) (x : X) (n u v : Int)
    (hb : u*2+v*n=1) (hn : n • f x=0) (hne : f x ≠ 0)
    (choices : g=f ∨ g=(fun y => -f y)) (fixed : g x=f x) : g=f := by
  rcases choices with same | negative
  · exact same
  · rw [negative] at fixed
    exact False.elim (not_neg_fixed_of_bezout (f x) n u v hb hn hne fixed)

theorem quarter_order109 {A : Type} [IntModule A]
    (a : A) (hn : (109 : Int) • a=0) :
    (4 : Int) • ((82 : Int) • a)=a := by grind

theorem quarter_order113 {A : Type} [IntModule A]
    (a : A) (hn : (113 : Int) • a=0) :
    (4 : Int) • ((85 : Int) • a)=a := by grind

end RichelotDescent
#print axioms RichelotDescent.not_neg_fixed_of_bezout
#print axioms RichelotDescent.sign_ambiguity_removed_by_odd_witness
#print axioms RichelotDescent.quarter_order109
#print axioms RichelotDescent.quarter_order113
