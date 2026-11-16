module
public import HermiteError
public section
namespace HermiteReconstruction
open Polynomial NodeMultiplicity HermiteUniqueness HermiteBasis HermiteError LocalIntegral

@[expose] noncomputable def inverseChart (a : ℕ) (t : Fin (2^(a+1))) : Polynomial ℚ :=
  C ((sourceScale (R:=ℚ) a)⁻¹)*(X-C (sourcePoint (R:=ℚ) a t))

theorem sourceScaleNonzero (a : ℕ) : sourceScale (R:=ℚ) a≠0 := by simp [sourceScale]

theorem inverseChartCompSource (a : ℕ) (t : Fin (2^(a+1))) :
    (inverseChart a t).comp (C (sourcePoint (R:=ℚ) a t)+C (sourceScale (R:=ℚ) a)*X)=X := by
  simp only [inverseChart,mul_comp,C_comp,sub_comp,X_comp]
  calc
    _ = C ((sourceScale (R:=ℚ) a)⁻¹*sourceScale (R:=ℚ) a)*X := by rw [C_mul]; ring
    _ = _ := by rw [inv_mul_cancel₀ (sourceScaleNonzero a),C_1,one_mul]

theorem sourceChartBasis (a M u : ℕ) (t : Fin (2^(a+1))) :
    sourceChart a t (basis a M u t)=(localBasis a M u t).map integralRing.subtype := by
  apply Polynomial.funext
  intro z
  simp only [sourceChart,eval_comp,eval_add,eval_C,eval_mul,eval_X,eval_map]
  simpa [sourcePoint,sourceScale] using (actualBasisLocalEval a M u t z).symm

@[expose] noncomputable def rationalInverse (a M u : ℕ) (t : Fin (2^(a+1))) : PowerSeries ℚ :=
  PowerSeries.map integralRing.subtype (localInverse a M u t)

theorem actualChartSeriesInverse (a M u : ℕ) (t : Fin (2^(a+1))) :
    (sourceChart a t (basis a M u t)).toPowerSeries*rationalInverse a M u t=1 := by
  have he : (sourceChart a t (basis a M u t)).toPowerSeries=
      PowerSeries.map integralRing.subtype (localSeries a M u t) := by
    ext n
    simp only [PowerSeries.coeff_map,localSeries,Polynomial.coeff_coe]
    rw [sourceChartBasis,coeff_map]
  rw [he]
  unfold rationalInverse
  rw [←map_mul,actualBasisInverse,map_one]

@[expose] noncomputable def quotientSeries (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ) : PowerSeries ℚ :=
  (sourceChart a t F).toPowerSeries*rationalInverse a M u t

@[expose] noncomputable def exactQuotient (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ) : Polynomial ℚ :=
  PowerSeries.trunc (sourceOrder a M u t) (quotientSeries a M u t F)

@[expose] noncomputable def interpolationTerm (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ) : Polynomial ℚ :=
  basis a M u t*(exactQuotient a M u t F).comp (inverseChart a t)

theorem termOwnChart (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ) :
    sourceChart a t (interpolationTerm a M u t F)=
      sourceChart a t (basis a M u t)*exactQuotient a M u t F := by
  unfold sourceChart interpolationTerm
  rw [mul_comp,comp_assoc,inverseChartCompSource,comp_X]

theorem truncatedMultiplyCoefficient {R : Type*} [CommRing R] (D F : PowerSeries R) (q j : ℕ) (hj : j<q) :
    PowerSeries.coeff j (D*(PowerSeries.trunc q F).toPowerSeries)=PowerSeries.coeff j (D*F) := by
  rw [PowerSeries.coeff_mul,PowerSeries.coeff_mul]
  apply Finset.sum_congr rfl
  intro k hk
  have he := Finset.mem_antidiagonal.mp hk
  have hkq : k.2<q := by omega
  rw [Polynomial.coeff_coe,PowerSeries.coeff_trunc,if_pos hkq]

theorem termOwnJets (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ)
    (j : ℕ) (hj : j<sourceOrder a M u t) :
    (sourceChart a t (interpolationTerm a M u t F)).coeff j=(sourceChart a t F).coeff j := by
  have he : (sourceChart a t (basis a M u t)).toPowerSeries*quotientSeries a M u t F=
      (sourceChart a t F).toPowerSeries := by
    unfold quotientSeries
    calc
      _ = (sourceChart a t F).toPowerSeries*
          ((sourceChart a t (basis a M u t)).toPowerSeries*rationalInverse a M u t) := by ring
      _ = _ := by rw [actualChartSeriesInverse,mul_one]
  have hh := truncatedMultiplyCoefficient (sourceChart a t (basis a M u t)).toPowerSeries
    (quotientSeries a M u t F) (sourceOrder a M u t) j hj
  rw [he] at hh
  rw [termOwnChart]
  simpa only [←Polynomial.coe_mul,Polynomial.coeff_coe,exactQuotient] using hh

