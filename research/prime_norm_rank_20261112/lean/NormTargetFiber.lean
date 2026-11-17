module
public import NormPoleTransport
import all NormPoleTransport
import all NormWeightedLeading
import all NormMembershipElimination
public section

namespace NormTargetFiber
open Polynomial
section Ring
variable {R : Type*} [CommRing R]

/-- Universal ring version of the existing norm-derivative identity. -/
theorem normDerivativeRing (f A B : R[X]) :
    (A^2-f*B^2).derivative=
      C (2 : R)*A*A.derivative-f.derivative*B^2-C (2 : R)*f*B*B.derivative := by
  simp only [derivative_sub,derivative_mul,derivative_pow]
  norm_num
  ring

/-- Evaluation of the actually computed quotient at an actual root. -/
theorem linearQuotientValue (N : R[X]) (r : R) (hr : N.eval r=0) :
    (N /ₘ (X-C r)).eval r=N.derivative.eval r := by
  have he : (X-C r)*(N /ₘ (X-C r))=N := mul_divByMonic_eq_iff_isRoot.mpr hr
  calc
    (N /ₘ (X-C r)).eval r=(((X-C r)*(N /ₘ (X-C r))).derivative).eval r := by simp
    _ = N.derivative.eval r := by rw [he]

/-- Target incidence factors the actual norm quotient value, without a supplied witness. -/
theorem normQuotientTargetFactor (f A B : R[X]) (r z : R)
    (hp : z^2=f.eval r) (ht : A.eval r=z*B.eval r) :
    ((A^2-f*B^2) /ₘ (X-C r)).eval r=
      B.eval r*(2*z*A.derivative.eval r-f.derivative.eval r*B.eval r-2*z^2*B.derivative.eval r) := by
  rw [linearQuotientValue _ r (NormMembershipElimination.normAtTarget f A B r z hp ht),
    normDerivativeRing]
  simp only [eval_sub,eval_mul,eval_C,eval_pow,ht,← hp]
  ring

def polar (a a1 a2 c0 c1 r z : R) : R :=
  2*z*(3*r^2+2*a2*r+a1)-(3*r^2+a)*(c0+c1*r)-2*z^2*c1

noncomputable def support (a b a1 a2 c0 c1 r z : R) : R[X] :=
  NormPoleTransport.actualNorm a b a1 a2 c0 c1 r z /ₘ (X-C r)

/-- The two target factors are affine in the actual four retained coefficients. -/
theorem supportTargetValue (a b a1 a2 c0 c1 r z : R) (hp : z^2=r^3+a*r+b) :
    (support a b a1 a2 c0 c1 r z).eval r=(c0+c1*r)*polar a a1 a2 c0 c1 r z := by
  unfold support NormPoleTransport.actualNorm NormPoleTransport.actualA
  rw [normQuotientTargetFactor _ _ _ r z (by simpa using hp)
    (NormMembershipElimination.completionRow _ _ _ _)]
  simp [polar,NormMembershipElimination.completeTarget,derivative_add,derivative_mul,derivative_pow]
  ring

theorem remainderTargetIdentity (a b a1 a2 c0 c1 r z : R) (hp : z^2=r^3+a*r+b) (P : R[X]) :
    (P %ₘ support a b a1 a2 c0 c1 r z).eval r=
      P.eval r-(c0+c1*r)*polar a a1 a2 c0 c1 r z*(P /ₘ support a b a1 a2 c0 c1 r z).eval r := by
  have h := congrArg (fun p : R[X] => p.eval r) (modByMonic_add_div P (support a b a1 a2 c0 c1 r z))
  simp only [eval_add,eval_mul,supportTargetValue a b a1 a2 c0 c1 r z hp] at h
  linear_combination h

noncomputable def scaledSupport (e a b r z a1 a2 c0 c1 : R) : R[X] :=
  support (e^4*a) (e^6*b) a1 a2 c0 c1 (e^2*r) (e^3*z)

