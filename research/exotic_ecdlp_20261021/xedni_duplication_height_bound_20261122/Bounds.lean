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

theorem common_divisor_dvd_resultant_of_coprime_B
    (A B d : ℤ) (hF : d ∣ F A B) (hG : d ∣ G A B)
    (hcop : Int.gcd d B = 1) : d ∣ (8038359040000 : ℤ) := by
  have hprod : d ∣ (8038359040000 : ℤ) * B ^ 7 := by
    rw [← homogeneous_bezout_identity]
    exact dvd_add (dvd_mul_of_dvd_right hF _) (dvd_mul_of_dvd_right hG _)
  have strip : ∀ n : ℕ, d ∣ (8038359040000 : ℤ) * B ^ n →
      d ∣ (8038359040000 : ℤ) := by
    intro n
    induction n with
    | zero => simpa
    | succ n ih =>
        intro hn
        apply ih
        apply Int.dvd_of_dvd_mul_left_of_gcd_one (b := (8038359040000 : ℤ) * B ^ n)
          (c := B) ?_ hcop
        simpa [pow_succ, mul_assoc] using hn
  exact strip 7 hprod

theorem common_divisor_dvd_resultant
    (A B d : ℤ) (hAB : Int.gcd A B = 1)
    (hF : d ∣ F A B) (hG : d ∣ G A B) :
    d ∣ (8038359040000 : ℤ) := by
  let e : ℤ := Int.gcd d B
  have heD : e ∣ d := Int.gcd_dvd_left d B
  have heB : e ∣ B := Int.gcd_dvd_right d B
  have heF : e ∣ F A B := heD.trans hF
  have hfactor : e ∣ F A B - A ^ 4 := by
    have hdiv := dvd_mul_of_dvd_left heB
      (-1502 * A ^ 2 * B + 31976 * A * B ^ 2 - 187435 * B ^ 3)
    convert hdiv using 1 <;> unfold F <;> ring
  have heA4 : e ∣ A ^ 4 := by
    have hsub := dvd_sub heF hfactor
    convert hsub using 1 <;> ring
  obtain ⟨x, y, hbez⟩ :=
    (Int.gcd_dvd_iff (a := A) (b := B) (n := 1)).mp (by simp [hAB])
  simp only [Nat.cast_one] at hbez
  have heOne : e ∣ (1 : ℤ) := by
    have hpow : (1 : ℤ) = (A * x + B * y) ^ 4 := by
      calc
        (1 : ℤ) = (1 : ℤ) ^ 4 := by norm_num
        _ = (A * x + B * y) ^ 4 := congrArg (· ^ 4) hbez
    have hterm : e ∣ (A * x) ^ 4 := by
      simpa [mul_pow] using dvd_mul_of_dvd_left heA4 (x ^ 4)
    have hcross : e ∣ B * (4 * A ^ 3 * x ^ 3 * y +
        6 * A ^ 2 * x ^ 2 * B * y ^ 2 +
        4 * A * x * B ^ 2 * y ^ 3 + B ^ 3 * y ^ 4) :=
      dvd_mul_of_dvd_left heB _
    have hident : (A * x + B * y) ^ 4 = (A * x) ^ 4 +
        B * (4 * A ^ 3 * x ^ 3 * y +
        6 * A ^ 2 * x ^ 2 * B * y ^ 2 +
        4 * A * x * B ^ 2 * y ^ 3 + B ^ 3 * y ^ 4) := by ring
    rw [hpow, hident]
    exact dvd_add hterm hcross
  have hnat : Int.gcd d B ∣ 1 := (Int.natCast_dvd_natCast).mp heOne
  have hcop : Int.gcd d B = 1 := Nat.dvd_one.mp hnat
  exact common_divisor_dvd_resultant_of_coprime_B A B d hF hG hcop

theorem upper_coefficient_bound :
    1 + 1502 + 31976 + 187435 < (2 : ℤ) ^ 18 ∧
    4 + 188 + 3004 + 15988 < (2 : ℤ) ^ 18 := by
  norm_num

theorem resultant_lt_pow43 : (8038359040000 : ℤ) < 2 ^ 43 := by
  norm_num

