import S3Completion
import Mathlib.FieldTheory.IsAlgClosed.Basic
import Mathlib.FieldTheory.Separable
import Mathlib.Algebra.Polynomial.SpecificDegree

namespace ThreeTorsionRankAudit
noncomputable section
open Polynomial
variable {K : Type*} [Field K]

def cubicValue (a b x : K) : K := x^3+a*x+b
def quartic (a b : K) : K[X] :=
  C 3*X^4+(C (6*a)*X^2+C (12*b)*X-C (a^2))

theorem quartic_eval (a b x : K) :
    (quartic a b).eval x=3*x^4+6*a*x^2+12*b*x-a^2 := by
  simp [quartic]; ring

theorem quartic_identity (a b x : K) :
    (quartic a b).eval x=12*x*cubicValue a b x-(3*x^2+a)^2 := by
  rw [quartic_eval]; unfold cubicValue; ring

theorem quartic_cubic_no_common_root (a b : K)
    (hd : 4*a^3+27*b^2≠0) (x : K) (hx : (quartic a b).eval x=0) :
    cubicValue a b x≠0 := by
  intro hg
  have hs : (3*x^2+a)^2=0 := by
    rw [quartic_identity] at hx
    rw [hg,mul_zero,zero_sub,neg_eq_zero] at hx
    exact hx
  have ha : a= -3*x^2 := by
    have hh := (sq_eq_zero_iff).mp hs
    linear_combination hh
  have hb : b=2*x^3 := by
    unfold cubicValue at hg
    rw [ha] at hg
    linear_combination hg
  apply hd
  rw [ha,hb]
  ring

theorem quartic_derivative (a b : K) :
    (quartic a b).derivative=C 12*(X^3+C a*X+C b) := by
  simp [quartic,derivative_add,derivative_sub,derivative_mul,derivative_pow]
  simp only [map_ofNat]
  ring

theorem quartic_natDegree (a b : K) (h3 : (3:K)≠0) :
    (quartic a b).natDegree=4 := by
  unfold quartic
  rw [natDegree_add_eq_left_of_natDegree_lt]
  · simp [h3]
  · compute_degree!; simp [natDegree_C_mul h3]

theorem quartic_separable [IsAlgClosed K] (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    (quartic a b).Separable := by
  rw [separable_def]
  apply (Polynomial.isCoprime_iff_aeval_ne_zero_of_isAlgClosed K K
    (quartic a b) (quartic a b).derivative).mpr
  intro x
  by_cases hx : (quartic a b).eval x=0
  · right
    have hg := quartic_cubic_no_common_root a b hd x hx
    have h12 : (12:K)≠0 := by
      rw [show (12:K)=2*2*3 by ring]
      exact mul_ne_zero (mul_ne_zero h2 h2) h3
    simpa [quartic_derivative,cubicValue] using mul_ne_zero h12 hg
  · left; simpa using hx

theorem quartic_root_card [IsAlgClosed K] (a b : K)
    (hd : 4*a^3+27*b^2≠0) (h2 : (2:K)≠0) (h3 : (3:K)≠0) :
    Fintype.card ((quartic a b).rootSet K)=4 := by
  rw [card_rootSet_eq_natDegree (quartic_separable a b hd h2 h3)
    (IsAlgClosed.splits _),quartic_natDegree a b h3]

#print axioms quartic_root_card
end
end ThreeTorsionRankAudit
