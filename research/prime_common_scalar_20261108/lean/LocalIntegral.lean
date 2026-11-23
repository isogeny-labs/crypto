module
public import RationalScalar
public import Mathlib.NumberTheory.Padics.PadicNorm
public import Mathlib.RingTheory.PowerSeries.Inverse
public section
namespace LocalIntegral
open CauchyNormalizer

/-- The actual rational two-adic integral subring, not an arbitrary lattice. -/
@[expose] def integralRing : Subring ℚ where
  carrier := {q | padicNorm 2 q ≤ 1}
  zero_mem' := by simp
  one_mem' := by change padicNorm 2 1 ≤ 1; rw [padicNorm.one]
  add_mem' {a b} ha hb := le_trans padicNorm.nonarchimedean (max_le ha hb)
  neg_mem' {a} ha := by simpa using ha
  mul_mem' {a b} ha hb := by
    change padicNorm 2 (a*b) ≤ 1
    rw [padicNorm.mul]
    calc
      _ ≤ 1 * padicNorm 2 b := mul_le_mul_of_nonneg_right ha (padicNorm.nonneg b)
      _ ≤ 1 := by simpa using hb

/-- A bounded nonzero integer has fewer than a+1 factors of two. -/
theorem bounded_valuation (a n : ℕ) (hn : n ≠ 0) (hbound : n < 2^(a+1)) :
    padicValNat 2 n < a+1 := by
  have hp : 2^(padicValNat 2 n) ≤ n := Nat.le_of_dvd (Nat.pos_of_ne_zero hn) pow_padicValNat_dvd
  by_contra! h
  have hx := Nat.pow_le_pow_right (n:=2) (by decide) h
  omega

theorem integer_ratio_integral (a : ℕ) (d : ℤ) (hd : d ≠ 0)
    (hbound : d.natAbs < 2^(a+1)) :
    padicNorm 2 (((2^(a+1) : ℕ) : ℚ) / d) ≤ 1 := by
  have hdn : d.natAbs ≠ 0 := Int.natAbs_ne_zero.mpr hd
  have hv := bounded_valuation a d.natAbs hdn hbound
  have hq : (d : ℚ) ≠ 0 := by exact_mod_cast hd
  have hp : (((2^(a+1) : ℕ) : ℚ)) ≠ 0 := by positivity
  have hn : padicValRat 2 (((2^(a+1) : ℕ) : ℚ)) = (a+1 : ℕ) := by
    rw [Nat.cast_pow, padicValRat.pow, padicValRat.self (by decide)]
    simp
  have hpos : 0 < padicValRat 2 (((2^(a+1) : ℕ) : ℚ) / d) := by
    rw [padicValRat.div hp hq, hn, padicValRat.of_int]
    unfold padicValInt
    omega
  rw [padicNorm.eq_zpow_of_nonzero (div_ne_zero hp hq)]
  exact zpow_le_one_of_nonpos₀ (by norm_num) (by omega)

theorem grid_ratio_integral (a : ℕ) (m j : Fin (2^(a+1))) :
    padicNorm 2 (((2^(a+1) : ℕ) : ℚ) / ((m : ℚ)-(j : ℚ))) ≤ 1 := by
  by_cases h : m=j
  · subst j; simp
  have hd : (m : ℤ)-(j : ℤ) ≠ 0 := by
    intro hz
    have : m.val=j.val := by omega
    exact h (Fin.ext this)
  have hbound : ((m : ℤ)-(j : ℤ)).natAbs < 2^(a+1) := by
    have hm := m.isLt
    have hj := j.isLt
    omega
  have hh := integer_ratio_integral a ((m : ℤ)-(j : ℤ)) hd hbound
  simpa only [Int.cast_sub, Int.cast_natCast] using hh

@[expose] def chartRatio (a : ℕ) (m j : Fin (2^(a+1))) : integralRing :=
  ⟨((2^(a+1) : ℕ) : ℚ) / ((m : ℚ)-(j : ℚ)), grid_ratio_integral a m j⟩

theorem rho_norm (a : ℕ) (m : Fin (2^(a+1))) :
    padicNorm 2 (cauchyRho a m) = 1 := by
  rw [padicNorm.eq_zpow_of_nonzero (cauchyRho_ne_zero a m m.isLt), cauchyRho_unit a m m.isLt]
  simp

@[expose] def rhoUnit (a : ℕ) (m : Fin (2^(a+1))) : integralRingˣ where
  val := ⟨cauchyRho a m, le_of_eq (rho_norm a m)⟩
  inv := ⟨(cauchyRho a m)⁻¹, by
    change padicNorm 2 ((cauchyRho a m)⁻¹) ≤ 1
    rw [← one_div, padicNorm.div, padicNorm.one, rho_norm]
    norm_num⟩
  val_inv := by
    apply Subtype.ext
    exact mul_inv_cancel₀ (cauchyRho_ne_zero a m m.isLt)
  inv_val := by
    apply Subtype.ext
    exact inv_mul_cancel₀ (cauchyRho_ne_zero a m m.isLt)

end LocalIntegral
#print axioms LocalIntegral.bounded_valuation
#print axioms LocalIntegral.integer_ratio_integral
#print axioms LocalIntegral.grid_ratio_integral
#print axioms LocalIntegral.rho_norm
