import Solutions
import Mathlib.RingTheory.AdjoinRoot

namespace CoordinateLengthAudit
noncomputable section
open Polynomial

variable (K : Type*) [Field K]

abbrev Thick (m : ℕ) := AdjoinRoot (X^m : Polynomial K)

def thickRoot (m : ℕ) : Thick K m := AdjoinRoot.root (X^m : Polynomial K)

instance thickFinite (m : ℕ) : Module.Finite K (Thick K m) :=
  (monic_X.pow m).finite_adjoinRoot

theorem thick_dimension (m : ℕ) : Module.finrank K (Thick K m) = m := by
  change Module.finrank K (Polynomial K ⧸ Ideal.span {(X^m : Polynomial K)}) = m
  rw [finrank_quotient_span_eq_natDegree]
  simp

theorem thick_scalar_length (m : ℕ) : Module.length K (Thick K m) = m := by
  rw [Module.length_eq_finrank,thick_dimension]

theorem thick_root_power (m : ℕ) : (thickRoot K m)^m = 0 := by
  have h := AdjoinRoot.mk_self (f := (X^m : Polynomial K))
  simpa [map_pow,AdjoinRoot.mk_X,thickRoot] using h

theorem thick_previous_power_ne_zero (m : ℕ) (hm : 0 < m) :
    (thickRoot K m)^(m-1) ≠ 0 := by
  have h := AdjoinRoot.mk_ne_zero_of_natDegree_lt (monic_X.pow m : (X^m : Polynomial K).Monic)
    (g := X^(m-1)) (by simp) (by simp; omega)
  simpa [map_pow,AdjoinRoot.mk_X,thickRoot] using h

variable {K} {L : Type*} [Field L] [Algebra K L]

def thickZeroHom (m : ℕ) (hm : 0 < m) : Thick K m →ₐ[K] L :=
  AdjoinRoot.liftAlgHom (X^m : Polynomial K) (Algebra.ofId K L) 0
    (by simp [Nat.ne_of_gt hm])

theorem thick_hom_root_zero (m : ℕ) (f : Thick K m →ₐ[K] L) :
    f (thickRoot K m) = 0 := by
  have h : (f (thickRoot K m))^m = 0 := by rw [← map_pow,thick_root_power,map_zero]
  exact eq_zero_of_pow_eq_zero h

theorem thick_hom_unique (m : ℕ) (hm : 0 < m) (f : Thick K m →ₐ[K] L) :
    f = thickZeroHom m hm := by
  apply AdjoinRoot.algHom_ext
  change f (thickRoot K m) = thickZeroHom m hm (thickRoot K m)
  rw [thick_hom_root_zero]
  simp [thickZeroHom,thickRoot]

theorem thick_hom_card (m : ℕ) (hm : 0 < m) : Nat.card (Thick K m →ₐ[K] L) = 1 := by
  let : Unique (Thick K m →ₐ[K] L) :=
    ⟨⟨thickZeroHom m hm⟩,thick_hom_unique m hm⟩
  exact Nat.card_unique

def thickOneEquiv : Thick K 1 ≃ₐ[K] K :=
  AlgEquiv.ofAlgHom (thickZeroHom 1 (by omega)) (Algebra.ofId K (Thick K 1))
    (by
      apply AlgHom.ext
      intro a
      change thickZeroHom 1 (by omega) (algebraMap K (Thick K 1) a) = a
      rw [AlgHom.commutes]
      rfl)
    (by
      apply AdjoinRoot.algHom_ext
      change algebraMap K (Thick K 1) (thickZeroHom 1 (by omega) (thickRoot K 1)) = thickRoot K 1
      have hr : thickRoot K 1 = 0 := by simpa using thick_root_power K 1
      rw [hr,map_zero,map_zero])

theorem thick_not_reduced (m : ℕ) (hm : 1 < m) : ¬ IsReduced (Thick K m) := by
  intro h
  let : IsReduced (Thick K m) := h
  have hz : thickRoot K m = 0 := (show IsNilpotent (thickRoot K m) from ⟨m,thick_root_power K m⟩).eq_zero
  have hne := thick_previous_power_ne_zero K m (by omega)
  exact hne (by rw [hz,zero_pow (by omega : m-1 ≠ 0)])

end
end CoordinateLengthAudit