def scaledPolar (e a r z a1 a2 c0 c1 : R) : R :=
  2*z*a1+4*e^2*z*r*a2+6*e^4*z*r^2-
    e*(3*r^2+a)*(c0+c1*e^2*r)-2*e^3*z^2*c1

theorem scaledCurveEquation (e a b r z : R) (hp : z^2=r^3+a*r+b) :
    (e^3*z)^2=(e^2*r)^3+(e^4*a)*(e^2*r)+e^6*b := by
  linear_combination e^6*hp

/-- Uniform third-order factor in the actual support evaluated at the scaled target. -/
theorem scaledSupportTargetValue (e a b r z a1 a2 c0 c1 : R) (hp : z^2=r^3+a*r+b) :
    (scaledSupport e a b r z a1 a2 c0 c1).eval (e^2*r)=
      e^3*(c0+c1*e^2*r)*scaledPolar e a r z a1 a2 c0 c1 := by
  unfold scaledSupport
  rw [supportTargetValue _ _ _ _ _ _ _ _ (scaledCurveEquation e a b r z hp)]
  unfold polar scaledPolar
  ring

noncomputable def master (e : R) : ℕ → R[X]
  | 0 => X
  | n+1 => master e n^2-C (2*e^(2^(n+2)))

/-- Target evaluation of the entire scaled master, including parameter zero. -/
theorem masterAtScaledTarget (e r : R) (n : ℕ) :
    (master e n).eval (e^2*r)=e^(2^(n+1))*(NormMembershipElimination.master (2 : R) n).eval r := by
  induction n with
  | zero => simp [master,NormMembershipElimination.master]
  | succ n ih =>
      simp only [master,NormMembershipElimination.master,eval_sub,eval_pow,eval_C,ih]
      have hh : e^(2^(n+2))=(e^(2^(n+1)))^2 := by rw [← pow_mul,pow_succ]
      rw [hh]
      ring

noncomputable def dividend (e terminal : R) (n : ℕ) : R[X] :=
  master e n-C (terminal*e^(2^(n+1)))

noncomputable def quotient (e a b r z a1 a2 c0 c1 terminal : R) (n : ℕ) : R[X] :=
  dividend e terminal n /ₘ scaledSupport e a b r z a1 a2 c0 c1

noncomputable def membership (e a b r z a1 a2 c0 c1 terminal : R) (n : ℕ) : R[X] :=
  dividend e terminal n %ₘ scaledSupport e a b r z a1 a2 c0 c1

