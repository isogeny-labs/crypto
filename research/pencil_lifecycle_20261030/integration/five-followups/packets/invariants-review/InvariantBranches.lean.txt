import SameSupport
set_option autoImplicit false
namespace InvariantBranches
open Polynomial
variable {K : Type*} [Field K]

-- This is the field interpretation of weight_degree_two followed by normalize.
theorem weight_decode (q r s t z w : K) (hz : z ≠ 0) (hw : w ≠ 0) :
    (q*z^2)/z^2 = q ∧ (r*z^4)/z^4 = r ∧
    (s*(z^3*w))/(z^3*w) = s ∧ (t*(z^5*w))/(z^5*w) = t := by
  simp [hz, hw]

-- Reweighting preserves the normalized Mumford congruence, not just coordinates.
theorem weight_validity (f : K[X]) (q r s t z w : K)
    (hz : z ≠ 0) (hw : w ≠ 0)
    (hv : (X^2 + C q*X + C r) ∣ f - (C s*X + C t)^2) :
    (X^2 + C ((q*z^2)/z^2)*X + C ((r*z^4)/z^4)) ∣
      f - (C ((s*(z^3*w))/(z^3*w))*X + C ((t*(z^5*w))/(z^5*w)))^2 := by
  simpa [hz, hw] using hv

theorem negation_validity (f u v : K[X]) (h : u ∣ f-v^2) :
    u ∣ f-(-v)^2 := by simpa using h

-- Matches bb and dq/dr expressions after cross scaling, before bb==0 dispatch.
theorem shared_zero_q_implies_same_support (q1 q2 r1 r2 : K)
    (hq : q1-q2 = 0)
    (hb : (q1-q2)*(q2*r1-q1*r2)-(r1-r2)^2 = 0) :
    q1=q2 ∧ r1=r2 := by
  have hq' : q1=q2 := sub_eq_zero.mp hq
  have hr2 : (r1-r2)^2 = 0 := by simpa [hq] using hb
  have hr : r1-r2=0 := mul_self_eq_zero.mp (by simpa [pow_two] using hr2)
  exact ⟨hq', sub_eq_zero.mp hr⟩

theorem shared_zero_q_excluded (q1 q2 r1 r2 : K)
    (hdifferent : ¬ (q1=q2 ∧ r1=r2))
    (hb : (q1-q2)*(q2*r1-q1*r2)-(r1-r2)^2 = 0) : q1-q2 ≠ 0 := by
  intro hq
  exact hdifferent (shared_zero_q_implies_same_support q1 q2 r1 r2 hq hb)

#print axioms weight_decode
#print axioms weight_validity
#print axioms negation_validity
#print axioms shared_zero_q_implies_same_support
#print axioms shared_zero_q_excluded
end InvariantBranches
