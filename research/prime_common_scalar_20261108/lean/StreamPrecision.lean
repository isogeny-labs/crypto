module
public import FiniteRecovery
public section
namespace StreamPrecision
open LocalIntegral ResidueEncoding FiniteBuffers

/-- Equality of actual coefficient residues is precisely divisibility. -/
theorem residueEqIff (p : ℕ) (x y : integralRing) :
    toResidue p x=toResidue p y ↔ (2:integralRing)^p ∣ x-y := by
  rw [←sub_eq_zero,←map_sub]
  change x-y∈RingHom.ker (toResidue p) ↔ _
  rw [residue_kernel,coefficientIdeal,Ideal.mem_span_singleton]

/-- Cancellation is valid across the increased precision, not within one
fixed characteristic ring. This supports a cleared modular accumulator. -/
theorem crossPrecisionCancellation (p K : ℕ) (x y : integralRing) :
    toResidue (p+K) ((2:integralRing)^K*x)=
      toResidue (p+K) ((2:integralRing)^K*y) ↔ toResidue p x=toResidue p y := by
  rw [residueEqIff,residueEqIff,←mul_sub,pow_add]
  have hn : (2:integralRing)^K≠0 := pow_ne_zero _ (by norm_num)
  constructor
  · rintro ⟨r,hr⟩
    refine ⟨r,?_⟩
    apply mul_left_cancel₀ hn
    calc
      (2:integralRing)^K*(x-y)=((2:integralRing)^p*2^K)*r := hr
      _ = (2:integralRing)^K*(2^p*r) := by ring
  · rintro ⟨r,hr⟩
    refine ⟨r,?_⟩
    rw [hr]
    ring


open NodeMultiplicity HermiteError HermiteUniqueness HermiteBasis

@[expose] def globalOverhead (a M u : ℕ) : ℕ :=
  8*sourceCeiling M u*2^a+8*M*(a+1)

theorem childOverheadBound (a M u : ℕ) (t : Fin (2^(a+1))) :
    childPrecision 0 a M u t ≤ globalOverhead a M u := by
  simp only [childPrecision,nodeDensity,globalOverhead,zero_add]
  gcongr
  exact Nat.sub_le a _

/-- Every actual recovery coefficient becomes integral under the same explicit
scaling. Native numerator construction and its costs remain separate. -/
theorem actualScaledCoefficientIntegral (a M u : ℕ) (hMu : u≤M)
    (t : Fin (2^(a+1))) (ht0 : gridNode a t≠0)
    (j : ℕ) (hj : j<sourceOrder a M u t) :
    padicNorm 2 ((2:ℚ)^(globalOverhead a M u)*(basis a M u t).eval 0*
      (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))^j)≤1 := by
  let k := childPrecision 0 a M u t
  let x : ℚ := -(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ)
  have hi : padicNorm 2 ((2:ℚ)^k*(basis a M u t).eval 0*x^j)≤1 := by
    have hh := actualCoefficientErrorStable 0 a M u hMu t ht0 j hj
      ((2:integralRing)^k) (by exact dvd_rfl)
    simp only [pow_zero,div_one,Subring.coe_pow,integralTwoValue] at hh
    change padicNorm 2 ((basis a M u t).eval 0*(2:ℚ)^k*x^j)≤1 at hh
    simpa only [mul_comm ((basis a M u t).eval 0) ((2:ℚ)^k)] using hh
  let r : integralRing := ⟨(2:ℚ)^k*(basis a M u t).eval 0*x^j,hi⟩
  let e : integralRing := (2:integralRing)^(globalOverhead a M u-k)*r
  have hb : k≤globalOverhead a M u := childOverheadBound a M u t
  have he : (e:ℚ)=(2:ℚ)^(globalOverhead a M u)*(basis a M u t).eval 0*x^j := by
    simp only [e,r,Subring.coe_mul,Subring.coe_pow,integralTwoValue]
    calc
      _ = ((2:ℚ)^(globalOverhead a M u-k)*(2:ℚ)^k)*(basis a M u t).eval 0*x^j := by ring
      _ = _ := by rw [←pow_add,Nat.sub_add_cancel hb]
  change padicNorm 2 ((2:ℚ)^(globalOverhead a M u)*(basis a M u t).eval 0*x^j)≤1
  rw [←he]
  exact e.property


