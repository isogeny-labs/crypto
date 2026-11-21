import Init.Grind.Module.Basic

/-! Structural implications used by the transported odd-isogeny square.
Types model point groups only here. Regular quotient descent, polarizations,
finite-field divisors and the actual map hypotheses remain written bridges.
No torsion-free assumption is imposed on geometric point groups. -/
set_option autoImplicit false
namespace ChainTransport
open Lean.Grind

theorem chain_kills_two_torsion {A B C D : Type}
    [IntModule B] [IntModule D]
    (r : A → B) (rh : B → A) (phi : A → C) (s : C → D)
    (hr : ∀ b, r (rh b) = (2 : Int) • b)
    (hk : ∀ a, r a = 0 → s (phi a) = 0)
    (b : B) (hb : (2 : Int) • b = 0) :
    s (phi (rh b)) = 0 := by
  apply hk
  rw [hr, hb]

theorem transported_square {A B C D : Type}
    [IntModule A] [IntModule B]
    (r : A → B) (rh : B → A) (phi : A → C)
    (s : C → D) (psi : B → D)
    (h2 : ∀ a : A, ∃ x : A, (2 : Int) • x = a)
    (hr2 : ∀ a, r ((2 : Int) • a) = (2 : Int) • r a)
    (hrh : ∀ a, rh (r a) = (2 : Int) • a)
    (hfactor : ∀ b, psi ((2 : Int) • b) = s (phi (rh b))) :
    ∀ a, psi (r a) = s (phi a) := by
  intro a
  obtain ⟨x, hx⟩ := h2 a
  rw [← hx, hr2, hfactor, hrh]

theorem composite_norm28 {A B C D : Type}
    [IntModule A] [IntModule B] [IntModule C]
    (r : A → B) (rh : B → A) (phi : A → C) (phih : C → A)
    (s : C → D) (sh : D → C)
    (hr : ∀ b, r (rh b) = (2 : Int) • b)
    (hp : ∀ a, phih (phi a) = (7 : Int) • a)
    (hs : ∀ c, sh (s c) = (2 : Int) • c)
    (hrlin : ∀ n : Int, ∀ a, r (n • a) = n • r a)
    (hplin : ∀ n : Int, ∀ c, phih (n • c) = n • phih c) :
    ∀ b, r (phih (sh (s (phi (rh b))))) = (28 : Int) • b := by
  intro b
  rw [hs, hplin, hrlin, hp, hrlin, hr]
  grind

theorem half_input_order12317 {A : Type} [IntModule A]
    (a : A) (ha : (12317 : Int) • a = 0) :
    (2 : Int) • ((6159 : Int) • a) = a := by grind

theorem full_source_reverse28 {E A B D : Type}
    [IntModule E] [IntModule A] [IntModule B]
    (pull : E → A) (norm : A → E) (r : A → B) (rh : B → A)
    (psi : B → D) (psih : D → B)
    (hn : ∀ e, norm (pull e) = (2 : Int) • e)
    (hr : ∀ a, rh (r a) = (2 : Int) • a)
    (hp : ∀ b, psih (psi b) = (7 : Int) • b)
    (hrlin : ∀ n : Int, ∀ b, rh (n • b) = n • rh b)
    (hnlin : ∀ n : Int, ∀ a, norm (n • a) = n • norm a) :
    ∀ e, norm (rh (psih (psi (r (pull e))))) = (28 : Int) • e := by
  intro e
  rw [hp, hrlin, hnlin, hr, hnlin, hn]
  grind

theorem target_coordinate4 {E A B C D F : Type}
    [IntModule C] [IntModule F]
    (pull : E → A) (r : A → B) (phi : A → C) (psi : B → D)
    (s : C → D) (sh : D → C) (norm : C → F) (alpha : E → F)
    (hsquare : ∀ a, psi (r a) = s (phi a))
    (hs : ∀ c, sh (s c) = (2 : Int) • c)
    (hn : ∀ e, norm (phi (pull e)) = (2 : Int) • alpha e)
    (hnlin : ∀ n : Int, ∀ c, norm (n • c) = n • norm c) :
    ∀ e, norm (sh (psi (r (pull e)))) = (4 : Int) • alpha e := by
  intro e
  rw [hsquare, hs, hnlin, hn]
  grind

end ChainTransport
#print axioms ChainTransport.chain_kills_two_torsion
#print axioms ChainTransport.transported_square
#print axioms ChainTransport.composite_norm28
#print axioms ChainTransport.half_input_order12317
#print axioms ChainTransport.full_source_reverse28
#print axioms ChainTransport.target_coordinate4
