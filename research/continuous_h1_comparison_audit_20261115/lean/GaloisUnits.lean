module
public import GaloisGenerator
public import Mathlib.Algebra.Group.Units.Equiv

@[expose] public section
namespace GaloisCharacterPairingAudit
noncomputable section
open ProcyclicGaloisCocycleAudit

variable (K : Type*) [Field K] [Fintype K]

def baseUnitMap : Kˣ →* (Omega K)ˣ := Units.map (algebraMap K (Omega K)).toMonoidHom

omit [Fintype K] in
theorem baseUnitMap_injective : Function.Injective (baseUnitMap K) :=
  Units.map_injective (algebraMap K (Omega K)).injective

/-- The actual Galois action on units of the actual algebraic closure. -/
def unitGalois : Gamma K →* MulAut (Omega K)ˣ where
  toFun σ := Units.mapEquiv σ.toRingEquiv.toMulEquiv
  map_one' := by ext u; rfl
  map_mul' _ _ := by ext u; rfl

omit [Fintype K] in
theorem unitGalois_base (σ : Gamma K) (u : Kˣ) :
    unitGalois K σ (baseUnitMap K u) = baseUnitMap K u := by
  apply Units.ext
  exact σ.commutes (u : K)

/-- Root values descend to actual base-field units; descent is not a premise. -/
theorem root_unit_from_base (u : (Omega K)ˣ) (hu : u ^ (Fintype.card K - 1) = 1) :
    ∃ v : Kˣ, baseUnitMap K v = u := by
  have hv : (u : Omega K) ^ (Fintype.card K - 1) = 1 := congrArg Units.val hu
  have hq : Fintype.card K - 1 + 1 = Fintype.card K :=
    Nat.sub_add_cancel (Nat.succ_le_of_lt Fintype.card_pos)
  have hp : (u : Omega K) ^ Fintype.card K = u := by
    nth_rw 1 [← hq]
    rw [pow_add, hv, pow_one, one_mul]
  obtain ⟨a, ha⟩ := (frobenius_fixed_iff K (u : Omega K)).mp hp
  have ha0 : a ≠ 0 := by
    intro hz
    have hh : (u : Omega K) = 0 := by simpa [hz] using ha.symm
    exact u.ne_zero hh
  refine ⟨Units.mk0 a ha0, ?_⟩
  apply Units.ext
  exact ha

variable (M : Type*) [CommGroup M]

def baseCharacterMap : (M →* Kˣ) →* (M →* (Omega K)ˣ) where
  toFun χ := (baseUnitMap K).comp χ
  map_one' := by ext m; simp
  map_mul' _ _ := by ext m; simp

omit [Fintype K] in
theorem baseCharacterMap_injective : Function.Injective (baseCharacterMap K M) := by
  intro χ ψ h
  apply MonoidHom.ext
  intro m
  exact baseUnitMap_injective K (congrArg (fun f : M →* (Omega K)ˣ => f m) h)

theorem baseCharacterMap_surjective (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    Function.Surjective (baseCharacterMap K M) := by
  intro χ
  have hvalue (m : M) : ∃ u : Kˣ, baseUnitMap K u = χ m := by
    apply root_unit_from_base K
    rw [← map_pow, hM, map_one]
  let f (m : M) : Kˣ := Classical.choose (hvalue m)
  have hf (m : M) : baseUnitMap K (f m) = χ m := Classical.choose_spec (hvalue m)
  let ψ : M →* Kˣ := {
    toFun := f
    map_one' := by
      apply baseUnitMap_injective K
      rw [hf, map_one, map_one]
    map_mul' := by
      intro a b
      apply baseUnitMap_injective K
      calc
        baseUnitMap K (f (a * b)) = χ (a * b) := hf _
        _ = χ a * χ b := χ.map_mul _ _
        _ = baseUnitMap K (f a) * baseUnitMap K (f b) := by rw [hf, hf]
        _ = baseUnitMap K (f a * f b) := (map_mul (baseUnitMap K) _ _).symm }
  exact ⟨ψ, MonoidHom.ext hf⟩

def baseCharacterEquiv (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1) :
    (M →* Kˣ) ≃* (M →* (Omega K)ˣ) :=
  MulEquiv.ofBijective (baseCharacterMap K M)
    ⟨baseCharacterMap_injective K M, baseCharacterMap_surjective K M hM⟩

theorem character_descent_value (hM : ∀ m : M, m ^ (Fintype.card K - 1) = 1)
    (χ : M →* (Omega K)ˣ) (m : M) :
    baseUnitMap K ((baseCharacterEquiv K M hM).symm χ m) = χ m := by
  exact congrArg (fun f : M →* (Omega K)ˣ => f m)
    ((baseCharacterEquiv K M hM).apply_symm_apply χ)

#print axioms root_unit_from_base
#print axioms baseCharacterEquiv
#print axioms character_descent_value
end
end GaloisCharacterPairingAudit
