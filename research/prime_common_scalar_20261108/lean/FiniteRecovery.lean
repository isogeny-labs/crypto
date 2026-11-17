module
public import IntegralRecovery
public section
namespace FiniteRecovery
open Polynomial NodeMultiplicity HermiteUniqueness HermiteBasis HermiteError IntegralRecovery
open LocalIntegral FiniteBuffers ResidueEncoding

/-- Mathematical finite-buffer operator: no caller-supplied full inverse. -/
@[expose] noncomputable def finiteQuotient {R : Type*} [CommRing R] (q : ℕ)
    (f d : Fin q→R) : Fin q→R :=
  fun n => PowerSeries.coeff n (liftBuffer q f*liftBuffer q (invertBuffer q d 1))

theorem lowProductCoefficients {R : Type*} [CommRing R] (q : ℕ) (F G I J : PowerSeries R)
    (hF : ∀ n<q, PowerSeries.coeff n F=PowerSeries.coeff n G)
    (hI : ∀ n<q, PowerSeries.coeff n I=PowerSeries.coeff n J)
    (n : ℕ) (hn : n<q) : PowerSeries.coeff n (F*I)=PowerSeries.coeff n (G*J) := by
  rw [PowerSeries.coeff_mul,PowerSeries.coeff_mul]
  apply Finset.sum_congr rfl
  intro k hk
  have he := Finset.mem_antidiagonal.mp hk
  rw [hF k.1 (by omega),hI k.2 (by omega)]

theorem finiteQuotientCorrect {R : Type*} [CommRing R] (π : integralRing→+*R)
    (a M u : ℕ) (t : Fin (2^(a+1))) (q : ℕ) (F : PowerSeries integralRing) :
    finiteQuotient q (fun n : Fin q => π (PowerSeries.coeff n F))
      (fun n : Fin q => π (PowerSeries.coeff n (localSeries a M u t))) =
        fun n : Fin q => π (PowerSeries.coeff n (F*localInverse a M u t)) := by
  unfold finiteQuotient
  rw [actualBasisMappedInverseBuffer π a M u t q]
  funext n
  have hF : ∀ k<q, PowerSeries.coeff k (liftBuffer q (fun n : Fin q => π (PowerSeries.coeff n F)))=
      PowerSeries.coeff k (PowerSeries.map π F) := by
    intro k hk
    simpa only [PowerSeries.coeff_map] using liftBuffer_coeff q
      (fun n : Fin q => π (PowerSeries.coeff n F)) (⟨k,hk⟩ : Fin q)
  have hI : ∀ k<q, PowerSeries.coeff k
      (liftBuffer q (fun n : Fin q => π (PowerSeries.coeff n (localInverse a M u t))))=
      PowerSeries.coeff k (PowerSeries.map π (localInverse a M u t)) := by
    intro k hk
    simpa only [PowerSeries.coeff_map] using liftBuffer_coeff q
      (fun n : Fin q => π (PowerSeries.coeff n (localInverse a M u t))) (⟨k,hk⟩ : Fin q)
  have hh := lowProductCoefficients q _ _ _ _ hF hI n n.isLt
  rw [←map_mul,PowerSeries.coeff_map] at hh
  exact hh

/-- Lift only a finite residue vector, using its canonical integer values. -/
@[expose] noncomputable def liftResiduePolynomial (p q : ℕ) (v : Fin q→ZMod (2^p)) : Polynomial integralRing :=
  PowerSeries.trunc q (liftBuffer q (fun n => ((v n).val:integralRing)))

theorem liftResidueCoefficient (p q : ℕ) (v : Fin q→ZMod (2^p)) (n : Fin q) :
    toResidue p ((liftResiduePolynomial p q v).coeff n)=v n := by
  have hc : (liftResiduePolynomial p q v).coeff n=((v n).val:integralRing) := by
    unfold liftResiduePolynomial
    rw [PowerSeries.coeff_trunc,ite_eq_left n.isLt]
    exact liftBuffer_coeff q (fun n => ((v n).val:integralRing)) n
  rw [hc,map_natCast,ZMod.natCast_zmod_val]

theorem liftResidueDegree (p q : ℕ) (v : Fin q→ZMod (2^p)) (hq : 0<q) :
    (liftResiduePolynomial p q v).natDegree<q := by
  cases q with
  | zero => omega
  | succ q => exact PowerSeries.natDegree_trunc_lt _ _

@[expose] noncomputable def computedQuotient {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) : Polynomial integralRing :=
  let q := sourceOrder a M u t
  let pt := childPrecision p a M u t
  liftResiduePolynomial pt q (finiteQuotient q
    (fun n : Fin q => toResidue pt (PowerSeries.coeff n (integralChart a place A B t).toPowerSeries))
    (fun n : Fin q => toResidue pt (PowerSeries.coeff n (localSeries a M u t))))

