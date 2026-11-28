import FiveForward

namespace FiveTorsionRankAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine ThreeTorsionRankAudit.General
variable {K : Type*} [Field K] [DecidableEq K]

theorem five_root_implies_point (W : WeierstrassCurve.Affine K) {x y : K}
    (hN : W.Nonsingular x y) (hx : (psiFive W).IsRoot x) : (5:ℕ) • Point.some x y hN=0 := by
  let P : W.Point := Point.some x y hN
  obtain ⟨hd,hT⟩ := five_root_data W hN hx
  have hy : y≠W.negY x y := (denominator_ne_iff W x y).mp (five_root_denominator W hN hx)
  cases he : P+P with
  | zero =>
    have hh := Point.add_self_of_Y_ne (h₁:=hN) hy
    change P+P=_ at hh
    rw [he] at hh
    exact (Point.some_ne_zero _ hh.symm).elim
  | some u v hU =>
    have hfirst := actual_double_relation W hN hU hy he
    have helim := twice_double_elimination W x u hd hfirst
    have hz : (psiFive W).eval x=0 := hx
    rw [hz,mul_zero] at helim
    have hrel : doubleNumerator W u=W.Ψ₂Sq.eval u*x :=
      sub_eq_zero.mp ((mul_eq_zero.mp helim).resolve_left (pow_ne_zero 4 hd))
    have hdu : W.Ψ₂Sq.eval u≠0 := by
      rcases W.den_duplication_ne_zero_or_num_duplication_ne_zero hU with hden | hnum
      · simpa only [two_eval] using hden
      · intro he0
        apply hnum
        change doubleNumerator W u=0
        rw [hrel,he0,zero_mul]
    have hv : v≠W.negY u v := by
      apply (W.den_duplication_eq_zero_iff hU.1).not.mp
      simpa only [two_eval] using hdu
    have hw : W.addX u u (W.slope u u v v)=x := by
      apply mul_left_cancel₀ hdu
      exact (actual_double_x W hU hv).symm.trans hrel
    have hcases : Point.some u v hU+Point.some u v hU=P ∨
        Point.some u v hU+Point.some u v hU= -P := by
      rw [Point.add_self_of_Y_ne (h₁:=hU) hv]
      exact Point.X_eq_iff.mp hw
    rcases hcases with hplus | hminus
    · have h4 : (4:ℕ) • P=P := by
        simpa only [show (4:ℕ)=2+2 by decide,add_nsmul,two_nsmul,he] using hplus
      have h3 : (3:ℕ) • P=0 := by
        have hh : (3:ℕ) • P+P=0+P := by
          simpa only [show (4:ℕ)=3+1 by decide,add_nsmul,one_nsmul,zero_add] using h4
        exact add_right_cancel hh
      exact (hT ((three_point_iff W hN).mp h3)).elim
    · have hh : (Point.some u v hU+Point.some u v hU)+P=0 :=
        eq_neg_iff_add_eq_zero.mp hminus
      change (5:ℕ) • P=0
      simpa only [show (5:ℕ)=2+2+1 by decide,add_nsmul,two_nsmul,one_nsmul,he] using hh

/-- Literal actual nonsingular point equivalence, including characteristic five. -/
theorem five_point_iff (W : WeierstrassCurve.Affine K) {x y : K} (hN : W.Nonsingular x y) :
    (5:ℕ) • Point.some x y hN=0 ↔ (psiFive W).IsRoot x :=
  ⟨five_point_implies_root W hN,five_root_implies_point W hN⟩

#print axioms five_point_iff
end
end FiveTorsionRankAudit
