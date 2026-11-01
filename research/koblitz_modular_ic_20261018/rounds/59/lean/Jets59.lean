import Mathlib.Algebra.Group.Commute.Basic
import Mathlib.Algebra.Module.Submodule.Range
import Mathlib.Data.Matrix.Mul
import Mathlib.Tactic.LinearCombination

namespace Jets59

/-- The global commuting annihilator product gives its fourth-power product.
For the source operators f is the scalar multiplication endomorphism. -/
theorem fourth_annihilator {R : Type*} [Monoid R] (A C f : R)
    (hcomm : Commute A C) (hprod : A * C = f) : A ^ 4 * C ^ 4 = f ^ 4 := by
  calc
    A ^ 4 * C ^ 4 = (A * C) ^ 4 := (hcomm.mul_pow 4).symm
    _ = f ^ 4 := by rw [hprod]

/-- A residue idempotent lifts by fourth power through a fourth-nilpotent
error in characteristic two. Actual source/word errors are native premises. -/
theorem fourth_idempotent {R : Type*} [CommRing R] (e : R)
    (h2 : (2 : R) = 0) (herr : (e ^ 2 - e) ^ 4 = 0) :
    (e ^ 4) ^ 2 = e ^ 4 := by
  linear_combination herr + (2*e^7-3*e^6+2*e^5-e^4) * h2

/-- Orthogonality lifts along the same fourth-power nilpotent bound. -/
theorem fourth_orthogonal {R : Type*} [CommRing R] (e f : R)
    (h : (e * f) ^ 4 = 0) : e ^ 4 * f ^ 4 = 0 := by
  rw [← mul_pow, h]

/-- The checked invertible Jordan model transfers fourth nilpotence to A. -/
theorem fourth_nilpotent_model {R M N : Type*} [CommRing R]
    [AddCommGroup M] [Module R M] [AddCommGroup N] [Module R N]
    (A : M →ₗ[R] M) (T : N →ₗ[R] N) (J : N →ₗ[R] M)
    (hsurj : Function.Surjective J) (hAJ : ∀ x, A (J x) = J (T x))
    (hT : ∀ x, T (T (T (T x))) = 0) : ∀ x, A (A (A (A x))) = 0 := by
  intro x
  obtain ⟨y, rfl⟩ := hsurj x
  rw [hAJ, hAJ, hAJ, hAJ, hT, map_zero]

/-- Every native left inverse remains a left inverse after coefficient
extension, so the eighteen exclusions also hold over F32. -/
theorem left_inverse_basechange {R S m n : Type*} [Semiring R] [Semiring S]
    [Fintype m] [Fintype n] [DecidableEq n] (φ : R →+* S)
    (C : Matrix m n R) (H : Matrix n m R) (h : H * C = 1) :
    H.map φ * C.map φ = 1 := by
  rw [← Matrix.map_mul, h]
  exact Matrix.map_one φ (map_zero φ) (map_one φ)

end Jets59