theorem computedQuotientResidue {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) (n : Fin (sourceOrder a M u t)) :
    toResidue (childPrecision p a M u t) ((computedQuotient p a u place A B t).coeff n)=
      toResidue (childPrecision p a M u t) ((integralQuotient a u place A B t).coeff n) := by
  unfold computedQuotient
  rw [liftResidueCoefficient,finiteQuotientCorrect]
  unfold integralQuotient
  rw [PowerSeries.coeff_trunc,ite_eq_left n.isLt]

theorem computedQuotientCongruence {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) (j : ℕ) (hj : j<sourceOrder a M u t) :
    (2:integralRing)^(childPrecision p a M u t) ∣
      (computedQuotient p a u place A B t).coeff j-(integralQuotient a u place A B t).coeff j := by
  have he := computedQuotientResidue p a u place A B t (⟨j,hj⟩ : Fin (sourceOrder a M u t))
  have hz : toResidue (childPrecision p a M u t)
      ((computedQuotient p a u place A B t).coeff j-(integralQuotient a u place A B t).coeff j)=0 := by
    rw [map_sub,he,sub_self]
  have hk : (computedQuotient p a u place A B t).coeff j-(integralQuotient a u place A B t).coeff j∈
      RingHom.ker (toResidue (childPrecision p a M u t)) := hz
  rw [residue_kernel,coefficientIdeal,Ideal.mem_span_singleton] at hk
  exact hk

theorem computedQuotientDegree {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) (ht : t∈sampleNodes a M u) :
    (computedQuotient p a u place A B t).natDegree<sourceOrder a M u t := by
  unfold computedQuotient
  exact liftResidueDegree _ _ _ ((Finset.mem_filter.mp ht).2)

/-- Actual recovery from the constructed finite buffers; no caller-supplied
quotient congruence or reconstruction equality remains. -/
theorem computedPackedRecoveryPrecision {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (hinj : Function.Injective place) (A B : (Fin M→Bool)→integralRing) :
    padicNorm 2 ((recoveryFunctional a M u (computedQuotient p a u place A B)-
      ((∑ x, A x*B x:integralRing):ℚ))/(2:ℚ)^p)≤1 := by
  apply actualPackedRecoveryPrecision p a u hu hua hMu place hinj A B (computedQuotient p a u place A B)
  · exact computedQuotientDegree p a u place A B
  · intro t ht j hj
    exact computedQuotientCongruence p a u place A B t j hj

theorem integralApproximation (p : ℕ) (r : integralRing) (x : ℚ)
    (hprec : padicNorm 2 ((x-(r:ℚ))/(2:ℚ)^p)≤1) : padicNorm 2 x≤1 := by
  let e : integralRing := ⟨(x-(r:ℚ))/(2:ℚ)^p,hprec⟩
  let v : integralRing := r+(2:integralRing)^p*e
  have hv : (v:ℚ)=x := by
    simp only [v,e,Subring.coe_add,Subring.coe_mul,Subring.coe_pow,integralTwoValue]
    field_simp <;> ring
  rw [←hv]
  exact v.property

theorem integralApproximationResidue (p : ℕ) (r x : integralRing)
    (hprec : padicNorm 2 (((x:ℚ)-(r:ℚ))/(2:ℚ)^p)≤1) : toResidue p x=toResidue p r := by
  let e : integralRing := ⟨((x:ℚ)-(r:ℚ))/(2:ℚ)^p,hprec⟩
  have hd : x-r=(2:integralRing)^p*e := by
    apply Subtype.ext
    change (x:ℚ)-(r:ℚ)=(((2:integralRing)^p:integralRing):ℚ)*(((x:ℚ)-(r:ℚ))/(2:ℚ)^p)
    simp only [Subring.coe_pow,integralTwoValue]
    field_simp
  have hz : (2:ZMod (2^p))^p=0 := by
    simpa only [Nat.cast_pow,Nat.cast_ofNat] using ZMod.natCast_self (2^p)
  have he : toResidue p (x-r)=0 := by
    rw [hd,map_mul,map_pow,map_ofNat,hz,zero_mul]
  rw [map_sub] at he
  exact sub_eq_zero.mp he

/-- The computational value is the finite recovery sum. Its proof of
integrality may use the target, but that target is not the returned value. -/
@[expose] noncomputable def integralOutput {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (hinj : Function.Injective place) (A B : (Fin M→Bool)→integralRing) : integralRing :=
  ⟨recoveryFunctional a M u (computedQuotient p a u place A B),
    integralApproximation p (∑ x, A x*B x) _ (computedPackedRecoveryPrecision p a u hu hua hMu place hinj A B)⟩

theorem computedPackedRecoveryResidue {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (hinj : Function.Injective place) (A B : (Fin M→Bool)→integralRing) :
    toResidue p (integralOutput p a u hu hua hMu place hinj A B)=toResidue p (∑ x, A x*B x) := by
  exact integralApproximationResidue p _ _ (computedPackedRecoveryPrecision p a u hu hua hMu place hinj A B)

end FiniteRecovery
#print axioms FiniteRecovery.finiteQuotientCorrect
#print axioms FiniteRecovery.computedQuotientResidue
#print axioms FiniteRecovery.computedPackedRecoveryPrecision
#print axioms FiniteRecovery.computedPackedRecoveryResidue
