import OutputBridge

namespace XedniDuplicationUpperBridge

open XedniDuplicationBound
  XedniDuplicationRationalBridge XedniDuplicationPointBridge

private theorem four_term_abs (w x y z : ℝ) :
    |w - x + y - z| ≤ |w| + |x| + |y| + |z| := by
  have h1 : |w - x + y - z| ≤ |w - x + y| + |z| := by
    simpa [sub_eq_add_neg, abs_neg] using abs_add_le (w - x + y) (-z)
  have h2 := abs_add_le (w - x) y
  have h3 : |w - x| ≤ |w| + |x| := by
    simpa [sub_eq_add_neg, abs_neg] using abs_add_le w (-x)
  linarith

theorem homogeneous_upper (A B : ℤ) :
    max |(F A B : ℝ)| |(G A B : ℝ)| ≤
      (2 : ℝ) ^ 18 * (max |(A : ℝ)| |(B : ℝ)|) ^ 4 := by
  let a : ℝ := |(A : ℝ)|
  let b : ℝ := |(B : ℝ)|
  let M : ℝ := max a b
  have ha : a ≤ M := le_max_left _ _
  have hb : b ≤ M := le_max_right _ _
  have h0a : 0 ≤ a := abs_nonneg _
  have h0b : 0 ≤ b := abs_nonneg _
  have h0M : 0 ≤ M := le_trans h0a ha
  have h40 : a ^ 4 ≤ M ^ 4 := by gcongr
  have h31 : a ^ 3 * b ≤ M ^ 4 := by
    calc
      a ^ 3 * b ≤ M ^ 3 * M := by gcongr
      _ = M ^ 4 := by ring
  have h22 : a ^ 2 * b ^ 2 ≤ M ^ 4 := by
    calc
      a ^ 2 * b ^ 2 ≤ M ^ 2 * M ^ 2 := by gcongr
      _ = M ^ 4 := by ring
  have h13 : a * b ^ 3 ≤ M ^ 4 := by
    calc
      a * b ^ 3 ≤ M * M ^ 3 := by gcongr
      _ = M ^ 4 := by ring
  have h04 : b ^ 4 ≤ M ^ 4 := by gcongr
  have hFexpr : (F A B : ℝ) = (A : ℝ) ^ 4 - 1502 * (A : ℝ) ^ 2 * (B : ℝ) ^ 2 +
      31976 * (A : ℝ) * (B : ℝ) ^ 3 - 187435 * (B : ℝ) ^ 4 := by
    simp [F]
  have hGexpr : (G A B : ℝ) = 4 * (A : ℝ) ^ 3 * (B : ℝ) -
      188 * (A : ℝ) ^ 2 * (B : ℝ) ^ 2 + 3004 * (A : ℝ) * (B : ℝ) ^ 3 -
      15988 * (B : ℝ) ^ 4 := by
    simp [G]
  have hFtri : |(F A B : ℝ)| ≤
      a ^ 4 + 1502 * a ^ 2 * b ^ 2 + 31976 * a * b ^ 3 + 187435 * b ^ 4 := by
    rw [hFexpr]
    convert four_term_abs ((A : ℝ) ^ 4) (1502 * (A : ℝ) ^ 2 * (B : ℝ) ^ 2)
      (31976 * (A : ℝ) * (B : ℝ) ^ 3) (187435 * (B : ℝ) ^ 4) using 1 <;>
      simp [a, b, abs_mul, abs_pow, mul_assoc]
  have hGtri : |(G A B : ℝ)| ≤
      4 * a ^ 3 * b + 188 * a ^ 2 * b ^ 2 + 3004 * a * b ^ 3 + 15988 * b ^ 4 := by
    rw [hGexpr]
    convert four_term_abs (4 * (A : ℝ) ^ 3 * (B : ℝ))
      (188 * (A : ℝ) ^ 2 * (B : ℝ) ^ 2)
      (3004 * (A : ℝ) * (B : ℝ) ^ 3) (15988 * (B : ℝ) ^ 4) using 1 <;>
      simp [a, b, abs_mul, abs_pow, mul_assoc]
  have hF : |(F A B : ℝ)| ≤ 2 ^ 18 * M ^ 4 := by nlinarith
  have hG : |(G A B : ℝ)| ≤ 2 ^ 18 * M ^ 4 := by nlinarith
  exact max_le hF hG

theorem canonical_output_upper (A B : ℤ) (hG : G A B ≠ 0) :
    let q : ℚ := (F A B : ℚ) / (G A B : ℚ)
    max |(q.num : ℝ)| |(q.den : ℝ)| ≤
      (2 : ℝ) ^ 18 * (max |(A : ℝ)| |(B : ℝ)|) ^ 4 := by
  dsimp
  let q : ℚ := (F A B : ℚ) / (G A B : ℚ)
  have hqdf : q = Rat.divInt (F A B) (G A B) := by
    simp [q, Rat.divInt_eq_div]
  obtain ⟨c, hFn, hGn⟩ := Rat.num_den_mk hG hqdf
  have hc : c ≠ 0 := by
    intro hz
    apply hG
    simpa [hz] using hGn
  have hc1 : (1 : ℝ) ≤ |(c : ℝ)| := by
    exact_mod_cast Int.one_le_abs hc
  have hN : 0 ≤ max |(q.num : ℝ)| |(q.den : ℝ)| :=
    le_trans (abs_nonneg _) (le_max_left _ _)
  have hforms : max |(F A B : ℝ)| |(G A B : ℝ)| =
      |(c : ℝ)| * max |(q.num : ℝ)| |(q.den : ℝ)| := by
    rw [hFn, hGn]
    push_cast
    rw [abs_mul, abs_mul]
    exact (mul_max_of_nonneg _ _ (abs_nonneg (c : ℝ))).symm
  have hscaled : max |(q.num : ℝ)| |(q.den : ℝ)| ≤
      |(c : ℝ)| * max |(q.num : ℝ)| |(q.den : ℝ)| := by
    nlinarith [mul_nonneg (sub_nonneg.mpr hc1) hN]
  calc
    max |(q.num : ℝ)| |(q.den : ℝ)| ≤
        max |(F A B : ℝ)| |(G A B : ℝ)| := by rw [hforms]; exact hscaled
    _ ≤ (2 : ℝ) ^ 18 * (max |(A : ℝ)| |(B : ℝ)|) ^ 4 := homogeneous_upper A B

theorem point_canonical_output_upper {x y : ℚ}
    (h : FixedCurve.Nonsingular x y) (hn : y ≠ FixedCurve.negY x y) :
    let q : ℚ := (F x.num x.den : ℚ) / (G x.num x.den : ℚ)
    max |(q.num : ℝ)| |(q.den : ℝ)| ≤
      (2 : ℝ) ^ 18 * (max |(x.num : ℝ)| |(x.den : ℝ)|) ^ 4 :=
  canonical_output_upper x.num x.den (denominator_ne_of_double_ne h hn)

#print axioms homogeneous_upper
#print axioms canonical_output_upper
#print axioms point_canonical_output_upper

end XedniDuplicationUpperBridge
