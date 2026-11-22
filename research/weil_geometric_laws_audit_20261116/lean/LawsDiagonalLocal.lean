import LawsBilinear

namespace WeilGeometricLawsAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit WeilAuxiliaryRatioAudit WeilDivisorChoiceAudit EllipticTameSymbolAudit
variable {K : Type*} [Field K]

theorem order_of_multiple (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
    (n : ℤ) (D : W.Point →₀ ℤ) (hf : functionDivisor W f f.ne_zero=n • D) (P : W.Point) :
    pointOrder W f P=n*D P := by
  have h:=congrArg (fun d : W.Point →₀ ℤ=>d P) hf
  simpa only [function_divisor_apply,Finsupp.smul_apply,smul_eq_mul] using h

theorem proper_factor_of_unit (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (P : W.Point) (h0 : pointOrder W f P=0) :
    properFactor W D f hd P=localUnitValue W P f h0^D P := by
  by_cases hn : D P=0
  · rw [proper_factor_zero W D f hd P hn,hn,zpow_zero]
  · exact proper_factor_nonzero W D f hd P hn h0

theorem diagonal_local_sign (n d : ℤ) : (-1 : Kˣ)^(n*d*(-d))=(-1 : Kˣ)^(n*d) := by
  simp only [neg_one_zpow_eq_ite,Int.even_mul,even_neg,or_assoc,or_self]

/-- Actual tame word at a diagonal correction; no evaluation of a zero/pole is used. -/
theorem diagonal_local_correction (W : WeierstrassCurve.Affine K) (n : ℤ)
    (D E : W.Point →₀ ℤ) (f h : W.FunctionFieldˣ)
    (hf : functionDivisor W f f.ne_zero=n • D)
    (hh : functionDivisor W h h.ne_zero=E-D) (hd : Disjoint D.support E.support)
    (hGD : Disjoint D.support (functionDivisor W (↑(f*h^n) : W.FunctionField) (f*h^n).ne_zero).support)
    (hFE : Disjoint E.support (functionDivisor W f f.ne_zero).support) (P : W.Point) :
    tameSymbol W P f h*properFactor W D (f*h^n) hGD P/properFactor W E f hFE P=
      (-1 : Kˣ)^(n*D P) := by
  have hfo:=order_of_multiple W f n D hf P
  have hho : pointOrder W h P=E P-D P := by
    have he:=congrArg (fun d : W.Point →₀ ℤ=>d P) hh
    simpa only [function_divisor_apply,Finsupp.sub_apply] using he
  by_cases hD : D P=0
  · have hF0 : pointOrder W f P=0 := by rw [hfo,hD,mul_zero]
    rw [tame_first_unit W P f h hF0,hho,hD,sub_zero,
      proper_factor_zero W D (f*h^n) hGD P hD,
      proper_factor_of_unit W E f hFE P hF0,mul_one,div_self',mul_zero,zpow_zero]
  · have hE : E P=0 := Finsupp.notMem_support_iff.mp
      (fun hm=>Finset.disjoint_left.mp hd (Finsupp.mem_support_iff.mpr hD) hm)
    have hG0 : pointOrder W (↑(f*h^n) : W.FunctionField) P=0 :=
      (mem_support_units_iff W D.support (f*h^n)).mp
        ((disjoint_principal_support_iff W (f*h^n) D).mp hGD) P (Finsupp.mem_support_iff.mpr hD)
    let gu : (RegularRing W P).unitGroup := ⟨f*h^n,(mem_point_units_iff W P _).mpr hG0⟩
    have hb : balancedUnit W P f h=(f*h^n)^(-D P) := by
      simp only [balancedUnit,hfo,hho,hE,zero_sub,mul_zpow,zpow_mul,zpow_neg,div_eq_mul_inv,mul_inv_rev]
      exact mul_comm _ _
    have hbl : balancedLocalUnit W P f h=gu^(-D P) := Subtype.ext hb
    rw [tameSymbol,hfo,hho,hE,zero_sub,hbl,map_zpow,
      proper_factor_of_unit W D (f*h^n) hGD P hG0,proper_factor_zero W E f hFE P hE,div_one]
    change (-1 : Kˣ)^(n*D P*(-D P))*localUnitValue W P (f*h^n) hG0^(-D P)*
      localUnitValue W P (f*h^n) hG0^D P=_
    rw [mul_assoc,←zpow_add,neg_add_cancel,zpow_zero,mul_one]
    exact diagonal_local_sign n (D P)

#print axioms diagonal_local_sign
#print axioms diagonal_local_correction
end
end WeilGeometricLawsAudit
