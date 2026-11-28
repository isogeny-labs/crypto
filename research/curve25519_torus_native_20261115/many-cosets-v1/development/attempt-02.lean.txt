import Mathlib.RingTheory.RootsOfUnity.PrimitiveRoots
import Mathlib.Data.Set.Card
import Mathlib.Tactic

set_option autoImplicit false

example {F : Type*} [Field F] (h : F)
    (hh : IsPrimitiveRoot h 12965132697890207) {a b : ℕ}
    (ha : a < 12965132697890207) (hb : b < 12965132697890207)
    (he : h^a = h^b) : a = b := hh.pow_inj ha hb he

example {F : Type*} [Field F] (h : F)
    (hh : IsPrimitiveRoot h 12965132697890207) :
    IsPrimitiveRoot (h^60824497) 213156431 := by
  simpa using hh.pow_of_dvd (p := 60824497) (by decide) (by norm_num)

namespace Curve25519CosetInstance
variable {F : Type*} [Field F]

theorem indexed_injective (h : F) (hh : IsPrimitiveRoot h 12965132697890207) :
    Function.Injective (fun i : Fin 20633 × Fin 213156431 =>
      h^(i.1.val+1+60824497*i.2.val)) := by
  intro i j he
  have e := hh.pow_inj (by omega) (by omega) he
  apply Prod.ext <;> apply Fin.ext <;> omega

theorem raw_cardinality (h : F) (hh : IsPrimitiveRoot h 12965132697890207) :
    (Set.range (fun i : Fin 20633 × Fin 213156431 =>
      h^(i.1.val+1+60824497*i.2.val))).ncard = 4398056640823 := by
  rw [Set.ncard_range_of_injective (indexed_injective h hh)]
  norm_num [Nat.card_eq_fintype_card]

theorem no_identity (h : F) (hh : IsPrimitiveRoot h 12965132697890207)
    (i : Fin 20633 × Fin 213156431) : h^(i.1.val+1+60824497*i.2.val) ≠ 1 :=
  hh.pow_ne_one_of_pos_of_lt (by omega) (by omega)

theorem no_inverse_pair (h : F) (hh : IsPrimitiveRoot h 12965132697890207)
    (i j : Fin 20633 × Fin 213156431) :
    h^(i.1.val+1+60824497*i.2.val) * h^(j.1.val+1+60824497*j.2.val) ≠ 1 := by
  intro he
  rw [← pow_add] at he
  obtain ⟨c, hc⟩ := (hh.pow_eq_one_iff_dvd _).mp he
  omega

theorem in_parent_roots (h : F) (hh : IsPrimitiveRoot h 12965132697890207)
    (i : Fin 20633 × Fin 213156431) :
    (h^(i.1.val+1+60824497*i.2.val))^12965132697890207 = 1 := by
  rw [← pow_mul, Nat.mul_comm _ 12965132697890207, pow_mul, hh.pow_eq_one, one_pow]

theorem odd_roots_not_opposite (u v : F) (hchar : (1 : F) ≠ -1)
    (hu : u^12965132697890207 = 1) (hv : v^12965132697890207 = 1) : u ≠ -v := by
  intro he
  apply hchar
  calc
    (1 : F) = u^12965132697890207 := hu.symm
    _ = (-v)^12965132697890207 := congrArg (fun z : F => z^12965132697890207) he
    _ = -(v^12965132697890207) := (by decide : Odd 12965132697890207).neg_pow v
    _ = -1 := congrArg Neg.neg hv

theorem membership_fibers (h z : F) (hh : IsPrimitiveRoot h 12965132697890207) :
    (∃ i : Fin 20633, ∃ t : Fin 213156431, h^(i.val+1+60824497*t.val) = z) ↔
    ∃ i : Fin 20633, (z / h^(i.val+1))^213156431 = 1 := by
  have h0 : h ≠ 0 := (hh.isUnit (by decide)).ne_zero
  have hr : IsPrimitiveRoot (h^60824497) 213156431 := by
    simpa using hh.pow_of_dvd (p := 60824497) (by decide) (by norm_num)
  constructor
  · rintro ⟨i,t,rfl⟩
    refine ⟨i, ?_⟩
    rw [pow_add, pow_mul, mul_div_cancel_left₀ _ (pow_ne_zero _ h0),
      ← pow_mul, Nat.mul_comm _ 213156431, pow_mul, hr.pow_eq_one, one_pow]
  · rintro ⟨i,hi⟩
    obtain ⟨t,ht,he⟩ := hr.eq_pow_of_pow_eq_one hi
    refine ⟨i,⟨t,ht⟩, ?_⟩
    rw [pow_add, pow_mul, he, mul_div_cancel₀ _ (pow_ne_zero _ h0)]

theorem membership_lookup (h z : F) (hh : IsPrimitiveRoot h 12965132697890207) :
    (∃ i : Fin 20633, ∃ t : Fin 213156431, h^(i.val+1+60824497*t.val) = z) ↔
    ∃ i : Fin 20633, z^213156431 = (h^(i.val+1))^213156431 := by
  rw [membership_fibers h z hh]
  have h0 : h ≠ 0 := (hh.isUnit (by decide)).ne_zero
  apply exists_congr
  intro i
  rw [div_pow, div_eq_one_iff_eq (pow_ne_zero _ (pow_ne_zero _ h0))]

theorem no_sign_pair (h : F) (hh : IsPrimitiveRoot h 12965132697890207)
    (hchar : (1 : F) ≠ -1) (i j : Fin 20633 × Fin 213156431) :
    h^(i.1.val+1+60824497*i.2.val) ≠ -h^(j.1.val+1+60824497*j.2.val) :=
  odd_roots_not_opposite _ _ hchar (in_parent_roots h hh i) (in_parent_roots h hh j)

theorem no_negative_inverse_pair (h : F) (hh : IsPrimitiveRoot h 12965132697890207)
    (hchar : (1 : F) ≠ -1) (i j : Fin 20633 × Fin 213156431) :
    h^(i.1.val+1+60824497*i.2.val) * h^(j.1.val+1+60824497*j.2.val) ≠ -1 := by
  apply odd_roots_not_opposite _ (1 : F) hchar
  · rw [mul_pow, in_parent_roots h hh i, in_parent_roots h hh j, one_mul]
  · exact one_pow _

#print axioms indexed_injective
#print axioms raw_cardinality
#print axioms no_identity
#print axioms no_inverse_pair
#print axioms in_parent_roots
#print axioms odd_roots_not_opposite
#print axioms membership_fibers
#print axioms membership_lookup
#print axioms no_sign_pair
#print axioms no_negative_inverse_pair

end Curve25519CosetInstance
