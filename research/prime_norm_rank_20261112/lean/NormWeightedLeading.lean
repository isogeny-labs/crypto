module
public import NormMembershipElimination
import all NormMembershipElimination
public import Mathlib.Algebra.GroupWithZero.Associated
public import Mathlib.Tactic
public section

/-!
Explicit candidate leading support for the four-coefficient pole filtration.
No assertion here identifies it with the concrete fixture's initial forms.
-/
namespace NormWeightedLeading
open Polynomial
variable {K : Type*} [Field K]

noncomputable def support (a1 a2 c0 c1 : K) : K[X] :=
  X^5+C (2*a2-c1^2)*X^4+C (a2^2+2*a1-2*c0*c1)*X^3+
    C (2*a1*a2-c0^2)*X^2+C (a1^2)*X

/-- A monic divisor of a pure power is itself a pure power, over any field. -/
theorem monicDivisorPower (U : K[X]) (hU : U.Monic) (n : ℕ)
    (hd : U ∣ X^n) : U=X^U.natDegree := by
  obtain ⟨i, _, hi⟩ := (dvd_prime_pow (prime_X (R := K)) n).mp hd
  have he := eq_of_monic_of_associated hU (monic_X_pow i) hi
  have hn := congrArg Polynomial.natDegree he
  simp only [natDegree_X_pow] at hn
  simpa only [hn] using he

theorem supportMonic (a1 a2 c0 c1 : K) : (support a1 a2 c0 c1).Monic := by
  unfold support
  monicity <;> norm_num

theorem supportDegree (a1 a2 c0 c1 : K) :
    (support a1 a2 c0 c1).natDegree=5 := by
  unfold support
  compute_degree!

/-- Coefficients of this explicit leading norm support are triangular nilpotence tests. -/
theorem supportPowerForcesZero (a1 a2 c0 c1 : K)
    (h : support a1 a2 c0 c1=X^5) :
    a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  have h1 := congrArg (fun p : K[X] => p.coeff 1) h
  have h2 := congrArg (fun p : K[X] => p.coeff 2) h
  have h3 := congrArg (fun p : K[X] => p.coeff 3) h
  have h4 := congrArg (fun p : K[X] => p.coeff 4) h
  simp only [support, coeff_add, coeff_C_mul, coeff_X_pow, coeff_X] at h1 h2 h3 h4
  norm_num at h1 h2 h3 h4
  have ha1 : a1=0 := h1
  have hc0 : c0=0 := by simpa [ha1] using h2
  have ha2 : a2=0 := by simpa [ha1, hc0] using h3
  have hc1 : c1=0 := by simpa [ha2] using h4
  exact ⟨ha1, ha2, hc0, hc1⟩

/-- No nonzero tuple over any field makes the explicit support divide a pure power. -/
theorem supportDivisorForcesZero (a1 a2 c0 c1 : K) (n : ℕ)
    (hd : support a1 a2 c0 c1 ∣ X^n) :
    a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  apply supportPowerForcesZero
  simpa only [supportDegree] using
    monicDivisorPower _ (supportMonic a1 a2 c0 c1) n hd

/-- Actual monic remainder vanishing suffices; no divisor witness is supplied. -/
theorem supportRemainderForcesZero (a1 a2 c0 c1 : K) (n : ℕ)
    (hr : X^n %ₘ support a1 a2 c0 c1=0) :
    a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  apply supportDivisorForcesZero a1 a2 c0 c1 n
  exact (modByMonic_eq_zero_iff_dvd (supportMonic a1 a2 c0 c1)).mp hr

/-- The unique zero set is the origin, including its converse for every n>=5. -/
theorem supportRemainderIffZero (a1 a2 c0 c1 : K) (n : ℕ) (hn : 5≤n) :
    X^n %ₘ support a1 a2 c0 c1=0 ↔ a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  constructor
  · exact supportRemainderForcesZero a1 a2 c0 c1 n
  · rintro ⟨rfl, rfl, rfl, rfl⟩
    apply (modByMonic_eq_zero_iff_dvd (supportMonic 0 0 0 0)).mpr
    simpa [support] using pow_dvd_pow (X : K[X]) hn

