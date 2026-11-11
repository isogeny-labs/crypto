module
public import HermiteMoment
public section
namespace HermiteError
open Polynomial NodeMultiplicity HermiteUniqueness HermiteBasis HermiteMoment LocalIntegral

@[expose] def nodeDensity (a : ℕ) (t : Fin (2^(a+1))) : ℕ := 2^(a-padicValInt 2 (gridNode a t))
@[expose] def multiplierLoss (a M u : ℕ) (t : Fin (2^(a+1))) : ℕ := 6*sourceCeiling M u*nodeDensity a t
@[expose] def evaluationLoss (a M : ℕ) : ℕ := 4*M*(a+1)
@[expose] def childPrecision (p a M u : ℕ) (t : Fin (2^(a+1))) : ℕ :=
  p+8*sourceCeiling M u*nodeDensity a t+8*M*(a+1)
@[expose] def precisionSlack (a M u : ℕ) (t : Fin (2^(a+1))) : ℕ :=
  2*sourceCeiling M u*nodeDensity a t+4*M*(a+1)

@[simp] theorem integralTwoValue : ((2:integralRing):ℚ)=2 := by
  change integralRing.subtype (2:integralRing)=2
  rw [map_ofNat]

/-- The multiplier's actual valuation bound gives an actual integral scalar. -/
theorem actualScaledMultiplierIntegral (a M u : ℕ) (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0) :
    padicNorm 2 ((2:ℚ)^(multiplierLoss a M u t)*(basis a M u t).eval 0)≤1 := by
  by_cases hz : (basis a M u t).eval 0=0
  · simp [hz]
  have htwo : (2:ℚ)^(multiplierLoss a M u t)≠0 := by positivity
  have hn := mul_ne_zero htwo hz
  have hb := actualHermiteMultiplierLoss a M u t ht0
  have hvaltwo : padicValRat 2 (2:ℚ)=1 := padicValRat.self (p:=2) (by decide)
  have hv : 0≤padicValRat 2 ((2:ℚ)^(multiplierLoss a M u t)*(basis a M u t).eval 0) := by
    rw [padicValRat.mul htwo hz,padicValRat.pow,hvaltwo,mul_one]
    dsimp [multiplierLoss,nodeDensity]
    push_cast at hb ⊢
    nlinarith only [hb]
  rw [padicNorm.eq_zpow_of_nonzero hn]
  exact zpow_le_one_of_nonpos₀ (by norm_num) (by omega)

@[expose] noncomputable def scaledMultiplier (a M u : ℕ) (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0) :
    integralRing := ⟨_,actualScaledMultiplierIntegral a M u t ht0⟩

/-- Each coefficient error is carried through the ACTUAL Hermite multiplier
and evaluation point, with the original source precision and explicit slack. -/
theorem actualCoefficientErrorStable (p a M u : ℕ) (hMu : u≤M)
    (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0) (j : ℕ) (hj : j<sourceOrder a M u t)
    (c : integralRing) (hc : (2:integralRing)^(childPrecision p a M u t) ∣ c) :
    padicNorm 2 (((basis a M u t).eval 0*(c:ℚ)*
      (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))^j)/(2:ℚ)^p)≤1 := by
  obtain ⟨r,hr⟩ := hc
  let x : ℚ := -(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ)
  have hdeg : (monomial j r).natDegree<sourceOrder a M u t := (natDegree_monomial_le r).trans_lt hj
  have hv : padicNorm 2 ((2:ℚ)^(evaluationLoss a M)*(r:ℚ)*x^j)≤1 := by
    have hh := actualEvaluationLoss a M u hMu t (monomial j r) hdeg
    rw [eval₂_monomial] at hh
    change padicNorm 2 ((2:ℚ)^(evaluationLoss a M)*((r:ℚ)*x^j))≤1 at hh
    simpa only [mul_assoc] using hh
  let v : integralRing := ⟨(2:ℚ)^(evaluationLoss a M)*(r:ℚ)*x^j,hv⟩
  let e : integralRing := (2:integralRing)^(precisionSlack a M u t)*scaledMultiplier a M u t ht0*v
  have hprec : childPrecision p a M u t=p+precisionSlack a M u t+multiplierLoss a M u t+evaluationLoss a M := by
    dsimp [childPrecision,precisionSlack,multiplierLoss,evaluationLoss]
    ring
  have he : ((basis a M u t).eval 0*(c:ℚ)*x^j)/(2:ℚ)^p=(e:ℚ) := by
    rw [hr]
    simp only [e,scaledMultiplier,v,Subring.coe_mul,Subring.coe_pow,integralTwoValue]
    rw [hprec,pow_add,pow_add,pow_add]
    field_simp
  change padicNorm 2 (((basis a M u t).eval 0*(c:ℚ)*x^j)/(2:ℚ)^p)≤1
  rw [he]
  exact e.property

