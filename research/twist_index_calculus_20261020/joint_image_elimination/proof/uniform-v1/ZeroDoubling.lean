import Std.Tactic
namespace ZeroDoubling
variable {R : Type} [Lean.Grind.CommRing R]
theorem folded_triangle (w : R) (h : w^2+w+1=0) :
    (2:R)-1+w+w^2=0 := by
  grobner
#print axioms folded_triangle
end ZeroDoubling
