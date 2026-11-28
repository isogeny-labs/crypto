import Init.Grind.Ring.Basic

/-! Prime-field coordinate transport only; this does not identify a concrete
quartic, primitive generator, cover, Picard functor or Galois descent. -/
set_option autoImplicit false
namespace G3ModelTransport
open Lean.Grind

theorem primitive_shift_square {K : Type} [CommRing K] (g s : K) :
    (g+s)^2=g^2+2*s*g+s^2 := by grind

theorem triangular_moore_transport_inverse {K : Type} [CommRing K]
    (s w0 w1 w2 v0 v1 v2 : K)
    (first : v0=w0) (second : v1=w1+s*w0)
    (third : v2=w2+2*s*w1+s^2*w0) :
    v0=w0 ∧ v1-s*v0=w1 ∧ v2-2*s*v1+s^2*v0=w2 := by grind

#print axioms primitive_shift_square
#print axioms triangular_moore_transport_inverse
end G3ModelTransport
