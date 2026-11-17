import Survival

namespace StaticHashCollisionAudit
noncomputable section
open scoped BigOperators
open WalkWeightedGeneralAudit

def controlIncrements (i : Fin 2) : ZMod 3 := i.val + 1

def controlUpdate (x : ZMod 3) (i : Fin 2) : ZMod 3 := x + controlIncrements i

def controlWeights (_ : Fin 2) : ℝ := 1 / 2

def forbiddenWord : Fin 3 → Fin 2 := ![0, 1, 1]

theorem control_increments : Function.Injective controlIncrements ∧
    (∀ i : Fin 2, controlIncrements i ≠ 0) := by decide +kernel

theorem control_weights : (∀ i : Fin 2, 0 < controlWeights i) ∧
    (∑ i : Fin 2, controlWeights i) = 1 := by
  constructor
  · intro i
    norm_num [controlWeights]
  · norm_num [controlWeights, Fin.sum_univ_two]

/-- The first two labels0,1 return to state0; the next label cannot be1
because the static label at state0 was already0. -/
theorem forbidden_word_impossible :
    ∀ h : ZMod 3 → Fin 2, observations controlUpdate h 0 3 ≠ forbiddenWord := by
  decide +kernel

theorem forbidden_static_mass :
    expectation controlWeights (fun h : Hash (ZMod 3) (Fin 2) =>
      if observations controlUpdate h 0 3 = forbiddenWord then (1 : ℝ) else 0) = 0 := by
  simp [expectation, forbidden_word_impossible]

theorem forbidden_iid_mass : weight controlWeights forbiddenWord = 1 / 8 := by
  norm_num [weight, controlWeights, Fin.prod_univ_succ]

theorem forbidden_word_not_selfAvoiding : ¬SelfAvoiding controlUpdate 0 forbiddenWord := by
  decide +kernel

/-- Direct disproof of deleting the self-avoidance condition from the
chronological word-mass identity, with prime group/distinct nonzero steps. -/
theorem unrestricted_word_mass_false :
    ¬∀ w : Fin 3 → Fin 2,
      expectation controlWeights (fun h : Hash (ZMod 3) (Fin 2) =>
        if observations controlUpdate h 0 3 = w then (1 : ℝ) else 0) =
          weight controlWeights w := by
  intro he
  have hc := he forbiddenWord
  rw [forbidden_static_mass, forbidden_iid_mass] at hc
  norm_num at hc

#print axioms forbidden_word_impossible
#print axioms unrestricted_word_mass_false
end
end StaticHashCollisionAudit
