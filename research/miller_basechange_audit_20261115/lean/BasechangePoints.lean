module
public import BasechangeInfinity

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open WeierstrassCurve.Affine
variable {K L : Type*} [Field K] [Field L] [Algebra K L] [DecidableEq K] [DecidableEq L]

/-- The official actual group homomorphism on points under coefficient-field extension. -/
def pointBaseChange (W : WeierstrassCurve.Affine K) : W.Point →+ (extendedCurve (L := L) W).Point :=
  Point.map (W' := W) (Algebra.ofId K L)

theorem point_base_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (pointBaseChange (L := L) W) :=
  Point.map_injective (W' := W) (Algebra.ofId K L)

theorem point_base_zero (W : WeierstrassCurve.Affine K) :
    pointBaseChange (L := L) W 0 = 0 := rfl

theorem point_base_nsmul (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    pointBaseChange (L := L) W (n • P) = n • pointBaseChange (L := L) W P := by
  simp only [map_nsmul]

omit [DecidableEq K] [DecidableEq L] in
theorem base_nonsingular (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    (extendedCurve (L := L) W).Nonsingular (algebraMap K L x) (algebraMap K L y) :=
  (W.map_nonsingular (f := algebraMap K L) (algebraMap K L).injective x y).mpr h

theorem point_base_some (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Nonsingular x y) :
    pointBaseChange (L := L) W (Point.some x y h) =
      Point.some (algebraMap K L x) (algebraMap K L y) (base_nonsingular (L := L) W h) := rfl

#print axioms point_base_injective
end
end MillerBasechangeAudit
