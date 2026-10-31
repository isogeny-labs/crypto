import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 1000000
namespace ImageCoordinate
variable {R : Type} [Lean.Grind.CommRing R]
theorem coordinate (x y z w B : R)
    (hw : w^2+w+1=0) (hc : y^2=x^3+B)
    (hl : (w-1)^2*x^2*(z+(1+w)*x)=4*y^2) :
    3*x^2*z = -w^2*(x^3+4*B) := by
  grind
#print axioms coordinate
end ImageCoordinate
