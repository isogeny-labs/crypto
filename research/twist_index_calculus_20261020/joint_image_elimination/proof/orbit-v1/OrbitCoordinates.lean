import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 3000000
namespace JointOrbitCoordinates
variable {R : Type} [Lean.Grind.CommRing R]
theorem phi_cross (u T B w : R) :
    (3*u^2)*(-w^2*(T*u+4*B*u^10)) -
    (3*T)*(-w^2*(u^3+4*B)) = 12*w^2*B*(T-u^12) := by
  grind
theorem phi_on_orbit (u T B w : R) (hu : u^12=T) :
    (3*u^2)*(-w^2*(T*u+4*B*u^10)) =
    (3*T)*(-w^2*(u^3+4*B)) := by
  have h := phi_cross u T B w
  rw [hu] at h
  grind
theorem double_cross (u T B : R) :
    (4*(u^3+B))*(u*(T+8*B^4)-9*B*u^10+9*B^2*u^7-9*B^3*u^4) -
    (4*(T-B^4))*(u^4-8*B*u) = 36*B*u*(T-u^12) := by
  grind
theorem double_on_orbit (u T B : R) (hu : u^12=T) :
    (4*(u^3+B))*(u*(T+8*B^4)-9*B*u^10+9*B^2*u^7-9*B^3*u^4) =
    (4*(T-B^4))*(u^4-8*B*u) := by
  have h := double_cross u T B
  rw [hu] at h
  grind
def quarticD (a b c d : R) : R :=
  256*d^3-192*a*c*d^2-128*b^2*d^2+144*b*c^2*d-27*c^4+
  144*a^2*b*d^2-6*a^2*c^2*d-80*a*b^2*c*d+18*a*b*c^3+
  16*b^4*d-4*b^3*c^2-27*a^4*d^2+18*a^3*b*c*d-
  4*a^3*c^3-4*a^2*b^3*d+a^2*b^2*c^2
theorem quartic_discriminant (x y z w : R) :
    quarticD (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) =
    (x-y)^2*(x-z)^2*(x-w)^2*(y-z)^2*(y-w)^2*(z-w)^2 := by
  unfold quarticD
  grind
theorem quartic_nonrepetition (x y z w : R)
    (h : quarticD (x+y+z+w) (x*y+x*z+x*w+y*z+y*w+z*w)
      (x*y*z+x*y*w+x*z*w+y*z*w) (x*y*z*w) ≠ 0) :
    x≠y ∧ x≠z ∧ x≠w ∧ y≠z ∧ y≠w ∧ z≠w := by
  rw [quartic_discriminant] at h
  grind
#print axioms phi_cross
#print axioms phi_on_orbit
#print axioms double_cross
#print axioms double_on_orbit
#print axioms quartic_discriminant
#print axioms quartic_nonrepetition
end JointOrbitCoordinates
