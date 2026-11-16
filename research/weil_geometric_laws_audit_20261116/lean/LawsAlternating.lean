import LawsDiagonalLocal

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine Function
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit WeilAuxiliaryRatioAudit WeilDivisorChoiceAudit EllipticTameSymbolAudit
open EllipticPicardBridgeAudit EllipticGlobalReciprocityAudit
variable {K : Type*} [Field K]

theorem corrected_multiple_divisor (W : WeierstrassCurve.Affine K) (n : ℤ)
    (D E : W.Point →₀ ℤ) (f h : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=n • D) (hh : functionDivisor W h h.ne_zero=E-D) :
    functionDivisor W (↑(f*h^n) : W.FunctionField) (f*h^n).ne_zero=n • E := by
  change principalDivisorHom W (Additive.ofMul (f*h^n))=_
  rw [ofMul_mul,map_add]
  change functionDivisor W f f.ne_zero+functionDivisor W (↑(h^n) : W.FunctionField) (h^n).ne_zero=_
  rw [hf,unit_function_divisor_zpow,hh,smul_sub]
  abel

variable [IsAlgClosed K]

/-- Full diagonal cancellation, including even levels, from actual balanced local units. -/
theorem diagonal_corrected_value (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (D E : W.Point →₀ ℤ) (f h : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=n • D) (hh : functionDivisor W h h.ne_zero=E-D)
    (hd : Disjoint D.support E.support) :
    evaluateDivisor W D (f*h^n)
      (domain_of_multiple W E D n (f*h^n) (corrected_multiple_divisor W n D E f h hf hh) hd)/
    evaluateDivisor W E f (domain_of_multiple W D E n f hf hd.symm)=1 := by
  let hGD:=domain_of_multiple W E D n (f*h^n) (corrected_multiple_divisor W n D E f h hf hh) hd
  let hFE:=domain_of_multiple W D E n f hf hd.symm
  have hT:=tame_support_finite W f h
  have hD:=proper_factor_finite W D (f*h^n) hGD
  have hE:=proper_factor_finite W E f hFE
  have he : (∏ᶠ P,tameSymbol W P f h*properFactor W D (f*h^n) hGD P/properFactor W E f hFE P)=
      globalTameProduct W f f := by
    apply finprod_congr
    intro P
    rw [diagonal_local_correction W n D E f h hf hh hd hGD hFE P,tame_diagonal,order_of_multiple W f n D hf P]
  rw [finprod_div_distrib (f:=fun P=>tameSymbol W P f h*properFactor W D (f*h^n) hGD P)
      (g:=fun P=>properFactor W E f hFE P) (hT.mul hD) hE,
    finprod_mul_distrib (f:=fun P=>tameSymbol W P f h)
      (g:=fun P=>properFactor W D (f*h^n) hGD P) hT hD,
    ←proper_divisor_finprod,←proper_divisor_finprod] at he
  change globalTameProduct W f h*evaluateDivisor W D (f*h^n) hGD/evaluateDivisor W E f hFE=globalTameProduct W f f at he
  rw [actual_geometric_reciprocity W hΔ f h,geometric_diagonal_product W hΔ f,one_mul] at he
  exact he

variable [DecidableEq K]

/-- Actual alternating law, without odd-level or characteristic restrictions. -/
theorem geometric_pairing_alternating (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (n : ℤ)
    (P : torsionPoints W n) : geometricPairing W hΔ n P P=1 := by
  let A:=selectedPair W hΔ n P P
  have hp : A.E-A.D ∈ principalPointDivisors W := by
    apply (principal_iff_degree_and_point_sum W hΔ _).mpr
    refine ⟨by rw [map_sub,A.degE,A.degD,sub_self],?_⟩
    rw [map_sub,(selected_pair_classes W hΔ n P P).1,(selected_pair_classes W hΔ n P P).2,sub_self]
  obtain ⟨u,hu⟩:=hp
  let B : DivisorPair W n := ⟨⟨⟨A.D,A.E⟩,⟨A.f,A.f*u.toMul^n⟩⟩,
    A.degD,A.degE,A.disjoint,A.divF,corrected_multiple_divisor W n A.D A.E A.f u.toMul A.divF hu⟩
  change pairValue W n A=1
  rw [pair_value_function_choice W hΔ n A B rfl rfl]
  exact diagonal_corrected_value W hΔ n A.D A.E A.f u.toMul A.divF hu A.disjoint

#print axioms corrected_multiple_divisor
#print axioms diagonal_corrected_value
#print axioms geometric_pairing_alternating
end
end WeilGeometricLawsAudit
