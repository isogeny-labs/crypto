import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point
import Mathlib.GroupTheory.OrderOfElement
import Mathlib.Algebra.Field.ZMod
import Mathlib.Tactic.NormNum
import Mathlib.Tactic.Ring

/-! A fixed counterexample to proposed point labels. No Neron model is postulated. -/
namespace ComponentLabelGate
open WeierstrassCurve.Affine

instance : Fact (Nat.Prime 59) := ⟨by decide⟩
instance : Fact (Nat.Prime 5) := ⟨by decide⟩

def curve : WeierstrassCurve (ZMod 59) := ⟨1, 0, 0, -64, 288⟩

instance : curve.IsElliptic where
  isUnit := by
    apply isUnit_iff_ne_zero.mpr
    norm_num [curve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
      WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]
    decide

def P : curve.toAffine.Point := .some (x := 11) (y := 28) (by
  rw [← equation_iff_nonsingular, equation_iff]
  norm_num [curve]
  decide)

def Q : curve.toAffine.Point := .some (x := 11) (y := 20) (by
  rw [← equation_iff_nonsingular, equation_iff]
  norm_num [curve]
  decide)

def P2 : curve.toAffine.Point := .some (x := 43) (y := 25) (by
  rw [← equation_iff_nonsingular, equation_iff]
  norm_num [curve]
  decide)

theorem Q_eq_neg_P : Q = -P := by
  simp only [Q, P, Point.neg_some, negY, curve]

theorem P_double : P + P = P2 := by
  have hslope : (271 : ZMod 59) / 67 = 56 :=
    (div_eq_iff (by decide : (67 : ZMod 59) ≠ 0)).2 (by decide)
  norm_num [P, P2, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, curve,
    show (28 : ZMod 59) ≠ -39 from by decide]
  rw [hslope]
  decide

theorem P2_double : P2 + P2 = Q := by
  have hslope : (5458 : ZMod 59) / 93 = 46 :=
    (div_eq_iff (by decide : (93 : ZMod 59) ≠ 0)).2 (by decide)
  norm_num [P2, Q, Point.add_def, Point.add, negY, slope,
    addX, addY, negAddY, curve,
    show (25 : ZMod 59) ≠ -68 from by decide]
  rw [hslope]
  decide

theorem Q_add_P : Q + P = 0 := by rw [Q_eq_neg_P, neg_add_cancel]

theorem P_order : addOrderOf P = 5 := by
  apply addOrderOf_eq_prime
  · change ((2 + 2 + 1 : ℕ) • P) = 0
    simp only [add_nsmul, two_nsmul, one_nsmul, P_double, P2_double, Q_add_P]
  · exact Point.some_ne_zero _

theorem Q_order : addOrderOf Q = 5 := by rw [Q_eq_neg_P, addOrderOf_neg, P_order]

theorem no_compatible_labels :
    ¬ ∃ f : curve.toAffine.Point →+ ZMod 5, f P = 1 ∧ f Q = 3 := by
  rintro ⟨f, hP, hQ⟩
  have h := f.map_neg P
  rw [← Q_eq_neg_P, hP, hQ] at h
  exact (by decide : (3 : ZMod 5) ≠ -1) h

/-- The denominator-cleared nodal parametrization used by the target control. -/
theorem nodal_parameter_identity :
    (Polynomial.X : Polynomial ℤ) ^ 4 + Polynomial.X ^ 3 * (1 - Polynomial.X) -
      Polynomial.X ^ 3 = 0 := by ring

end ComponentLabelGate
