module
public import HermiteReconstruction
public section
namespace IntegralRecovery
open Polynomial BooleanContraction NodeMultiplicity HermiteUniqueness HermiteBasis HermiteError
open HermiteReconstruction LocalIntegral LocalCharts PackedContraction

@[expose] noncomputable def localNumerator {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (t : Fin (2^(a+1))) (i j : Fin M) : Polynomial integralRing :=
  if (place i).1=(place j).1 then (localBase a t (slotIndex a (place i).2 (place j).2))^2 else 0

@[expose] noncomputable def integralChart {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) : Polynomial integralRing :=
  contractionPolynomial A B (localDenominator a t) (localNumerator a place t)

@[expose] noncomputable def actualPolynomial {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing) : Polynomial ℚ :=
  packedContractionPolynomial a place (fun y => (A y:ℚ)) (fun x => (B x:ℚ))

theorem actualPolynomialFormula {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing) :
    actualPolynomial a place A B=
      contractionPolynomial (fun y => (A y:ℚ)) (fun x => (B x:ℚ))
        (denominatorPolynomial a) (packedNumeratorPolynomial a place) := by
  simp only [actualPolynomial,packedContractionPolynomial,Algebra.algebraMap_self,Polynomial.map_id]

theorem contractionPolynomialEval₂ {R S : Type*} [CommRing R] [CommRing S] {M : ℕ}
    (π : R→+*S) (A B : (Fin M→Bool)→R) (D : Polynomial R) (N : Fin M→Fin M→Polynomial R) (z : S) :
    (contractionPolynomial A B D N).eval₂ π z=
      clearedContraction (fun y => π (A y)) (fun x => π (B x)) (D.eval₂ π z)
        (fun i j => (N i j).eval₂ π z) := by
  simpa only [contractionPolynomial,Polynomial.coe_eval₂RingHom,eval₂_C] using
    mapClearedContraction (Polynomial.eval₂RingHom π z) (fun y => C (A y)) (fun x => C (B x)) D N

theorem actualLocalNumeratorEval {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (t : Fin (2^(a+1))) (z : ℚ) (i j : Fin M) :
    (localNumerator a place t i j).eval₂ integralRing.subtype z=
      (packedBaseValue a place ((gridNode a t:ℚ)+((2^(a+1):ℕ):ℚ)*z) i j)^2 := by
  unfold localNumerator packedBaseValue
  split_ifs
  · simp only [eval₂_pow,localBase_eval]
    congr 2
    simp [gridNode]
  · simp

theorem actualIntegralChart {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) :
    (integralChart a place A B t).map integralRing.subtype=sourceChart a t (actualPolynomial a place A B) := by
  apply Polynomial.funext
  intro z
  rw [eval_map]
  unfold integralChart
  rw [contractionPolynomialEval₂,localDenominator_eval]
  simp_rw [actualLocalNumeratorEval]
  unfold sourceChart
  rw [eval_comp,eval_add,eval_C,eval_mul,eval_C,eval_X,actualPolynomialFormula,contractionPolynomialEval,
    denominatorPolynomialEval]
  simp_rw [packedNumeratorEval]
  congr 1
  · simp [sourcePoint,sourceScale,gridNode]
  · funext i j
    simp [sourcePoint,sourceScale]

@[expose] noncomputable def integralQuotient {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) : Polynomial integralRing :=
  PowerSeries.trunc (sourceOrder a M u t)
    ((integralChart a place A B t).toPowerSeries*localInverse a M u t)

theorem integralQuotientMap {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) :
    (integralQuotient a u place A B t).map integralRing.subtype=
      exactQuotient a M u t (actualPolynomial a place A B) := by
  have he : PowerSeries.map integralRing.subtype (integralChart a place A B t).toPowerSeries=
      (sourceChart a t (actualPolynomial a place A B)).toPowerSeries := by
    ext n
    simp only [PowerSeries.coeff_map,Polynomial.coeff_coe]
    rw [←actualIntegralChart a place A B t,coeff_map]
  unfold integralQuotient exactQuotient quotientSeries rationalInverse
  rw [←PowerSeries.trunc_map,map_mul,he]

theorem actualPolynomialDegreeEnough {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (A B : (Fin M→Bool)→integralRing) :
    (actualPolynomial a place A B).natDegree<∑ t, sourceOrder a M u t := by
  have hs := sourceJetDegreeMargin a u M hu hua hMu
  have hd := actualPackedContractionDegree a place (fun y => (A y:ℚ)) (fun x => (B x:ℚ))
  have hcomm : M*(2*(2^(a+1)-1))=2*M*(2^(a+1)-1) := by ring
  rw [hcomm] at hd
  change (actualPolynomial a place A B).natDegree<sourceJetTotal a (sourceCeiling M u) u
  exact hd.trans_lt hs

theorem actualRecoveryExact {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (A B : (Fin M→Bool)→integralRing) :
    recoveryFunctional a M u (integralQuotient a u place A B)=(actualPolynomial a place A B).eval 0 := by
  have he := exactHermiteEvaluation a M u (actualPolynomial a place A B)
    (actualPolynomialDegreeEnough a u hu hua hMu place A B)
  calc
    _ = rationalRecoveryFunctional a M u (fun t => exactQuotient a M u t (actualPolynomial a place A B)) := by
      unfold recoveryFunctional rationalRecoveryFunctional
      apply Finset.sum_congr rfl
      intro t ht
      dsimp only
      rw [←integralQuotientMap a u place A B t,eval_map]
    _ = _ := he.symm

theorem integralQuotientDegree {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a u : ℕ) (place : Fin M→κ×Fin (2^a)) (A B : (Fin M→Bool)→integralRing)
    (t : Fin (2^(a+1))) (ht : t∈sampleNodes a M u) :
    (integralQuotient a u place A B t).natDegree<sourceOrder a M u t := by
  have hq := (Finset.mem_filter.mp ht).2
  unfold integralQuotient
  have he : sourceOrder a M u t=(sourceOrder a M u t-1)+1 := by omega
  rw [he]
  exact PowerSeries.natDegree_trunc_lt _ _

theorem actualPolynomialAtZero {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a : ℕ) (place : Fin M→κ×Fin (2^a)) (hinj : Function.Injective place)
    (A B : (Fin M→Bool)→integralRing) :
    (actualPolynomial a place A B).eval 0=((∑ x, A x*B x:integralRing):ℚ) := by
  have he := packedPointClearing a place (fun y => (A y:ℚ)) (fun x => (B x:ℚ)) 0
    (by rw [RationalScalar.denominator_origin]; norm_num)
  simp only [RationalScalar.denominator_origin,one_pow,one_mul] at he
  rw [actualPolynomialFormula,←he,actualPackedOriginContraction a place hinj]
  change (∑ x, (A x:ℚ)*(B x:ℚ))=integralRing.subtype (∑ x, A x*B x)
  rw [map_sum]
  apply Finset.sum_congr rfl
  intro x hx
  rw [map_mul]
  rfl

/-- Complete rational-coefficient recovery precision for actual quotient jets.
The supplied jet congruence is explicit; finite input construction follows separately. -/
theorem actualPackedRecoveryPrecision {κ : Type*} [DecidableEq κ] {M : ℕ}
    (p a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (hinj : Function.Injective place) (A B : (Fin M→Bool)→integralRing)
    (J : Fin (2^(a+1))→Polynomial integralRing)
    (hJ : ∀ t∈sampleNodes a M u, (J t).natDegree<sourceOrder a M u t)
    (hc : ∀ t∈sampleNodes a M u, ∀ j<sourceOrder a M u t,
      (2:integralRing)^(childPrecision p a M u t) ∣
        (J t).coeff j-(integralQuotient a u place A B t).coeff j) :
    padicNorm 2 ((recoveryFunctional a M u J-((∑ x, A x*B x:integralRing):ℚ))/(2:ℚ)^p)≤1 := by
  have he := actualRecoveryFunctionalStable p a M u hMu J (integralQuotient a u place A B) hJ
    (integralQuotientDegree a u place A B) hc
  rw [actualRecoveryExact a u hu hua hMu place A B,actualPolynomialAtZero a place hinj A B] at he
  exact he

end IntegralRecovery
#print axioms IntegralRecovery.actualIntegralChart
#print axioms IntegralRecovery.integralQuotientMap
#print axioms IntegralRecovery.actualRecoveryExact
#print axioms IntegralRecovery.actualPackedRecoveryPrecision
