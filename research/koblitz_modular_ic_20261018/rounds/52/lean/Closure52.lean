import Mathlib.Algebra.Module.Submodule.Map

namespace Closure52

variable {R M N : Type*} [CommRing R] [AddCommGroup M] [Module R M]
  [AddCommGroup N] [Module R N]

/-- A full matrix intertwining certificate makes the image module invariant.
The actual native matrices and their source bindings are external inputs. -/
theorem range_stable_of_intertwining (f : M →ₗ[R] M) (i : N →ₗ[R] M)
    (a : N →ₗ[R] N) (h : f.comp i = i.comp a) :
    ∀ x ∈ LinearMap.range i, f x ∈ LinearMap.range i := by
  intro x hx
  obtain ⟨y, rfl⟩ := hx
  exact ⟨a y, (LinearMap.congr_fun h y).symm⟩

/-- Two complete change-of-generator products certify equality of images. -/
theorem ranges_eq_of_two_products (i j : N →ₗ[R] M) (a b : N →ₗ[R] N)
    (hij : i.comp a = j) (hji : j.comp b = i) :
    LinearMap.range i = LinearMap.range j := by
  apply le_antisymm
  · rintro x ⟨y, rfl⟩
    exact ⟨b y, LinearMap.congr_fun hji y⟩
  · rintro x ⟨y, rfl⟩
    exact ⟨a y, LinearMap.congr_fun hij y⟩

def stage (f : M →ₗ[R] M) (S : Submodule R M) : ℕ → Submodule R M
  | 0 => S
  | n + 1 => stage f S n ⊔ (stage f S n).map f

/-- Every stage stays in every invariant module containing the starting one. -/
theorem stage_le_of_stable (f : M →ₗ[R] M) (S Q : Submodule R M)
    (hSQ : S ≤ Q) (hQ : ∀ x ∈ Q, f x ∈ Q) : ∀ n, stage f S n ≤ Q := by
  intro n
  induction n with
  | zero => exact hSQ
  | succ n ih =>
    apply sup_le ih
    rintro x ⟨y, hy, rfl⟩
    exact hQ y (ih hy)

/-- The checked stability test says precisely that the next adjunction adds
no elements. This does not assert that the stable module is the normalization. -/
theorem next_eq_iff_stable (f : M →ₗ[R] M) (S : Submodule R M) :
    S ⊔ S.map f = S ↔ ∀ x ∈ S, f x ∈ S := by
  constructor
  · intro h x hx
    have hi : f x ∈ S ⊔ S.map f := le_sup_right (show f x ∈ S.map f from ⟨x, hx, rfl⟩)
    simpa only [h] using hi
  · intro h
    apply le_antisymm
    · apply sup_le le_rfl
      rintro x ⟨y, hy, rfl⟩
      exact h y hy
    · exact le_sup_left

end Closure52
