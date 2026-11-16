import Std.Tactic
import Init.GrindInstances.Ring.Fin
set_option autoImplicit false
open Lean.Grind
namespace CubicBasisInjectivity22303

theorem three_coefficient_separation {R : Type} [CommRing R] [IsCharP R 223]
    (a b c d e f : R)
    (h0 : a+b+c=d+e+f)
    (h1 : a+183*b+39*c=d+183*e+39*f)
    (h2 : a+39*b+183*c=d+39*e+183*f) : a=d ∧ b=e ∧ c=f := by
  constructor
  · grind
  · constructor <;> grind

theorem cubic_generator_cancel {R : Type} [CommRing R] [IsCharP R 223]
    (z u v : R) (hz : z^3=3) (h : u*z=v*z) : u=v := by
  have h3 := congrArg (fun t => t*z^2) h
  have hc : 3*u=3*v := by grind
  grind

theorem cubic_squared_generator_cancel {R : Type} [CommRing R] [IsCharP R 223]
    (z u v : R) (hz : z^3=3) (h : u*z^2=v*z^2) : u=v := by
  have h3 := congrArg (fun t => t*z) h
  have hc : 3*u=3*v := by grind
  grind

end CubicBasisInjectivity22303
#print axioms CubicBasisInjectivity22303.three_coefficient_separation
#print axioms CubicBasisInjectivity22303.cubic_generator_cancel
#print axioms CubicBasisInjectivity22303.cubic_squared_generator_cancel