/-- The pole-scaled curve and target satisfy the actual curve equation. -/
theorem scaledCurveAtTarget (e a b r z : K) (hp : z^2=r^3+a*r+b) :
    (e^3*z)^2=(X^3+C (e^4*a)*X+C (e^6*b) : K[X]).eval (e^2*r) := by
  simp only [eval_add, eval_mul, eval_pow, eval_X, eval_C]
  linear_combination e^6*hp

/-- Exact norm identity at the special fiber, without a curve/group oracle. -/
theorem leadingNormIdentity (a1 a2 c0 c1 : K) :
    (X^3+C a2*X^2+C a1*X)^2-X^3*(C c0+C c1*X)^2=
      X*support a1 a2 c0 c1 := by
  unfold support
  simp only [map_sub, map_add, map_mul, map_pow, map_ofNat]
  ring

noncomputable def scaledSupport (e a b r z a1 a2 c0 c1 : K) : K[X] :=
  let D := X^3+C a2*X^2+C a1*X
  let B := C c0+C c1*X
  let A := NormMembershipElimination.completeTarget D B (e^2*r) (e^3*z)
  (A^2-(X^3+C (e^4*a)*X+C (e^6*b))*B^2) /ₘ (X-C (e^2*r))

/-- The special fiber of the explicit target-completed family is the support above. -/
theorem scaledSupportZero (a b r z a1 a2 c0 c1 : K) :
    scaledSupport 0 a b r z a1 a2 c0 c1=support a1 a2 c0 c1 := by
  simp only [scaledSupport, NormMembershipElimination.completeTarget]
  simp only [zero_pow (by decide : 2≠0), zero_pow (by decide : 3≠0),
    zero_pow (by decide : 4≠0), zero_pow (by decide : 6≠0), zero_mul,
    eval_add, eval_pow, eval_X, eval_mul, eval_C, mul_zero,
    add_zero, sub_zero, map_zero]
  rw [leadingNormIdentity, mul_divByMonic_cancel_left _ monic_X]

/-- At e=1 the family recovers the actual target-completed norm quotient. -/
theorem scaledSupportOne (a b r z a1 a2 c0 c1 : K) :
    scaledSupport 1 a b r z a1 a2 c0 c1=
      ((NormMembershipElimination.completeTarget
        (X^3+C a2*X^2+C a1*X) (C c0+C c1*X) r z)^2-
        (X^3+C a*X+C b)*(C c0+C c1*X)^2) /ₘ (X-C r) := by
  simp [scaledSupport]

/-- The whole explicit family is an exact target-completed norm quotient. -/
theorem scaledSupportExact (e a b r z a1 a2 c0 c1 : K)
    (hp : z^2=r^3+a*r+b) :
    (X-C (e^2*r))*scaledSupport e a b r z a1 a2 c0 c1=
      (NormMembershipElimination.completeTarget
        (X^3+C a2*X^2+C a1*X) (C c0+C c1*X) (e^2*r) (e^3*z))^2-
        (X^3+C (e^4*a)*X+C (e^6*b))*(C c0+C c1*X)^2 := by
  unfold scaledSupport
  apply NormMembershipElimination.canonicalNormQuotient _ _ _ _ (e^3*z)
  · exact scaledCurveAtTarget e a b r z hp
  · exact NormMembershipElimination.completionRow _ _ _ _

/-- Vanishing remainder at the special fiber forces the origin, in every field. -/
theorem scaledFiberRemainderForcesZero (a b r z a1 a2 c0 c1 : K) (n : ℕ)
    (hr : X^n %ₘ scaledSupport 0 a b r z a1 a2 c0 c1=0) :
    a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  rw [scaledSupportZero] at hr
  exact supportRemainderForcesZero a1 a2 c0 c1 n hr