/-- Explicit e^3 factor in the actually computed target-evaluated membership remainder. -/
theorem membershipTargetFactor (e a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    (membership e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)=
      e^3*(e^(2^(n+1)-3)*((NormMembershipElimination.master (2 : R) n).eval r-terminal)-
        (c0+c1*e^2*r)*scaledPolar e a r z a1 a2 c0 c1*
          (quotient e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)) := by
  have h := congrArg (fun p : R[X] => p.eval (e^2*r))
    (modByMonic_add_div (dividend e terminal n) (scaledSupport e a b r z a1 a2 c0 c1))
  simp only [eval_add,eval_mul,scaledSupportTargetValue e a b r z a1 a2 c0 c1 hp] at h
  have ht : (dividend e terminal n).eval (e^2*r)=
      e^(2^(n+1))*((NormMembershipElimination.master (2 : R) n).eval r-terminal) := by
    simp only [dividend,eval_sub,eval_C,masterAtScaledTarget]
    ring
  rw [ht] at h
  have he : e^(2^(n+1))=e^3*e^(2^(n+1)-3) := by
    rw [← pow_add]
    congr 1
    omega
  unfold membership quotient
  rw [he] at h
  linear_combination h

/-- Actual divisibility, valid in parameter polynomial rings as well as fields. -/
theorem membershipTargetDivisibility (e a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    e^3 ∣ (membership e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r) := by
  use e^(2^(n+1)-3)*((NormMembershipElimination.master (2 : R) n).eval r-terminal)-
    (c0+c1*e^2*r)*scaledPolar e a r z a1 a2 c0 c1*
      (quotient e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)
  exact membershipTargetFactor e a b r z a1 a2 c0 c1 terminal hp n hd

/-- Actual parameter-polynomial divisibility; no finite-field evaluation argument. -/
theorem polynomialParameterDivisibility (a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    (X : R[X])^3 ∣
      (membership (X : R[X]) (C a) (C b) (C r) (C z) (C a1) (C a2) (C c0) (C c1) (C terminal) n).eval
        ((X : R[X])^2*C r) := by
  have hC := congrArg (fun t : R => (C t : R[X])) hp
  simp only [map_pow,map_add,map_mul] at hC
  exact membershipTargetDivisibility (X : R[X]) (C a) (C b) (C r) (C z)
    (C a1) (C a2) (C c0) (C c1) (C terminal) hC n hd

theorem normWindowRing [Nontrivial R] (a b a1 a2 c0 c1 r z : R) :
    (NormPoleTransport.actualNorm a b a1 a2 c0 c1 r z).Monic ∧
      (NormPoleTransport.actualNorm a b a1 a2 c0 c1 r z).natDegree=6 := by
  constructor
  · unfold NormPoleTransport.actualNorm NormPoleTransport.actualA NormMembershipElimination.completeTarget
    monicity <;> norm_num
  · unfold NormPoleTransport.actualNorm NormPoleTransport.actualA NormMembershipElimination.completeTarget
    compute_degree!

theorem supportWindowRing [Nontrivial R] (a b a1 a2 c0 c1 r z : R)
    (hp : z^2=r^3+a*r+b) :
    (support a b a1 a2 c0 c1 r z).Monic ∧ (support a b a1 a2 c0 c1 r z).natDegree=5 := by
  have hN := normWindowRing a b a1 a2 c0 c1 r z
  have he : (X-C r)*support a b a1 a2 c0 c1 r z=NormPoleTransport.actualNorm a b a1 a2 c0 c1 r z := by
    unfold support NormPoleTransport.actualNorm NormPoleTransport.actualA
    apply NormMembershipElimination.canonicalNormQuotient _ _ _ r z
    · simpa using hp
    · exact NormMembershipElimination.completionRow _ _ _ _
  refine ⟨(monic_X_sub_C r).of_mul_monic_left (he.symm ▸ hN.1),?_⟩
  unfold support
  rw [natDegree_divByMonic _ (monic_X_sub_C r),hN.2,natDegree_X_sub_C]

theorem scaledSupportWindowRing [Nontrivial R] (e a b r z a1 a2 c0 c1 : R)
    (hp : z^2=r^3+a*r+b) :
    (scaledSupport e a b r z a1 a2 c0 c1).Monic ∧
      (scaledSupport e a b r z a1 a2 c0 c1).natDegree=5 :=
  supportWindowRing _ _ _ _ _ _ _ _ (scaledCurveEquation e a b r z hp)

/-- Target evaluation replaces the constant row without changing a bounded remainder's zero set. -/
theorem targetRowsIff (V : R[X]) (q : R) (hV : V.natDegree≤4) :
    V=0 ↔ V.eval q=0 ∧ ∀ j, 1≤j → j≤4 → V.coeff j=0 := by
  constructor
  · intro hz
    simp [hz]
  · rintro ⟨he,hc⟩
    have h1 := hc 1 (by decide) (by decide)
    have h2 := hc 2 (by decide) (by decide)
    have h3 := hc 3 (by decide) (by decide)
    have h4 := hc 4 (by decide) (by decide)
    have hexp := congrArg (fun p : R[X] => p.eval q)
      (V.as_sum_range_C_mul_X_pow' (by omega : V.natDegree<5))
    have hzero : V.coeff 0=0 := by
      simpa [Finset.sum_range_succ,h1,h2,h3,h4,he] using hexp.symm
    ext j
    rw [coeff_zero]
    by_cases hj : j=0
    · simpa only [hj] using hzero
    · by_cases hs : j≤4
      · exact hc j (by omega) hs
      · exact coeff_eq_zero_of_natDegree_lt (by omega)

theorem membershipTargetRowsIff [Nontrivial R] (e a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) :
    membership e a b r z a1 a2 c0 c1 terminal n=0 ↔
      (membership e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)=0 ∧
        ∀ j, 1≤j → j≤4 → (membership e a b r z a1 a2 c0 c1 terminal n).coeff j=0 := by
  apply targetRowsIff
  have hu := scaledSupportWindowRing e a b r z a1 a2 c0 c1 hp
  have hn : scaledSupport e a b r z a1 a2 c0 c1≠1 := by
    intro hh
    have hh' := congrArg Polynomial.natDegree hh
    rw [hu.2,natDegree_one] at hh'
    contradiction
  have hd := natDegree_modByMonic_lt (dividend e terminal n) hu.1 hn
  rw [hu.2] at hd
  exact Nat.le_of_lt_succ hd

end Ring

section BaseChange
variable {R S : Type*} [CommRing R] [CommRing S]

theorem mapActualA (φ : R →+* S) (a1 a2 c0 c1 r z : R) :
    (NormPoleTransport.actualA a1 a2 c0 c1 r z).map φ=
      NormPoleTransport.actualA (φ a1) (φ a2) (φ c0) (φ c1) (φ r) (φ z) := by
  simp [NormPoleTransport.actualA,NormMembershipElimination.completeTarget]

theorem mapSupport (φ : R →+* S) (a b a1 a2 c0 c1 r z : R) :
    (support a b a1 a2 c0 c1 r z).map φ=
      support (φ a) (φ b) (φ a1) (φ a2) (φ c0) (φ c1) (φ r) (φ z) := by
  unfold support
  rw [map_divByMonic φ (monic_X_sub_C r)]
  simp [NormPoleTransport.actualNorm,mapActualA]

theorem mapMaster (φ : R →+* S) (e : R) (n : ℕ) :
    (master e n).map φ=master (φ e) n := by
  induction n with
  | zero => simp [master]
  | succ n ih => simp [master,ih,map_ofNat]

theorem mapMembership [Nontrivial R] (φ : R →+* S) (e a b r z a1 a2 c0 c1 terminal : R)
    (hp : z^2=r^3+a*r+b) (n : ℕ) :
    (membership e a b r z a1 a2 c0 c1 terminal n).map φ=
      membership (φ e) (φ a) (φ b) (φ r) (φ z) (φ a1) (φ a2) (φ c0) (φ c1) (φ terminal) n := by
  unfold membership
  rw [map_modByMonic φ (scaledSupportWindowRing e a b r z a1 a2 c0 c1 hp).1]
  simp [dividend,scaledSupport,mapSupport,mapMaster]

end BaseChange

section Field
variable {K : Type*} [Field K]

theorem scaledSupportField (e a b r z a1 a2 c0 c1 : K) :
    scaledSupport e a b r z a1 a2 c0 c1=
      NormWeightedLeading.scaledSupport e a b r z a1 a2 c0 c1 := by rfl

theorem masterField (e : K) (n : ℕ) : master e n=NormWeightedLeading.scaledMaster e n := by
  induction n with
  | zero => rfl
  | succ n ih => simp only [master,NormWeightedLeading.scaledMaster,ih]

theorem membershipField (e a b r z a1 a2 c0 c1 terminal : K) (n : ℕ) :
    membership e a b r z a1 a2 c0 c1 terminal n=
      NormWeightedLeading.scaledMembership e a b r z a1 a2 c0 c1 terminal n := by
  unfold membership dividend NormWeightedLeading.scaledMembership
  rw [masterField,scaledSupportField]

/-- Specialization of the parameter-polynomial family is exactly the existing field model. -/
theorem parameterSpecialization (e a b r z a1 a2 c0 c1 terminal : K)
    (hp : z^2=r^3+a*r+b) (n : ℕ) :
    (membership (X : K[X]) (C a) (C b) (C r) (C z) (C a1) (C a2) (C c0) (C c1) (C terminal) n).map
      (evalRingHom e)=NormWeightedLeading.scaledMembership e a b r z a1 a2 c0 c1 terminal n := by
  have hC := congrArg (fun t : K => (C t : K[X])) hp
  simp only [map_pow,map_add,map_mul] at hC
  rw [mapMembership _ _ _ _ _ _ _ _ _ _ _ hC n]
  simpa using membershipField e a b r z a1 a2 c0 c1 terminal n

/-- Removing the known external parameter factor preserves the actual target equation. -/
theorem normalizedTargetEquation (e a b r z a1 a2 c0 c1 terminal : K) (he : e≠0)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    (membership e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)=0 ↔
      e^(2^(n+1)-3)*((NormMembershipElimination.master (2 : K) n).eval r-terminal)-
        (c0+c1*e^2*r)*scaledPolar e a r z a1 a2 c0 c1*
          (quotient e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)=0 := by
  rw [membershipTargetFactor _ _ _ _ _ _ _ _ _ _ hp n hd]
  exact mul_eq_zero_iff_left (pow_ne_zero 3 he)

/-- Full lossless equation replacement, using only the explicit public nonzero parameter. -/
theorem membershipNormalizedRowsIff (e a b r z a1 a2 c0 c1 terminal : K) (he : e≠0)
    (hp : z^2=r^3+a*r+b) (n : ℕ) (hd : 3≤2^(n+1)) :
    membership e a b r z a1 a2 c0 c1 terminal n=0 ↔
      (e^(2^(n+1)-3)*((NormMembershipElimination.master (2 : K) n).eval r-terminal)-
        (c0+c1*e^2*r)*scaledPolar e a r z a1 a2 c0 c1*
          (quotient e a b r z a1 a2 c0 c1 terminal n).eval (e^2*r)=0) ∧
        ∀ j, 1≤j → j≤4 → (membership e a b r z a1 a2 c0 c1 terminal n).coeff j=0 := by
  rw [membershipTargetRowsIff _ _ _ _ _ _ _ _ _ _ hp n,
    normalizedTargetEquation _ _ _ _ _ _ _ _ _ _ he hp n hd]

end Field
end NormTargetFiber

#print axioms NormTargetFiber.normDerivativeRing
#print axioms NormTargetFiber.linearQuotientValue
#print axioms NormTargetFiber.normQuotientTargetFactor
#print axioms NormTargetFiber.supportTargetValue
#print axioms NormTargetFiber.remainderTargetIdentity
#print axioms NormTargetFiber.scaledCurveEquation
#print axioms NormTargetFiber.scaledSupportTargetValue
#print axioms NormTargetFiber.masterAtScaledTarget
#print axioms NormTargetFiber.membershipTargetFactor
#print axioms NormTargetFiber.membershipTargetDivisibility
#print axioms NormTargetFiber.polynomialParameterDivisibility
#print axioms NormTargetFiber.normWindowRing
#print axioms NormTargetFiber.supportWindowRing
#print axioms NormTargetFiber.scaledSupportWindowRing
#print axioms NormTargetFiber.targetRowsIff
#print axioms NormTargetFiber.membershipTargetRowsIff
#print axioms NormTargetFiber.mapActualA
#print axioms NormTargetFiber.mapSupport
#print axioms NormTargetFiber.mapMaster
#print axioms NormTargetFiber.mapMembership
#print axioms NormTargetFiber.scaledSupportField
#print axioms NormTargetFiber.masterField
#print axioms NormTargetFiber.membershipField
#print axioms NormTargetFiber.parameterSpecialization
#print axioms NormTargetFiber.normalizedTargetEquation
#print axioms NormTargetFiber.membershipNormalizedRowsIff
