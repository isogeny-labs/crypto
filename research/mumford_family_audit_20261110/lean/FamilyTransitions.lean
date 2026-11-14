import OrbitFamily
import NormalizedCantor

namespace MumfordFamilyTransitionsAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit
abbrev Pair (t : ℕ) := ReducedPair (genus t) (curvePolynomial t)

theorem x_power_join (a b n : ℕ) (h : a+b=n) :
    (X:K[X])^a*X^b=X^n := by rw [←pow_add,h]

theorem positive_run (t d : ℕ) (hd : 0<d) (hb : d+1≤genus t)
    (P Q R : Pair t) (hPu : P.u=X) (hPv : P.v=1)
    (hQu : Q.u=X^d) (hQv : Q.v=1)
    (hRu : R.u=X^(d+1)) (hRv : R.v=1) : OneStepRun 0 0 2 P Q R := by
  have hp : (X:K[X])^(d+1)*X^(exponent t-(d+1))=X^(exponent t) := by
    apply x_power_join
    have hn:=exponent_genus t
    omega
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨1,X^(d+1),1,2*X^(exponent t-(d+1)),X,X^d,2,
    X^(exponent t-(d+1)),monic_one,Polynomial.monic_X_pow _,?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · norm_num
    linear_combination polynomial_three_zero
  · simp [pow_succ']
  · unfold curvePolynomial
    linear_combination -2*hp+polynomial_three_zero
  · simp only [Polynomial.degree_one,Polynomial.degree_X_pow]
    exact_mod_cast (show 0<d+1 by omega)
  · simpa only [curvePolynomial,one_pow,add_sub_cancel_right] using hp
  · exact Or.inl ⟨by simpa using hb,rfl,rfl⟩

theorem crossing_run (t : ℕ) (P Q R : Pair t)
    (hPu : P.u=X) (hPv : P.v=1)
    (hQu : Q.u=X^(genus t)) (hQv : Q.v=1)
    (hRu : R.u=X^(genus t)) (hRv : R.v= -1) : OneStepRun 0 0 2 P Q R := by
  have hp : (X:K[X])^(genus t+1)*X^(genus t)=X^(exponent t) := by
    apply x_power_join
    rw [exponent_genus]
    omega
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨1,X^(genus t+1),1,2*X^(genus t),X,X^(genus t),2,
    X^(genus t),monic_one,Polynomial.monic_X_pow _,?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · norm_num
    linear_combination polynomial_three_zero
  · simp [pow_succ']
  · unfold curvePolynomial
    linear_combination -2*hp+polynomial_three_zero
  · simp only [Polynomial.degree_one,Polynomial.degree_X_pow]
    exact_mod_cast (show 0<genus t+1 by omega)
  · simpa only [curvePolynomial,one_pow,add_sub_cancel_right] using hp
  · refine Or.inr ⟨by simp,1,0,one_ne_zero,?_,?_⟩
    · simpa only [curvePolynomial,one_pow,add_sub_cancel_right,map_one,one_mul] using hp
    · ring

theorem negative_run (t d : ℕ) (hd : 2≤d) (hb : d≤genus t)
    (P Q R : Pair t) (hPu : P.u=X) (hPv : P.v=1)
    (hQu : Q.u=X^d) (hQv : Q.v= -1)
    (hRu : R.u=X^(d-1)) (hRv : R.v= -1) : OneStepRun 1 0 0 P Q R := by
  have hx : (X:K[X])*X^(d-1)=X^d := by
    rw [←pow_succ']
    congr 1
    omega
  have hp : (X:K[X])^(d-1)*X^(exponent t-(d-1))=X^(exponent t) := by
    apply x_power_join
    have hn:=exponent_genus t
    omega
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨X,X^(d-1),-1,0,1,X^(d-1),0,X^(exponent t-(d-1)),
    Polynomial.monic_X,Polynomial.monic_X_pow _,?_,hx,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · linear_combination X*hx
  · ring
  · simp only [Polynomial.degree_neg,Polynomial.degree_one,Polynomial.degree_X_pow]
    exact_mod_cast (show 0<d-1 by omega)
  · simpa only [curvePolynomial,neg_one_sq,add_sub_cancel_right] using hp
  · exact Or.inl ⟨by simp only [Polynomial.natDegree_X_pow];omega,rfl,rfl⟩

theorem cancel_run (t : ℕ) (P Q R : Pair t)
    (hPu : P.u=X) (hPv : P.v=1)
    (hQu : Q.u=X) (hQv : Q.v= -1)
    (hRu : R.u=1) (hRv : R.v=0) : OneStepRun 1 0 0 P Q R := by
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨X,1,0,-1,1,1,0,curvePolynomial t,Polynomial.monic_X,monic_one,
    ?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · ring
  · ring
  · ring
  · simp
  · ring
  · exact Or.inl ⟨by simp,rfl,rfl⟩

theorem identity_run (t : ℕ) (P Q R : Pair t)
    (hPu : P.u=X) (hPv : P.v=1)
    (hQu : Q.u=1) (hQv : Q.v=0)
    (hRu : R.u=X) (hRv : R.v=1) : OneStepRun 0 1 0 P Q R := by
  have hp : (X:K[X])*X^(exponent t-1)=X^(exponent t) := by
    rw [←pow_succ']
    congr 1
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨1,X,1,0,X,1,1,X^(exponent t-1),monic_one,Polynomial.monic_X,
    ?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · ring
  · ring
  · ring
  · simp
  · simpa only [curvePolynomial,one_pow,add_sub_cancel_right] using hp
  · exact Or.inl ⟨by simp [genus],rfl,rfl⟩

#print axioms positive_run
#print axioms crossing_run
#print axioms negative_run
#print axioms cancel_run
#print axioms identity_run
end
end MumfordFamilyTransitionsAudit
