import CrossCharacteristicExample
import Mathlib.Algebra.CharP.Two
import Mathlib.FieldTheory.Finite.GaloisField

/-!
# Odd-to-binary worked example

The source is the existing order-three subgroup over F₅. The target curve
is y² + y = x³ in characteristic two, with R = (0,0), S = (0,1).
The construction works over every characteristic-two field, and is
specialized below to the genuine field F₄ = GaloisField 2 2.
-/

namespace CrossCharacteristicTwo

open WeierstrassCurve.Affine
open CrossCharacteristicExample (P Q P_order sourceGenerator challenge
  challenge_as_multiple source_scalar_mod)

instance : Fact (Nat.Prime 2) := ⟨by decide⟩

variable (K : Type*) [Field K] [DecidableEq K] [CharP K 2]

/-- The binary Weierstrass model y² + y = x³. -/
def binaryCurve : WeierstrassCurve K := ⟨0, 0, 1, 0, 0⟩

instance : (binaryCurve K).IsElliptic where
  isUnit := by
    apply isUnit_iff_ne_zero.mpr
    norm_num [binaryCurve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈,
      CharTwo.neg_eq, CharP.cast_eq_zero_iff K 2]
    exact fun h => (by decide : ¬ 2 ∣ 27) ((CharP.cast_eq_zero_iff K 2 27).mp h)

/-- The target generator (0,0), already rational over the prime subfield. -/
def R : (binaryCurve K).toAffine.Point :=
  .some (x := 0) (y := 0) (by
    rw [nonsingular_iff', equation_iff]
    norm_num [binaryCurve])

/-- The target challenge (0,1). -/
def S : (binaryCurve K).toAffine.Point :=
  .some (x := 0) (y := 1) (by
    rw [nonsingular_iff', equation_iff]
    norm_num [binaryCurve]
    exact ⟨CharTwo.two_eq_zero,
      fun h => (by decide : ¬ 2 ∣ 3) ((CharP.cast_eq_zero_iff K 2 3).mp h)⟩)

theorem R_double : R K + R K = S K := by
  norm_num [R, S, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, binaryCurve, CharTwo.neg_eq]
  exact CharTwo.neg_eq _

theorem S_add_R : S K + R K = 0 := by
  apply Point.add_of_Y_eq rfl
  norm_num [negY, binaryCurve, CharTwo.neg_eq]
  exact (CharTwo.neg_eq _).symm

theorem R_order : addOrderOf (R K) = 3 := by
  apply addOrderOf_eq_prime
  · simpa only [show (3 : ℕ) = 2 + 1 from rfl, add_nsmul, two_nsmul,
      one_nsmul, R_double] using S_add_R K
  · exact Point.some_ne_zero _

def targetGenerator : AddSubgroup.zmultiples (R K) := ⟨R K, by simp⟩

/-- Apply Mathlib's existing constructor to the two order-three subgroups. -/
noncomputable def phi : AddSubgroup.zmultiples P ≃+ AddSubgroup.zmultiples (R K) :=
  addEquivOfAddOrderOfEq (g := sourceGenerator) (g' := targetGenerator K)
    (AddSubgroup.forall_zmultiples.mpr (fun n => ⟨n, rfl⟩))
    (AddSubgroup.forall_zmultiples.mpr (fun n => ⟨n, rfl⟩))
    (by simpa only [sourceGenerator, targetGenerator, AddSubgroup.addOrderOf_mk]
      using P_order.trans (R_order K).symm)

theorem phi_zero : phi K 0 = 0 := map_zero (phi K)

theorem phi_generator : phi K sourceGenerator = targetGenerator K :=
  addEquivOfAddOrderOfEq_apply_gen _ _ _

theorem phi_challenge : (phi K challenge : (binaryCurve K).toAffine.Point) = S K := by
  rw [challenge_as_multiple, map_zsmul, phi_generator]
  change (2 : ℤ) • R K = S K
  simpa only [two_zsmul] using R_double K

/-- Preservation and reflection of every integer scalar in the source subgroup. -/
theorem transfer_dlog (T : AddSubgroup.zmultiples P) (k : ℤ) :
    (phi K T : (binaryCurve K).toAffine.Point) = k • R K ↔
      (T : CrossCharacteristicExample.curve5.toAffine.Point) = k • P := by
  constructor
  · intro h
    have hsub : phi K T = phi K (k • sourceGenerator) := by
      rw [map_zsmul, phi_generator]
      exact Subtype.ext h
    exact congrArg Subtype.val ((phi K).injective hsub)
  · intro h
    have hsub : T = k • sourceGenerator := Subtype.ext h
    rw [hsub, map_zsmul, phi_generator]
    rfl

theorem example_dlog (k : ℤ) : S K = k • R K ↔ Q = k • P := by
  have h := transfer_dlog K challenge k
  simpa only [phi_challenge] using h

theorem target_scalar_mod (k : ℤ) : S K = k • R K ↔ (2 : ℤ) ≡ k [ZMOD 3] := by
  rw [example_dlog K k, source_scalar_mod k]

omit [DecidableEq K] in
/-- The point-subgroup transfer cannot be a unital map of the base fields. -/
theorem no_field_map : IsEmpty (ZMod 5 →+* K) := by
  refine ⟨fun f => ?_⟩
  have h : (5 : ℕ) = 2 :=
    CharP.eq K (charP_of_injective_ringHom f.injective 5) inferInstance
  exact (by decide : (5 : ℕ) ≠ 2) h

/-- A field of four elements, not the nonfield ring ZMod 4. -/
abbrev F4 := GaloisField 2 2

noncomputable instance : DecidableEq F4 := Classical.decEq _

theorem F4_card : Nat.card F4 = 4 := GaloisField.card 2 2 (by decide)

/-- The promised odd-characteristic to even-characteristic specialization. -/
theorem F5_to_F4_dlog (k : ℤ) : S F4 = k • R F4 ↔ Q = k • P :=
  example_dlog F4 k

end CrossCharacteristicTwo
