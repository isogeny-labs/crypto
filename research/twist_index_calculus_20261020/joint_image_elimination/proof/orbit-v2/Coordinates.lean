import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 3000000
namespace JointOrbitCoordinates
variable {R : Type} [Lean.Grind.CommRing R]
theorem phi_cross (u T B w : R) :
    (3*u^2)*(-w^2*(T*u+4*B*u^10)) -
    (3*T)*(-w^2*(u^3+4*B)) = 12*w^2*B*(T-u^12) := by
  grobner
#print axioms phi_cross
theorem phi_on_orbit (u T B w : R) (hu : u^12=T) :
    (3*u^2)*(-w^2*(T*u+4*B*u^10)) =
    (3*T)*(-w^2*(u^3+4*B)) := by
  have h := phi_cross u T B w
  rw [hu] at h
  grind
#print axioms phi_on_orbit
theorem double_cross (u T B : R) :
    (4*(u^3+B))*(u*(T+8*B^4)-9*B*u^10+9*B^2*u^7-9*B^3*u^4) -
    (4*(T-B^4))*(u^4-8*B*u) = 36*B*u*(T-u^12) := by
  grobner
#print axioms double_cross
theorem double_on_orbit (u T B : R) (hu : u^12=T) :
    (4*(u^3+B))*(u*(T+8*B^4)-9*B*u^10+9*B^2*u^7-9*B^3*u^4) =
    (4*(T-B^4))*(u^4-8*B*u) := by
  have h := double_cross u T B
  rw [hu] at h
  grind
#print axioms double_on_orbit
end JointOrbitCoordinates
