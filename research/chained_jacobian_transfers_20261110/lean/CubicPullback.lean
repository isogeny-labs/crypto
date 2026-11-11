import Init.Grind.Module.Basic
import Init.Grind.Ring.Basic

/-! Structural components of the cubic prototype. These do not construct
Picard groups, finite extensions or geometric trace maps. -/
set_option autoImplicit false
namespace CubicPullback

def inflate {K : Type} [Zero K] (d : Nat) (a : Nat → K) : Nat → K :=
  fun n => if n % d = 0 then a (n / d) else 0
def deflate {K : Type} (d : Nat) (a : Nat → K) : Nat → K := fun n => a (n*d)

theorem coefficient_round_trip {K : Type} [Zero K] (d : Nat) (hd : 0 < d)
    (a : Nat → K) : deflate d (inflate d a) = a := by
  funext n
  simp [deflate, inflate, Nat.mul_mod_left, Nat.mul_div_cancel n hd]

theorem coefficient_injective {K : Type} [Zero K] (d : Nat) (hd : 0 < d) :
    Function.Injective (inflate (K := K) d) := by
  intro a b h
  have h' := congrArg (deflate d) h
  simpa only [coefficient_round_trip d hd] using h'

open Lean.Grind
def orbit {A : Type} (sigma : A → A) : Nat → A → A
  | 0, x => x
  | n+1, x => sigma (orbit sigma n x)
def trace {A : Type} [IntModule A] (sigma : A → A) : Nat → A → A
  | 0, _ => 0
  | n+1, x => trace sigma n x + orbit sigma n x

theorem orbit_trace_telescopes {A : Type} [IntModule A]
    (sigma : A → A) (hzero : sigma 0 = 0)
    (hadd : ∀ a b, sigma (a+b) = sigma a + sigma b) (n : Nat) (x : A) :
    sigma (trace sigma n x) + x = trace sigma n x + orbit sigma n x := by
  induction n with
  | zero => simp [trace, orbit, hzero, AddCommMonoid.zero_add]
  | succ n ih =>
    simp only [trace, orbit, hadd]
    grind

theorem closed_orbit_trace_invariant {A : Type} [IntModule A]
    (sigma : A → A) (hzero : sigma 0 = 0)
    (hadd : ∀ a b, sigma (a+b) = sigma a + sigma b)
    (n : Nat) (x : A) (hclosed : orbit sigma n x = x) :
    sigma (trace sigma n x) = trace sigma n x := by
  have h := orbit_trace_telescopes sigma hzero hadd n x
  rw [hclosed] at h
  grind

end CubicPullback
#print axioms CubicPullback.coefficient_round_trip
#print axioms CubicPullback.coefficient_injective
#print axioms CubicPullback.orbit_trace_telescopes
#print axioms CubicPullback.closed_orbit_trace_invariant
