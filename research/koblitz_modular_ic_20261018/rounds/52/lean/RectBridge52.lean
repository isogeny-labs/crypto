import Mathlib.Algebra.Module.Submodule.Range

namespace RectBridge52

/-- Both full generator products identify the image submodules, even when the
numbers of generators differ. In the native closure step these domains are
R^64 and R^32. The actual matrices/products remain external inputs. -/
theorem ranges_eq_of_two_rectangular_products
    {R M N P : Type*} [CommRing R]
    [AddCommGroup M] [Module R M] [AddCommGroup N] [Module R N]
    [AddCommGroup P] [Module R P]
    (i : N →ₗ[R] M) (j : P →ₗ[R] M)
    (a : P →ₗ[R] N) (b : N →ₗ[R] P)
    (hij : i.comp a = j) (hji : j.comp b = i) :
    LinearMap.range i = LinearMap.range j := by
  apply le_antisymm
  · rintro x ⟨y, rfl⟩
    exact ⟨b y, LinearMap.congr_fun hji y⟩
  · rintro x ⟨y, rfl⟩
    exact ⟨a y, LinearMap.congr_fun hij y⟩

end RectBridge52
