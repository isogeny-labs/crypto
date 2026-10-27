/- Exact j=0 S4 resultant identity, generated from the native integer coefficients. -/
import Std.Tactic
set_option maxRecDepth 10000
set_option maxHeartbeats 1000000
namespace TwistS4Identity
variable {R : Type} [Lean.Grind.CommRing R]
def c0 (x y B : R) : R := (x*y)^2 - 4*B*(x+y)
def c1 (x y B : R) : R := -2*((x+y)*(x*y)+2*B)
def c2 (x y : R) : R := (x-y)^2
def result (x y z X B : R) : R :=
  (c2 x y*c0 z X B-c0 x y B*c2 z X)^2 -
  (c2 x y*c1 z X B-c1 x y B*c2 z X)*
  (c1 x y B*c0 z X B-c0 x y B*c1 z X B)
def H (e1 e2 e3 X B : R) : R :=
  (-64 : R) * X^3 * B^3
  + (-512 : R) * e3^1 * B^3
  + (-192 : R) * e3^1 * X^3 * B^2
  + (192 : R) * e3^2 * B^2
  + (-192 : R) * e3^2 * X^3 * B^1
  + (-24 : R) * e3^3 * B^1
  + (-64 : R) * e3^3 * X^3
  + (1 : R) * e3^4
  + (-256 : R) * e2^1 * X^1 * B^3
  + (672 : R) * e2^1 * e3^1 * X^1 * B^2
  + (-48 : R) * e2^1 * e3^2 * X^1 * B^1
  + (-4 : R) * e2^1 * e3^3 * X^1
  + (240 : R) * e2^2 * X^2 * B^2
  + (-24 : R) * e2^2 * e3^1 * X^2 * B^1
  + (6 : R) * e2^2 * e3^2 * X^2
  + (-64 : R) * e2^3 * B^2
  + (-32 : R) * e2^3 * X^3 * B^1
  + (-4 : R) * e2^3 * e3^1 * X^3
  + (1 : R) * e2^4 * X^4
  + (64 : R) * e1^1 * X^2 * B^3
  + (-96 : R) * e1^1 * e3^1 * X^2 * B^2
  + (-168 : R) * e1^1 * e3^2 * X^2 * B^1
  + (-8 : R) * e1^1 * e3^3 * X^2
  + (256 : R) * e1^1 * e2^1 * B^3
  + (64 : R) * e1^1 * e2^1 * X^3 * B^2
  + (32 : R) * e1^1 * e2^1 * e3^1 * B^2
  + (80 : R) * e1^1 * e2^1 * e3^1 * X^3 * B^1
  + (-8 : R) * e1^1 * e2^1 * e3^2 * B^1
  + (16 : R) * e1^1 * e2^1 * e3^2 * X^3
  + (-128 : R) * e1^1 * e2^2 * X^1 * B^2
  + (-8 : R) * e1^1 * e2^2 * X^4 * B^1
  + (16 : R) * e1^1 * e2^2 * e3^1 * X^1 * B^1
  + (-8 : R) * e1^1 * e2^2 * e3^1 * X^4
  + (-8 : R) * e1^1 * e2^3 * X^2 * B^1
  + (64 : R) * e1^2 * X^1 * B^3
  + (16 : R) * e1^2 * X^4 * B^2
  + (-160 : R) * e1^2 * e3^1 * X^1 * B^2
  + (32 : R) * e1^2 * e3^1 * X^4 * B^1
  + (-8 : R) * e1^2 * e3^2 * X^1 * B^1
  + (16 : R) * e1^2 * e3^2 * X^4
  + (-96 : R) * e1^2 * e2^1 * X^2 * B^2
  + (48 : R) * e1^2 * e2^1 * e3^1 * X^2 * B^1
  + (16 : R) * e1^2 * e2^2 * B^2
  + (-8 : R) * e1^2 * e2^2 * X^3 * B^1
  + (-64 : R) * e1^3 * B^3
  + (-32 : R) * e1^3 * X^3 * B^2
  + (32 : R) * e1^3 * e3^1 * X^3 * B^1
  + (32 : R) * e1^3 * e2^1 * X^1 * B^2
  + (16 : R) * e1^4 * X^2 * B^2
theorem symmetrised_resultant (x y z X B : R) :
  result x y z X B = H (x+y+z) (x*y+x*z+y*z) (x*y*z) X B := by
  unfold result c0 c1 c2 H
  grind
#print axioms symmetrised_resultant
def H4 (s1 s2 s3 s4 B : R) : R :=
  (-64 : R) * s4^3
  + (-512 : R) * s3^1 * B^3
  + (192 : R) * s3^2 * B^2
  + (-24 : R) * s3^3 * B^1
  + (1 : R) * s3^4
  + (256 : R) * s2^1 * s4^1 * B^2
  + (32 : R) * s2^1 * s3^1 * s4^1 * B^1
  + (-8 : R) * s2^1 * s3^2 * s4^1
  + (16 : R) * s2^2 * s4^2
  + (-64 : R) * s2^3 * B^2
  + (-192 : R) * s1^1 * s4^2 * B^1
  + (256 : R) * s1^1 * s2^1 * B^3
  + (32 : R) * s1^1 * s2^1 * s3^1 * B^2
  + (-8 : R) * s1^1 * s2^1 * s3^2 * B^1
  + (32 : R) * s1^1 * s2^2 * s4^1 * B^1
  + (-192 : R) * s1^2 * s4^1 * B^2
  + (16 : R) * s1^2 * s2^2 * B^2
  + (-64 : R) * s1^3 * B^3
theorem fully_symmetrised_resultant (x y z X B : R) :
  result x y z X B = H4 (x+y+z+X) (x*y+x*z+x*X+y*z+y*X+z*X)
    (x*y*z+x*y*X+x*z*X+y*z*X) (x*y*z*X) B := by
  unfold result c0 c1 c2 H4
  grind
#print axioms fully_symmetrised_resultant
end TwistS4Identity
