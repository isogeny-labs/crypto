import CryptoLeanCore.TaperedPrecisionRoot

/-! Study instances of the shared source-expression laws. Deletion witnesses
only concern the weakened scalar domain. None is a production Setup call. -/
namespace NativeRootPrecision
open CryptoLeanCore.TaperedPrecision

/-- The source-normalized root is covered when the exact array guard holds. -/
theorem normalized_root (E u i : Nat) (hE : 0 < E) (hi : i < u)
    (harray : 2 ^ (2 * u) * 64 ^ (4 * u) ≤ 2 * E) :
    childPrecision (E + 1) (2 * E) u (2 * u) i * u ≤
      35 * (2 * E) * 2 ^ (2 * u - i) :=
  root_precision_of_array _ _ _ _ (by omega) hi harray

/-- The complete exact u=2 active-node family, including the additive bit. -/
theorem normalized_two_identity (E : Nat) :
    childPrecision (E + 1) (2 * E) 2 4 1 * 2 = 546 * E + 2 := by
  rw [childPrecision_formula]
  have hh : (3 * (2 * E) + 2 - 1) / 2 = 3 * E := by omega
  rw [hh]
  simp only [Nat.mul_assoc]
  omega

/-- Constant34 fails even with all hypotheses of the weaker margin domain. -/
theorem constant34_family (E : Nat) (hE : 32 ≤ E) :
    E + 1 ≤ 2 * E ∧ 32 * 2 ≤ 2 * E ∧
    34 * (2 * E) * 2 ^ (4 - 1) <
      childPrecision (E + 1) (2 * E) 2 4 1 * 2 := by
  rw [normalized_two_identity]
  simp only [Nat.mul_assoc]
  omega

/-- Positive margin-domain example; no full array guard is asserted. -/
theorem scalar_positive :
    childPrecision 33 64 2 4 1 * 2 ≤ 35 * 64 * 2 ^ (4 - 1) := by
  exact root_precision_mul_le 33 64 2 1 (by decide) (by decide) (by decide)

/-- Deleting p<=M breaks35 while margin and active valuation hold. -/
theorem missing_initial_precision :
    32 * 1 ≤ 32 ∧ 0 < 1 ∧
    35 * 32 * 2 ^ (2 - 0) < childPrecision 1024 32 1 2 0 * 1 := by
  decide

/-- Deleting margin also deletes the stronger array guard that implies it. -/
theorem missing_margin :
    3 ≤ 3 ∧ 1 < 2 ∧
    35 * 3 * 2 ^ (4 - 1) < childPrecision 3 3 2 4 1 * 2 := by
  decide

/-- Deleting active valuation breaks35 with the other two scalar premises. -/
theorem missing_active_valuation :
    32 ≤ 32 ∧ 32 * 1 ≤ 32 ∧
    35 * 32 * 2 ^ (2 - 1) < childPrecision 32 32 1 2 1 * 1 := by
  decide

#print axioms normalized_root
#print axioms normalized_two_identity
#print axioms constant34_family
#print axioms scalar_positive
#print axioms missing_initial_precision
#print axioms missing_margin
#print axioms missing_active_valuation
#print axioms CryptoLeanCore.TaperedPrecision.root_evaluation_envelope
#print axioms CryptoLeanCore.TaperedPrecision.root_four_scale
#print axioms CryptoLeanCore.TaperedPrecision.root_array_margin
#print axioms CryptoLeanCore.TaperedPrecision.root_precision_mul_le
#print axioms CryptoLeanCore.TaperedPrecision.root_precision_of_array
end NativeRootPrecision
