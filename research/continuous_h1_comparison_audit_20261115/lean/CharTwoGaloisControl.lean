module
public import PerfectCocyclePairing
public import Mathlib.Data.ZMod.Basic

@[expose] public section
namespace GaloisCharacterPairingAudit
noncomputable section
open ProcyclicGaloisCocycleAudit

abbrev TwoTarget := Multiplicative (ZMod 2)
local instance : TopologicalSpace TwoTarget := ⊥
local instance : DiscreteTopology TwoTarget := ⟨rfl⟩
local instance : TopologicalSpace (MulAut TwoTarget) := ⊥
local instance : DiscreteTopology (MulAut TwoTarget) := ⟨rfl⟩

variable (K : Type*) [Field K] [Fintype K]

def twoAction : Gamma K →ₜ* MulAut TwoTarget := 1
abbrev TwoCharacters := galoisCharacters K TwoTarget (twoAction K)
abbrev TwoClasses := CocycleClasses K TwoTarget (twoAction K)

def missingTwoCoinvariant : Coinvariants K TwoTarget (twoAction K) :=
  QuotientGroup.mk (Multiplicative.ofAdd (1 : ZMod 2))

theorem missing_two_coinvariant_ne_one : missingTwoCoinvariant K ≠ 1 := by
  intro h
  have hm := (QuotientGroup.eq_one_iff (Multiplicative.ofAdd (1 : ZMod 2))).mp h
  obtain ⟨m, he⟩ := hm
  change m / m = Multiplicative.ofAdd (1 : ZMod 2) at he
  have hh : (1 : TwoTarget) = Multiplicative.ofAdd (1 : ZMod 2) := by simpa using he
  exact (by decide +kernel : (1 : TwoTarget) ≠ Multiplicative.ofAdd (1 : ZMod 2)) hh

def missingTwoClass : TwoClasses K :=
  (classEvaluationEquiv K TwoTarget (twoAction K)).symm (missingTwoCoinvariant K)

theorem missing_two_class_ne_one : missingTwoClass K ≠ 1 := by
  intro h
  apply missing_two_coinvariant_ne_one K
  have he := congrArg (classEvaluationEquiv K TwoTarget (twoAction K)) h
  simpa only [missingTwoClass, MulEquiv.apply_symm_apply, map_one] using he

variable [CharP K 2]

omit [Fintype K] in
/-- Credited prime-power-root argument from PowerTargets, applied in the actual
algebraic closure. This does not replace its units by a numerical model. -/
theorem closure_square_root_one (u : (Omega K)ˣ) (hu : u ^ 2 = 1) : u = 1 := by
  apply Units.ext
  apply iterateFrobenius_inj (Omega K) 2 1
  change (u : Omega K) ^ (2 ^ 1) = (1 : Omega K) ^ (2 ^ 1)
  simpa only [pow_one, Units.val_pow_eq_pow_val, Units.val_one, one_pow]
    using congrArg Units.val hu

theorem two_characters_trivial (χ : TwoCharacters K) : χ = 1 := by
  apply Subtype.ext
  apply MonoidHom.ext
  intro m
  change χ.val m = 1
  apply closure_square_root_one K
  rw [← map_pow]
  have hm : m ^ 2 = 1 := (by decide +kernel : ∀ a : TwoTarget, a ^ 2 = 1) m
  rw [hm, map_one]

/-- On the same actual Galois-character and all-continuous-cocycle-class domains,
deleting the annihilation hypothesis cannot yield a right-nondegenerate pairing. -/
theorem annihilation_guard_needed :
    ¬ ∃ e : TwoCharacters K →* (TwoClasses K →* Kˣ),
      ∀ c : TwoClasses K, (∀ χ : TwoCharacters K, e χ c = 1) → c = 1 := by
  rintro ⟨e, he⟩
  apply missing_two_class_ne_one K
  apply he
  intro χ
  rw [two_characters_trivial K χ, map_one]
  rfl

theorem two_annihilation_false :
    ¬ ∀ m : TwoTarget, m ^ (Fintype.card K - 1) = 1 := by
  intro hM
  apply annihilation_guard_needed K
  let e : TwoCharacters K →* (TwoClasses K →* Kˣ) :=
    (leftDuality K TwoTarget (twoAction K) hM).toMonoidHom
  refine ⟨e, ?_⟩
  intro c hc
  exact pairing_right_nondegenerate K TwoTarget (twoAction K) hM c hc

#print axioms missing_two_class_ne_one
#print axioms closure_square_root_one
#print axioms two_characters_trivial
#print axioms annihilation_guard_needed
#print axioms two_annihilation_false
end
end GaloisCharacterPairingAudit
