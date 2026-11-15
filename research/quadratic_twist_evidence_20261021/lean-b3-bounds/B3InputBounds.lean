import Mathlib

/-! Cubic membership is an input-equality control, not a solver-degree theorem. -/
namespace B3InputBounds
open Polynomial
noncomputable section
variable {R : Type*} [CommRing R] [Nontrivial R]
def cubic (r s t : R) : R[X] := (X-C r)*(X-C s)*(X-C t)
def s3 (a b y z : R) : R[X] :=
 (X-C y)^2*C (z^2)-C (2*z)*((X+C y)*(X*C y+C a)+C (2*b))+
 (X*C y-C a)^2-C (4*b)*(X+C y)
theorem cubic_monic (r s t : R) : (cubic r s t).Monic := by
 unfold cubic
 monicity!
theorem cubic_degree (r s t : R) : (cubic r s t).degree=3 := by
 unfold cubic
 compute_degree!
theorem s3_degree (a b y z : R) : (s3 a b y z).degree<3 := by
 unfold s3
 compute_degree!
theorem cubic_remainder_identity (a b y z r s t : R) :
 s3 a b y z %ₘ cubic r s t=s3 a b y z := by
 apply (modByMonic_eq_self_iff (cubic_monic r s t)).mpr
 rw [cubic_degree]
 exact s3_degree a b y z
theorem native_value_binding (a b x y z : R) :
 (s3 a b y z).eval x =
 (x-y)^2*z^2-2*z*((x+y)*(x*y+a)+2*b)+(x*y-a)^2-4*b*(x+y) := by
 simp [s3] <;> ring
theorem both_coordinate_remainders (a b x y z r s t u v w : R) :
 s3 a b y z %ₘ cubic r s t=s3 a b y z ∧
 s3 a b x z %ₘ cubic u v w=s3 a b x z := by
 exact ⟨cubic_remainder_identity _ _ _ _ _ _ _,cubic_remainder_identity _ _ _ _ _ _ _⟩
end
end B3InputBounds
