import CollisionTails
import PostCollisionControl

namespace StaticHashCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit RhoFirstCollisionAudit

theorem static_hash_collision_certificate :
    (∀ (G I : Type*) [Fintype G] [Fintype I] [DecidableEq G] [DecidableEq I]
      (q : I → ℝ) (hq0 : ∀ i, 0 ≤ q i) (hq : ∑ i, q i = 1)
      (U : G → I → G) (x : G) (t : ℕ),
      (∑ h : Hash G I, (hashLaw q hq0 hq h).toReal *
        (if StaticSurvives U h x t then (1 : ℝ) else 0)) =
      (∑ w : Fin t → I, (hashLaw q hq0 hq w).toReal *
        (if SelfAvoiding U x w then (1 : ℝ) else 0))) ∧
    (∀ (p : ℕ) [NeZero p] (I : Type*) [Fintype I] [DecidableEq I]
      (q : I → ℝ) (hq0 : ∀ i, 0 ≤ q i) (hq : ∑ i, q i = 1)
      (U : ZMod p → I → ZMod p) (x : ZMod p),
      (∀ t : ℕ,
        (∑ h : Hash (ZMod p) I, (hashLaw q hq0 hq h).toReal *
          (if t < firstCollision p (staticStep U h) x then (1 : ℝ) else 0)) =
            iidSurvival q U x t) ∧
      (∑ h : Hash (ZMod p) I, (hashLaw q hq0 hq h).toReal *
        (firstCollision p (staticStep U h) x : ℝ)) =
          ∑ t : Fin p, iidSurvival q U x t.val) ∧
    (Nat.Prime 3 ∧ Function.Injective controlIncrements ∧
      (∀ i : Fin 2, controlIncrements i ≠ 0) ∧
      (∀ i : Fin 2, 0 < controlWeights i) ∧ (∑ i : Fin 2, controlWeights i) = 1 ∧
      expectation controlWeights (fun h : Hash (ZMod 3) (Fin 2) =>
        if observations controlUpdate h 0 3 = forbiddenWord then (1 : ℝ) else 0) = 0 ∧
      weight controlWeights forbiddenWord = 1 / 8 ∧
      ¬SelfAvoiding controlUpdate 0 forbiddenWord ∧
      (¬∀ w : Fin 3 → Fin 2,
        expectation controlWeights (fun h : Hash (ZMod 3) (Fin 2) =>
          if observations controlUpdate h 0 3 = w then (1 : ℝ) else 0) =
            weight controlWeights w)) := by
  refine ⟨?_, ?_, by decide, control_increments.1, control_increments.2,
    control_weights.1, control_weights.2, forbidden_static_mass, forbidden_iid_mass,
    forbidden_word_not_selfAvoiding, unrestricted_word_mass_false⟩
  · intro G I _ _ _ _ q hq0 hq U x t
    exact actual_survival_pmf q hq0 hq U x t
  · intro p _ I _ _ q hq0 hq U x
    simp only [hashLaw_real_mass]
    exact ⟨fun t => first_collision_tail_mass p q hq U x t,
      first_collision_expectation p q hq U x⟩

#print axioms static_hash_collision_certificate
end
end StaticHashCollisionAudit
