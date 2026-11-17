import Init.Grind.Module.Basic

/-! Algebraic consequences of TWO actual transported seven-isogeny squares.
The variables are functions on modules, not constructed abelian varieties.
Regular quotient/descent/polarization and the actual map hypotheses remain
written obligations. No cancellation assumption on geometric points is used. -/
set_option autoImplicit false
namespace TwoHopTransport
open Lean.Grind

theorem two_commuting_squares {A B C T U V : Type}
    (r0 : A → T) (r1 : B → U) (r2 : C → V)
    (phi1 : A → B) (phi2 : B → C) (psi1 : T → U) (psi2 : U → V)
    (h1 : ∀ a, psi1 (r0 a) = r1 (phi1 a))
    (h2 : ∀ b, psi2 (r1 b) = r2 (phi2 b)) :
    ∀ a, psi2 (psi1 (r0 a)) = r2 (phi2 (phi1 a)) := by
  intro a
  rw [h1, h2]

theorem two_chain_adjoint_norm49 {A B C : Type}
    [IntModule A] [IntModule B]
    (f : A → B) (fh : B → A) (g : B → C) (gh : C → B)
    (hf : ∀ a, fh (f a) = (7 : Int) • a)
    (hg : ∀ b, gh (g b) = (7 : Int) • b)
    (hfl : ∀ n : Int, ∀ b, fh (n • b) = n • fh b) :
    ∀ a, fh (gh (g (f a))) = (49 : Int) • a := by
  intro a
  rw [hg, hfl, hf]
  grind

theorem full_source_reverse196 {E A T U V : Type}
    [IntModule E] [IntModule A] [IntModule T] [IntModule U]
    (pull : E → A) (norm : A → E) (r : A → T) (rh : T → A)
    (f : T → U) (fh : U → T) (g : U → V) (gh : V → U)
    (hn : ∀ e, norm (pull e) = (2 : Int) • e)
    (hr : ∀ a, rh (r a) = (2 : Int) • a)
    (hf : ∀ t, fh (f t) = (7 : Int) • t)
    (hg : ∀ u, gh (g u) = (7 : Int) • u)
    (hfl : ∀ n : Int, ∀ u, fh (n • u) = n • fh u)
    (hrl : ∀ n : Int, ∀ t, rh (n • t) = n • rh t)
    (hnl : ∀ n : Int, ∀ a, norm (n • a) = n • norm a) :
    ∀ e, norm (rh (fh (gh (g (f (r (pull e))))))) = (196 : Int) • e := by
  intro e
  rw [two_chain_adjoint_norm49 f fh g gh hf hg hfl, hrl, hnl, hr, hnl, hn]
  grind

theorem recovery196_order109 {A : Type} [IntModule A]
    (a : A) (ha : (109 : Int) • a = 0) :
    (104 : Int) • ((196 : Int) • a) = a := by grind

theorem recovery196_order113 {A : Type} [IntModule A]
    (a : A) (ha : (113 : Int) • a = 0) :
    (64 : Int) • ((196 : Int) • a) = a := by grind

end TwoHopTransport
#print axioms TwoHopTransport.two_commuting_squares
#print axioms TwoHopTransport.two_chain_adjoint_norm49
#print axioms TwoHopTransport.full_source_reverse196
#print axioms TwoHopTransport.recovery196_order109
#print axioms TwoHopTransport.recovery196_order113
