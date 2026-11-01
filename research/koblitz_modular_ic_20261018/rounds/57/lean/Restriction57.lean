import Mathlib.Algebra.Module.Submodule.Range

namespace Restriction57

variable {F U V W : Type*} [Field F]
  [AddCommGroup U] [Module F U] [AddCommGroup V] [Module F V]
  [AddCommGroup W] [Module F W]

/-- The native C K=0 and K R+H C=I products certify every kernel vector,
not only the supplied columns. The local RR interpretation is a separate premise. -/
theorem complete_kernel (C : U →ₗ[F] W) (K : V →ₗ[F] U)
    (R : U →ₗ[F] V) (H : W →ₗ[F] U)
    (hCK : ∀ v, C (K v) = 0)
    (hsplit : ∀ u, K (R u) + H (C u) = u) :
    LinearMap.ker C = LinearMap.range K := by
  ext u
  constructor
  · intro hu
    have hzero : C u = 0 := hu
    have hrecover : K (R u) = u := by simpa [hzero] using hsplit u
    exact ⟨R u, hrecover⟩
  · rintro ⟨v, rfl⟩
    exact hCK v

/-- R K=I certifies uniqueness of the recovered kernel coordinates. -/
theorem kernel_coordinates_injective (K : V →ₗ[F] U) (R : U →ₗ[F] V)
    (hRK : ∀ v, R (K v) = v) : Function.Injective K := by
  intro x y h
  have he := congrArg R h
  simpa only [hRK] using he

/-- An explicit left inverse of the coefficient row transformation preserves
all coefficient-kernel vectors. Native R57 also checks the right inverse. -/
theorem row_change_kernel (C : U →ₗ[F] W) (P Q : W →ₗ[F] W)
    (hQP : ∀ w, Q (P w) = w) :
    LinearMap.ker (P.comp C) = LinearMap.ker C := by
  ext u
  constructor
  · intro hu
    have hzero : P (C u) = 0 := hu
    have he := congrArg Q hzero
    change C u = 0
    simpa only [hQP, map_zero] using he
  · intro hu
    have hzero : C u = 0 := hu
    change P (C u) = 0
    simp only [hzero, map_zero]

/-- A stable column space inherits the actual involution. -/
theorem restricted_involution (S : U →ₗ[F] U) (K : V →ₗ[F] U)
    (T : V →ₗ[F] V) (hinj : Function.Injective K)
    (hst : ∀ v, S (K v) = K (T v))
    (hsq : ∀ u, S (S u) = u) : ∀ v, T (T v) = v := by
  intro v
  apply hinj
  calc
    K (T (T v)) = S (K (T v)) := (hst (T v)).symm
    _ = S (S (K v)) := by rw [← hst v]
    _ = K v := hsq (K v)

end Restriction57
