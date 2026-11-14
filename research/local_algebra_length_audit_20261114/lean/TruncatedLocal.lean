import PowerBasisLength
import Bivariate
import Mathlib.RingTheory.LocalRing.Basic
import Mathlib.RingTheory.Nilpotent.Basic
import Mathlib.Algebra.Polynomial.Inductions

namespace LocalAlgebraLengthAudit
noncomputable section
open CoordinateLengthAudit Polynomial
variable (K : Type*) [Field K]

def thickPowerBasis (m : ℕ) : PowerBasis K (Thick K m) :=
  AdjoinRoot.powerBasis' (monic_X.pow m : (X ^ m : Polynomial K).Monic)

theorem thickPowerBasis_dim (m : ℕ) : (thickPowerBasis K m).dim = m := by
  simp [thickPowerBasis, AdjoinRoot.powerBasis'_dim]

theorem thickPowerBasis_nilpotent (m : ℕ) :
    (thickPowerBasis K m).gen ^ (thickPowerBasis K m).dim = 0 := by
  rw [thickPowerBasis_dim]
  exact thick_root_power K m

def biPowerBasis (m : ℕ) : PowerBasis K (BiAlgebra K m) :=
  (thickPowerBasis K m).map (biEquiv K m).symm

theorem biPowerBasis_dim (m : ℕ) : (biPowerBasis K m).dim = m := by
  simp only [biPowerBasis, PowerBasis.map_dim, thickPowerBasis_dim]

theorem biPowerBasis_nilpotent (m : ℕ) :
    (biPowerBasis K m).gen ^ (biPowerBasis K m).dim = 0 := by
  simp only [biPowerBasis, PowerBasis.map_gen, PowerBasis.map_dim, ← map_pow,
    thickPowerBasis_nilpotent, map_zero]

theorem thick_ring_length (m : ℕ) : Module.length (Thick K m) (Thick K m) = m := by
  simpa only [thickPowerBasis_dim] using
    nilpotent_powerBasis_ring_length (thickPowerBasis K m) (thickPowerBasis_nilpotent K m)

theorem bi_ring_length (m : ℕ) : Module.length (BiAlgebra K m) (BiAlgebra K m) = m := by
  simpa only [biPowerBasis_dim] using
    nilpotent_powerBasis_ring_length (biPowerBasis K m) (biPowerBasis_nilpotent K m)

theorem thick_unit_or_nilpotent (m : ℕ) (z : Thick K m) : IsUnit z ∨ IsNilpotent z := by
  obtain ⟨p, rfl⟩ := AdjoinRoot.mk_surjective z
  let r : Thick K m := thickRoot K m * AdjoinRoot.mk (X ^ m : Polynomial K) p.divX
  have hn : IsNilpotent r := ⟨m, by
    dsimp [r]
    rw [mul_pow, thick_root_power, zero_mul]⟩
  have he : AdjoinRoot.mk (X ^ m : Polynomial K) p =
      r + algebraMap K (Thick K m) (p.coeff 0) := by
    calc
      _ = AdjoinRoot.mk (X ^ m : Polynomial K) (X * p.divX + C (p.coeff 0)) :=
        congrArg _ (X_mul_divX_add p).symm
      _ = _ := by
        simp only [map_add, map_mul, AdjoinRoot.mk_X, AdjoinRoot.mk_C, r, thickRoot,
          AdjoinRoot.algebraMap_eq]
  by_cases hc : p.coeff 0 = 0
  · right
    simpa only [he, hc, map_zero, add_zero] using hn
  · left
    rw [he]
    exact hn.isUnit_add_right_of_commute
      ((isUnit_iff_ne_zero.mpr hc).map (algebraMap K (Thick K m))) (Commute.all _ _)

theorem thick_local (m : ℕ) (hm : 0 < m) : IsLocalRing (Thick K m) := by
  let : Nontrivial (Thick K m) :=
    ⟨⟨(thickRoot K m) ^ (m - 1), 0, thick_previous_power_ne_zero K m hm⟩⟩
  apply IsLocalRing.of_isUnit_or_isUnit_one_sub_self
  intro z
  rcases thick_unit_or_nilpotent K m z with hu | hn
  · exact Or.inl hu
  · exact Or.inr hn.isUnit_one_sub

theorem bi_local (m : ℕ) (hm : 0 < m) : IsLocalRing (BiAlgebra K m) := by
  let : IsLocalRing (Thick K m) := thick_local K m hm
  let : Nontrivial (BiAlgebra K m) := (biEquiv K m).symm.injective.nontrivial
  exact IsLocalRing.of_surjective' (biEquiv K m).symm.toRingHom (biEquiv K m).symm.surjective

theorem thick_zero_not_local : ¬IsLocalRing (Thick K 0) := by
  intro h
  let : IsLocalRing (Thick K 0) := h
  have hz : (1 : Thick K 0) = 0 := by simpa only [pow_zero] using thick_root_power K 0
  exact one_ne_zero hz

theorem bi_zero_not_local : ¬IsLocalRing (BiAlgebra K 0) := by
  intro h
  let : IsLocalRing (BiAlgebra K 0) := h
  have hz : (1 : BiAlgebra K 0) = 0 := by simpa only [pow_zero] using bi_coord_power K 0
  exact one_ne_zero hz

theorem unbounded_ring_length (b : ℕ) :
    ∃ m : ℕ, 1 < m ∧ (b : ℕ∞) < Module.length (BiAlgebra K m) (BiAlgebra K m) ∧
      IsLocalRing (BiAlgebra K m) := by
  refine ⟨b + 2, by omega, ?_, bi_local K (b + 2) (by omega)⟩
  rw [bi_ring_length]
  exact_mod_cast (by omega : b < b + 2)

#print axioms thick_ring_length
#print axioms bi_ring_length
#print axioms thick_local
#print axioms bi_local
#print axioms unbounded_ring_length
end
end LocalAlgebraLengthAudit
