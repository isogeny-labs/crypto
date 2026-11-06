import Mathlib.Tactic

/-!
# Polynomial part of the degree-two target-section norm

This proves the factorization used in the direct elliptic norm comparison.
It does not construct the map between curves, its divisor pushforward, or
the function-field norm. Those remain geometric hypotheses in the report.
-/

set_option autoImplicit false

namespace EngePrimefieldCover

variable {R : Type*} [CommRing R]

/-- The two conjugate factors multiply to a difference of squares. -/
theorem target_section_factorization (a c t dx : R) :
    (a - c * t * dx) * (a + c * t * dx) =
      a * a - c * c * (t * t) * (dx * dx) := by
  ring

/-- Substitute the degree-two cover equation t² = x-s. -/
theorem target_section_norm_after_cover_equation
    (a c t dx x s : R) (hcover : t * t = x - s) :
    (a - c * t * dx) * (a + c * t * dx) =
      a * a - c * c * (x - s) * (dx * dx) := by
  rw [target_section_factorization, hcover]

#print axioms target_section_factorization
#print axioms target_section_norm_after_cover_equation

end EngePrimefieldCover
