import Mathlib.NumberTheory.LegendreSymbol.QuadraticReciprocity
import Mathlib.Tactic.FieldSimp
import Mathlib.Tactic.Ring
import Mathlib.Tactic.NormNum

namespace ConductorModEight

/-- Number-theoretic part of the odd-exponent conductor argument, with its
discriminant identity and prime divisor supplied explicitly. This does not
construct an elliptic curve, certify a recorded conductor, or prove a
volcano-degree statement. -/
theorem primeDivisorModEight (p m : ℕ) [Fact p.Prime]
    (hp : p ≠ 2) (hm : Odd m) (t f D : ℤ)
    (hf : (p : ℤ) ∣ f)
    (hd : t ^ 2 - 4 * (2 : ℤ) ^ m = f ^ 2 * D) :
    p % 8 = 1 ∨ p % 8 = 7 := by
  have hf0 : (f : ZMod p) = 0 :=
    (ZMod.intCast_zmod_eq_zero_iff_dvd f p).mpr hf
  have hd0 : (t : ZMod p) ^ 2 - 4 * (2 : ZMod p) ^ m =
      (f : ZMod p) ^ 2 * (D : ZMod p) := by
    simpa only [Int.cast_sub, Int.cast_mul, Int.cast_pow, Int.cast_ofNat]
      using congrArg (fun z : ℤ => (z : ZMod p)) hd
  have ht : (t : ZMod p) ^ 2 = 4 * (2 : ZMod p) ^ m := by
    apply sub_eq_zero.mp
    simpa [hf0] using hd0
  have htwo : (2 : ZMod p) ≠ 0 := by
    intro hz
    have hdiv : p ∣ 2 := (ZMod.natCast_eq_zero_iff 2 p).mp hz
    exact hp ((Nat.prime_dvd_prime_iff_eq Fact.out Nat.prime_two).mp hdiv)
  obtain ⟨k, rfl⟩ := hm
  have hsq : IsSquare (2 : ZMod p) := by
    apply (isSquare_iff_exists_sq _).mpr
    refine ⟨(t : ZMod p) / (2 : ZMod p) ^ (k + 1), ?_⟩
    rw [div_pow]
    apply (eq_div_iff (pow_ne_zero _ (pow_ne_zero _ htwo))).mpr
    rw [ht]
    simp only [two_mul, pow_add, pow_one]
    ring
  exact (ZMod.exists_sq_eq_two_iff hp).mp hsq

#print axioms ConductorModEight.primeDivisorModEight

end ConductorModEight
