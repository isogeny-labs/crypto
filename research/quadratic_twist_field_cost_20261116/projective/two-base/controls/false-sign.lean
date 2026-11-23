import Mathlib.Tactic
namespace TwistTwoBase
variable {K : Type*} [Field K]
def quadratic (a b c x : K) : K := a*x^2+b*x+c
def linearCover (a b c d e f x : K) : K := (d*b-a*e)*x+(d*c+a*f)
theorem original_input_identity (a b c d e f x : K) :
 d*quadratic a b c x-a*quadratic d e f x=linearCover a b c d e f x := by
 unfold quadratic linearCover
 ring
theorem common_root_is_cover_root (a b c d e f x : K)
 (ha : quadratic a b c x=0) (hb : quadratic d e f x=0) : linearCover a b c d e f x=0 := by
 rw [←original_input_identity,ha,hb]
 ring
theorem filtered_cover_exact (a b c d e f x : K) (h : a ≠ 0) :
 (quadratic a b c x=0 ∧ quadratic d e f x=0) ↔ (linearCover a b c d e f x=0 ∧ quadratic a b c x=0) := by
 constructor
 · rintro ⟨ha,hb⟩
   exact ⟨common_root_is_cover_root _ _ _ _ _ _ _ ha hb,ha⟩
 · rintro ⟨hc,ha⟩
   refine ⟨ha,?_⟩
   have eq := original_input_identity a b c d e f x
   rw [ha,hc] at eq
   have hz : a*quadratic d e f x=0 := by linear_combination -eq
   exact (mul_eq_zero.mp hz).resolve_left h
theorem linear_root_unique (s t x : K) (hs : s ≠ 0) (hx : s*x+t=0) : x = -t/s := by
 apply (eq_div_iff hs).2
 linear_combination hx
theorem proportional_cover (a b c d e f x : K) (ha : a ≠ 0)
 (hs : d*b-a*e=0) (ht : d*c-a*f=0) :
 quadratic a b c x=0 ↔ (quadratic a b c x=0 ∧ quadratic d e f x=0) := by
 constructor
 · intro hx
   apply (filtered_cover_exact a b c d e f x ha).2
   exact ⟨by simp [linearCover,hs,ht],hx⟩
 · exact And.left
theorem factored_membership (a b x : K) : (x-a)*(x-b)=0 ↔ x=a ∨ x=b := by
 rw [mul_eq_zero,sub_eq_zero,sub_eq_zero]
end TwistTwoBase
#print axioms TwistTwoBase.original_input_identity
#print axioms TwistTwoBase.common_root_is_cover_root
#print axioms TwistTwoBase.filtered_cover_exact
#print axioms TwistTwoBase.linear_root_unique
#print axioms TwistTwoBase.proportional_cover
#print axioms TwistTwoBase.factored_membership
