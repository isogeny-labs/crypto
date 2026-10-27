import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 3000000
namespace JointQuarticDiscriminant
variable {R : Type} [Lean.Grind.CommRing R]
local instance : Std.Associative (· * · : R → R → R) where
  assoc := Lean.Grind.Semiring.mul_assoc
local instance : Std.Commutative (· * · : R → R → R) where
  comm := Lean.Grind.CommSemiring.mul_comm
def quarticD (a b c d : R) : R :=
  256*d^3-192*a*c*d^2-128*b^2*d^2+144*b*c^2*d-27*c^4+
  144*a^2*b*d^2-6*a^2*c^2*d-80*a*b^2*c*d+18*a*b*c^3+
  16*b^4*d-4*b^3*c^2-27*a^4*d^2+18*a^3*b*c*d-
  4*a^3*c^3-4*a^2*b^3*d+a^2*b^2*c^2
def cubicD (a b c : R) : R :=
  a^2*b^2-4*b^3-4*a^3*c-27*c^2+18*a*b*c

-- Adding one root factors the discriminant. The intermediate statement
-- uses cubic coefficients, avoiding a single enormous four-root expansion.
theorem add_root (a b c w : R) :
    quarticD (a+w) (b+a*w) (c+b*w) (c*w) =
    cubicD a b c * (w^3-a*w^2+b*w-c)^2 := by
  unfold quarticD cubicD
  grobner
#print axioms add_root
theorem cubic_roots (x y z : R) :
    cubicD (x+y+z) (x*y+x*z+y*z) (x*y*z) =
    (x-y)^2*(x-z)^2*(y-z)^2 := by
  unfold cubicD
  grobner
#print axioms cubic_roots
theorem cubic_evaluation (x y z w : R) :
    w^3-(x+y+z)*w^2+(x*y+x*z+y*z)*w-x*y*z =
    (w-x)*(w-y)*(w-z) := by
  grobner
#print axioms cubic_evaluation
theorem product_square (a b c : R) : (a*b*c)^2=a^2*b^2*c^2 := by
  grobner
#print axioms product_square
theorem reverse_square (a b : R) : (a-b)^2=(b-a)^2 := by
  grobner
#print axioms reverse_square
theorem quartic_discriminant (x y z w : R) :
    quarticD (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) =
    (x-y)^2*(x-z)^2*(x-w)^2*(y-z)^2*(y-w)^2*(z-w)^2 := by
  have hb : x*y+x*z+x*w+y*z+y*w+z*w =
      (x*y+x*z+y*z)+(x+y+z)*w := by grobner
  have hc : x*y*z+x*y*w+x*z*w+y*z*w =
      (x*y*z)+(x*y+x*z+y*z)*w := by grobner
  rw [hb, hc, add_root, cubic_roots, cubic_evaluation, product_square,
      reverse_square w x, reverse_square w y, reverse_square w z]
  ac_rfl
#print axioms quartic_discriminant
theorem quartic_nonrepetition (x y z w : R)
    (h : quarticD (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) ≠ 0) :
    x≠y ∧ x≠z ∧ x≠w ∧ y≠z ∧ y≠w ∧ z≠w := by
  rw [quartic_discriminant] at h
  refine ⟨?_, ?_, ?_, ?_, ?_, ?_⟩
  · intro e; subst y; apply h; grobner
  · intro e; subst z; apply h; grobner
  · intro e; subst w; apply h; grobner
  · intro e; subst z; apply h; grobner
  · intro e; subst w; apply h; grobner
  · intro e; subst w; apply h; grobner
#print axioms quartic_nonrepetition
end JointQuarticDiscriminant
