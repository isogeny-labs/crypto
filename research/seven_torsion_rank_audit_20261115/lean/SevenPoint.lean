import SevenMultiples
import SevenRootData

namespace SevenTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine FiveTorsionRankAudit ThreeTorsionRankAudit.General

theorem coprime_seven_kill {G : Type*} [AddCommGroup G] (P : G) (n : ℕ)
    (hn : Nat.gcd 7 n=1) (h7 : (7:ℕ) • P=0) (hk : n • P=0) : P=0 := by
  have hd := Nat.dvd_gcd (addOrderOf_dvd_iff_nsmul_eq_zero.mpr h7)
    (addOrderOf_dvd_iff_nsmul_eq_zero.mpr hk)
  rw [hn] at hd
  exact AddMonoid.addOrderOf_eq_one_iff.mp (Nat.dvd_one.mp hd)

variable {K : Type*} [Field K] [DecidableEq K]

theorem seven_point_not_inverse (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (ht : (7:ℕ) • Point.some x y hN=0) : y≠W.negY x y := by
  intro hy
  have h2 : (2:ℕ) • Point.some x y hN=0 := by
    simpa only [two_nsmul] using Point.add_self_of_Y_eq (h₁:=hN) hy
  exact Point.some_ne_zero hN (coprime_seven_kill _ 2 (by decide) ht h2)

theorem seven_point_domain (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (h7 : (7:ℕ) • Point.some x y hN=0) :
    W.Ψ₂Sq.eval x≠0 ∧ W.Ψ₃.eval x≠0 ∧ W.preΨ₄.eval x≠0 := by
  have hy := seven_point_not_inverse W hN h7
  have hd : W.Ψ₂Sq.eval x≠0 := by
    rw [two_eval]
    exact (W.den_duplication_eq_zero_iff hN.1).not.mpr hy
  have ht : W.Ψ₃.eval x≠0 := by
    intro he
    have h3 := (three_point_iff W hN).mpr he
    exact Point.some_ne_zero hN (coprime_seven_kill _ 3 (by decide) h7 h3)
  refine ⟨hd,ht,?_⟩
  let P : W.Point := Point.some x y hN
  let u := W.addX x x (W.slope x x y y)
  let v := W.addY x x y (W.slope x x y y)
  have hQ : W.Nonsingular u v := W.nonsingular_add hN hN (fun hxy => hy hxy.2)
  let Q : W.Point := Point.some u v hQ
  have he : P+P=Q := Point.add_self_of_Y_ne hy
  have h7Q : (7:ℕ) • Q=0 := by rw [←he,nsmul_add,h7,zero_add]
  have hv := seven_point_not_inverse W hQ h7Q
  have hdu : W.Ψ₂Sq.eval u≠0 := by
    rw [two_eval]
    exact (W.den_duplication_eq_zero_iff hQ.1).not.mpr hv
  have hf := actual_double_relation W hN hQ hy he
  have hden := double_denominator_identity W x u hd hf
  intro hh
  rw [hh,zero_pow (by decide : (2:ℕ)≠0)] at hden
  exact mul_ne_zero (pow_ne_zero 3 hd) hdu hden

theorem seven_point_implies_root (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (h7 : (7:ℕ) • Point.some x y hN=0) : (psiSeven W).IsRoot x := by
  obtain ⟨hd,ht,hh⟩ := seven_point_domain W hN h7
  have he : (4:ℕ) • Point.some x y hN= -((3:ℕ) • Point.some x y hN) := by
    apply eq_neg_iff_add_eq_zero.mpr
    simpa only [show (7:ℕ)=4+3 by decide,add_nsmul] using h7
  have hx := congrArg (fun Q : W.Point => Q.xRep) he
  rw [quadruple_xRep W hN hd hh,Point.xRep_neg,triple_xRep W hN hd ht] at hx
  have hx' : quadrupleX W x=tripleX W x := by simpa using congrFun hx (0:Fin 2)
  have hg := multiple_gap W x hd ht hh
  rw [hx',sub_self,mul_zero] at hg
  exact neg_eq_zero.mp hg.symm

theorem seven_root_implies_point (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : (psiSeven W).IsRoot x) : (7:ℕ) • Point.some x y hN=0 := by
  obtain ⟨hd,ht,hh⟩ := seven_root_data W hN hx
  have hz : (psiSeven W).eval x=0 := hx
  have hg := multiple_gap W x hd ht hh
  rw [hz,neg_zero] at hg
  have hx' : quadrupleX W x=tripleX W x := sub_eq_zero.mp
    ((mul_eq_zero.mp hg).resolve_left
      (mul_ne_zero (mul_ne_zero hd (pow_ne_zero 2 hh)) (pow_ne_zero 2 ht)))
  have he : ((4:ℕ) • Point.some x y hN).xRep=((3:ℕ) • Point.some x y hN).xRep := by
    rw [quadruple_xRep W hN hd hh,triple_xRep W hN hd ht,hx']
  rcases Point.eq_or_eq_neg_of_xRep_eq_xRep he with hplus | hminus
  · let P : W.Point := Point.some x y hN
    change (4:ℕ) • P=(3:ℕ) • P at hplus
    have hp : P=0 := by
      have ha : P+(3:ℕ) • P=0+(3:ℕ) • P := by
        calc P+(3:ℕ) • P=(4:ℕ) • P := by
               rw [show (4:ℕ)=1+3 by decide,add_nsmul,one_nsmul]
             _=(3:ℕ) • P := hplus
             _=0+(3:ℕ) • P := (zero_add _).symm
      exact add_right_cancel ha
    exact (Point.some_ne_zero hN hp).elim
  · have ha : (4:ℕ) • Point.some x y hN+(3:ℕ) • Point.some x y hN=0 :=
      eq_neg_iff_add_eq_zero.mp hminus
    simpa only [show (7:ℕ)=4+3 by decide,add_nsmul] using ha

/-- Actual literal seventh-polynomial criterion in every characteristic. -/
theorem seven_point_iff (W : WeierstrassCurve.Affine K) {x y : K} (hN : W.Nonsingular x y) :
    (7:ℕ) • Point.some x y hN=0 ↔ (psiSeven W).IsRoot x :=
  ⟨seven_point_implies_root W hN,seven_root_implies_point W hN⟩

#print axioms seven_point_iff
end
end SevenTorsionRankAudit
