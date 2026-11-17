import GeneralCollisionCertificate

namespace SidonRhoCounterfamilyAudit

/-- Explicit first twenty A005282 values, checked here as a finite table.
No greedy-generation or infinite-sequence property is assumed. -/
def base : Fin 20 → ℕ :=
  ![1, 2, 4, 8, 13, 21, 31, 45, 66, 81, 97, 123, 148, 182, 204, 252, 290, 361, 401, 475]

def stepNat (i : Fin 20) : ℕ := base i + 476

theorem base_injective : Function.Injective base := by decide +kernel

theorem base_sidon : ∀ i j k l : Fin 20,
    base i + base j = base k + base l →
      (i = k ∧ j = l) ∨ (i = l ∧ j = k) := by
  intro i
  fin_cases i <;> decide +kernel

theorem step_bounds : ∀ i, 477 ≤ stepNat i ∧ stepNat i ≤ 951 := by decide +kernel

theorem step_injective : Function.Injective stepNat := by
  intro i j h
  apply base_injective
  unfold stepNat at h
  omega

theorem step_sidon (i j k l : Fin 20)
    (h : stepNat i + stepNat j = stepNat k + stepNat l) :
    (i = k ∧ j = l) ∨ (i = l ∧ j = k) := by
  apply base_sidon
  unfold stepNat at h
  omega

def increments (p : ℕ) (i : Fin 20) : ZMod p := (stepNat i : ZMod p)

theorem cast_injective_below (p a b : ℕ) (ha : a < p) (hb : b < p)
    (h : (a : ZMod p) = (b : ZMod p)) : a = b := by
  have hv := congrArg ZMod.val h
  simpa only [ZMod.val_natCast_of_lt ha, ZMod.val_natCast_of_lt hb] using hv

theorem generic (p : ℕ) (hp : 1902 < p) :
    TwoStepCollisionAudit.DegreeTwoGeneric (increments p) := by
  refine ⟨?_, ?_, ?_, ?_, ?_⟩
  · intro i j h
    apply step_injective
    exact cast_injective_below p _ _ (by have := (step_bounds i).2; omega)
      (by have := (step_bounds j).2; omega) h
  · intro i h
    have hi := step_bounds i
    have he := cast_injective_below p (stepNat i) 0 (by omega) (by omega)
      (by simpa only [increments, Nat.cast_zero] using h)
    omega
  · intro i j h
    have hi := step_bounds i
    have hj := step_bounds j
    have he := cast_injective_below p (stepNat i + stepNat j) 0 (by omega) (by omega)
      (by simpa only [increments, Nat.cast_add, Nat.cast_zero] using h)
    omega
  · intro i j k h
    have hi := step_bounds i
    have hj := step_bounds j
    have hk := step_bounds k
    have he := cast_injective_below p (stepNat i + stepNat j) (stepNat k)
      (by omega) (by omega) (by simpa only [increments, Nat.cast_add] using h)
    omega
  · intro i j k l h
    have hi := step_bounds i
    have hj := step_bounds j
    have hk := step_bounds k
    have hl := step_bounds l
    apply step_sidon
    exact cast_injective_below p _ _ (by omega) (by omega)
      (by simpa only [increments, Nat.cast_add] using h)

#print axioms base_sidon
#print axioms generic
end SidonRhoCounterfamilyAudit
