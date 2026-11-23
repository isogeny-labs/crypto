import Mathlib
import FullLift4096

/-! A rational affine point whose x-coordinate has a primitive square-denominator
representation acquires an integral y numerator. The finite certificate then
excludes reduction to the declared target within the height box. -/

namespace XedniRationalBridge

open XedniSieve4096

private theorem cube_denominator_numerator (a : ℤ) (b : ℕ) (x y : ℚ)
    (hb : 0 < b)
    (hx : x = (a : ℚ) / (b : ℚ)^2)
    (hcurve : y^2 = x^3 + 9*x + 1) :
    ∃ c : ℤ, y * (b : ℚ)^3 = c := by
  have hbq : (b : ℚ) ≠ 0 := by exact_mod_cast (Nat.ne_of_gt hb)
  have hsq : (y * (b : ℚ)^3)^2 =
      ((a^3 + 9*a*(b : ℤ)^4 + (b : ℤ)^6 : ℤ) : ℚ) := by
    calc
      _ = y^2 * (b : ℚ)^6 := by ring
      _ = _ := by
        rw [hcurve, hx]
        push_cast
        field_simp [hbq]
  have hint : IsIntegral ℤ ((y * (b : ℚ)^3)^2) := by
    rw [hsq]
    exact isIntegral_intCast _
  obtain ⟨c, hc⟩ :=
    IsIntegrallyClosed.exists_algebraMap_eq_of_isIntegral_pow
      (R := ℤ) (K := ℚ) (x := y * (b : ℚ)^3) (by omega : 0 < 2) hint
  exact ⟨c, hc.symm⟩

theorem no_x_lift_height4096 (a c : ℤ) (b : ℕ)
    (ha₁ : -4096 ≤ a) (ha₂ : a ≤ 4096)
    (hb₁ : 1 ≤ b) (hb₂ : b ≤ 4096)
    (hcoprime : a.natAbs.Coprime b)
    (hx : (a - 744 * (b : ℤ)^2) % 1009 = 0)
    (hcurve : c^2 = curveRight a b) : False := by
  have hi : b - 1 ∈ List.range 4096 := List.mem_range.mpr (by omega)
  have hcheck : checkB b 4096 = true := by
    have h := (List.all_eq_true.mp check4096) (b - 1) hi
    simpa [fullCheck, Nat.sub_add_cancel hb₁] using h
  have hamem : a ∈ candidateList b :=
    XedniCandidateList.x_candidate_mem a b ha₁ ha₂ hx
  have hpass : rejected a b = true := by
    have h := (List.all_eq_true.mp hcheck) a hamem
    simpa [checkB, ha₁, ha₂, hcoprime] using h
  have hdisj : curveRight a b < 0 ∨
      ∃ m ∈ witnessPrimes, nonsquareMod a b m = true := by
    simpa [rejected] using hpass
  rcases hdisj with hneg | ⟨m, hmem, hnon⟩
  · nlinarith [sq_nonneg c]
  have hm : 0 < m := by
    simp [witnessPrimes] at hmem
    omega
  obtain ⟨z, hz, hroot⟩ :=
    XedniSquareModulo.square_has_modular_root c (curveRight a b) m hm hcurve
  have hreject : ∀ x < m, ¬(m : ℤ) ∣ (x : ℤ)^2 - curveRight a b := by
    simpa [nonsquareMod] using hnon
  exact (hreject z hz) (Int.dvd_iff_emod_eq_zero.mpr hroot)

theorem no_bounded_rational_affine_lift (a : ℤ) (b : ℕ) (x y : ℚ)
    (ha₁ : -4096 ≤ a) (ha₂ : a ≤ 4096)
    (hb₁ : 1 ≤ b) (hb₂ : b ≤ 4096)
    (hcoprime : a.natAbs.Coprime b)
    (hxrepr : x = (a : ℚ) / (b : ℚ)^2)
    (hcurve : y^2 = x^3 + 9*x + 1)
    (hxred : (a - 744 * (b : ℤ)^2) % 1009 = 0) : False := by
  obtain ⟨c, hc⟩ := cube_denominator_numerator a b x y hb₁ hxrepr hcurve
  have hbq : (b : ℚ) ≠ 0 := by exact_mod_cast (Nat.ne_of_gt hb₁)
  have hceq : c^2 = a^3 + 9*a*(b : ℤ)^4 + (b : ℤ)^6 := by
    have hsq : (y * (b : ℚ)^3)^2 =
        ((a^3 + 9*a*(b : ℤ)^4 + (b : ℤ)^6 : ℤ) : ℚ) := by
      calc
        _ = y^2 * (b : ℚ)^6 := by ring
        _ = _ := by
          rw [hcurve, hxrepr]
          push_cast
          field_simp [hbq]
    rw [hc] at hsq
    exact_mod_cast hsq
  exact no_x_lift_height4096 a c b
    ha₁ ha₂ hb₁ hb₂ hcoprime hxred hceq

#print axioms cube_denominator_numerator
#print axioms no_x_lift_height4096
#print axioms no_bounded_rational_affine_lift

end XedniRationalBridge
