import Tangent

/-! A modular gate for every integer 77-shift of a centered CRT representative. -/

namespace XedniSixPairShift

open XedniCollisionTangent

def residual (k1 k2 x1 x2 y1 y2 : ℤ) : ℤ :=
  (x1-x2)^2*(47*k2+x1+2*x2)-47*k1*(x1-x2)*(y1-y2)-(y1-y2)^2

theorem residual_shift (k1 k2 x1 x2 y1 y2 u v : ℤ) :
    residual (k1+77*u) (k2+77*v) x1 x2 y1 y2 =
      residual k1 k2 x1 x2 y1 y2 +
        (47*77)*((x1-x2)^2*v-(x1-x2)*(y1-y2)*u) := by
  dsimp [residual]
  ring

theorem no_relation_of_shift_mod3619
    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic]
    (k1 k2 x1 x2 y1 y2 u v : ℤ)
    (ha1 : W.a₁ = ((47*(k1+77*u) : ℤ) : ℚ))
    (ha2 : W.a₂ = ((47*(k2+77*v) : ℤ) : ℚ))
    (hP : W.Nonsingular (x1 : ℚ) (y1 : ℚ))
    (hQ : W.Nonsingular (x2 : ℚ) (y2 : ℚ))
    (hmod : residual k1 k2 x1 x2 y1 y2 % (47*77) ≠ 0) :
    (WeierstrassCurve.Affine.Point.some (x1 : ℚ) (y1 : ℚ) hP) +
      (WeierstrassCurve.Affine.Point.some (x2 : ℚ) (y2 : ℚ) hQ) +
      (WeierstrassCurve.Affine.Point.some (x2 : ℚ) (y2 : ℚ) hQ) ≠ 0 := by
  intro hrel
  have hob := obstruction_of_point_relation W _ _ _ _ hP hQ hrel
  rw [ha1, ha2] at hob
  have hzero : residual (k1+77*u) (k2+77*v) x1 x2 y1 y2 = 0 := by
    have hq : ((residual (k1+77*u) (k2+77*v) x1 x2 y1 y2 : ℤ) : ℚ) = 0 := by
      change ((x1-x2 : ℚ)^2*(((47*(k2+77*v) : ℤ) : ℚ)+(x1 : ℚ)+2*(x2 : ℚ))-
        (((47*(k1+77*u) : ℤ) : ℚ))*(x1-x2 : ℚ)*(y1-y2 : ℚ)-
        (y1-y2 : ℚ)^2) = 0 at hob
      simpa only [residual, Int.cast_sub, Int.cast_add, Int.cast_mul, Int.cast_pow,
        Int.cast_ofNat] using hob
    exact_mod_cast hq
  rw [residual_shift] at hzero
  have hdiv : (47*77 : ℤ) ∣ residual k1 k2 x1 x2 y1 y2 := by
    refine ⟨-((x1-x2)^2*v-(x1-x2)*(y1-y2)*u), ?_⟩
    linear_combination hzero
  exact hmod (Int.emod_eq_zero_of_dvd hdiv)

#print axioms residual_shift
#print axioms no_relation_of_shift_mod3619

end XedniSixPairShift