/-- Finite polynomial coefficient congruence preserves the source contribution
modulo the requested precision. It does not assume a reconstruction equality. -/
theorem actualContributionPrecisionStable (p a M u : ℕ) (hMu : u≤M)
    (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0) (P Q : Polynomial integralRing)
    (hP : P.natDegree<sourceOrder a M u t) (hQ : Q.natDegree<sourceOrder a M u t)
    (hc : ∀ j<sourceOrder a M u t,
      (2:integralRing)^(childPrecision p a M u t) ∣ P.coeff j-Q.coeff j) :
    padicNorm 2 (((basis a M u t).eval 0*
      (P.eval₂ integralRing.subtype (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))-
       Q.eval₂ integralRing.subtype (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))))/(2:ℚ)^p)≤1 := by
  let x : ℚ := -(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ)
  have he : ((basis a M u t).eval 0*(P.eval₂ integralRing.subtype x-Q.eval₂ integralRing.subtype x))/(2:ℚ)^p =
      ∑ j∈Finset.range (sourceOrder a M u t),
        ((basis a M u t).eval 0*((P.coeff j-Q.coeff j:integralRing):ℚ)*x^j)/(2:ℚ)^p := by
    rw [eval₂_eq_sum_range' integralRing.subtype hP,eval₂_eq_sum_range' integralRing.subtype hQ,
      ←Finset.sum_sub_distrib,Finset.mul_sum,Finset.sum_div]
    apply Finset.sum_congr rfl
    intro j hj
    change _ = ((basis a M u t).eval 0*integralRing.subtype (P.coeff j-Q.coeff j)*x^j)/(2:ℚ)^p
    rw [map_sub]
    ring
  change padicNorm 2 (((basis a M u t).eval 0*(P.eval₂ integralRing.subtype x-Q.eval₂ integralRing.subtype x))/(2:ℚ)^p)≤1
  rw [he]
  change (∑ j∈Finset.range (sourceOrder a M u t),
    ((basis a M u t).eval 0*((P.coeff j-Q.coeff j:integralRing):ℚ)*x^j)/(2:ℚ)^p)∈integralRing
  apply integralRing.sum_mem
  intro j hj
  exact actualCoefficientErrorStable p a M u hMu t ht0 j (Finset.mem_range.mp hj)
    (P.coeff j-Q.coeff j) (hc j (Finset.mem_range.mp hj))

@[expose] def sampleNodes (a M u : ℕ) : Finset (Fin (2^(a+1))) :=
  Finset.univ.filter (fun t => 0<sourceOrder a M u t)

/-- D.9's explicit finite functional on supplied truncated quotient polynomials.
Exact reconstruction from input jets is a separate theorem. -/
@[expose] noncomputable def recoveryFunctional (a M u : ℕ)
    (J : Fin (2^(a+1))→Polynomial integralRing) : ℚ :=
  ∑ t∈sampleNodes a M u, (basis a M u t).eval 0*
    (J t).eval₂ integralRing.subtype (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))

theorem actualRecoveryFunctionalStable (p a M u : ℕ) (hMu : u≤M)
    (P Q : Fin (2^(a+1))→Polynomial integralRing)
    (hP : ∀ t∈sampleNodes a M u, (P t).natDegree<sourceOrder a M u t)
    (hQ : ∀ t∈sampleNodes a M u, (Q t).natDegree<sourceOrder a M u t)
    (hc : ∀ t∈sampleNodes a M u, ∀ j<sourceOrder a M u t,
      (2:integralRing)^(childPrecision p a M u t) ∣ (P t).coeff j-(Q t).coeff j) :
    padicNorm 2 ((recoveryFunctional a M u P-recoveryFunctional a M u Q)/(2:ℚ)^p)≤1 := by
  unfold recoveryFunctional
  rw [←Finset.sum_sub_distrib,Finset.sum_div]
  change (∑ t∈sampleNodes a M u,
    ((basis a M u t).eval 0*(P t).eval₂ integralRing.subtype (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))-
     (basis a M u t).eval 0*(Q t).eval₂ integralRing.subtype (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ)))/(2:ℚ)^p)∈integralRing
  apply integralRing.sum_mem
  intro t ht
  have ht0 : gridNode a t≠0 := by
    intro hzero
    have ho := (Finset.mem_filter.mp ht).2
    simp only [sourceOrder,hzero,ite_true] at ho
    omega
  change padicNorm 2 _≤1
  simpa only [mul_sub] using
    actualContributionPrecisionStable p a M u hMu t ht0 (P t) (Q t) (hP t ht) (hQ t ht) (hc t ht)

/-- Actual integral basis inversion loses no input coefficient precision. -/
theorem actualLocalQuotientBufferStable (a M u : ℕ) (t : Fin (2^(a+1))) (p q : ℕ)
    (F G : PowerSeries integralRing)
    (hinput : FiniteBuffers.buffer p q F=FiniteBuffers.buffer p q G) :
    FiniteBuffers.buffer p q (F*localInverse a M u t)=
      FiniteBuffers.buffer p q (G*localInverse a M u t) := by
  have hd := (FiniteBuffers.buffer_eq_iff_jet p q F G).mp hinput
  apply (FiniteBuffers.buffer_eq_iff_jet p q _ _).mpr
  have hm := (LocalPrecision.jetIdeal p q).mul_mem_right (localInverse a M u t) hd
  simpa only [sub_mul] using hm

end HermiteError
#print axioms HermiteError.actualScaledMultiplierIntegral
#print axioms HermiteError.actualCoefficientErrorStable
#print axioms HermiteError.actualContributionPrecisionStable
#print axioms HermiteError.actualRecoveryFunctionalStable
#print axioms HermiteError.actualLocalQuotientBufferStable
