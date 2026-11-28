import Mathlib
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

namespace XedniFiniteOrder

local instance : Fact (Nat.Prime 47) := ⟨by norm_num⟩
local instance : Fact (Nat.Prime 43) := ⟨by norm_num⟩

def E : WeierstrassCurve.Affine (ZMod 47) := ⟨0, 0, 0, -1, -2⟩

private theorem E_discriminant_ne_zero : E.Δ ≠ 0 := by decide

instance : E.IsElliptic := ⟨(isUnit_iff_ne_zero).2 E_discriminant_ne_zero⟩

private theorem Q_on_curve : E.Nonsingular 17 10 := by
  rw [E.nonsingular_iff']
  constructor
  · exact (E.equation_iff 17 10).2 (by decide)
  · right
    decide

private theorem T_on_curve : E.Nonsingular 10 1 := by
  rw [E.nonsingular_iff']
  constructor
  · exact (E.equation_iff 10 1).2 (by decide)
  · right
    decide

def Q : E.Point := .some 17 10 Q_on_curve
def T : E.Point := .some 10 1 T_on_curve

theorem Q_ne_zero : Q ≠ 0 := by decide

private theorem on_curve (x y : ZMod 47)
    (h : E.Equation x y) (hy : 2 * y ≠ 0) : E.Nonsingular x y := by
  rw [E.nonsingular_iff']
  refine ⟨h, Or.inr ?_⟩
  simpa [E] using hy

def P2 : E.Point := .some 14 40
  (on_curve 14 40 ((E.equation_iff 14 40).2 (by decide)) (by decide))
def P4 : E.Point := .some 44 16
  (on_curve 44 16 ((E.equation_iff 44 16).2 (by decide)) (by decide))
def P8 : E.Point := .some 23 4
  (on_curve 23 4 ((E.equation_iff 23 4).2 (by decide)) (by decide))
def P16 : E.Point := .some 15 31
  (on_curve 15 31 ((E.equation_iff 15 31).2 (by decide)) (by decide))
def P32 : E.Point := .some 29 23
  (on_curve 29 23 ((E.equation_iff 29 23).2 (by decide)) (by decide))

private theorem add_q_q : Q + Q = P2 := by
  have hy : (10 : ZMod 47) ≠ WeierstrassCurve.Affine.negY E 17 10 := by
    decide
  simp only [Q, P2]
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hy]
  change _ = WeierstrassCurve.Affine.Point.some 14 40 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right (show (10 : ZMod 47) ≠ -10 by decide)]
  have hslope : (866 / 20 : ZMod 47) = 1 := by
    apply (div_eq_iff (by decide : (20 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide
private theorem add_p2_p2 : P2 + P2 = P4 := by
  have hy : (40 : ZMod 47) ≠ E.negY 14 40 := by decide
  simp only [P2, P4]
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hy]
  change _ = WeierstrassCurve.Affine.Point.some 44 16 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right (show (40 : ZMod 47) ≠ -40 by decide)]
  have hslope : (587 / 80 : ZMod 47) = 42 := by
    apply (div_eq_iff (by decide : (80 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p4_p4 : P4 + P4 = P8 := by
  have hy : (16 : ZMod 47) ≠ E.negY 44 16 := by decide
  simp only [P4, P8]
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hy]
  change _ = WeierstrassCurve.Affine.Point.some 23 4 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right (show (16 : ZMod 47) ≠ -16 by decide)]
  have hslope : (5807 / 32 : ZMod 47) = 39 := by
    apply (div_eq_iff (by decide : (32 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p8_p8 : P8 + P8 = P16 := by
  have hy : (4 : ZMod 47) ≠ E.negY 23 4 := by decide
  simp only [P8, P16]
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hy]
  change _ = WeierstrassCurve.Affine.Point.some 15 31 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right (show (4 : ZMod 47) ≠ -4 by decide)]
  have hslope : (1586 / 8 : ZMod 47) = 22 := by
    apply (div_eq_iff (by decide : (8 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p16_p16 : P16 + P16 = P32 := by
  have hy : (31 : ZMod 47) ≠ E.negY 15 31 := by decide
  simp only [P16, P32]
  rw [WeierstrassCurve.Affine.Point.add_self_of_Y_ne hy]
  change _ = WeierstrassCurve.Affine.Point.some 29 23 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right (show (31 : ZMod 47) ≠ -31 by decide)]
  have hslope : (674 / 62 : ZMod 47) = 23 := by
    apply (div_eq_iff (by decide : (62 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

def P20 : E.Point := .some 37 29
  (on_curve 37 29 ((E.equation_iff 37 29).2 (by decide)) (by decide))
def P40 : E.Point := .some 22 7
  (on_curve 22 7 ((E.equation_iff 22 7).2 (by decide)) (by decide))
def P42 : E.Point := .some 17 37
  (on_curve 17 37 ((E.equation_iff 17 37).2 (by decide)) (by decide))

private theorem add_p16_p4 : P16 + P4 = P20 := by
  have hx : (15 : ZMod 47) ≠ 44 := by decide
  simp only [P16, P4, P20]
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  change _ = WeierstrassCurve.Affine.Point.some 37 29 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right hx]
  have hslope : (15 / -29 : ZMod 47) = 40 := by
    apply (div_eq_iff (by decide : (-29 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p20_p2 : P20 + P2 = T := by
  have hx : (37 : ZMod 47) ≠ 14 := by decide
  simp only [P20, P2, T]
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  change _ = WeierstrassCurve.Affine.Point.some 10 1 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right hx]
  have hslope : (-11 / 23 : ZMod 47) = 22 := by
    apply (div_eq_iff (by decide : (23 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p32_p8 : P32 + P8 = P40 := by
  have hx : (29 : ZMod 47) ≠ 23 := by decide
  simp only [P32, P8, P40]
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  change _ = WeierstrassCurve.Affine.Point.some 22 7 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right hx]
  have hslope : (19 / 6 : ZMod 47) = 11 := by
    apply (div_eq_iff (by decide : (6 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p40_p2 : P40 + P2 = P42 := by
  have hx : (22 : ZMod 47) ≠ 14 := by decide
  simp only [P40, P2, P42]
  rw [WeierstrassCurve.Affine.Point.add_of_X_ne hx]
  change _ = WeierstrassCurve.Affine.Point.some 17 37 _
  norm_num [WeierstrassCurve.Affine.addX, WeierstrassCurve.Affine.addY,
    WeierstrassCurve.Affine.slope, WeierstrassCurve.Affine.negY,
    WeierstrassCurve.Affine.negAddY, E]
  simp only [ite_eq_right hx]
  have hslope : (-33 / 8 : ZMod 47) = 37 := by
    apply (div_eq_iff (by decide : (8 : ZMod 47) ≠ 0)).2
    decide
  rw [hslope]
  decide

private theorem add_p42_q : P42 + Q = 0 := by
  have hx : (17 : ZMod 47) = 17 := rfl
  have hy : (37 : ZMod 47) = E.negY 17 10 := by decide
  simp only [P42, Q]
  exact WeierstrassCurve.Affine.Point.add_of_Y_eq hx hy

private theorem add_target : P16 + P4 + P2 = T := by
  rw [add_p16_p4, add_p20_p2]

private theorem add_zero_43 : P32 + P8 + P2 + Q = 0 := by
  rw [add_p32_p8, add_p40_p2, add_p42_q]

private theorem pow2_multiples :
    (2 : ℕ) • Q = P2 ∧ (4 : ℕ) • Q = P4 ∧
    (8 : ℕ) • Q = P8 ∧ (16 : ℕ) • Q = P16 ∧
    (32 : ℕ) • Q = P32 := by
  have h2 : (2 : ℕ) • Q = P2 := by simpa [two_nsmul] using add_q_q
  have h4 : (4 : ℕ) • Q = P4 := by
    calc
      _ = (2 : ℕ) • ((2 : ℕ) • Q) := by rw [smul_smul]; norm_num
      _ = P4 := by rw [h2, two_nsmul, add_p2_p2]
  have h8 : (8 : ℕ) • Q = P8 := by
    calc
      _ = (2 : ℕ) • ((4 : ℕ) • Q) := by rw [smul_smul]; norm_num
      _ = P8 := by rw [h4, two_nsmul, add_p4_p4]
  have h16 : (16 : ℕ) • Q = P16 := by
    calc
      _ = (2 : ℕ) • ((8 : ℕ) • Q) := by rw [smul_smul]; norm_num
      _ = P16 := by rw [h8, two_nsmul, add_p8_p8]
  have h32 : (32 : ℕ) • Q = P32 := by
    calc
      _ = (2 : ℕ) • ((16 : ℕ) • Q) := by rw [smul_smul]; norm_num
      _ = P32 := by rw [h16, two_nsmul, add_p16_p16]
  exact ⟨h2, h4, h8, h16, h32⟩

theorem Q_mul22_target : (22 : ℕ) • Q = T := by
  rcases pow2_multiples with ⟨h2, h4, _, h16, _⟩
  calc
    _ = (16 + 4 + 2 : ℕ) • Q := by norm_num
    _ = (16 : ℕ) • Q + (4 : ℕ) • Q + (2 : ℕ) • Q := by simp only [add_nsmul]
    _ = T := by rw [h16, h4, h2, add_target]

theorem Q_mul43_zero : (43 : ℕ) • Q = 0 := by
  rcases pow2_multiples with ⟨h2, _, h8, _, h32⟩
  calc
    _ = (32 + 8 + 2 + 1 : ℕ) • Q := by norm_num
    _ = (32 : ℕ) • Q + (8 : ℕ) • Q + (2 : ℕ) • Q + Q := by
      simp only [add_nsmul, one_nsmul]
    _ = 0 := by rw [h32, h8, h2, add_zero_43]

theorem Q_order : addOrderOf Q = 43 := by
  exact addOrderOf_eq_prime Q_mul43_zero Q_ne_zero

theorem target_residue_class (n : ℤ) (h : n • Q = T) : n % 43 = 22 := by
  have h22 : (22 : ℤ) • Q = T := by
    exact (ofNat_zsmul Q 22).trans Q_mul22_target
  have hzero : (n - 22) • Q = 0 := by
    rw [sub_zsmul, h, h22, add_neg_cancel]
  have hdiv0 : (addOrderOf Q : ℤ) ∣ n - 22 := by
    exact (addOrderOf_dvd_iff_zsmul_eq_zero).2 hzero
  have hdiv : (43 : ℤ) ∣ n - 22 := by simpa [Q_order] using hdiv0
  omega

#print axioms Q_ne_zero
#print axioms Q_mul43_zero
#print axioms Q_mul22_target
#print axioms Q_order
#print axioms target_residue_class

end XedniFiniteOrder
