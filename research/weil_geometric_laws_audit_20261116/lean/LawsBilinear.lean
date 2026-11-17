import LawsInputs

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit
open WeilDivisorChoiceAudit EllipticPicardBridgeAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

omit [DecidableEq K] [IsAlgClosed K] in
theorem sum_disjoint (W : WeierstrassCurve.Affine K) (D E T : W.Point →₀ ℤ)
    (hD : Disjoint D.support T.support) (hE : Disjoint E.support T.support) :
    Disjoint (D+E).support T.support := by
  apply Finset.disjoint_left.mpr
  intro P hP hT
  have hd : D P=0 := Finsupp.notMem_support_iff.mp (fun hm=>Finset.disjoint_left.mp hD hm hT)
  have he : E P=0 := Finsupp.notMem_support_iff.mp (fun hm=>Finset.disjoint_left.mp hE hm hT)
  exact (Finsupp.mem_support_iff.mp hP) (by rw [Finsupp.add_apply,hd,he,zero_add])

omit [DecidableEq K] [IsAlgClosed K] in
theorem sum_function_divisor (W : WeierstrassCurve.Affine K) (n : ℤ)
    (A B : DivisorPair W n) :
    functionDivisor W (↑(A.f*B.f) : W.FunctionField) (A.f*B.f).ne_zero=n • (A.D+B.D) := by
  change principalDivisorHom W (Additive.ofMul (A.f*B.f))=_
  rw [ofMul_mul,map_add]
  change functionDivisor W A.f A.f.ne_zero+functionDivisor W B.f B.f.ne_zero=_
  rw [A.divF,B.divF,smul_add]

def sumPair (W : WeierstrassCurve.Affine K) (n : ℤ) (A B : DivisorPair W n)
    (he : A.E=B.E) : DivisorPair W n :=
  ⟨⟨⟨A.D+B.D,A.E⟩,⟨A.f*B.f,A.g⟩⟩,
    by rw [map_add,A.degD,B.degD,zero_add],A.degE,
    sum_disjoint W A.D B.D A.E A.disjoint (by rw [he];exact B.disjoint),
    sum_function_divisor W n A B,A.divG⟩

omit [DecidableEq K] in
theorem pair_value_sum (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (A B : DivisorPair W n) (he : A.E=B.E) :
    pairValue W n (sumPair W n A B he)=pairValue W n A*pairValue W n B := by
  have hBG : Disjoint B.D.support (functionDivisor W A.g A.g.ne_zero).support :=
    domain_of_multiple W A.E B.D n A.g A.divG (by rw [he];exact B.disjoint)
  have hBB := domain_of_multiple W B.E B.D n B.g B.divG B.disjoint
  have hgg : functionDivisor W A.g A.g.ne_zero=functionDivisor W B.g B.g.ne_zero := by rw [A.divG,B.divG,he]
  have hg:=geometric_degree_zero_choice W hΔ B.D B.degD A.g B.g hBG hBB hgg
  simp only [proper_evaluation_helper] at hg
  have hAF:=domain_of_multiple W A.D A.E n A.f A.divF A.disjoint.symm
  have hBF:=domain_of_multiple W B.D A.E n B.f B.divF (by rw [he];exact B.disjoint.symm)
  have hf:=evaluate_divisor_mul W A.E A.f B.f hAF hBF
  simp only [proper_evaluation_helper] at hf
  rw [pair_value_helper,pair_value_helper,pair_value_helper]
  change (exponentEvaluation W A.g (A.D+B.D)).toMul/
    (exponentEvaluation W (A.f*B.f) A.E).toMul=_
  rw [map_add,toMul_add,hf,hg,he]
  simp only [div_eq_mul_inv,mul_inv_rev]
  ac_rfl

theorem geometric_pairing_add_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P R Q : torsionPoints W n) :
    geometricPairing W hΔ n (P+R) Q=geometricPairing W hΔ n P Q*geometricPairing W hΔ n R Q := by
  let A:=selectedPair W hΔ n P Q
  obtain ⟨B,hE,hG,hR⟩:=pair_with_right W hΔ n A.E A.degE A.g A.divG R
  have hP:= (selected_pair_classes W hΔ n P Q).1
  have hQ:= (selected_pair_classes W hΔ n P Q).2
  have hsum : divisorPointSum W (sumPair W n A B hE.symm).D=(P+R).val := by
    change divisorPointSum W (A.D+B.D)=P.val+R.val
    rw [map_add,hP,hR]
  rw [geometric_pairing_of_data W hΔ n (P+R) Q (sumPair W n A B hE.symm) hsum hQ,
    pair_value_sum W hΔ n A B hE.symm,
    geometric_pairing_of_data W hΔ n R Q B hR (by rw [hE];exact hQ)]
  rfl

theorem geometric_pairing_add_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q R : torsionPoints W n) :
    geometricPairing W hΔ n P (Q+R)=geometricPairing W hΔ n P Q*geometricPairing W hΔ n P R := by
  rw [geometric_pairing_swap,geometric_pairing_add_left,mul_inv_rev,
    geometric_pairing_swap W hΔ n Q P,geometric_pairing_swap W hΔ n R P]
  exact mul_comm _ _

theorem geometric_pairing_zero_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (Q : torsionPoints W n) : geometricPairing W hΔ n 0 Q=1 := by
  have hh:=geometric_pairing_add_left W hΔ n 0 0 Q
  simp only [zero_add] at hh
  exact mul_eq_left.mp hh.symm

theorem geometric_pairing_zero_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P : torsionPoints W n) : geometricPairing W hΔ n P 0=1 := by
  rw [geometric_pairing_swap,geometric_pairing_zero_left,inv_one]

theorem geometric_pairing_neg_left (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) : geometricPairing W hΔ n (-P) Q=(geometricPairing W hΔ n P Q)⁻¹ := by
  have hh:=geometric_pairing_add_left W hΔ n P (-P) Q
  rw [add_neg_cancel,geometric_pairing_zero_left] at hh
  apply (mul_left_cancel (a:=geometricPairing W hΔ n P Q))
  rw [←hh]
  simp

theorem geometric_pairing_neg_right (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P Q : torsionPoints W n) : geometricPairing W hΔ n P (-Q)=(geometricPairing W hΔ n P Q)⁻¹ := by
  rw [geometric_pairing_swap,geometric_pairing_neg_left,
    geometric_pairing_swap W hΔ n Q P,inv_inv]

#print axioms sum_function_divisor
#print axioms pair_value_sum
#print axioms geometric_pairing_add_left
#print axioms geometric_pairing_add_right
end
end WeilGeometricLawsAudit
