import CanonicalBridge
import HeightExclusion

namespace XedniDuplicationHeightExclusionBridge

open XedniDuplicationCanonicalBridge XedniDuplicationBound
  XedniCanonicalLimit XedniExplicitHeightExclusion

private theorem Q_nonsingular : XedniCanonicalLimit.FixedCurve.Nonsingular 17 10 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff']
  constructor
  · rw [WeierstrassCurve.Affine.equation_iff]
    norm_num [XedniCanonicalLimit.FixedCurve]
  · norm_num [XedniCanonicalLimit.FixedCurve]

def Q : XedniCanonicalLimit.FixedCurve.Point := .some 17 10 Q_nonsingular

theorem no_bounded_target_class_lift
    (P : XedniCanonicalLimit.FixedCurve.Point) (n : ℤ)
    (hP : P = n • Q) (hmod : n % 43 = 22)
    (hwitness : 2893 * Real.log 2 ≤ ((65 : ℤ) • Q).naiveHeight)
    (hcap : P.naiveHeight ≤ 260 * Real.log 2) : False := by
  obtain ⟨H, hH⟩ := explicit_fixed_curve_canonical_height
  have hQquad := (hH Q).2.2.2
  have h65quad : H ((65 : ℤ) • Q) = 4225 * H Q := by
    have h := hQquad 65
    norm_num at h
    exact h
  have hRquad : H (n • Q) = (n : ℝ) ^ 2 * H Q := hQquad n
  have hgap65 := (hH ((65 : ℤ) • Q)).2.1
  have hgapR := (hH P).2.1
  have hgap : (53 / 3 : ℝ) * Real.log 2 < 14 := numerical_height_gap_lt14
  have h65lower : 2893 * Real.log 2 ≤ H ((65 : ℤ) • Q) + 14 := by
    have h := (abs_le.mp hgap65).1
    linarith
  have hRupper : H (n • Q) ≤ 260 * Real.log 2 + 14 := by
    rw [hP] at hgapR hcap
    have h := (abs_le.mp hgapR).2
    linarith
  have hQnonneg : 0 ≤ H Q := by
    have hlog := (log_two_bounds).1
    rw [h65quad] at h65lower
    linarith
  exact no_lift_under_cap n hmod (H Q) (H ((65 : ℤ) • Q)) (H (n • Q))
    hQnonneg h65quad h65lower hRquad hRupper

#print axioms no_bounded_target_class_lift

end XedniDuplicationHeightExclusionBridge
