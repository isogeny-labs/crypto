import FamilyCertificate

namespace MumfordAllPairRunsAudit
noncomputable section
open Polynomial MumfordFamilyPairsAudit MumfordCurveFamilyAudit MumfordOrbitFamilyAudit
  MumfordFamilyCantorAudit MumfordFamilyTransitionsAudit

theorem run_swap (t : ℕ) (a b c : K[X]) (P Q R : Pair t)
    (h : OneStepRun a b c P Q R) : OneStepRun b a c Q P R := by
  obtain ⟨d,U,V,q,A,B,C,F,hd,hU,hA,hB,hC,hbez,hcomp,hnum,hV,hF,hout⟩:=h
  refine ⟨d,U,V,q,B,A,C,F,hd,hU,hB,hA,?_,?_,?_,?_,hV,hF,hout⟩
  · simpa only [add_comm] using hC
  · linear_combination hbez
  · linear_combination hcomp
  · linear_combination hnum

theorem identity_left_run (t : ℕ) (P Q : Pair t) (hPu : P.u=1) (hPv : P.v=0) :
    OneStepRun 1 0 0 P Q Q := by
  obtain ⟨F,hF⟩:=Q.divides
  refine ⟨1,Q.u,Q.v,0,1,Q.u,Q.v,F,monic_one,Q.monic_u,?_,?_,?_,?_,?_,?_,
    Q.degree_v,?_,Or.inl ⟨Q.degree_u,rfl,rfl⟩⟩
  · simp [hPu]
  · ring
  · simp [hPv]
  · simp [hPu]
  · simp [hPu]
  · simp [hPu,hPv]
  · exact hF.symm

theorem same_sign_run (t a b : ℕ) (ha : 0<a) (hb : 0<b)
    (hga : a≤genus t) (hgb : b≤genus t) (s : K) (hs : s^2=1)
    (P Q R : Pair t) (hPu : P.u=X^a) (hPv : P.v=C s)
    (hQu : Q.u=X^b) (hQv : Q.v=C s)
    (hout : (a+b≤genus t ∧ R.u=X^(a+b) ∧ R.v=C s) ∨
      (genus t<a+b ∧ R.u=X^(exponent t-(a+b)) ∧ R.v= -C s)) :
    OneStepRun 0 0 (-C s) P Q R := by
  have hn:=exponent_genus t
  have hp : (X:K[X])^(a+b)*X^(exponent t-(a+b))=X^(exponent t) :=
    x_power_join _ _ _ (by omega)
  have hss : (C s:K[X])^2=1 := by simpa only [map_pow,map_one] using congrArg (C : K→K[X]) hs
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv]
  refine ⟨1,X^(a+b),C s,-C s*X^(exponent t-(a+b)),X^a,X^b,2*C s,
    X^(exponent t-(a+b)),monic_one,Polynomial.monic_X_pow _,?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · linear_combination -2*hss-polynomial_three_zero
  · simp [pow_add]
  · unfold curvePolynomial
    linear_combination -C s*hss+C s*hp-C s*polynomial_three_zero
  · apply lt_of_le_of_lt Polynomial.degree_C_le
    rw [Polynomial.degree_X_pow]
    exact_mod_cast (show 0<a+b by omega)
  · simpa only [curvePolynomial,hss,add_sub_cancel_right] using hp
  · rcases hout with ⟨h,hu,hv⟩|⟨h,hu,hv⟩
    · exact Or.inl ⟨by simpa using h,hu,hv⟩
    · refine Or.inr ⟨by simpa using h,1,0,one_ne_zero,?_,?_⟩
      · rw [hu,map_one,one_mul,hss]
        simpa only [curvePolynomial,add_sub_cancel_right] using hp
      · simp [hv]

theorem opposite_strict_run (t a b : ℕ) (hab : a<b) (hgb : b≤genus t)
    (s : K) (hs : s^2=1) (P Q R : Pair t)
    (hPu : P.u=X^a) (hPv : P.v=C s) (hQu : Q.u=X^b) (hQv : Q.v= -C s)
    (hRu : R.u=X^(b-a)) (hRv : R.v= -C s) : OneStepRun 1 0 0 P Q R := by
  have hx : (X:K[X])^a*X^(b-a)=X^b := x_power_join _ _ _ (by omega)
  have hp : (X:K[X])^(b-a)*X^(exponent t-(b-a))=X^(exponent t) := by
    apply x_power_join
    have hn:=exponent_genus t
    omega
  have hss : (C s:K[X])^2=1 := by simpa only [map_pow,map_one] using congrArg (C : K→K[X]) hs
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨X^a,X^(b-a),-C s,0,1,X^(b-a),0,X^(exponent t-(b-a)),
    Polynomial.monic_X_pow _,Polynomial.monic_X_pow _,?_,hx,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · linear_combination X^a*hx
  · ring
  · apply lt_of_le_of_lt (by simpa only [Polynomial.degree_neg] using (Polynomial.degree_C_le (a:=s)))
    rw [Polynomial.degree_X_pow]
    exact_mod_cast (show 0<b-a by omega)
  · simpa only [curvePolynomial,neg_sq,hss,add_sub_cancel_right] using hp
  · exact Or.inl ⟨by simp only [Polynomial.natDegree_X_pow];omega,rfl,rfl⟩

theorem opposite_cancel_run (t a : ℕ) (s : K) (P Q R : Pair t)
    (hPu : P.u=X^a) (hPv : P.v=C s) (hQu : Q.u=X^a) (hQv : Q.v= -C s)
    (hRu : R.u=1) (hRv : R.v=0) : OneStepRun 1 0 0 P Q R := by
  unfold OneStepRun
  rw [hPu,hPv,hQu,hQv,hRu,hRv]
  refine ⟨X^a,1,0,-C s,1,1,0,curvePolynomial t,
    Polynomial.monic_X_pow _,monic_one,?_,?_,?_,?_,?_,?_,?_,?_,?_⟩
  · ring
  · ring
  · ring
  · ring
  · ring
  · ring
  · simp
  · ring
  · exact Or.inl ⟨by simp,rfl,rfl⟩

#print axioms run_swap
#print axioms identity_left_run
#print axioms same_sign_run
#print axioms opposite_strict_run
#print axioms opposite_cancel_run
end
end MumfordAllPairRunsAudit
