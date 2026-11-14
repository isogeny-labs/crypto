import S3Roots
import Mathlib.Data.ZMod.Basic

/-! A faithful refutation of the rational-point strengthening of S3.
Semaev's actual algebraic-closure statement is NOT refuted. -/
namespace SkepticalAudit
open SemaevS3
private instance primeFive : Fact (Nat.Prime 5) := ⟨by decide⟩

def rationalCurve : WeierstrassCurve.Affine (ZMod 5) := ⟨0,0,0,0,1⟩

theorem rationalCurve_nonsingular : rationalCurve.Δ ≠ 0 := by decide

theorem rational_s3_zero : S3 (0:ZMod 5) 1 3 1 4=0 := by decide

theorem rational_x3_has_no_y : ∀ y : ZMod 5, y^2 ≠ 3^3+1 := by decide

theorem no_rational_point_above_three :
    ¬ ∃ P : rationalCurve.Point, pointX P=some 3 := by
  rintro ⟨P,hP⟩
  cases P with
  | zero => cases hP
  | some x y h =>
    have hx : x=3 := Option.some.inj hP
    subst x
    have he := h.1
    rw [WeierstrassCurve.Affine.equation_iff] at he
    exact rational_x3_has_no_y y (by simpa [rationalCurve] using he)

/-- Concrete contradiction of an unjustified rational-only converse.
The nonzero discriminant is supplied above; this is not a singular model. -/
theorem rational_only_s3_converse_false :
    ¬ ∀ x1 x2 x3 : ZMod 5,
      S3 rationalCurve.a₄ rationalCurve.a₆ x1 x2 x3=0 →
      ∃ P Q R : rationalCurve.Point,
        pointX P=some x1 ∧ pointX Q=some x2 ∧ pointX R=some x3 ∧ P+Q+R=0 := by
  intro h
  obtain ⟨P,Q,R,hP,_,_,_⟩ := h 3 1 4 rational_s3_zero
  exact no_rational_point_above_three ⟨P,hP⟩
end SkepticalAudit
