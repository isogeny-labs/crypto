import Mathlib

namespace XedniSquareDenominator

theorem square_of_cube_square {d : ℕ} (h : IsSquare (d ^ 3)) : IsSquare d := by
  apply Nat.isSquare_iff_even_factorization.mpr
  intro p hp
  have he := (Nat.isSquare_iff_even_factorization.mp h) p hp
  rw [Nat.factorization_pow] at he
  simp only [Finsupp.smul_apply, nsmul_eq_mul] at he
  rcases he with ⟨k, hk⟩
  exact ⟨k - d.factorization p, by omega⟩

#print axioms square_of_cube_square

theorem square_denominator_of_affine (x y : ℚ)
    (hcurve : y^2 = x^3 + 9*x + 1) : IsSquare x.den := by
  let a : ℤ := x.num
  let d : ℕ := x.den
  let n : ℤ := a^3 + 9*a*(d : ℤ)^2 + (d : ℤ)^3
  have hd : d ≠ 0 := x.den_nz
  have hdq : (d : ℚ) ≠ 0 := by exact_mod_cast hd
  have hcop : a.natAbs.Coprime d := x.reduced
  have hgcd : Int.gcd a (d : ℤ) = 1 := by
    simpa [Int.gcd_def, Nat.Coprime] using hcop
  have hgn : Int.gcd n (d : ℤ) = 1 := by
    have hn : n = a^3 + (d : ℤ) * (9*a*(d : ℤ) + (d : ℤ)^2) := by
      dsimp [n]
      ring
    rw [hn, Int.gcd_add_mul_left_left]
    exact Int.gcd_pow_left_of_gcd_eq_one hgcd
  have hgnd : n.natAbs.Coprime (d^3) := by
    have h := Int.gcd_pow_right_of_gcd_eq_one hgn (k := 3)
    simpa [Int.gcd_def, Nat.Coprime, Int.natAbs_pow] using h
  have hrat : y^2 = mkRat n (d^3) := by
    rw [hcurve, Rat.mkRat_eq_div]
    have hx : x = (a : ℚ) / (d : ℚ) := by
      simpa [a, d] using x.num_div_den.symm
    rw [hx]
    push_cast
    field_simp [hdq]
    dsimp [n]
    push_cast
    ring
  have hden : (mkRat n (d^3)).den = d^3 := by
    rw [Rat.den_mkRat]
    have hpow : d^3 ≠ 0 := pow_ne_zero _ hd
    rw [ite_eq_right hpow, hgnd.symm.gcd_eq_one]
    simp
  have heq : y.den^2 = d^3 := by
    have h := congrArg Rat.den hrat
    simpa [Rat.den_pow, hden] using h
  have hs : IsSquare (d^3) := by rw [← heq]; exact IsSquare.sq y.den
  simpa [d] using square_of_cube_square hs

#print axioms square_denominator_of_affine

theorem exists_primitive_square_denominator (x y : ℚ)
    (hcurve : y^2 = x^3 + 9*x + 1) :
    ∃ (a : ℤ) (b : ℕ), 0 < b ∧ a.natAbs.Coprime b ∧
      x = (a : ℚ) / (b : ℚ)^2 := by
  obtain ⟨b, hb⟩ := square_denominator_of_affine x y hcurve
  have hden : x.den = b^2 := by simpa [pow_two] using hb
  have hbpos : 0 < b := by
    have hne : b ≠ 0 := by
      intro hz
      simp [hz] at hden
    exact Nat.pos_of_ne_zero hne
  have hcop : x.num.natAbs.Coprime b := by
    have hcop2 : x.num.natAbs.Coprime (b^2) := by
      simpa [← hden] using x.reduced
    exact (Nat.coprime_pow_right_iff (by omega : 0 < 2) x.num.natAbs b).mp hcop2
  refine ⟨x.num, b, hbpos, hcop, ?_⟩
  simpa [hden, Nat.cast_pow] using x.num_div_den.symm

#print axioms exists_primitive_square_denominator

end XedniSquareDenominator
