import Std.Tactic

/-!
Algebraic dependencies of the projected-query law. These do not establish an
elliptic curve's cardinality, complementary subgroups, a sampler, or uniformity.
The hypotheses are explicit theorem parameters, not custom axioms.
-/
namespace ICProjectedQuery
open Lean.Grind

theorem kernel_translation_projects_away {A : Type}
    (add : A → A → A) (zero : A) (project : A → A)
    (target shift kernel : A)
    (hadd : ∀ x y, project (add x y) = add (project x) (project y))
    (hzero : ∀ x, add x zero = x)
    (hkernel : project kernel = zero) :
    project (add (add target shift) kernel) =
      add (project target) (project shift) := by
  rw [hadd, hkernel, hzero, hadd]

theorem recover_scalar_from_projected_sum {F : Type} [CommRing F]
    (h hInv d k logSum : F)
    (hinverse : hInv * h = 1)
    (hlog : logSum = h * (d + k)) :
    hInv * logSum - k = d := by
  grind

theorem known_setup_right_hand_side {F : Type} [CommRing F]
    (h hInv k logSum : F)
    (hinverse : hInv * h = 1)
    (hlog : logSum = h * k) :
    hInv * logSum = k := by
  grind

theorem projected_identity_hit_scalar {F : Type} [CommRing F]
    (h hInv d k : F)
    (hinverse : hInv * h = 1)
    (hzero : h * (d + k) = 0) :
    d = -k := by
  grind

theorem certified_coordinates_give_unique_decomposition {A B E : Type}
    (combine : A → B → E) (left : E → A) (right : E → B)
    (hleft : ∀ a b, left (combine a b) = a)
    (hright : ∀ a b, right (combine a b) = b)
    (a a' : A) (b b' : B)
    (heq : combine a b = combine a' b') :
    a = a' ∧ b = b' := by
  constructor
  · have h := congrArg left heq
    simpa only [hleft] using h
  · have h := congrArg right heq
    simpa only [hright] using h

#print axioms kernel_translation_projects_away
#print axioms recover_scalar_from_projected_sum
#print axioms known_setup_right_hand_side
#print axioms projected_identity_hit_scalar
#print axioms certified_coordinates_give_unique_decomposition
end ICProjectedQuery
