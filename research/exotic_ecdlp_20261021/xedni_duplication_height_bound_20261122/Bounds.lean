import Mathlib.Analysis.Complex.ExponentialBounds
import Mathlib.Tactic

/-!
Exact polynomial and numerical portions of the direct duplication-height bound.
The interval certificate and the gcd-to-height bridge are audited separately.
-/

namespace XedniDuplicationBound

def F (A B : ℤ) : ℤ :=
  A ^ 4 - 1502 * A ^ 2 * B ^ 2 + 31976 * A * B ^ 3 - 187435 * B ^ 4

def G (A B : ℤ) : ℤ :=
  4 * A ^ 3 * B - 188 * A ^ 2 * B ^ 2 + 3004 * A * B ^ 3 - 15988 * B ^ 4

def U (A B : ℤ) : ℤ :=
  136089600 * A ^ 2 * B - 4264140800 * A * B ^ 2 + 36063744000 * B ^ 3

def V (A B : ℤ) : ℤ :=
  -34022400 * A ^ 3 - 533017600 * A ^ 2 * B +
    42584704000 * A * B ^ 2 - 423295360000 * B ^ 3

theorem homogeneous_bezout_identity (A B : ℤ) :
    U A B * F A B + V A B * G A B = 8038359040000 * B ^ 7 := by
  unfold U V F G
  ring

theorem upper_coefficient_bound :
    1 + 1502 + 31976 + 187435 < (2 : ℤ) ^ 18 ∧
    4 + 188 + 3004 + 15988 < (2 : ℤ) ^ 18 := by
  norm_num

theorem resultant_lt_pow43 : (8038359040000 : ℤ) < 2 ^ 43 := by
  norm_num

theorem outer_tail_coefficient_bound :
    (1502 : ℚ) / 128 ^ 2 + 31976 / 128 ^ 3 + 187435 / 128 ^ 4 < 1 / 4 := by
  norm_num

theorem numerical_height_gap_lt14 : (53 / 3 : ℝ) * Real.log 2 < 14 := by
  have hlog := Real.log_two_lt_d9
  linarith

theorem doubled_generator_x : F 17 1 = 5600 ∧ G 17 1 = 400 := by
  norm_num [F, G]

#print axioms homogeneous_bezout_identity
#print axioms upper_coefficient_bound
#print axioms resultant_lt_pow43
#print axioms outer_tail_coefficient_bound
#print axioms numerical_height_gap_lt14
#print axioms doubled_generator_x

end XedniDuplicationBound
