import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 3000000
namespace SquareRootBridge
variable {R : Type} [Lean.Grind.CommRing R]
def C3 (a b c : R) : R := -512*c+256*a*b-64*a^3
def defect (a b d : R) : R := 64*d-(4*b-a^2)^2
def quartic (a b c d x : R) : R := x^4-a*x^3+b*x^2-c*x+d
def quadratic (a b x : R) : R := 8*x^2-4*a*x+4*b-a^2

theorem square_bridge (a b c d x : R) :
    512*quartic a b c d x - 8*(quadratic a b x)^2 =
    C3 a b c*x + 8*defect a b d := by
  unfold quartic quadratic C3 defect
  grobner
#print axioms square_bridge

theorem quadratic_difference (a b x y : R) :
    quadratic a b x-quadratic a b y=(x-y)*(8*(x+y)-4*a) := by
  unfold quadratic
  grobner
#print axioms quadratic_difference

theorem quartic_factors (x y z w t : R) :
    quartic (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) t =
    (t-x)*(t-y)*(t-z)*(t-w) := by
  unfold quartic
  grobner
#print axioms quartic_factors

theorem quartic_root_is_quadratic_root {F : Type} [Lean.Grind.Field F]
    (a b c d x : F) (h8 : (8:F)≠0) (hc : C3 a b c=0)
    (hs : defect a b d=0) (hx : quartic a b c d x=0) :
    quadratic a b x=0 := by
  have h := square_bridge a b c d x
  rw [hx,hc,hs] at h
  have hp : (8:F)*(quadratic a b x)^2=0 := by grobner
  rcases Lean.Grind.Field.of_mul_eq_zero hp with hbad | hsq
  · exact False.elim (h8 hbad)
  · exact Lean.Grind.Field.of_pow_eq_zero (quadratic a b x) 2 hsq
#print axioms quartic_root_is_quadratic_root

theorem quadratic_cannot_have_three_distinct_roots {F : Type} [Lean.Grind.Field F]
    (a b x y z : F) (h8 : (8:F)≠0)
    (hx : quadratic a b x=0) (hy : quadratic a b y=0) (hz : quadratic a b z=0)
    (hxy : x≠y) (hxz : x≠z) : y=z := by
  have hdiffxy := quadratic_difference a b x y
  rw [hx,hy] at hdiffxy
  have pxy : (x-y)*(8*(x+y)-4*a)=0 := by grobner
  have lxy : 8*(x+y)-4*a=0 := by
    rcases Lean.Grind.Field.of_mul_eq_zero pxy with hbad | hgood
    · exfalso; apply hxy; grobner
    · exact hgood
  have hdiffxz := quadratic_difference a b x z
  rw [hx,hz] at hdiffxz
  have pxz : (x-z)*(8*(x+z)-4*a)=0 := by grobner
  have lxz : 8*(x+z)-4*a=0 := by
    rcases Lean.Grind.Field.of_mul_eq_zero pxz with hbad | hgood
    · exfalso; apply hxz; grobner
    · exact hgood
  have pyz : (8:F)*(y-z)=0 := by grobner
  rcases Lean.Grind.Field.of_mul_eq_zero pyz with hbad | hgood
  · exact False.elim (h8 hbad)
  · grobner
#print axioms quadratic_cannot_have_three_distinct_roots

-- No assumption says the desired conclusion. The hypotheses are the two
-- explicit coefficient conditions still to be derived from the H4 ideal.
theorem no_three_distinct_quartic_roots {F : Type} [Lean.Grind.Field F]
    (a b c d x y z : F) (h8 : (8:F)≠0)
    (hc : C3 a b c=0) (hs : defect a b d=0)
    (hx : quartic a b c d x=0) (hy : quartic a b c d y=0)
    (hz : quartic a b c d z=0) (hxy : x≠y) (hxz : x≠z) (hyz : y≠z) : False := by
  have qx := quartic_root_is_quadratic_root a b c d x h8 hc hs hx
  have qy := quartic_root_is_quadratic_root a b c d y h8 hc hs hy
  have qz := quartic_root_is_quadratic_root a b c d z h8 hc hs hz
  exact hyz (quadratic_cannot_have_three_distinct_roots a b x y z h8 qx qy qz hxy hxz)
#print axioms no_three_distinct_quartic_roots

theorem no_three_distinct_elementary_roots {F : Type} [Lean.Grind.Field F]
    (x y z w : F) (h8 : (8:F)≠0)
    (hc : C3 (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w)=0)
    (hs : defect (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w) (x*y*z*w)=0)
    (hxy : x≠y) (hxz : x≠z) (hyz : y≠z) : False := by
  apply no_three_distinct_quartic_roots
    (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
    (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) x y z h8 hc hs
    ?_ ?_ ?_ hxy hxz hyz
  · rw [quartic_factors]; grobner
  · rw [quartic_factors]; grobner
  · rw [quartic_factors]; grobner
#print axioms no_three_distinct_elementary_roots
end SquareRootBridge
