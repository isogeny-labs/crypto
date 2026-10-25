import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 1000000

namespace RepeatedFactor
variable {R : Type} [Lean.Grind.CommRing R]

theorem cubic_identity (a x y z : R) :
    a^3 - a^2*(x+y+z) + a*(x*y+x*z+y*z) - x*y*z =
      (a-x)*(a-y)*(a-z) := by
  grind

-- The explicit no-zero-divisor hypothesis holds in a field. No assumption
-- states the desired repeated-factor conclusion.
theorem power_zero (noZero : ∀ u v : R, u*v=0 → u=0 ∨ v=0)
    (g : R) (k : Nat) (h : g^k=0) : g=0 := by
  have aux : ∀ n : Nat, g^n=0 → g=0 := by
    intro n
    induction n with
    | zero => intro hn; grind
    | succ n ih =>
      intro hn
      have hp : g^n*g=0 := by simpa [Lean.Grind.Semiring.pow_succ] using hn
      rcases noZero (g^n) g hp with hpow | hg
      · exact ih hpow
      · exact hg
  exact aux k h

theorem rejection_of_certificate
    (noZero : ∀ u v : R, u*v=0 → u=0 ∨ v=0)
    (a x y z f0 f1 f2 f3 h0 h1 h2 h3 : R) (k : Nat)
    (roots : f0=0 ∧ f1=0 ∧ f2=0 ∧ f3=0)
    (certificate :
      (a^3-a^2*(x+y+z)+a*(x*y+x*z+y*z)-x*y*z)^k =
        h0*f0+h1*f1+h2*f2+h3*f3) :
    a=x ∨ a=y ∨ a=z := by
  have hz : (a^3-a^2*(x+y+z)+a*(x*y+x*z+y*z)-x*y*z)^k=0 := by
    grind
  have hg := power_zero noZero _ k hz
  rw [cubic_identity] at hg
  rcases noZero ((a-x)*(a-y)) (a-z) hg with hxy | hz
  · rcases noZero (a-x) (a-y) hxy with hx | hy
    · left; grind
    · right; left; grind
  · right; right; grind

#print axioms cubic_identity
#print axioms power_zero
#print axioms rejection_of_certificate
end RepeatedFactor