theorem outer_tail_coefficient_bound :
    (1502 : ℚ) / 128 ^ 2 + 31976 / 128 ^ 3 + 187435 / 128 ^ 4 < 1 / 4 := by
  norm_num

theorem bernstein_quartic_lower
    (b₀ b₁ b₂ b₃ b₄ K s : ℝ)
    (hs₀ : 0 ≤ s) (hs₁ : s ≤ 1)
    (hb₀ : K ≤ b₀) (hb₁ : K ≤ b₁) (hb₂ : K ≤ b₂)
    (hb₃ : K ≤ b₃) (hb₄ : K ≤ b₄) :
    K ≤ b₀ * (1 - s) ^ 4 +
      b₁ * (4 * s * (1 - s) ^ 3) +
      b₂ * (6 * s ^ 2 * (1 - s) ^ 2) +
      b₃ * (4 * s ^ 3 * (1 - s)) + b₄ * s ^ 4 := by
  let w₀ : ℝ := (1 - s) ^ 4
  let w₁ : ℝ := 4 * s * (1 - s) ^ 3
  let w₂ : ℝ := 6 * s ^ 2 * (1 - s) ^ 2
  let w₃ : ℝ := 4 * s ^ 3 * (1 - s)
  let w₄ : ℝ := s ^ 4
  have ht : 0 ≤ 1 - s := sub_nonneg.mpr hs₁
  have hw₀ : 0 ≤ w₀ := pow_nonneg ht _
  have hw₁ : 0 ≤ w₁ := by dsimp [w₁]; positivity
  have hw₂ : 0 ≤ w₂ := by dsimp [w₂]; positivity
  have hw₃ : 0 ≤ w₃ := by dsimp [w₃]; positivity
  have hw₄ : 0 ≤ w₄ := pow_nonneg hs₀ _
  have hsum : w₀ + w₁ + w₂ + w₃ + w₄ = 1 := by
    dsimp [w₀, w₁, w₂, w₃, w₄]
    ring
  change K ≤ b₀ * w₀ + b₁ * w₁ + b₂ * w₂ + b₃ * w₃ + b₄ * w₄
  calc
    K = K * (w₀ + w₁ + w₂ + w₃ + w₄) := by rw [hsum]; ring
    _ = K * w₀ + K * w₁ + K * w₂ + K * w₃ + K * w₄ := by ring
    _ ≤ b₀ * w₀ + b₁ * w₁ + b₂ * w₂ + b₃ * w₃ + b₄ * w₄ := by
      gcongr

theorem interval_26_positive_F (t : ℝ)
    (hl : (191 / 16 : ℝ) ≤ t) (hu : t ≤ (383 / 32 : ℝ)) :
    (21517662721 / 1073741824 : ℝ) ≤
      t ^ 4 - 1502 * t ^ 2 + 31976 * t - 187435 := by
  let s : ℝ := 32 * t - 382
  have hs₀ : 0 ≤ s := by dsimp [s]; linarith
  have hs₁ : s ≤ 1 := by dsimp [s]; linarith
  have hb := bernstein_quartic_lower
    (35716865 / 65536 : ℝ) (74424129 / 131072 : ℝ)
    (464404355 / 786432 : ℝ) (321454273 / 524288 : ℝ)
    (666501633 / 1048576 : ℝ)
    (21517662721 / 1073741824 : ℝ) s hs₀ hs₁
    (by norm_num) (by norm_num) (by norm_num)
    (by norm_num) (by norm_num)
  convert hb using 1 <;> dsimp [s] <;> ring

theorem numerical_height_gap_lt14 : (53 / 3 : ℝ) * Real.log 2 < 14 := by
  have hlog := Real.log_two_lt_d9
  linarith

theorem doubled_generator_x : F 17 1 = 5600 ∧ G 17 1 = 400 := by
  norm_num [F, G]

#print axioms homogeneous_bezout_identity
#print axioms common_divisor_dvd_resultant_of_coprime_B
#print axioms common_divisor_dvd_resultant
#print axioms upper_coefficient_bound
#print axioms resultant_lt_pow43
#print axioms outer_tail_coefficient_bound
#print axioms bernstein_quartic_lower
#print axioms interval_26_positive_F
#print axioms numerical_height_gap_lt14
#print axioms doubled_generator_x

end XedniDuplicationBound