theorem basisOtherNodeDivisibility (a M u : ℕ) (t s : Fin (2^(a+1))) (hst : s≠t) :
    (X-C (gridNode a s:ℚ))^(sourceOrder a M u s) ∣ basis a M u t := by
  have hf : (X-C (gridNode a s:ℚ))^(sourceOrder a M u s) ∣
      (C (((gridNode a t:ℚ)-(gridNode a s:ℚ))⁻¹)*(X-C (gridNode a s:ℚ)))^(sourceOrder a M u s) := by
    rw [mul_pow]
    exact ⟨(C (((gridNode a t:ℚ)-(gridNode a s:ℚ))⁻¹))^(sourceOrder a M u s),by ring⟩
  exact hf.trans (Finset.dvd_prod_of_mem _ (Finset.mem_erase.mpr ⟨hst,Finset.mem_univ s⟩))

theorem scaledJetsZeroOfDivisibility (a : ℕ) (s : Fin (2^(a+1))) (q : ℕ) (P : Polynomial ℚ)
    (hd : (X-C (gridNode a s:ℚ))^q ∣ P) (j : ℕ) (hj : j<q) : (sourceChart a s P).coeff j=0 := by
  obtain ⟨H,hH⟩ := hd
  have hlinear : (X-C (gridNode a s:ℚ)).comp
      (C (sourcePoint (R:=ℚ) a s)+C (sourceScale (R:=ℚ) a)*X)=C (sourceScale (R:=ℚ) a)*X := by
    simp [sub_comp,sourcePoint]
  have he : (X:Polynomial ℚ)^q ∣ sourceChart a s P := by
    unfold sourceChart
    rw [hH,mul_comp,pow_comp,hlinear,mul_pow]
    exact ⟨(C (sourceScale (R:=ℚ) a))^q*H.comp
      (C (sourcePoint (R:=ℚ) a s)+C (sourceScale (R:=ℚ) a)*X),by ring⟩
  exact (Polynomial.X_pow_dvd_iff.mp he) j hj

theorem termOtherJets (a M u : ℕ) (t s : Fin (2^(a+1))) (hst : s≠t) (F : Polynomial ℚ)
    (j : ℕ) (hj : j<sourceOrder a M u s) :
    (sourceChart a s (interpolationTerm a M u t F)).coeff j=0 := by
  exact scaledJetsZeroOfDivisibility a s _ _
    (dvd_mul_of_dvd_left (basisOtherNodeDivisibility a M u t s hst) _) j hj

theorem normalizedLinearDegree (c d : ℚ) : (C c*(X-C d)).natDegree≤1 := by
  calc
    _ ≤ (C c).natDegree+(X-C d).natDegree := natDegree_mul_le
    _ = _ := by simp

theorem basisDegreeBound (a M u : ℕ) (t : Fin (2^(a+1))) :
    (basis a M u t).natDegree≤∑ s∈Finset.univ.erase t, sourceOrder a M u s := by
  unfold basis
  apply (natDegree_prod_le _ _).trans
  apply Finset.sum_le_sum
  intro s hs
  simpa only [mul_one] using
    natDegree_pow_le_of_le (sourceOrder a M u s)
      (normalizedLinearDegree (((gridNode a t:ℚ)-(gridNode a s:ℚ))⁻¹) (gridNode a s:ℚ))

theorem exactQuotientDegree (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ)
    (hq : 0<sourceOrder a M u t) : (exactQuotient a M u t F).natDegree<sourceOrder a M u t := by
  unfold exactQuotient
  have he : sourceOrder a M u t=(sourceOrder a M u t-1)+1 := by omega
  rw [he]
  exact PowerSeries.natDegree_trunc_lt _ _

theorem termDegreeBound (a M u : ℕ) (t : Fin (2^(a+1))) (F : Polynomial ℚ)
    (hq : 0<sourceOrder a M u t) :
    (interpolationTerm a M u t F).natDegree<∑ s, sourceOrder a M u s := by
  have hinv : (inverseChart a t).natDegree≤1 :=
    normalizedLinearDegree _ _
  have hcomp : ((exactQuotient a M u t F).comp (inverseChart a t)).natDegree≤
      (exactQuotient a M u t F).natDegree := by
    exact natDegree_comp_le.trans (by nlinarith)
  have hbase := basisDegreeBound a M u t
  have hQ := exactQuotientDegree a M u t F hq
  have hsum := Finset.sum_erase_add Finset.univ (sourceOrder a M u) (Finset.mem_univ t)
  have hterm := natDegree_mul_le (p:=basis a M u t)
    (q:=(exactQuotient a M u t F).comp (inverseChart a t))
  change (interpolationTerm a M u t F).natDegree≤_ at hterm
  omega

@[expose] noncomputable def interpolationPolynomial (a M u : ℕ) (F : Polynomial ℚ) : Polynomial ℚ :=
  ∑ t∈sampleNodes a M u, interpolationTerm a M u t F

