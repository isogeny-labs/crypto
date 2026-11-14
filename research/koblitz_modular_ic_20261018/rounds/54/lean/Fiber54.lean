import Mathlib.Algebra.Module.Submodule.Range
import Mathlib.Tactic.Abel

namespace Fiber54

variable {R M N : Type*} [CommRing R]
  [AddCommGroup M] [Module R M] [AddCommGroup N] [Module R N]

/-- The full F K = I product makes the actual basis map injective.
Concrete arrays and the QR presentation remain native premises. -/
theorem basisMap_injective (K : N →ₗ[R] M) (F : M →ₗ[R] N)
    (h : F.comp K = LinearMap.id) : Function.Injective K := by
  intro x y hxy
  have hx := LinearMap.congr_fun h x
  have hy := LinearMap.congr_fun h y
  change F (K x) = x at hx
  change F (K y) = y at hy
  rw [← hx, ← hy, hxy]

/-- The other full product recovers every element of the projector component. -/
theorem recover_fixed (K : N →ₗ[R] M) (F : M →ₗ[R] N)
    (E : M →ₗ[R] M) (h : K.comp F = E) (x : M) (hx : E x = x) :
    K (F x) = x := (LinearMap.congr_fun h x).trans hx

/-- Both products identify exactly the component, not just an inclusion. -/
theorem range_iff_fixed (K : N →ₗ[R] M) (F : M →ₗ[R] N)
    (E : M →ₗ[R] M) (hFK : F.comp K = LinearMap.id)
    (hKF : K.comp F = E) (x : M) : x ∈ LinearMap.range K ↔ E x = x := by
  constructor
  · rintro ⟨y, rfl⟩
    have hy := LinearMap.congr_fun hFK y
    change F (K y) = y at hy
    calc
      E (K y) = K (F (K y)) := (LinearMap.congr_fun hKF (K y)).symm
      _ = K y := by rw [hy]
  · intro hx
    exact ⟨F x, recover_fixed K F E hKF x hx⟩

def orbitTrace (t : M →ₗ[R] M) (x : M) : M :=
  x + t x + t (t x) + t (t (t x))

/-- On an order-four orbit the trace is fixed by the square. In the native
fiber the Frobenius square is tested against the deck action separately. -/
theorem orbitTrace_fixed_by_square (t : M →ₗ[R] M) (x : M)
    (h4 : t (t (t (t x))) = x) :
    t (t (orbitTrace t x)) = orbitTrace t x := by
  simp only [orbitTrace, map_add, h4]
  abel

/-- A sign-minus projection kills these orbit traces when sign is t squared.
This is conditional; no intended Jacobian-factor action is asserted here. -/
theorem anti_orbitTrace_zero (t : M →ₗ[R] M) (x : M)
    (h4 : t (t (t (t x))) = x) :
    orbitTrace t x - t (t (orbitTrace t x)) = 0 := by
  rw [orbitTrace_fixed_by_square t x h4]
  exact sub_self _

end Fiber54
