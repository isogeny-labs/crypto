import Mathlib.Algebra.Module.LinearMap.Basic

/-! Cancellation is performed in the integral source domain before reducing
at q=0. Native arrays and local geometry are not imported as Lean evidence. -/
namespace QColonSyzygy34
variable {R M V : Type*} [CommRing R] [IsDomain R]
    [AddCommGroup M] [Module R M] [AddCommGroup V] [Module R V]

theorem parameter_cancellation (q : R) (hq : q ≠ 0)
    (F : M →ₗ[R] R) (g : M) (h : F (q • g) = 0) : F g = 0 := by
  have hh : q * F g = 0 := by
    simpa only [map_smul, smul_eq_mul] using h
  exact (mul_eq_zero.mp hh).resolve_left hq

theorem lifted_relation_syzygy (q : R) (hq : q ≠ 0)
    (F : M →ₗ[R] R) (S : V →ₗ[R] M) (g : M) (v : V)
    (hFS : ∀ w, F (S w) = 0) (hdiv : q • g = S v) : F g = 0 := by
  apply parameter_cancellation q hq F g
  rw [hdiv]
  exact hFS v

end QColonSyzygy34
