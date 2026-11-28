import FiveDuplication

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine ThreeTorsionRankAudit.General

theorem coprime_five_kill {G : Type*} [AddCommGroup G] (P : G) (n : ℕ)
    (hn : Nat.gcd 5 n=1) (h5 : (5:ℕ) • P=0) (hk : n • P=0) : P=0 := by
  have hd := Nat.dvd_gcd (addOrderOf_dvd_iff_nsmul_eq_zero.mpr h5)
    (addOrderOf_dvd_iff_nsmul_eq_zero.mpr hk)
  rw [hn] at hd
  exact AddMonoid.addOrderOf_eq_one_iff.mp (Nat.dvd_one.mp hd)

variable {K : Type*} [Field K] [DecidableEq K]

theorem five_point_not_inverse (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (ht : (5:ℕ) • Point.some x y hN=0) : y≠W.negY x y := by
  intro hy
  have h2 : (2:ℕ) • Point.some x y hN=0 := by
    simpa only [two_nsmul] using Point.add_self_of_Y_eq (h₁:=hN) hy
  exact Point.some_ne_zero hN (coprime_five_kill _ 2 (by decide) ht h2)

theorem five_point_three_nonzero (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (ht : (5:ℕ) • Point.some x y hN=0) : W.Ψ₃.eval x≠0 := by
  intro hz
  have h3 := (three_point_iff W hN).mpr hz
  exact Point.some_ne_zero hN (coprime_five_kill _ 3 (by decide) ht h3)

theorem five_point_implies_root (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (ht : (5:ℕ) • Point.some x y hN=0) : (psiFive W).IsRoot x := by
  let P : W.Point := Point.some x y hN
  have hP : P≠0 := Point.some_ne_zero hN
  have hy := five_point_not_inverse W hN ht
  have hd : W.Ψ₂Sq.eval x≠0 := by
    rw [two_eval]
    exact (W.den_duplication_eq_zero_iff hN.1).not.mpr hy
  have hT := five_point_three_nonzero W hN ht
  have hQ : P+P≠0 := by
    intro hz
    exact hP (coprime_five_kill P 2 (by decide) ht (by simpa only [two_nsmul] using hz))
  have hfour : (P+P)+(P+P)= -P := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (5:ℕ)=2+2+1 by decide,add_nsmul,two_nsmul,one_nsmul] using ht
  cases he : P+P with
  | zero => exact (hQ he).elim
  | some u v hU =>
    have hfirst := actual_double_relation W hN hU hy he
    have h5Q : (5:ℕ) • Point.some u v hU=0 := by
      rw [←he,nsmul_add,ht,zero_add]
    have hv := five_point_not_inverse W hU h5Q
    have hsecond : Point.some u v hU+Point.some u v hU= -P := by rwa [←he]
    have hX := (Point.some.inj ((Point.add_self_of_Y_ne (h₁:=hU) hv).symm.trans hsecond)).1
    have hsecondRel := actual_double_x W hU hv
    rw [hX] at hsecondRel
    have hz := twice_double_elimination W x u hd hfirst
    rw [hsecondRel,sub_self,mul_zero] at hz
    exact (mul_eq_zero.mp hz.symm).resolve_left (neg_ne_zero.mpr hT)

#print axioms five_point_implies_root
end
end FiveTorsionRankAudit
