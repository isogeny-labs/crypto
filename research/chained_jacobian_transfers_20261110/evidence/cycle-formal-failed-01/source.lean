import Init.Grind.Ring.Basic

/-! Exact CM algebra and finite monodromy for the bounded actual graph walk.
The actual curves, global endomorphism classification, quotient existence,
polarization, field descent and identification of their torsion modules are
written bridges. No axiom here asserts one of those conclusions. -/
set_option autoImplicit false
namespace IsogenyCycles
open Lean.Grind

theorem quadratic_norm_multiplicative {R : Type} [CommRing R]
    (t p a b c d : R) :
    (a^2+t*a*b+p*b^2)*(c^2+t*c*d+p*d^2) =
    (a*c-p*b*d)^2+t*(a*c-p*b*d)*(a*d+b*c+t*b*d)+
      p*(a*d+b*c+t*b*d)^2 := by grind

theorem first_cycle_polynomial {R : Type} [CommRing R] (x : R) :
    (x-1)^3-1=(x^2-x+1)*(x-2) := by grind

theorem second_cycle_polynomial {R : Type} [CommRing R] (x : R) :
    (x+2)^4+x=(x^2-x+1)*(x^2+9*x+32)+4*(14*x-4) := by grind

theorem cm_cycle_norms :
    (31 : Int)^2-7*31*5+101*5^2=7^4 ∧
    (22 : Int)^2-11*22+101=7^3 := by decide

abbrev V4 := Fin 4 × Fin 4
def frobenius4 (v : V4) : V4 := (3*v.2,v.1+v.2)
def opposite4 (v : V4) : V4 := (3*v.1,3*v.2)
def firstCycle4 (v : V4) : V4 := (3*v.1+3*v.2,v.1)
def secondCycle4 (v : V4) : V4 := (2*v.1+3*v.2,v.1+3*v.2)
def iterate {A : Type} (f : A → A) : Nat → A → A
  | 0,a => a
  | n+1,a => f (iterate f n a)

theorem exact_finite_monodromy :
    (∀ v : V4, iterate frobenius4 3 v=opposite4 v) ∧
    (∀ v : V4, iterate firstCycle4 3 v=v) ∧
    (∀ v : V4, iterate secondCycle4 4 v=iterate frobenius4 4 v) ∧
    (∀ v : V4, iterate secondCycle4 12 v=v) := by decide

theorem no_early_relative_sign :
    ∀ n : Fin 2,
      (∃ v : V4, iterate secondCycle4 (4*(n.val+1)) v≠v) ∧
      (∃ v : V4, iterate secondCycle4 (4*(n.val+1)) v≠opposite4 v) := by decide

/-- Surjectivity of the actual kernel projection is a genuine geometric
input. A noncentral target action cannot preserve its graph up to sign. -/
theorem noncentral_action_does_not_close_graph {E F : Type}
    (k : E → F) (T neg : F → F)
    (onto : ∀ q, ∃ p, k p=q)
    (notIdentity : ∃ q, T q≠q) (notNegative : ∃ q, T q≠neg q) :
    ¬ ((∀ p, T (k p)=k p) ∨ (∀ p, T (k p)=neg (k p))) := by
  intro h
  rcases h with positive | negative
  · obtain ⟨q,hq⟩ := notIdentity
    obtain ⟨p,hp⟩ := onto q
    exact hq (by rw [← hp]; exact positive p)
  · obtain ⟨q,hq⟩ := notNegative
    obtain ⟨p,hp⟩ := onto q
    exact hq (by rw [← hp]; exact negative p)

theorem public_closed_walk_crt :
    (12035 : Nat)%109=45 ∧ (12035 : Nat)%113=57 ∧
    (73 : Nat)^9%109=45 ∧ (23 : Nat)^12%113=56 ∧
    (113-56 : Nat)=57 := by decide

end IsogenyCycles
#print axioms IsogenyCycles.quadratic_norm_multiplicative
#print axioms IsogenyCycles.first_cycle_polynomial
#print axioms IsogenyCycles.second_cycle_polynomial
#print axioms IsogenyCycles.cm_cycle_norms
#print axioms IsogenyCycles.exact_finite_monodromy
#print axioms IsogenyCycles.no_early_relative_sign
#print axioms IsogenyCycles.noncentral_action_does_not_close_graph
#print axioms IsogenyCycles.public_closed_walk_crt