/-- Four nonconstant remainder coefficients suffice, as the constant is identically zero. -/
theorem fourCoefficientsForceZero (a1 a2 c0 c1 : K) (n : ℕ) (hn : 0<n)
    (hc : ∀ j, 1≤j → j≤4 → (X^n %ₘ support a1 a2 c0 c1).coeff j=0) :
    a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  apply supportRemainderForcesZero a1 a2 c0 c1 n
  have hconstant := congrArg (fun p : K[X] => p.eval 0)
    (modByMonic_add_div (X^n) (support a1 a2 c0 c1))
  have hz : (X^n %ₘ support a1 a2 c0 c1).coeff 0=0 := by
    simpa [support, hn.ne', coeff_zero_eq_eval_zero] using hconstant
  have hdeg := degree_modByMonic_lt (X^n) (supportMonic a1 a2 c0 c1)
  rw [degree_eq_natDegree (supportMonic a1 a2 c0 c1).ne_zero, supportDegree] at hdeg
  ext j
  rw [coeff_zero]
  by_cases hj : j=0
  · simpa only [hj] using hz
  · by_cases hsmall : j≤4
    · exact hc j (by omega) hsmall
    · exact coeff_eq_zero_of_degree_lt (hdeg.trans_le (by exact_mod_cast (by omega : 5≤j)))

noncomputable def scaledMaster (e : K) : ℕ → K[X]
  | 0 => X
  | n+1 => scaledMaster e n^2-C (2*e^(2^(n+2)))

/-- The Dickson tower degenerates to a pure power at the special fiber. -/
theorem scaledMasterZero (n : ℕ) : scaledMaster (0 : K) n=X^(2^n) := by
  induction n with
  | zero => simp [scaledMaster]
  | succ n ih =>
      have hz : (0:K)^(2^(n+2))=0 := zero_pow (by positivity)
      rw [scaledMaster, ih, hz, mul_zero, map_zero, sub_zero, ← pow_mul, pow_succ]

/-- The other endpoint is the actual Dickson recurrence, with unchanged parameter2. -/
theorem scaledMasterOne (n : ℕ) :
    scaledMaster (1 : K) n=NormMembershipElimination.master 2 n := by
  induction n with
  | zero => rfl
  | succ n ih =>
      simp only [scaledMaster, NormMembershipElimination.master, ih]
      simp

noncomputable def scaledMembership (e a b r z a1 a2 c0 c1 terminal : K) (n : ℕ) : K[X] :=
  (scaledMaster e n-C (terminal*e^(2^(n+1)))) %ₘ scaledSupport e a b r z a1 a2 c0 c1

/-- This is an actual remainder, at the special fiber of the explicit family. -/
theorem scaledMembershipZero (a b r z a1 a2 c0 c1 terminal : K) (n : ℕ) :
    scaledMembership 0 a b r z a1 a2 c0 c1 terminal n=
      X^(2^n) %ₘ support a1 a2 c0 c1 := by
  simp [scaledMembership, scaledMasterZero, scaledSupportZero]

/-- At depth4, four explicit coefficient equations force the origin at the special fiber. -/
theorem membershipFourCoefficients (a b r z a1 a2 c0 c1 terminal : K)
    (hc : ∀ j, 1≤j → j≤4 →
      (scaledMembership 0 a b r z a1 a2 c0 c1 terminal 4).coeff j=0) :
    a1=0 ∧ a2=0 ∧ c0=0 ∧ c1=0 := by
  apply fourCoefficientsForceZero a1 a2 c0 c1 16 (by decide)
  simpa only [scaledMembershipZero, show (2:ℕ)^4=16 by decide] using hc

end NormWeightedLeading

#print axioms NormWeightedLeading.monicDivisorPower
#print axioms NormWeightedLeading.supportMonic
#print axioms NormWeightedLeading.supportDegree
#print axioms NormWeightedLeading.supportPowerForcesZero
#print axioms NormWeightedLeading.supportDivisorForcesZero
#print axioms NormWeightedLeading.supportRemainderForcesZero
#print axioms NormWeightedLeading.supportRemainderIffZero
#print axioms NormWeightedLeading.scaledCurveAtTarget
#print axioms NormWeightedLeading.leadingNormIdentity
#print axioms NormWeightedLeading.scaledSupportZero
#print axioms NormWeightedLeading.scaledSupportOne
#print axioms NormWeightedLeading.scaledSupportExact
#print axioms NormWeightedLeading.scaledFiberRemainderForcesZero
#print axioms NormWeightedLeading.fourCoefficientsForceZero
#print axioms NormWeightedLeading.scaledMasterZero
#print axioms NormWeightedLeading.scaledMasterOne
#print axioms NormWeightedLeading.scaledMembershipZero
#print axioms NormWeightedLeading.membershipFourCoefficients
