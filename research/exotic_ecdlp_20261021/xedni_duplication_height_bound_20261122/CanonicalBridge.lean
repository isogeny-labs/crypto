import TorsionBridge
import CanonicalLimit

namespace XedniDuplicationCanonicalBridge

open XedniDuplicationBound XedniDuplicationPointBridge
  XedniDuplicationTorsionBridge XedniCanonicalLimit

theorem explicit_fixed_curve_canonical_height :
    ∃ H : XedniCanonicalLimit.FixedCurve.Point → ℝ,
      ∀ P : XedniCanonicalLimit.FixedCurve.Point,
      Filter.Tendsto
        (normalized (fun n => (((2 ^ n : ℕ) • P).naiveHeight)))
        Filter.atTop (nhds (H P)) ∧
      |H P - P.naiveHeight| ≤ (53 / 3 : ℝ) * Real.log 2 ∧
      H ((2 : ℕ) • P) = 4 * H P ∧
      ∀ n : ℤ, H (n • P) = (n : ℝ) ^ 2 * H P := by
  obtain ⟨D, hD, H, hH⟩ := fixed_curve_naive_height_function
  refine ⟨H, ?_⟩
  intro P
  obtain ⟨hlim, _, hdouble, hquad⟩ := hH P
  have hC : 0 ≤ (53 : ℝ) * Real.log 2 := by
    exact mul_nonneg (by norm_num) (Real.log_nonneg (by norm_num))
  have hstep : ∀ n : ℕ,
      |(((2 ^ (n + 1) : ℕ) • P).naiveHeight) -
        4 * (((2 ^ n : ℕ) • P).naiveHeight)| ≤ 53 * Real.log 2 := by
    intro n
    have hnext : ((2 ^ (n + 1) : ℕ) • P) =
        ((2 ^ n : ℕ) • P) + ((2 ^ n : ℕ) • P) := by
      rw [pow_succ', mul_smul, two_nsmul]
    rw [hnext]
    exact all_point_abs_height_defect _
  obtain ⟨L, hL, hcmp⟩ := normalized_limit
    (fun n => (((2 ^ n : ℕ) • P).naiveHeight))
    (53 * Real.log 2) hC hstep
  have hEq : L = H P := tendsto_nhds_unique hL hlim
  subst L
  refine ⟨hlim, ?_, hdouble, hquad⟩
  simpa [div_eq_mul_inv, mul_assoc, mul_left_comm, mul_comm] using hcmp

theorem explicit_fixed_curve_height_gap_lt14 :
    ∃ H : XedniCanonicalLimit.FixedCurve.Point → ℝ,
      ∀ P : XedniCanonicalLimit.FixedCurve.Point,
        |H P - P.naiveHeight| < 14 := by
  obtain ⟨H, hH⟩ := explicit_fixed_curve_canonical_height
  refine ⟨H, ?_⟩
  intro P
  exact lt_of_le_of_lt (hH P).2.1 numerical_height_gap_lt14

#print axioms explicit_fixed_curve_canonical_height
#print axioms explicit_fixed_curve_height_gap_lt14

end XedniDuplicationCanonicalBridge
