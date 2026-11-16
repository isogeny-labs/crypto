import Mathlib.FieldTheory.RatFunc.Degree
import Mathlib.Algebra.Polynomial.SpecificDegree
import Mathlib.Tactic

namespace SemaevIrreducibilityAudit
open Polynomial
noncomputable section
variable {K : Type*} [Field K]

/-- Odd polynomial degree prevents a square in the actual rational-function
field, even after multiplying by any nonzero scalar. -/
theorem odd_degree_not_square (p : Polynomial K) (hp : Odd p.natDegree)
    (c : K) (hc : c≠0) (r : RatFunc K) :
    r^2≠RatFunc.C c*algebraMap (Polynomial K) (RatFunc K) p := by
  have hp0 : p≠0 := by
    intro hz
    simpa [hz] using hp
  have hj : algebraMap (Polynomial K) (RatFunc K) p≠0 := by
    intro hz
    apply hp0
    apply IsFractionRing.injective (Polynomial K) (RatFunc K)
    simpa using hz
  have hc0 : RatFunc.C c≠0 := by simpa using hc
  intro he
  have hr0 : r≠0 := by
    intro hz
    rw [hz,zero_pow (by decide : (2:ℕ)≠0)] at he
    exact mul_ne_zero hc0 hj he.symm
  have hd := congrArg RatFunc.intDegree he
  rw [pow_two,RatFunc.intDegree_mul hr0 hr0,RatFunc.intDegree_mul hc0 hj,
    RatFunc.intDegree_C,RatFunc.intDegree_polynomial,zero_add] at hd
  obtain ⟨m,hm⟩ := hp
  omega

def cubic (a b : K) : Polynomial K := X^3+C a*X+C b

theorem cubic_natDegree (a b : K) : (cubic a b).natDegree=3 := by
  unfold cubic
  compute_degree
  all_goals norm_num

theorem cubic_ne_zero (a b : K) : cubic a b≠0 := by
  intro hz
  have hd := cubic_natDegree a b
  rw [hz,natDegree_zero] at hd
  contradiction

theorem cubic_as_ratfunc (a b : K) :
    algebraMap (Polynomial K) (RatFunc K) (cubic a b)=
      RatFunc.X^3+RatFunc.C a*RatFunc.X+RatFunc.C b := by
  simp [cubic,RatFunc.algebraMap_X]

theorem cubic_ratfunc_ne_zero (a b : K) :
    RatFunc.X^3+RatFunc.C a*RatFunc.X+RatFunc.C b≠0 := by
  rw [← cubic_as_ratfunc]
  intro hz
  apply cubic_ne_zero a b
  apply IsFractionRing.injective (Polynomial K) (RatFunc K)
  simpa using hz

theorem cubic_multiple_not_square (a b c : K) (hc : c≠0) (r : RatFunc K) :
    r^2≠RatFunc.C c*(RatFunc.X^3+RatFunc.C a*RatFunc.X+RatFunc.C b) := by
  rw [← cubic_as_ratfunc]
  apply odd_degree_not_square _ _ c hc r
  rw [cubic_natDegree]
  decide

end
end SemaevIrreducibilityAudit
