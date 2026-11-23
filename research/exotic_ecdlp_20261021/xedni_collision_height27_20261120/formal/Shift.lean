import Mathlib

/-! A p-squared coefficient shift preserves the reduced a4 of a two-point lift. -/

namespace XedniCollisionHeight

variable {R F : Type*} [CommRing R] [Field F]

def onCurve (a1 a2 a3 a4 a6 x y : R) : Prop :=
  y*y+a1*x*y+a3*y = x*x*x+a2*x*x+a4*x+a6

theorem incidence_difference (a1 a2 a3 a4 a6 x1 x2 y1 y2 : R)
    (h1 : onCurve a1 a2 a3 a4 a6 x1 y1)
    (h2 : onCurve a1 a2 a3 a4 a6 x2 y2) :
    a1*(x1*y1-x2*y2)+a3*(y1-y2)-a2*(x1*x1-x2*x2)-a4*(x1-x2) =
      (x1*x1*x1-x2*x2*x2)-(y1*y1-y2*y2) := by
  dsimp [onCurve] at h1 h2
  linear_combination h1 - h2

theorem reduced_a4_of_square_shift [IsDomain R]
    (p u x1 x2 y1 y2 a1 a2 a3 a4 a6 b1 b2 b3 b4 b6 t1 t2 t3 : R)
    (f : R →+* F) (hp : p ≠ 0) (hfp : f p = 0) (hu : f u ≠ 0)
    (hx : x1-x2 = p*u)
    (ha1 : onCurve a1 a2 a3 a4 a6 x1 y1)
    (ha2 : onCurve a1 a2 a3 a4 a6 x2 y2)
    (hb1 : onCurve b1 b2 b3 b4 b6 x1 y1)
    (hb2 : onCurve b1 b2 b3 b4 b6 x2 y2)
    (hs1 : b1-a1 = p*p*t1) (hs2 : b2-a2 = p*p*t2)
    (hs3 : b3-a3 = p*p*t3) :
    f a4 = f b4 := by
  have hA := incidence_difference a1 a2 a3 a4 a6 x1 x2 y1 y2 ha1 ha2
  have hB := incidence_difference b1 b2 b3 b4 b6 x1 x2 y1 y2 hb1 hb2
  have hdiff :
      (b1-a1)*(x1*y1-x2*y2)+(b3-a3)*(y1-y2)
        -(b2-a2)*(x1*x1-x2*x2)-(b4-a4)*(x1-x2) = 0 := by
    linear_combination hB - hA
  have hfactor :
      p * (p*(t1*(x1*y1-x2*y2)+t3*(y1-y2)-t2*(x1*x1-x2*x2))
           -(b4-a4)*u) = 0 := by
    calc
      _ = (b1-a1)*(x1*y1-x2*y2)+(b3-a3)*(y1-y2)
            -(b2-a2)*(x1*x1-x2*x2)-(b4-a4)*(x1-x2) := by
        rw [hs1, hs2, hs3, hx]
        ring
      _ = 0 := hdiff
  have hcancel :
      p*(t1*(x1*y1-x2*y2)+t3*(y1-y2)-t2*(x1*x1-x2*x2))
        -(b4-a4)*u = 0 := (mul_eq_zero.mp hfactor).resolve_left hp
  have hm := congrArg f hcancel
  have hm0 : (f b4 - f a4) * f u = 0 := by
    simpa [map_mul, map_sub, hfp] using hm
  have hba : f b4 - f a4 = 0 := (mul_eq_zero.mp hm0).resolve_right hu
  exact (sub_eq_zero.mp hba).symm

#print axioms incidence_difference
#print axioms reduced_a4_of_square_shift

end XedniCollisionHeight