@[expose] noncomputable def scaledCoefficient (a M u : ℕ) (hMu : u≤M)
    (t : Fin (2^(a+1))) (j : ℕ) : integralRing :=
  if h : gridNode a t≠0 ∧ j<sourceOrder a M u t then
    ⟨(2:ℚ)^(globalOverhead a M u)*(basis a M u t).eval 0*
      (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))^j,
      actualScaledCoefficientIntegral a M u hMu t h.1 j h.2⟩ else 0

@[expose] noncomputable def scaledFunctional (a M u : ℕ) (hMu : u≤M)
    (J : Fin (2^(a+1))→Polynomial integralRing) : integralRing :=
  ∑ t∈sampleNodes a M u, ∑ j∈Finset.range (sourceOrder a M u t),
    scaledCoefficient a M u hMu t j*(J t).coeff j

/-- The integral scaled functional is the actual Hermite sum with a single
common power-of-two denominator cleared. -/
theorem scaledFunctionalMap (a M u : ℕ) (hMu : u≤M)
    (J : Fin (2^(a+1))→Polynomial integralRing)
    (hJ : ∀ t∈sampleNodes a M u, (J t).natDegree<sourceOrder a M u t) :
    ((scaledFunctional a M u hMu J:integralRing):ℚ)=
      (2:ℚ)^(globalOverhead a M u)*recoveryFunctional a M u J := by
  classical
  change integralRing.subtype (scaledFunctional a M u hMu J)=_
  unfold scaledFunctional recoveryFunctional
  rw [map_sum,Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro t ht
  have ht0 : gridNode a t≠0 := by
    intro hz
    have hp := (Finset.mem_filter.mp ht).2
    simp [sourceOrder,hz] at hp
  rw [map_sum,Polynomial.eval₂_eq_sum_range' integralRing.subtype (hJ t ht),
    Finset.mul_sum,Finset.mul_sum]
  apply Finset.sum_congr rfl
  intro j hj
  have hp : gridNode a t≠0 ∧ j<sourceOrder a M u t := ⟨ht0,Finset.mem_range.mp hj⟩
  simp only [map_mul,scaledCoefficient,dif_pos hp]
  change ((2:ℚ)^(globalOverhead a M u)*(basis a M u t).eval 0*
      (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))^j)*((J t).coeff j:ℚ)=
      (2:ℚ)^(globalOverhead a M u)*((basis a M u t).eval 0*
      (((J t).coeff j:ℚ)*(-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))^j))
  ring


/-- Actual finite input jets give the target in the scaled accumulator's
larger residue ring. No target is an input to the scaled functional. -/
theorem computedScaledFunctionalResidue {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M)
    (place : Fin M→κ×Fin (2^a)) (hinj : Function.Injective place)
    (A B : (Fin M→Bool)→integralRing) :
    toResidue (p+globalOverhead a M u)
      (scaledFunctional a M u hMu (FiniteRecovery.computedQuotient p a u place A B))=
    toResidue (p+globalOverhead a M u)
      ((2:integralRing)^(globalOverhead a M u)*(∑ x, A x*B x)) := by
  have hm : scaledFunctional a M u hMu (FiniteRecovery.computedQuotient p a u place A B)=
      (2:integralRing)^(globalOverhead a M u)*
        FiniteRecovery.integralOutput p a u hu hua hMu place hinj A B := by
    apply Subtype.ext
    rw [scaledFunctionalMap a M u hMu _ (FiniteRecovery.computedQuotientDegree p a u place A B)]
    simp only [Subring.coe_mul,Subring.coe_pow,integralTwoValue]
    rfl
  rw [hm]
  exact (crossPrecisionCancellation p (globalOverhead a M u) _ _).mpr
    (FiniteRecovery.computedPackedRecoveryResidue p a u hu hua hMu place hinj A B)

end StreamPrecision
#print axioms StreamPrecision.residueEqIff
#print axioms StreamPrecision.crossPrecisionCancellation
#print axioms StreamPrecision.childOverheadBound
#print axioms StreamPrecision.actualScaledCoefficientIntegral
#print axioms StreamPrecision.scaledFunctionalMap
#print axioms StreamPrecision.computedScaledFunctionalResidue
