import Mathlib

/-! A secant-tangent constraint on two points that coincide modulo a prime. -/

namespace XedniSamePoint

variable {R F : Type*} [CommRing R] [IsDomain R] [Field F]

def onCurve (a1 a2 a3 a4 a6 x y : R) : Prop :=
  y*y + a1*x*y + a3*y = x*x*x + a2*x*x + a4*x + a6

def tangentNumerator (x1 x2 y1 y2 u v a1 a2 a3 : R) : R :=
  v*(y1+y2) + a1*(x1*v+y2*u) + a3*v
    -u*(x1*x1+x1*x2+x2*x2) -a2*u*(x1+x2)

theorem tangent_identity (p u v x1 x2 y1 y2 a1 a2 a3 a4 a6 : R)
    (hp : p ≠ 0) (hx : x1-x2 = p*u) (hy : y1-y2 = p*v)
    (h1 : onCurve a1 a2 a3 a4 a6 x1 y1)
    (h2 : onCurve a1 a2 a3 a4 a6 x2 y2) :
    a4*u = tangentNumerator x1 x2 y1 y2 u v a1 a2 a3 := by
  dsimp [onCurve] at h1 h2
  have hdiff :
      (y1*y1 + a1*x1*y1 + a3*y1 - (x1*x1*x1 + a2*x1*x1 + a4*x1 + a6)) -
      (y2*y2 + a1*x2*y2 + a3*y2 - (x2*x2*x2 + a2*x2*x2 + a4*x2 + a6)) = 0 := by
    rw [h1, h2]
    ring
  have hexpand :
      (y1*y1 + a1*x1*y1 + a3*y1 - (x1*x1*x1 + a2*x1*x1 + a4*x1 + a6)) -
      (y2*y2 + a1*x2*y2 + a3*y2 - (x2*x2*x2 + a2*x2*x2 + a4*x2 + a6)) =
      p * (tangentNumerator x1 x2 y1 y2 u v a1 a2 a3 - a4*u) := by
    calc
      _ = (y1-y2)*(y1+y2) + a1*(x1*(y1-y2)+y2*(x1-x2)) + a3*(y1-y2)
            -(x1-x2)*(x1*x1+x1*x2+x2*x2) -a2*(x1-x2)*(x1+x2)
            -a4*(x1-x2) := by ring
      _ = p * (tangentNumerator x1 x2 y1 y2 u v a1 a2 a3 - a4*u) := by
        rw [hx, hy]
        unfold tangentNumerator
        ring
  have hzero : tangentNumerator x1 x2 y1 y2 u v a1 a2 a3 - a4*u = 0 := by
    have hprod : p * (tangentNumerator x1 x2 y1 y2 u v a1 a2 a3 - a4*u) = 0 :=
      hexpand.symm.trans hdiff
    exact (mul_eq_zero.mp hprod).resolve_left hp
  exact (sub_eq_zero.mp hzero).symm

theorem a4_reduction_unique (p u v x1 x2 y1 y2 a1 a2 a3 a4 a6
    b1 b2 b3 b4 b6 : R) (f : R →+* F)
    (hp : p ≠ 0) (hu : f u ≠ 0)
    (hx : x1-x2 = p*u) (hy : y1-y2 = p*v)
    (ha1 : onCurve a1 a2 a3 a4 a6 x1 y1)
    (ha2 : onCurve a1 a2 a3 a4 a6 x2 y2)
    (hb1 : onCurve b1 b2 b3 b4 b6 x1 y1)
    (hb2 : onCurve b1 b2 b3 b4 b6 x2 y2)
    (h1 : f a1 = f b1) (h2 : f a2 = f b2) (h3 : f a3 = f b3) :
    f a4 = f b4 := by
  have hn : f (tangentNumerator x1 x2 y1 y2 u v a1 a2 a3) =
      f (tangentNumerator x1 x2 y1 y2 u v b1 b2 b3) := by
    simp [tangentNumerator, h1, h2, h3]
  have hm : f (a4*u) = f (b4*u) := by
    rw [tangent_identity p u v x1 x2 y1 y2 a1 a2 a3 a4 a6 hp hx hy ha1 ha2,
        tangent_identity p u v x1 x2 y1 y2 b1 b2 b3 b4 b6 hp hx hy hb1 hb2]
    exact hn
  exact mul_right_cancel₀ hu (by simpa only [map_mul] using hm)

#print axioms tangent_identity
#print axioms a4_reduction_unique

end XedniSamePoint
