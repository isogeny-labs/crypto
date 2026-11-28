import SquareDenominator
import RationalBridge

/-! The canonical denominator theorem feeds the bounded public x-coordinate sieve. -/

namespace XedniSquareDenominator

local instance : Fact (Nat.Prime 1009) := ⟨by norm_num⟩

theorem no_bounded_rational_affine_point (x y : ℚ)
    (hcurve : y^2 = x^3 + 9*x + 1)
    (ha₁ : -4096 ≤ x.num) (ha₂ : x.num ≤ 4096)
    (hb : x.den.sqrt ≤ 4096)
    (hxred : (x.num - 744 * ((x.den.sqrt : ℕ) : ℤ)^2) % 1009 = 0) : False := by
  obtain ⟨b, hdenmul⟩ := square_denominator_of_affine x y hcurve
  have hden : x.den = b^2 := by simpa [pow_two] using hdenmul
  have hbsqrt : x.den.sqrt = b := by rw [hden, Nat.sqrt_eq']
  have hbpos : 1 ≤ b := by
    have hne : b ≠ 0 := by
      intro hz
      simp [hz] at hden
    omega
  have hcop : x.num.natAbs.Coprime b := by
    have hcop2 : x.num.natAbs.Coprime (b^2) := by
      simpa [← hden] using x.reduced
    exact (Nat.coprime_pow_right_iff (by omega : 0 < 2) x.num.natAbs b).mp hcop2
  have hxrepr : x = (x.num : ℚ) / (b : ℚ)^2 := by
    simpa [hden, Nat.cast_pow] using x.num_div_den.symm
  exact XedniRationalBridge.no_bounded_rational_affine_lift
    x.num b x y ha₁ ha₂ hbpos (by simpa [hbsqrt] using hb)
    hcop hxrepr hcurve (by simpa [hbsqrt] using hxred)

#print axioms no_bounded_rational_affine_point

theorem no_bounded_affine_with_reduced_x (x y : ℚ)
    (hcurve : y^2 = x^3 + 9*x + 1)
    (ha₁ : -4096 ≤ x.num) (ha₂ : x.num ≤ 4096)
    (hb : x.den.sqrt ≤ 4096)
    (hunit : ((x.den.sqrt : ℕ) : ZMod 1009) ≠ 0)
    (hxred : ((x.num : ℤ) : ZMod 1009) /
      (((x.den.sqrt : ℕ) : ZMod 1009)^2) = 744) : False := by
  have hunit2 : (((x.den.sqrt : ℕ) : ZMod 1009)^2) ≠ 0 := pow_ne_zero _ hunit
  have heq : ((x.num : ℤ) : ZMod 1009) =
      744 * (((x.den.sqrt : ℕ) : ZMod 1009)^2) :=
    (div_eq_iff hunit2).mp hxred
  have hzero : (((x.num - 744 * ((x.den.sqrt : ℕ) : ℤ)^2) : ℤ) : ZMod 1009) = 0 := by
    push_cast
    rw [heq]
    ring
  have hdiv : (1009 : ℤ) ∣ x.num - 744 * ((x.den.sqrt : ℕ) : ℤ)^2 :=
    (ZMod.intCast_zmod_eq_zero_iff_dvd _ _).mp hzero
  exact no_bounded_rational_affine_point x y hcurve ha₁ ha₂ hb
    (Int.dvd_iff_emod_eq_zero.mp hdiv)

#print axioms no_bounded_affine_with_reduced_x

end XedniSquareDenominator
