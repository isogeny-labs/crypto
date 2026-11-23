module
public import HermiteUniqueness
public section
namespace HermiteBasis
open Polynomial NodeMultiplicity HermiteUniqueness LocalIntegral FiniteBuffers

theorem sourceCeilingUpper (M u : ℕ) (hMu : u≤M) : sourceCeiling M u*u≤4*M := by
  have hb : sourceCeiling M u*u≤3*M+u-1 := by
    unfold sourceCeiling
    rw [Nat.ceilDiv_eq_add_pred_div]
    exact Nat.div_mul_le_self _ _
  have hs := Nat.sub_le (3*M+u) 1
  omega

theorem sourceOrderUpper (a M u : ℕ) (hMu : u≤M) (m : Fin (2^(a+1))) :
    sourceOrder a M u m≤4*M := by
  unfold sourceOrder
  split_ifs
  · omega
  · exact (Nat.mul_le_mul_left _ (Nat.sub_le _ _)).trans (sourceCeilingUpper M u hMu)

@[expose] noncomputable def basis (a M u : ℕ) (t : Fin (2^(a+1))) : Polynomial ℚ :=
  ∏ s ∈ Finset.univ.erase t,
    (C (((gridNode a t:ℚ)-(gridNode a s:ℚ))⁻¹)*(X-C (gridNode a s:ℚ)))^(sourceOrder a M u s)

@[expose] noncomputable def localBasis (a M u : ℕ) (t : Fin (2^(a+1))) : Polynomial integralRing :=
  ∏ s ∈ Finset.univ.erase t, (1+C (chartRatio a t s)*X)^(sourceOrder a M u s)

theorem gridDifferenceNonzero (a : ℕ) (t s : Fin (2^(a+1))) (hst : s≠t) :
    (gridNode a t:ℚ)-(gridNode a s:ℚ)≠0 := by
  have hn := (sourceGridInjective a).ne hst.symm
  exact_mod_cast sub_ne_zero.mpr hn

/-- The integral local polynomial represents the actual normalized Hermite basis. -/
theorem actualBasisLocalEval (a M u : ℕ) (t : Fin (2^(a+1))) (z : ℚ) :
    (localBasis a M u t).eval₂ integralRing.subtype z =
      (basis a M u t).eval ((gridNode a t:ℚ)+((2^(a+1):ℕ):ℚ)*z) := by
  unfold localBasis basis
  simp only [eval₂_finsetProd,eval₂_pow,eval₂_add,eval₂_one,eval₂_mul,eval₂_C,eval₂_X,
    eval_prod,eval_pow,eval_mul,eval_C,eval_sub,eval_X]
  apply Finset.prod_congr rfl
  intro s hs
  congr 1
  have hn := gridDifferenceNonzero a t s (Finset.ne_of_mem_erase hs)
  change 1+(((2^(a+1):ℕ):ℚ)/((t:ℚ)-(s:ℚ)))*z =
    ((gridNode a t:ℚ)-(gridNode a s:ℚ))⁻¹*
      (((gridNode a t:ℚ)+((2^(a+1):ℕ):ℚ)*z)-(gridNode a s:ℚ))
  have he : (gridNode a t:ℚ)-(gridNode a s:ℚ)=(t:ℚ)-(s:ℚ) := by
    simp only [gridNode,Int.cast_sub,Int.cast_natCast]
    ring
  rw [he] at hn ⊢
  simp only [gridNode,Int.cast_sub,Int.cast_natCast]
  field_simp [hn]
  <;> ring

theorem localBasisConstant (a M u : ℕ) (t : Fin (2^(a+1))) :
    (localBasis a M u t).coeff 0=1 := by
  simp [localBasis,coeff_zero_eq_eval_zero,eval_prod]

theorem actualBasisAtNode (a M u : ℕ) (t : Fin (2^(a+1))) :
    (basis a M u t).eval (gridNode a t:ℚ)=1 := by
  have he := actualBasisLocalEval a M u t 0
  simpa only [mul_zero,add_zero,eval₂_at_zero,localBasisConstant,map_one] using he.symm

@[expose] noncomputable def localSeries (a M u : ℕ) (t : Fin (2^(a+1))) : PowerSeries integralRing :=
  (localBasis a M u t).toPowerSeries

@[expose] noncomputable def localInverse (a M u : ℕ) (t : Fin (2^(a+1))) : PowerSeries integralRing :=
  PowerSeries.invOfUnit (localSeries a M u t) 1

theorem localSeriesConstant (a M u : ℕ) (t : Fin (2^(a+1))) :
    PowerSeries.constantCoeff (localSeries a M u t)=1 := localBasisConstant a M u t

theorem actualBasisInverse (a M u : ℕ) (t : Fin (2^(a+1))) :
    localSeries a M u t*localInverse a M u t=1 :=
  PowerSeries.mul_invOfUnit _ _ (localSeriesConstant a M u t)

