import MixedCancellation
set_option autoImplicit false
namespace Degree22Guards
variable {K : Type*} [Field K]
-- Literal native double22_weighted_variant b tree, lines735–737.
def doubleB (q r s t : K) : K := 2*((q*s-t)*t-(r*s)*s)

theorem b_zero_s_zero_forces_t_zero (q r s t : K)
 (hodd : (2:K) ≠ 0) (hb : doubleB q r s t=0) (hs : s=0) : t=0 := by
 have hi : (q*s-t)*t-(r*s)*s=0 := (mul_eq_zero.mp hb).resolve_left hodd
 rw [hs] at hi
 have htt : t*t=0 := by simpa using hi
 exact mul_self_eq_zero.mp htt

-- Native equal arm is entered only after the inverse guard was false.
theorem double_identity_excluded (q r s t z1 z2 w1 w2 s2 t2 : K)
 (hodd : (2:K) ≠ 0)
 (heqS : s*z2^3*w2=s2*z1^3*w1)
 (heqT : t*z2^5*w2=t2*z1^5*w1)
 (hnotInverse : ¬(s*z2^3*w2= -(s2*z1^3*w1) ∧
                   t*z2^5*w2= -(t2*z1^5*w1))) :
 ¬(doubleB q r s t=0 ∧ s=0) := by
 rintro ⟨hb,hs⟩
 have ht := b_zero_s_zero_forces_t_zero q r s t hodd hb hs
 apply hnotInverse
 constructor
 · rw [hs] at heqS ⊢
   simpa using heqS
 · rw [ht] at heqT ⊢
   simpa using heqT

theorem double_b_zero_forces_point_arm (q r s t z1 z2 w1 w2 s2 t2 : K)
 (hodd : (2:K) ≠ 0)
 (heqS : s*z2^3*w2=s2*z1^3*w1)
 (heqT : t*z2^5*w2=t2*z1^5*w1)
 (hnotInverse : ¬(s*z2^3*w2= -(s2*z1^3*w1) ∧
                   t*z2^5*w2= -(t2*z1^5*w1)))
 (hb : doubleB q r s t=0) : s ≠ 0 := by
 intro hs
 exact double_identity_excluded q r s t z1 z2 w1 w2 s2 t2 hodd heqS heqT hnotInverse ⟨hb,hs⟩
end Degree22Guards
