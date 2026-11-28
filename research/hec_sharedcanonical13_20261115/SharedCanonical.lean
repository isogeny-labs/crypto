import Mathlib
namespace SharedCanonical
/-- The lifted equations preserve exactly the original equations. -/
theorem equations_iff {R I : Type*} [AddGroup R] (a : I → R) (h : R) :
    (∀ i, a i - h = 0) ↔ ∃ z : R, z = h ∧ ∀ i, a i - z = 0 := by
  constructor
  · intro ha
    exact ⟨h, rfl, ha⟩
  · rintro ⟨z, rfl, hz⟩
    exact hz
/-- The auxiliary coordinate is uniquely determined by the projected point. -/
theorem unique_lift {K V I : Type*} [Field K] [AddCommGroup V] [Module K V]
    (rows : I → V →ₗ[K] K) (h : V →ₗ[K] K) (x : V)
    (hx : ∀ i, rows i x - h x = 0) :
    ∃! z : K, z = h x ∧ ∀ i, rows i x - z = 0 := by
  refine ⟨h x, ⟨rfl, hx⟩, ?_⟩
  intro z hz
  exact hz.1
end SharedCanonical
#print axioms SharedCanonical.equations_iff
#print axioms SharedCanonical.unique_lift
