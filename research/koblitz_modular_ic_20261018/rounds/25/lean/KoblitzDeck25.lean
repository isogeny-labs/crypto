import Mathlib.Algebra.CharP.Basic
import Mathlib.Tactic.Ring
import Mathlib.Tactic.LinearCombination

/-! Universal identities for the original quadratic source quotient.
Native source residuals and generic field identification are separate evidence.
No Jacobian/Picard map or order-11 subgroup action is assumed here. -/
namespace KoblitzDeck25
variable {R : Type*} [CommRing R]

def s1 (a b l : R) : R := b+l^2+a*l
def s2 (a b l n : R) : R := a*n+b*l
def s3 (b n : R) : R := n^2+b*n
def deck (a b : R) (p : R×R) : R×R := (p.1+a,p.2+b)

theorem s1_integer_lift (a b l : R) :
    s1 a b (l+a)=s1 a b l+2*a*(l+a) := by unfold s1;ring
theorem s2_integer_lift (a b l n : R) :
    s2 a b (l+a) (n+b)=s2 a b l n+2*a*b := by unfold s2;ring
theorem s3_integer_lift (b n : R) :
    s3 b (n+b)=s3 b n+2*b*(n+b) := by unfold s3;ring

theorem s1_invariant [CharP R 2] (a b l : R) : s1 a b (l+a)=s1 a b l := by
  rw [s1_integer_lift]
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  simp [h2]
theorem s2_invariant [CharP R 2] (a b l n : R) :
    s2 a b (l+a) (n+b)=s2 a b l n := by
  rw [s2_integer_lift]
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  simp [h2]
theorem s3_invariant [CharP R 2] (b n : R) : s3 b (n+b)=s3 b n := by
  rw [s3_integer_lift]
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  simp [h2]

theorem deck_involutive [CharP R 2] (a b : R) : Function.Involutive (deck a b) := by
  intro p
  rcases p with ⟨l,n⟩
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  apply Prod.ext <;> dsimp [deck]
  · linear_combination a*h2
  · linear_combination b*h2

theorem deck_traces [CharP R 2] (a b l n : R) : l+(l+a)=a ∧ n+(n+b)=b := by
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  constructor
  · linear_combination l*h2
  · linear_combination n*h2

theorem deck_norms (a b l n : R) :
    l*(l+a)=s1 a b l-b ∧ n*(n+b)=s3 b n := by
  unfold s1 s3
  constructor <;> ring

theorem mixed_integer_lift (a b l n : R) :
    (s2 a b l n)^2+a*b*s2 a b l n+b^2*(s1 a b l-b)=
      a^2*s3 b n+2*(a*b*n*l+b^2*l^2+a*b^2*l) := by
  unfold s1 s2 s3
  ring

theorem mixed_norm_relation [CharP R 2] (a b l n : R) :
    a^2*s3 b n=(s2 a b l n)^2+a*b*s2 a b l n+b^2*(s1 a b l-b) := by
  rw [mixed_integer_lift]
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  simp [h2]

theorem deck_nontrivial (a b l n : R) (ha : a≠0) : deck a b (l,n)≠(l,n) := by
  intro h
  have hh : l+a=l := congrArg Prod.fst h
  apply ha
  linear_combination hh

theorem quotient_inverse_numerator [CharP R 2] (a b l n : R) :
    s2 a b l n+a*n=b*l := by
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  unfold s2
  linear_combination a*n*h2

theorem quotient_s1_equation [CharP R 2] (a b l n : R) :
    b^2*s1 a b l=b^3+a^2*s3 b n+(s2 a b l n)^2+a*b*s2 a b l n := by
  have h2 : (2:R)=0 := CharP.cast_eq_zero R 2
  linear_combination -(mixed_norm_relation a b l n)-((s2 a b l n)^2+a*b*s2 a b l n)*h2

end KoblitzDeck25
#print axioms KoblitzDeck25.s1_integer_lift
#print axioms KoblitzDeck25.s2_integer_lift
#print axioms KoblitzDeck25.s3_integer_lift
#print axioms KoblitzDeck25.s1_invariant
#print axioms KoblitzDeck25.s2_invariant
#print axioms KoblitzDeck25.s3_invariant
#print axioms KoblitzDeck25.deck_involutive
#print axioms KoblitzDeck25.deck_traces
#print axioms KoblitzDeck25.deck_norms
#print axioms KoblitzDeck25.mixed_integer_lift
#print axioms KoblitzDeck25.mixed_norm_relation
#print axioms KoblitzDeck25.deck_nontrivial
#print axioms KoblitzDeck25.quotient_inverse_numerator
#print axioms KoblitzDeck25.quotient_s1_equation
