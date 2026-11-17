import Init.Grind.Module.Basic

/-! The middle norm/pullback relation clears the apparent repeated quarter.
These are module consequences with explicit map identities and linearity.
No torsion-free assumption on geometric point groups is used. The second
statement explicitly concerns a domain where multiplication by4 is injective;
global cancellation in Hom remains a separate written geometric bridge. -/
set_option autoImplicit false
namespace ChainCompression
open Lean.Grind

theorem two_hop_double_cleared {J0 J1 J2 E0 E1 E2 : Type}
    [IntModule J1] [IntModule J2] [IntModule E1] [IntModule E2]
    (f : J0 → J1) (g : J1 → J2)
    (h0 : J0 → E0) (k1 : E1 → J1) (h1 : J1 → E1) (k2 : E2 → J2)
    (a : E0 → E1) (b : E1 → E2)
    (hf : ∀ x, (4 : Int) • f x = k1 (a (h0 x)))
    (hg : ∀ x, (4 : Int) • g x = k2 (b (h1 x)))
    (hm : ∀ e, h1 (k1 e) = (4 : Int) • e)
    (gl : ∀ n : Int, ∀ x, g (n • x) = n • g x)
    (bl : ∀ n : Int, ∀ e, b (n • e) = n • b e)
    (kl : ∀ n : Int, ∀ e, k2 (n • e) = n • k2 e) :
    ∀ x, (16 : Int) • g (f x) = (4 : Int) • k2 (b (a (h0 x))) := by
  intro x
  calc
    (16 : Int) • g (f x) = (4 : Int) • g ((4 : Int) • f x) := by
      rw [gl]
      grind
    _ = (4 : Int) • g (k1 (a (h0 x))) := by rw [hf]
    _ = k2 (b (h1 (k1 (a (h0 x))))) := hg _
    _ = k2 (b ((4 : Int) • a (h0 x))) := by rw [hm]
    _ = (4 : Int) • k2 (b (a (h0 x))) := by rw [bl, kl]

theorem rational_two_hop_factorization {J0 J1 J2 E0 E1 E2 : Type}
    [IntModule J1] [IntModule J2] [IntModule E1] [IntModule E2]
    (f : J0 → J1) (g : J1 → J2)
    (h0 : J0 → E0) (k1 : E1 → J1) (h1 : J1 → E1) (k2 : E2 → J2)
    (a : E0 → E1) (b : E1 → E2)
    (hf : ∀ x, (4 : Int) • f x = k1 (a (h0 x)))
    (hg : ∀ x, (4 : Int) • g x = k2 (b (h1 x)))
    (hm : ∀ e, h1 (k1 e) = (4 : Int) • e)
    (gl : ∀ n : Int, ∀ x, g (n • x) = n • g x)
    (bl : ∀ n : Int, ∀ e, b (n • e) = n • b e)
    (kl : ∀ n : Int, ∀ e, k2 (n • e) = n • k2 e)
    (cancel4 : ∀ u v : J2, (4 : Int) • u = (4 : Int) • v → u = v) :
    ∀ x, (4 : Int) • g (f x) = k2 (b (a (h0 x))) := by
  intro x
  apply cancel4
  have h := two_hop_double_cleared f g h0 k1 h1 k2 a b hf hg hm gl bl kl x
  grind

end ChainCompression
#print axioms ChainCompression.two_hop_double_cleared
#print axioms ChainCompression.rational_two_hop_factorization
