import ChoicePairing

namespace WeilDivisorChoiceAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit WeilAuxiliaryRatioAudit EllipticPicardBridgeAudit
open EllipticGlobalReciprocityAudit
variable {K : Type*} [Field K]

theorem corrected_function_divisor (W : WeierstrassCurve.Affine K) (n : ℤ)
    (A : DivisorPair W n) (D' : W.Point →₀ ℤ) (h : W.FunctionFieldˣ)
    (hh : functionDivisor W h h.ne_zero=D'-A.D) :
    functionDivisor W (↑(A.f*h^n) : W.FunctionField) (A.f*h^n).ne_zero=n • D' := by
  change principalDivisorHom W (Additive.ofMul (A.f*h^n))=_
  rw [ofMul_mul,map_add]
  change functionDivisor W A.f A.f.ne_zero+
    functionDivisor W (↑(h^n) : W.FunctionField) (h^n).ne_zero=_
  rw [A.divF,unit_function_divisor_zpow,hh,smul_sub]
  abel

def replaceLeft (W : WeierstrassCurve.Affine K) (n : ℤ) (A : DivisorPair W n)
    (D' : W.Point →₀ ℤ) (hdeg : divisorDegree W D'=0)
    (hd : Disjoint D'.support A.E.support) (h : W.FunctionFieldˣ)
    (hh : functionDivisor W h h.ne_zero=D'-A.D) : DivisorPair W n :=
  ⟨⟨⟨D',A.E⟩,⟨A.f*h^n,A.g⟩⟩,hdeg,A.degE,hd,
    corrected_function_divisor W n A D' h hh,A.divG⟩

theorem correction_domain (W : WeierstrassCurve.Affine K) (n : ℤ) (A : DivisorPair W n)
    (D' : W.Point →₀ ℤ) (hd : Disjoint D'.support A.E.support) (h : W.FunctionFieldˣ)
    (hh : functionDivisor W h h.ne_zero=D'-A.D) :
    Disjoint A.E.support (functionDivisor W h h.ne_zero).support := by
  apply Finset.disjoint_left.mpr
  intro P hE hH
  have h0 : A.D P=0 := Finsupp.notMem_support_iff.mp (fun hm=>Finset.disjoint_left.mp A.disjoint hm hE)
  have h1 : D' P=0 := Finsupp.notMem_support_iff.mp (fun hm=>Finset.disjoint_left.mp hd hm hE)
  apply Finsupp.mem_support_iff.mp hH
  rw [hh,Finsupp.sub_apply,h0,h1,sub_self]

variable [IsAlgClosed K]

/-- One actual representative change; all evaluation domains are derived. -/
theorem corrected_left_invariance (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (A : DivisorPair W n) (D' : W.Point →₀ ℤ) (hdeg : divisorDegree W D'=0)
    (hd : Disjoint D'.support A.E.support) (h : W.FunctionFieldˣ)
    (hh : functionDivisor W h h.ne_zero=D'-A.D) :
    pairValue W n (replaceLeft W n A D' hdeg hd h hh)=pairValue W n A := by
  have hEH:=correction_domain W n A D' hd h hh
  have hfg : Disjoint (functionDivisor W A.g A.g.ne_zero).support
      (functionDivisor W h h.ne_zero).support := by
    rw [A.divG]
    exact Finset.disjoint_of_subset_left Finsupp.support_smul hEH
  have hr:=actual_geometric_disjoint_evaluation W hΔ A.g h hfg
  rw [proper_evaluation_helper,proper_evaluation_helper,A.divG,
    (exponentEvaluation W h).map_zsmul n A.E,toMul_zsmul] at hr
  have hDE : D'=A.D+functionDivisor W h h.ne_zero := by rw [hh];abel
  have hnum : (exponentEvaluation W A.g D').toMul=
      (exponentEvaluation W A.g A.D).toMul*(exponentEvaluation W h A.E).toMul^n := by
    rw [hDE,map_add,toMul_add,hr]
  have hn : Disjoint A.E.support (functionDivisor W (↑(h^n) : W.FunctionField) (h^n).ne_zero).support := by
    rw [unit_function_divisor_zpow]
    exact Finset.disjoint_of_subset_right Finsupp.support_smul hEH
  have hf:=domain_of_multiple W A.D A.E n A.f A.divF A.disjoint.symm
  have he:=evaluate_divisor_mul W A.E A.f (h^n) hf hn
  rw [proper_evaluation_function_zpow W A.E h n hEH hn] at he
  simp only [proper_evaluation_helper] at he
  rw [pair_value_helper,pair_value_helper]
  change (exponentEvaluation W A.g D').toMul/
    (exponentEvaluation W (A.f*h^n) A.E).toMul=_
  rw [hnum,he]
  exact mul_div_mul_right_eq_div _ _ _

variable [DecidableEq K]

/-- Equal actual divisor classes suffice for a one-coordinate change. -/
theorem pair_value_left_choice (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (n : ℤ) (A B : DivisorPair W n) (hE : A.E=B.E)
    (hD : divisorPointSum W A.D=divisorPointSum W B.D) : pairValue W n A=pairValue W n B := by
  have hp : B.D-A.D ∈ principalPointDivisors W := by
    apply (principal_iff_degree_and_point_sum W hΔ _).mpr
    exact ⟨by rw [map_sub,B.degD,A.degD,sub_self],by rw [map_sub,hD,sub_self]⟩
  obtain ⟨u,hu⟩:=hp
  have hd : Disjoint B.D.support A.E.support := by rw [hE];exact B.disjoint
  let C:=replaceLeft W n A B.D B.degD hd u.toMul hu
  have hc : pairValue W n C=pairValue W n A := corrected_left_invariance W hΔ n A B.D B.degD hd u.toMul hu
  exact hc.symm.trans (pair_value_function_choice W hΔ n C B rfl hE)

#print axioms corrected_function_divisor
#print axioms corrected_left_invariance
#print axioms pair_value_left_choice
end
end WeilDivisorChoiceAudit
