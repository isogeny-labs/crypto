module
public import CurveDomain
public import Mathlib.GroupTheory.Coset.Card
public import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots

@[expose] public section
namespace CompositeTateDomainAudit
noncomputable section
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : CommRing F := (inferInstance : Field F).toCommRing

def negativeUnit : Fˣ := ⟨-1, -1, by decide +kernel, by decide +kernel⟩
def twoUnit : Fˣ := ⟨2, 3, by decide +kernel, by decide +kernel⟩

theorem negative_unit_nontrivial : negativeUnit ≠ 1 := by
  intro h
  exact (by decide +kernel : (-1 : F) ≠ 1) (congrArg Units.val h)

theorem negative_unit_sixth : negativeUnit ^ 6 = 1 := by
  apply Units.ext
  decide +kernel

theorem two_sixth : twoUnit ^ 6 = negativeUnit := by
  apply Units.ext
  decide +kernel

theorem unit_card : Nat.card Fˣ = 4 := by
  rw [Nat.card_eq_fintype_card, Fintype.card_units, ZMod.card]

theorem unit_four (u : Fˣ) : u ^ 4 = 1 := by
  simpa only [unit_card] using (pow_card_eq_one' (x := u))

theorem unit_six_eq_square (u : Fˣ) : u ^ 6 = u ^ 2 := by
  calc
    u ^ 6 = u ^ 4 * u ^ 2 := pow_add u 4 2
    _ = u ^ 2 := by rw [unit_four, one_mul]

theorem sixth_power_cases (u : Fˣ) : u ^ 6 = 1 ∨ u ^ 6 = negativeUnit := by
  have h : ∀ x : F, x ≠ 0 → x ^ 6 = 1 ∨ x ^ 6 = -1 := by decide +kernel
  rcases h (u : F) u.ne_zero with h | h
  · exact Or.inl (Units.ext h)
  · exact Or.inr (Units.ext h)

def sixPowers : Subgroup Fˣ := (powMonoidHom 6 : Fˣ →* Fˣ).range
abbrev Target := Fˣ ⧸ sixPowers

theorem mem_six_powers (u : Fˣ) : u ∈ sixPowers ↔ u = 1 ∨ u = negativeUnit := by
  change (∃ v : Fˣ, v ^ 6 = u) ↔ u = 1 ∨ u = negativeUnit
  constructor
  · rintro ⟨v, rfl⟩
    exact sixth_power_cases v
  · rintro (rfl | rfl)
    · exact ⟨1, one_pow 6⟩
    · exact ⟨twoUnit, two_sixth⟩

def powersEquivBool : sixPowers ≃ Bool where
  toFun u := if u.val = negativeUnit then true else false
  invFun
    | false => ⟨1, (mem_six_powers 1).mpr (Or.inl rfl)⟩
    | true => ⟨negativeUnit, (mem_six_powers negativeUnit).mpr (Or.inr rfl)⟩
  left_inv u := by
    apply Subtype.ext
    rcases (mem_six_powers u.val).mp u.property with h | h
    · simp [h, Ne.symm negative_unit_nontrivial]
    · simp [h]
  right_inv b := by
    cases b
    · simp [Ne.symm negative_unit_nontrivial]
    · simp

theorem six_powers_card : Nat.card sixPowers = 2 := by
  rw [Nat.card_congr powersEquivBool, Nat.card_eq_fintype_card]
  decide +kernel

theorem target_card : Nat.card Target = 2 := by
  have h := sixPowers.card_eq_card_quotient_mul_card_subgroup
  rw [unit_card, six_powers_card] at h
  change Nat.card (Fˣ ⧸ sixPowers) = 2
  omega

theorem target_square (z : Target) : z ^ 2 = 1 := by
  refine QuotientGroup.induction_on z (fun u => ?_)
  calc
    (QuotientGroup.mk' sixPowers u) ^ 2 = QuotientGroup.mk' sixPowers (u ^ 2) :=
      (map_pow (QuotientGroup.mk' sixPowers) u 2).symm
    _ = 1 := (QuotientGroup.eq_one_iff _).mpr ⟨u, unit_six_eq_square u⟩

theorem no_primitive_six : ¬ ∃ u : Fˣ, IsPrimitiveRoot u 6 := by
  rintro ⟨u, hu⟩
  have hd : 6 ∣ 4 := hu.dvd_of_pow_eq_one 4 (unit_four u)
  norm_num at hd

/-- The familiar prime-order interpretation has the stronger root guard. -/
theorem prime_root_primitive {M : Type*} [CommMonoid M] (ell : ℕ) (hell : Nat.Prime ell)
    (u : M) (hu : u ^ ell = 1) (hne : u ≠ 1) : IsPrimitiveRoot u ell := by
  apply IsPrimitiveRoot.iff_orderOf.mpr
  have hd : orderOf u ∣ ell := orderOf_dvd_iff_pow_eq_one.mpr hu
  rcases hell.eq_one_or_self_of_dvd _ hd with h | h
  · exact False.elim (hne (orderOf_eq_one_iff.mp h))
  · exact h

theorem negative_unit_order : orderOf negativeUnit = 2 := by
  have h2 : negativeUnit ^ 2 = 1 := by
    apply Units.ext
    decide +kernel
  exact (prime_root_primitive 2 (by decide) negativeUnit h2 negative_unit_nontrivial).eq_orderOf.symm

#print axioms negative_unit_sixth
#print axioms target_card
#print axioms target_square
#print axioms no_primitive_six
end
end CompositeTateDomainAudit
