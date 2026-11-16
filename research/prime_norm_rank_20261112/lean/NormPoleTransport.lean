module
public import NormWeightedLeading
import all NormWeightedLeading
import all NormMembershipElimination
public import Mathlib.Algebra.Polynomial.Eval.Degree
public section

namespace NormPoleTransport
open Polynomial

section Ring
variable {R : Type*} [CommRing R]

/-- Explicit inverse-pair arithmetic; no division or field-evaluation argument. -/
theorem inversePowers (e l : R) (he : e*l=1) (m n : ℕ) (hn : n≤m) :
    e^m*l^n=e^(m-n) := by
  calc
    e^m*l^n=e^(m-n)*(e^n*l^n) := by
      rw [← mul_assoc, ← pow_add, Nat.sub_add_cancel hn]
    _ = e^(m-n)*(e*l)^n := by rw [mul_pow]
    _ = e^(m-n) := by rw [he, one_pow, mul_one]

def completedValue (a1 a2 c0 c1 r z x : R) : R :=
  x^3+a2*x^2+a1*x+z*(c0+c1*r)-(r^3+a2*r^2+a1*r)

/-- Target completion respects exactly the four coefficient pole weights. -/
theorem completedValueScale (e l a1 a2 c0 c1 r z x : R) (he : e*l=1) :
    e^6*completedValue (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z (l^2*x)=
      completedValue a1 a2 c0 c1 (e^2*r) (e^3*z) x := by
  have h66 : e^6*l^6=1 := by simpa using inversePowers e l he 6 6 le_rfl
  have h63 : e^6*l^3=e^3 := inversePowers e l he 6 3 (by decide)
  have h61 : e^6*l=e^5 := by simpa using inversePowers e l he 6 1 (by decide)
  have h62 : e^6*l^2=e^4 := inversePowers e l he 6 2 (by decide)
  have h64 : e^6*l^4=e^2 := inversePowers e l he 6 4 (by decide)
  calc
    e^6*completedValue (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z (l^2*x)=
      (e^6*l^6)*x^3+(e^6*l^6)*a2*x^2+(e^6*l^6)*a1*x+
        (e^6*l^3)*z*c0+(e^6*l)*z*c1*r-e^6*r^3-
        (e^6*l^2)*a2*r^2-(e^6*l^4)*a1*r := by
      unfold completedValue
      ring
    _ = completedValue a1 a2 c0 c1 (e^2*r) (e^3*z) x := by
      rw [h66,h63,h61,h62,h64]
      unfold completedValue
      ring

theorem branchValueScale (e l c0 c1 x : R) (he : e*l=1) :
    e^3*(c0*l^3+(c1*l)*(l^2*x))=c0+c1*x := by
  have h33 : e^3*l^3=1 := by simpa using inversePowers e l he 3 3 le_rfl
  calc
    e^3*(c0*l^3+(c1*l)*(l^2*x))=(e^3*l^3)*(c0+c1*x) := by ring
    _ = c0+c1*x := by rw [h33,one_mul]

theorem curveValueScale (e l a b x : R) (he : e*l=1) :
    e^6*((l^2*x)^3+a*(l^2*x)+b)=x^3+e^4*a*x+e^6*b := by
  have h66 : e^6*l^6=1 := by simpa using inversePowers e l he 6 6 le_rfl
  have h62 : e^6*l^2=e^4 := inversePowers e l he 6 2 (by decide)
  calc
    e^6*((l^2*x)^3+a*(l^2*x)+b)=
      (e^6*l^6)*x^3+(e^6*l^2)*a*x+e^6*b := by ring
    _ = x^3+e^4*a*x+e^6*b := by rw [h66,h62,one_mul]

/-- A literal norm identity in every commutative ring with an explicit inverse pair. -/
theorem normValueScale (e l a b a1 a2 c0 c1 r z x : R) (he : e*l=1) :
    e^12*(completedValue (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z (l^2*x)^2-
      ((l^2*x)^3+a*(l^2*x)+b)*(c0*l^3+(c1*l)*(l^2*x))^2)=
    completedValue a1 a2 c0 c1 (e^2*r) (e^3*z) x^2-
      (x^3+e^4*a*x+e^6*b)*(c0+c1*x)^2 := by
  calc
    _ = (e^6*completedValue (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z (l^2*x))^2-
      (e^6*((l^2*x)^3+a*(l^2*x)+b))*(e^3*(c0*l^3+(c1*l)*(l^2*x)))^2 := by ring
    _ = _ := by rw [completedValueScale _ _ _ _ _ _ _ _ _ he,
      curveValueScale _ _ _ _ _ he, branchValueScale _ _ _ _ _ he]

noncomputable def actualA (a1 a2 c0 c1 r z : R) : R[X] :=
  NormMembershipElimination.completeTarget
    (X^3+C a2*X^2+C a1*X) (C c0+C c1*X) r z

/-- Lift the ring expression to an actual polynomial; no finite-field extensionality. -/
theorem completedValuePolynomial (a1 a2 c0 c1 r z : R) :
    completedValue (C a1) (C a2) (C c0) (C c1) (C r) (C z) X=
      actualA a1 a2 c0 c1 r z := by
  simp only [completedValue,actualA,NormMembershipElimination.completeTarget,
    eval_add,eval_pow,eval_X,eval_mul,eval_C,map_add,map_sub,map_mul,map_pow]
  ring

theorem actualAComp (a1 a2 c0 c1 r z t : R) :
    (actualA a1 a2 c0 c1 r z).comp (C t*X)=
      completedValue (C a1) (C a2) (C c0) (C c1) (C r) (C z) (C t*X) := by
  rw [← completedValuePolynomial]
  simp only [completedValue,sub_comp,add_comp,pow_comp,mul_comp,C_comp,X_comp]

/-- Polynomial transport for the actual completed A, not its finite-field evaluations. -/
theorem actualAPoleScale (e l a1 a2 c0 c1 r z : R) (he : e*l=1) :
    C (e^6)*(actualA (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).comp (C (l^2)*X)=
      actualA a1 a2 c0 c1 (e^2*r) (e^3*z) := by
  rw [actualAComp,← completedValuePolynomial]
  have hC : C e*C l=(1 : R[X]) := by rw [← map_mul,he,map_one]
  simpa only [map_mul,map_pow] using
    completedValueScale (C e) (C l) (C a1) (C a2) (C c0) (C c1) (C r) (C z) X hC

theorem branchPoleScale (e l c0 c1 : R) (he : e*l=1) :
    C (e^3)*(C (c0*l^3)+C (c1*l)*X).comp (C (l^2)*X)=C c0+C c1*X := by
  simp only [add_comp,mul_comp,C_comp,X_comp]
  have hC : C e*C l=(1 : R[X]) := by rw [← map_mul,he,map_one]
  simpa only [map_mul,map_pow] using branchValueScale (C e) (C l) (C c0) (C c1) X hC

theorem curvePoleScale (e l a b : R) (he : e*l=1) :
    C (e^6)*(X^3+C a*X+C b).comp (C (l^2)*X)=X^3+C (e^4*a)*X+C (e^6*b) := by
  simp only [add_comp,mul_comp,pow_comp,C_comp,X_comp]
  have hC : C e*C l=(1 : R[X]) := by rw [← map_mul,he,map_one]
  simpa only [map_mul,map_pow] using curveValueScale (C e) (C l) (C a) (C b) X hC

noncomputable def actualNorm (a b a1 a2 c0 c1 r z : R) : R[X] :=
  (actualA a1 a2 c0 c1 r z)^2-(X^3+C a*X+C b)*(C c0+C c1*X)^2

/-- Exact polynomial norm scaling, keeping the original curve and target on the left. -/
theorem normPoleScale (e l a b a1 a2 c0 c1 r z : R) (he : e*l=1) :
    C (e^12)*(actualNorm a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).comp (C (l^2)*X)=
      actualNorm (e^4*a) (e^6*b) a1 a2 c0 c1 (e^2*r) (e^3*z) := by
  have hA := actualAPoleScale e l a1 a2 c0 c1 r z he
  have hB := branchPoleScale e l c0 c1 he
  have hF := curvePoleScale e l a b he
  unfold actualNorm
  rw [sub_comp,pow_comp,mul_comp,pow_comp]
  rw [← hA,← hF,← hB]
  simp only [map_pow]
  ring

theorem targetFactorPoleScale (e l r : R) (he : e*l=1) :
    C (e^2)*(C (l^2)*X-C r)=X-C (e^2*r) := by
  have h22 : e^2*l^2=1 := by simpa using inversePowers e l he 2 2 le_rfl
  calc
    C (e^2)*(C (l^2)*X-C r)=C (e^2*l^2)*X-C (e^2*r) := by
      simp only [map_mul]
      ring
    _ = X-C (e^2*r) := by rw [h22,map_one,one_mul]

end Ring

section Field
variable {K : Type*} [Field K]

/-- Exact Dickson transport at every depth, not merely its two endpoints. -/
theorem masterPoleScale (e l : K) (he : e*l=1) (n : ℕ) :
    C (e^(2^(n+1)))*(NormMembershipElimination.master (2 : K) n).comp (C (l^2)*X)=
      NormWeightedLeading.scaledMaster e n := by
  induction n with
  | zero =>
      have h22 : e^2*l^2=1 := by simpa using inversePowers e l he 2 2 le_rfl
      rw [NormMembershipElimination.master,NormWeightedLeading.scaledMaster,X_comp]
      change C (e^2)*(C (l^2)*X)=X
      rw [← mul_assoc,← map_mul,h22,map_one,one_mul]
  | succ n ih =>
      change C (e^(2^(n+2)))*((NormMembershipElimination.master (2 : K) n)^2-C (2 : K)).comp (C (l^2)*X)=
        (NormWeightedLeading.scaledMaster e n)^2-C (2*e^(2^(n+2)))
      rw [sub_comp,pow_comp,C_comp,← ih]
      have hh : e^(2^(n+2))=(e^(2^(n+1)))^2 := by rw [← pow_mul,pow_succ]
      rw [hh]
      simp only [map_pow,map_mul,map_ofNat]
      ring

theorem terminalPoleScale (e l terminal : K) (he : e*l=1) (n : ℕ) :
    C (e^(2^(n+1)))*(NormMembershipElimination.master (2 : K) n-C terminal).comp (C (l^2)*X)=
      NormWeightedLeading.scaledMaster e n-C (terminal*e^(2^(n+1))) := by
  rw [sub_comp,C_comp,mul_sub,masterPoleScale e l he n]
  simp only [map_mul]
  ring

end Field

section Field
variable {K : Type*} [Field K]

noncomputable def actualSupport (a b a1 a2 c0 c1 r z : K) : K[X] :=
  actualNorm a b a1 a2 c0 c1 r z /ₘ (X-C r)

theorem actualSupportExact (a b a1 a2 c0 c1 r z : K) (hp : z^2=r^3+a*r+b) :
    (X-C r)*actualSupport a b a1 a2 c0 c1 r z=actualNorm a b a1 a2 c0 c1 r z := by
  unfold actualSupport actualNorm actualA
  apply NormMembershipElimination.canonicalNormQuotient _ _ _ r z
  · simpa using hp
  · exact NormMembershipElimination.completionRow _ _ _ _

/-- Actual support quotient transport, derived from the norm identity and monic uniqueness. -/
theorem supportPoleScale (e l a b a1 a2 c0 c1 r z : K) (he : e*l=1)
    (hp : z^2=r^3+a*r+b) :
    C (e^10)*(actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).comp (C (l^2)*X)=
      NormWeightedLeading.scaledSupport e a b r z a1 a2 c0 c1 := by
  have hs := actualSupportExact a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z hp
  have hc := congrArg (fun p : K[X] => p.comp (C (l^2)*X)) hs
  simp only [mul_comp,sub_comp,X_comp,C_comp] at hc
  have ht : actualNorm (e^4*a) (e^6*b) a1 a2 c0 c1 (e^2*r) (e^3*z)=
      (X-C (e^2*r))*(C (e^10)*
        (actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).comp (C (l^2)*X)) := by
    rw [← normPoleScale e l a b a1 a2 c0 c1 r z he,← hc]
    rw [← targetFactorPoleScale e l r he]
    simp only [map_pow]
    ring
  have hq := NormMembershipElimination.normQuotientUnique _ _ (e^2*r) ht
  exact hq

/-- Coefficient-level transport retains the exact 10-2j normalization before cancellation. -/
theorem supportCoefficientScale (e l a b a1 a2 c0 c1 r z : K) (he : e*l=1)
    (hp : z^2=r^3+a*r+b) (j : ℕ) :
    e^10*((actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).coeff j*(l^2)^j)=
      (NormWeightedLeading.scaledSupport e a b r z a1 a2 c0 c1).coeff j := by
  have h := congrArg (fun p : K[X] => p.coeff j)
    (supportPoleScale e l a b a1 a2 c0 c1 r z he hp)
  simpa only [coeff_C_mul,comp_C_mul_X_coeff] using h

/-- The support coefficient normalization is exactly its nominal pole weight. -/
theorem supportNormalizedCoefficient (e l a b a1 a2 c0 c1 r z : K) (he : e*l=1)
    (hp : z^2=r^3+a*r+b) (j : ℕ) (hj : j≤5) :
    e^(10-2*j)*(actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).coeff j=
      (NormWeightedLeading.scaledSupport e a b r z a1 a2 c0 c1).coeff j := by
  have hpower := inversePowers e l he 10 (2*j) (by omega)
  have hc := supportCoefficientScale e l a b a1 a2 c0 c1 r z he hp j
  calc
    _ = (e^10*l^(2*j))*(actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).coeff j := by rw [hpower]
    _ = e^10*((actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).coeff j*(l^2)^j) := by
      rw [pow_mul]
      ring
    _ = _ := hc

theorem actualSupportWindow (a b a1 a2 c0 c1 r z : K) (hp : z^2=r^3+a*r+b) :
    (actualSupport a b a1 a2 c0 c1 r z).Monic ∧
      (actualSupport a b a1 a2 c0 c1 r z).natDegree=5 := by
  have hD : (X^3+C a2*X^2+C a1*X : K[X]).Monic := by monicity <;> norm_num
  have hdD : (X^3+C a2*X^2+C a1*X : K[X]).natDegree=3 := by compute_degree!
  have hA := NormMembershipElimination.completionWindow _ (C c0+C c1*X) r z hD hdD
  unfold actualSupport actualNorm actualA
  apply NormMembershipElimination.supportWindow _ _ _ r z
  · compute_degree!
  · exact hA.1
  · exact hA.2
  · compute_degree!
  · simpa using hp
  · exact NormMembershipElimination.completionRow _ _ _ _

theorem scaledSupportWindow (e a b r z a1 a2 c0 c1 : K) (hp : z^2=r^3+a*r+b) :
    (NormWeightedLeading.scaledSupport e a b r z a1 a2 c0 c1).Monic ∧
      (NormWeightedLeading.scaledSupport e a b r z a1 a2 c0 c1).natDegree=5 := by
  apply actualSupportWindow (e^4*a) (e^6*b) a1 a2 c0 c1 (e^2*r) (e^3*z)
  simpa using NormWeightedLeading.scaledCurveAtTarget e a b r z hp

/-- Actual remainder transport under a linear variable change and explicit unit scaling. -/
theorem remainderPoleScale (e l : K) (he : e*l=1) (U V P : K[X]) (d : ℕ)
    (hU : U.Monic) (hUd : U.natDegree=5) (hV : V.Monic) (hVd : V.natDegree=5)
    (hscale : V=C (e^10)*U.comp (C (l^2)*X)) :
    C (e^d)*(P %ₘ U).comp (C (l^2)*X)=
      (C (e^d)*P.comp (C (l^2)*X)) %ₘ V := by
  have h1010 : e^10*l^10=1 := by simpa using inversePowers e l he 10 10 le_rfl
  have hs : C (e^10)*C (e^d*l^10)=(C (e^d) : K[X]) := by
    rw [← map_mul]
    congr 1
    calc e^10*(e^d*l^10)=e^d*(e^10*l^10) := by ring
         _ = e^d := by rw [h1010,mul_one]
  have hc := congrArg (fun p : K[X] => p.comp (C (l^2)*X)) (modByMonic_add_div P U)
  simp only [add_comp,mul_comp] at hc
  have hd : C (e^d)*P.comp (C (l^2)*X)=
      V*(C (e^d*l^10)*(P /ₘ U).comp (C (l^2)*X))+
        C (e^d)*(P %ₘ U).comp (C (l^2)*X) := by
    rw [hscale,← hc]
    calc
      _ = C (e^d)*(P %ₘ U).comp (C (l^2)*X)+
        (C (e^10)*C (e^d*l^10))*(U.comp (C (l^2)*X)*(P /ₘ U).comp (C (l^2)*X)) := by
          rw [hs]
          ring
      _ = _ := by ring
  have hUn : U≠1 := by
    intro hu
    have hn := congrArg Polynomial.natDegree hu
    rw [hUd,natDegree_one] at hn
    contradiction
  have hR : (P %ₘ U).natDegree<5 := by simpa only [hUd] using natDegree_modByMonic_lt P hU hUn
  have hS : (C (l^2)*X : K[X]).natDegree≤1 := by simpa only [natDegree_X] using natDegree_C_mul_le (l^2) (X : K[X])
  have hcomp : ((P %ₘ U).comp (C (l^2)*X)).natDegree≤(P %ₘ U).natDegree :=
    natDegree_comp_le.trans ((Nat.mul_le_mul_left _ hS).trans_eq (Nat.mul_one _))
  have hrem : (C (e^d)*(P %ₘ U).comp (C (l^2)*X)).degree<V.degree := by
    rw [degree_eq_natDegree hV.ne_zero,hVd]
    apply degree_le_natDegree.trans_lt
    exact_mod_cast (lt_of_le_of_lt ((natDegree_C_mul_le _ _).trans hcomp) hR)
  exact (NormMembershipElimination.divisionUnique _ _ _ _ hV hd hrem).2

/-- Exact transport of the full membership remainder at every Dickson depth. -/
theorem membershipPoleScale (e l a b a1 a2 c0 c1 r z terminal : K) (he : e*l=1)
    (hp : z^2=r^3+a*r+b) (n : ℕ) :
    C (e^(2^(n+1)))*((NormMembershipElimination.master (2 : K) n-C terminal) %ₘ
      actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).comp (C (l^2)*X)=
      NormWeightedLeading.scaledMembership e a b r z a1 a2 c0 c1 terminal n := by
  have hU := actualSupportWindow a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z hp
  have hV := scaledSupportWindow e a b r z a1 a2 c0 c1 hp
  have h := remainderPoleScale e l he _ _ (NormMembershipElimination.master (2 : K) n-C terminal)
    (2^(n+1)) hU.1 hU.2 hV.1 hV.2 (supportPoleScale e l a b a1 a2 c0 c1 r z he hp).symm
  rw [terminalPoleScale e l terminal he n] at h
  exact h

/-- Exact nominal-weight scaling for every coefficient of the actual membership remainder. -/
theorem membershipCoefficientScale (e l a b a1 a2 c0 c1 r z terminal : K) (he : e*l=1)
    (hp : z^2=r^3+a*r+b) (n j : ℕ) (hj : 2*j≤2^(n+1)) :
    e^(2^(n+1)-2*j)*((NormMembershipElimination.master (2 : K) n-C terminal) %ₘ
      actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).coeff j=
      (NormWeightedLeading.scaledMembership e a b r z a1 a2 c0 c1 terminal n).coeff j := by
  have h := congrArg (fun p : K[X] => p.coeff j)
    (membershipPoleScale e l a b a1 a2 c0 c1 r z terminal he hp n)
  simp only [coeff_C_mul,comp_C_mul_X_coeff] at h
  have hw := inversePowers e l he (2^(n+1)) (2*j) hj
  rw [← hw]
  calc
    _ = e^(2^(n+1))*(((NormMembershipElimination.master (2 : K) n-C terminal) %ₘ
      actualSupport a b (a1*l^4) (a2*l^2) (c0*l^3) (c1*l) r z).coeff j*(l^2)^j) := by
        rw [pow_mul]
        ring
    _ = _ := h

end Field
end NormPoleTransport

#print axioms NormPoleTransport.inversePowers
#print axioms NormPoleTransport.completedValueScale
#print axioms NormPoleTransport.branchValueScale
#print axioms NormPoleTransport.curveValueScale
#print axioms NormPoleTransport.normValueScale
#print axioms NormPoleTransport.completedValuePolynomial
#print axioms NormPoleTransport.actualAComp
#print axioms NormPoleTransport.actualAPoleScale
#print axioms NormPoleTransport.branchPoleScale
#print axioms NormPoleTransport.curvePoleScale
#print axioms NormPoleTransport.normPoleScale
#print axioms NormPoleTransport.targetFactorPoleScale
#print axioms NormPoleTransport.masterPoleScale
#print axioms NormPoleTransport.terminalPoleScale
#print axioms NormPoleTransport.actualSupportExact
#print axioms NormPoleTransport.supportPoleScale
#print axioms NormPoleTransport.supportCoefficientScale
#print axioms NormPoleTransport.supportNormalizedCoefficient
#print axioms NormPoleTransport.actualSupportWindow
#print axioms NormPoleTransport.scaledSupportWindow
#print axioms NormPoleTransport.remainderPoleScale
#print axioms NormPoleTransport.membershipPoleScale
#print axioms NormPoleTransport.membershipCoefficientScale