/-- The quotient's finite inverse is built from its input coefficients alone. -/
theorem actualBasisMappedInverseBuffer {R : Type*} [CommRing R]
    (π : integralRing →+* R) (a M u : ℕ) (t : Fin (2^(a+1))) (q : ℕ) :
    invertBuffer q (fun n : Fin q => π (PowerSeries.coeff n (localSeries a M u t))) 1 =
      fun n : Fin q => π (PowerSeries.coeff n (localInverse a M u t)) := by
  by_cases hq : q=0
  · subst q; funext n; exact Fin.elim0 n
  have hqpos : 0<q := Nat.pos_of_ne_zero hq
  let v : Fin q → R := fun n => π (PowerSeries.coeff n (localSeries a M u t))
  let D := PowerSeries.map π (localSeries a M u t)
  let I := PowerSeries.map π (localInverse a M u t)
  let E := liftBuffer q v
  let F := PowerSeries.invOfUnit E 1
  have hDI : D*I=1 := by
    dsimp [D,I]
    rw [←map_mul,actualBasisInverse,map_one]
  have hc : PowerSeries.constantCoeff E=1 := by
    rw [←PowerSeries.coeff_zero_eq_constantCoeff_apply]
    have he := liftBuffer_coeff q v (⟨0,hqpos⟩ : Fin q)
    change PowerSeries.coeff 0 E=1
    rw [he]
    change π (PowerSeries.coeff 0 (localSeries a M u t))=1
    rw [PowerSeries.coeff_zero_eq_constantCoeff_apply,localSeriesConstant,map_one]
  have hEF : E*F=1 := PowerSeries.mul_invOfUnit E 1 hc
  have hi : ∀ n<q, PowerSeries.coeff n E=PowerSeries.coeff n D := by
    intro n hn
    simpa only [E,D,PowerSeries.coeff_map,v] using liftBuffer_coeff q v (⟨n,hn⟩ : Fin q)
  funext n
  simpa only [invertBuffer,F,I,PowerSeries.coeff_map,v] using
    inverse_low_coefficients q D I E F hDI hEF hi n n.isLt

@[expose] noncomputable def integralEvalNumerator (P : Polynomial integralRing)
    (b t : integralRing) (q : ℕ) : integralRing :=
  ∑ j ∈ Finset.range q, P.coeff j*t^j*b^(q-j)

/-- Explicit denominator clearing bounds actual evaluation loss, independently
of interpolation and without treating a power of two as a finite-ring unit. -/
theorem evaluationDenominatorClearing (P : Polynomial integralRing) (q : ℕ)
    (hdeg : P.natDegree<q) (b t : integralRing) (hb : (b:ℚ)≠0) :
    (b:ℚ)^q*P.eval₂ integralRing.subtype ((t:ℚ)/(b:ℚ)) =
      (integralEvalNumerator P b t q:ℚ) := by
  rw [eval₂_eq_sum_range' integralRing.subtype hdeg,Finset.mul_sum]
  change _ = integralRing.subtype (∑ j ∈ Finset.range q, P.coeff j*t^j*b^(q-j))
  rw [map_sum]
  apply Finset.sum_congr rfl
  intro j hj
  simp only [map_mul,map_pow]
  change (b:ℚ)^q*((P.coeff j:ℚ)*((t:ℚ)/(b:ℚ))^j) =
    (P.coeff j:ℚ)*(t:ℚ)^j*(b:ℚ)^(q-j)
  rw [div_pow,pow_sub₀ (b:ℚ) hb (Nat.le_of_lt (Finset.mem_range.mp hj))]
  simp only [div_eq_mul_inv]
  ring

theorem actualEvaluationLoss (a M u : ℕ) (hMu : u≤M) (m : Fin (2^(a+1)))
    (P : Polynomial integralRing) (hdeg : P.natDegree<sourceOrder a M u m) :
    padicNorm 2 ((2:ℚ)^(4*M*(a+1))*
      P.eval₂ integralRing.subtype (-(gridNode a m:ℚ)/((2^(a+1):ℕ):ℚ)))≤1 := by
  let b : integralRing := (2:integralRing)^(a+1)
  let t : integralRing := -(gridNode a m:integralRing)
  have hb : (b:ℚ)=(2:ℚ)^(a+1) := by
    change integralRing.subtype ((2:integralRing)^(a+1))=_
    rw [map_pow,map_ofNat]
  have ht : (t:ℚ)=-(gridNode a m:ℚ) := by simp [t]
  have hn : (b:ℚ)≠0 := by rw [hb]; positivity
  have he := evaluationDenominatorClearing P (4*M)
    (hdeg.trans_le (sourceOrderUpper a M u hMu m)) b t hn
  have he' : (2:ℚ)^(4*M*(a+1))*
      P.eval₂ integralRing.subtype (-(gridNode a m:ℚ)/((2^(a+1):ℕ):ℚ)) =
        (integralEvalNumerator P b t (4*M):ℚ) := by
    simpa only [hb,ht,←pow_mul,Nat.mul_comm,Nat.cast_pow,Nat.cast_ofNat] using he
  rw [he']
  exact (integralEvalNumerator P b t (4*M)).property

end HermiteBasis
#print axioms HermiteBasis.sourceOrderUpper
#print axioms HermiteBasis.actualBasisLocalEval
#print axioms HermiteBasis.actualBasisInverse
#print axioms HermiteBasis.actualBasisMappedInverseBuffer
#print axioms HermiteBasis.actualEvaluationLoss