theorem interpolationDegree (a M u : ℕ) (F : Polynomial ℚ) (hW : 0<∑ s, sourceOrder a M u s) :
    (interpolationPolynomial a M u F).natDegree<∑ s, sourceOrder a M u s := by
  have hb : (interpolationPolynomial a M u F).natDegree≤(∑ s, sourceOrder a M u s)-1 := by
    unfold interpolationPolynomial
    apply natDegree_sum_le_of_forall_le
    intro t ht
    have hh := termDegreeBound a M u t F ((Finset.mem_filter.mp ht).2)
    omega
  omega

theorem interpolationJets (a M u : ℕ) (F : Polynomial ℚ) (s : Fin (2^(a+1)))
    (j : ℕ) (hj : j<sourceOrder a M u s) :
    (sourceChart a s (interpolationPolynomial a M u F)).coeff j=(sourceChart a s F).coeff j := by
  have hs : s∈sampleNodes a M u := Finset.mem_filter.mpr ⟨Finset.mem_univ s,by omega⟩
  unfold sourceChart interpolationPolynomial
  rw [sum_comp,finsetSum_coeff]
  rw [Finset.sum_eq_single s]
  · exact termOwnJets a M u s F j hj
  · intro t ht hts
    exact termOtherJets a M u t s hts.symm F j hj
  · intro hnot
    exact False.elim (hnot hs)

/-- Explicit reconstruction from the actual exact quotient truncations.
No interpolation equality is assumed; jets and degree are proved above. -/
theorem exactReconstruction (a M u : ℕ) (F : Polynomial ℚ)
    (hF : F.natDegree<∑ s, sourceOrder a M u s) : interpolationPolynomial a M u F=F := by
  have hW : 0<∑ s, sourceOrder a M u s := Nat.zero_le F.natDegree |>.trans_lt hF
  apply exactHermiteUnique (sourcePoint (R:=ℚ) a) (sourceOrder a M u) (sourcePointUnitDifferences a)
    (interpolationPolynomial a M u F) F (interpolationDegree a M u F hW) hF
  intro t j hj
  have he := interpolationJets a M u F t j hj
  rw [sourceChartCoefficient,sourceChartCoefficient] at he
  exact mul_right_cancel₀ (pow_ne_zero _ (sourceScaleNonzero a)) he

theorem actualPackedReconstruction {κ : Type*} [DecidableEq κ] {M : ℕ}
    (a u : ℕ) (hu : 3≤u) (hua : u≤a) (hMu : u≤M) (place : Fin M→κ×Fin (2^a))
    (A B : (Fin M→Bool)→ℚ) :
    interpolationPolynomial a M u (PackedContraction.packedContractionPolynomial a place A B)=
      PackedContraction.packedContractionPolynomial a place A B := by
  apply exactReconstruction
  have hb := sourceJetDegreeMargin a u M hu hua hMu
  have hd := PackedContraction.actualPackedContractionDegree a place A B
  have hcomm : M*(2*(2^(a+1)-1))=2*M*(2^(a+1)-1) := by ring
  rw [hcomm] at hd
  change (PackedContraction.packedContractionPolynomial a place A B).natDegree<
    sourceJetTotal a (sourceCeiling M u) u
  exact hd.trans_lt hb

@[expose] noncomputable def rationalRecoveryFunctional (a M u : ℕ)
    (J : Fin (2^(a+1))→Polynomial ℚ) : ℚ :=
  ∑ t∈sampleNodes a M u, (basis a M u t).eval 0*
    (J t).eval (-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ))

theorem inverseChartAtZero (a : ℕ) (t : Fin (2^(a+1))) :
    (inverseChart a t).eval 0=-(gridNode a t:ℚ)/((2^(a+1):ℕ):ℚ) := by
  simp [inverseChart,sourcePoint,sourceScale,div_eq_mul_inv,mul_comm]

/-- The actual D.9 value formula, derived from the constructed interpolation. -/
theorem exactHermiteEvaluation (a M u : ℕ) (F : Polynomial ℚ)
    (hF : F.natDegree<∑ s, sourceOrder a M u s) :
    F.eval 0=rationalRecoveryFunctional a M u (fun t => exactQuotient a M u t F) := by
  calc
    _ = (interpolationPolynomial a M u F).eval 0 :=
      congrArg (Polynomial.eval 0) (exactReconstruction a M u F hF).symm
    _ = _ := by
      unfold interpolationPolynomial rationalRecoveryFunctional
      rw [eval_finsetSum]
      apply Finset.sum_congr rfl
      intro t ht
      rw [interpolationTerm,eval_mul,eval_comp,inverseChartAtZero]

end HermiteReconstruction
#print axioms HermiteReconstruction.termOwnJets
#print axioms HermiteReconstruction.termOtherJets
#print axioms HermiteReconstruction.interpolationDegree
#print axioms HermiteReconstruction.exactReconstruction
#print axioms HermiteReconstruction.actualPackedReconstruction
#print axioms HermiteReconstruction.exactHermiteEvaluation
