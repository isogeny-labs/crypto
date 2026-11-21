import FullAffine4096
import Mathlib.AlgebraicGeometry.EllipticCurve.Affine.Point

/-! Connect the bounded rational affine theorem to Mathlib's elliptic-curve point type. -/

namespace XedniPointBridge

local instance : Fact (Nat.Prime 1009) := ⟨by norm_num⟩

def E9Q : WeierstrassCurve.Affine ℚ := ⟨0, 0, 0, 9, 1⟩

def BoundedPublicX : E9Q.Point → Prop
  | .zero => False
  | .some x _ _ =>
      -4096 ≤ x.num ∧ x.num ≤ 4096 ∧ x.den.sqrt ≤ 4096 ∧
      ((x.den.sqrt : ℕ) : ZMod 1009) ≠ 0 ∧
      ((x.num : ℤ) : ZMod 1009) /
        (((x.den.sqrt : ℕ) : ZMod 1009)^2) = 744

theorem no_point_with_bounded_public_x (P : E9Q.Point) : ¬ BoundedPublicX P := by
  intro h
  cases P with
  | zero => exact h
  | some x y hn =>
    rcases h with ⟨ha₁, ha₂, hb, hunit, hxred⟩
    have hcurve : y^2 = x^3 + 9*x + 1 := by
      have heq := (E9Q.equation_iff x y).mp hn.1
      simpa [E9Q] using heq
    exact XedniSquareDenominator.no_bounded_affine_with_reduced_x
      x y hcurve ha₁ ha₂ hb hunit hxred

#print axioms no_point_with_bounded_public_x

end XedniPointBridge
