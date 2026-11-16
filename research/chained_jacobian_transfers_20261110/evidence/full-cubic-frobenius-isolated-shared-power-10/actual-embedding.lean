import Std.Tactic
import Init.GrindInstances.Ring.Fin
set_option autoImplicit false
open Lean.Grind
attribute [local instance] Semiring.natCast
def embed {R : Type} [CommRing R] (a : Fin 223) : R := (a.val : R)

theorem embed_injective {R : Type} [CommRing R] [IsCharP R 223] :
    Function.Injective (embed (R := R)) := by
  intro a b h
  apply Fin.ext
  exact (IsCharP.natCast_eq_iff_of_lt 223 a.isLt b.isLt).mp h

theorem embed_add {R : Type} [CommRing R] [IsCharP R 223] (a b : Fin 223) :
    embed (R := R) (a+b)=embed a+embed b := by
  unfold embed
  rw [Fin.val_add,IsCharP.natCast_emod 223,Semiring.natCast_add]

theorem embed_mul {R : Type} [CommRing R] [IsCharP R 223] (a b : Fin 223) :
    embed (R := R) (a*b)=embed a*embed b := by
  unfold embed
  rw [Fin.val_mul,IsCharP.natCast_emod 223,Semiring.natCast_mul]

theorem embed_pow {R : Type} [CommRing R] [IsCharP R 223] (a : Fin 223) (n : Nat) :
    embed (R := R) (a^n)=(embed a)^n := by
  induction n with
  | zero =>
    simp only [Semiring.pow_zero]
    change ((1 : Nat) : R)=1
    exact Semiring.natCast_one
  | succ n ih =>
    rw [Semiring.pow_succ,embed_mul,ih,Semiring.pow_succ]

#print axioms embed_pow
