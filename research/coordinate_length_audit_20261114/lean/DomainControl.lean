import Bivariate
import Mathlib.Data.ZMod.Basic

namespace CoordinateLengthAudit
noncomputable section
open Polynomial

variable (K : Type*) [Field K]

def dualScale (a : K) : Thick K 2 →ₐ[K] Thick K 2 :=
  AdjoinRoot.liftAlgHom (X^2 : Polynomial K) (Algebra.ofId K (Thick K 2))
    (a • thickRoot K 2) (by
      rw [eval₂_pow,eval₂_X]
      rw [Algebra.smul_def,mul_pow,thick_root_power,mul_zero])

@[simp] theorem dualScale_root (a : K) :
    dualScale K a (thickRoot K 2) = a • thickRoot K 2 := by
  simp [dualScale,thickRoot]

theorem dualScale_injective : Function.Injective (dualScale K) := by
  intro a b h
  have hr : thickRoot K 2 ≠ 0 := by
    simpa using thick_previous_power_ne_zero K 2 (by omega)
  apply smul_left_injective K hr
  simpa using AlgHom.congr_fun h (thickRoot K 2)

private instance primeThree : Fact (Nat.Prime 3) := ⟨by decide⟩

abbrev DualThree := Thick (ZMod 3) 2

instance : Finite DualThree := Module.finite_of_finite (ZMod 3)

instance : Finite (DualThree →ₐ[ZMod 3] DualThree) :=
  Finite.of_injective (fun f : DualThree →ₐ[ZMod 3] DualThree => (f : DualThree → DualThree))
    DFunLike.coe_injective

/-- The target-domain condition is essential, even for actual finite algebras. -/
theorem target_domain_counterexample :
    Module.finrank (ZMod 3) DualThree = 2 ∧
    3 ≤ Nat.card (DualThree →ₐ[ZMod 3] DualThree) ∧
    ¬ (Nat.card (DualThree →ₐ[ZMod 3] DualThree) ≤ Module.finrank (ZMod 3) DualThree) ∧
    ¬ IsReduced DualThree := by
  have h := Nat.card_le_card_of_injective (dualScale (ZMod 3)) (dualScale_injective (ZMod 3))
  have hc : 3 ≤ Nat.card (DualThree →ₐ[ZMod 3] DualThree) := by simpa using h
  exact ⟨thick_dimension (ZMod 3) 2,hc,by rw [thick_dimension]; omega,
    thick_not_reduced 2 (by omega)⟩

end
end CoordinateLengthAudit
