import Mathlib.Algebra.Module.Submodule.Range

namespace Quotient55
variable {R M N : Type*} [CommRing R]
  [AddCommGroup M] [Module R M] [AddCommGroup N] [Module R N]

/-- The concrete right inverse is a complete surjectivity certificate. -/
theorem evaluation_surjective (E : M →ₗ[R] N) (G : N →ₗ[R] M)
    (hEG : E.comp G = LinearMap.id) : Function.Surjective E := by
  intro y
  exact ⟨G y, LinearMap.congr_fun hEG y⟩

/-- Two complete products identify the entire kernel, including unsampled classes. -/
theorem kernel_iff_image (E : M →ₗ[R] N) (A H : M →ₗ[R] M)
    (G : N →ₗ[R] M) (hEA : E.comp A = 0)
    (hAH : A.comp H = LinearMap.id - G.comp E) (x : M) :
    E x = 0 ↔ ∃ y, A y = x := by
  constructor
  · intro hx
    refine ⟨H x, ?_⟩
    have h := LinearMap.congr_fun hAH x
    change A (H x) = x - G (E x) at h
    simpa [hx] using h
  · rintro ⟨y, rfl⟩
    have h := LinearMap.congr_fun hEA y
    change E (A y) = 0 at h
    exact h

/-- Together with surjectivity, this gives the source quotient identification.
The actual finite field models and source geometry remain native premises. -/
theorem range_eq_kernel (E : M →ₗ[R] N) (A H : M →ₗ[R] M)
    (G : N →ₗ[R] M) (hEA : E.comp A = 0)
    (hAH : A.comp H = LinearMap.id - G.comp E) :
    LinearMap.range A = LinearMap.ker E := by
  ext x
  exact (kernel_iff_image E A H G hEA hAH x).symm

/-- A full scalar product proves the scalar multiple of the module is in
image A. Thus reducing by this scalar does not shrink the cokernel. -/
theorem scalar_mem_image (A C : M →ₗ[R] M) (a : R)
    (hAC : A.comp C = a • LinearMap.id) (x : M) :
    a • x ∈ LinearMap.range A := by
  refine ⟨C x, ?_⟩
  have h := LinearMap.congr_fun hAC x
  simpa only [LinearMap.comp_apply, LinearMap.smul_apply, LinearMap.id_apply] using h
end Quotient55
