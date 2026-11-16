import TameProduct

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open Function
variable {K : Type*} [Field K]

theorem tame_first_unit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hf : pointOrder W f P=0) :
    tameSymbol W P f g=(localUnitValue W P f hf)^pointOrder W g P := by
  have hb : balancedLocalUnit W P f g=
      (⟨f,(mem_point_units_iff W P f).mpr hf⟩ : (RegularRing W P).unitGroup)^pointOrder W g P := by
    apply Subtype.ext
    simp only [balancedLocalUnit,balancedUnit,hf,zpow_zero,div_one,Subgroup.coe_zpow]
  rw [tameSymbol,hf,zero_mul,zpow_zero,one_mul,hb,map_zpow]
  rfl

theorem tame_second_unit (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f g : W.FunctionFieldˣ) (hg : pointOrder W g P=0) :
    tameSymbol W P f g=((localUnitValue W P g hg)^pointOrder W f P)⁻¹ := by
  rw [tame_skew,tame_first_unit W P g f hg]

/-- An actual factor in the existing proper divisor evaluation, with its actual support guard. -/
def properFactor (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (P : W.Point) : Kˣ :=
  supportPointEvaluation W D.support P
    ⟨f,(disjoint_principal_support_iff W f D).mp hd⟩^D P

theorem proper_factor_zero (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (P : W.Point) (hP : D P=0) : properFactor W D f hd P=1 := by
  simp only [properFactor,hP,zpow_zero]

theorem proper_factor_nonzero (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support)
    (P : W.Point) (hP : D P≠0) (hf : pointOrder W f P=0) :
    properFactor W D f hd P=(localUnitValue W P f hf)^D P := by
  classical
  have hm : P∈D.support := Finsupp.mem_support_iff.mpr hP
  simp only [properFactor,supportPointEvaluation,dite_eq_left hm,MonoidHom.comp_apply]
  rfl

theorem proper_factor_finite (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support) :
    HasFiniteMulSupport (properFactor W D f hd) := by
  apply D.support.finite_toSet.subset
  intro P hP
  apply Finsupp.mem_support_iff.mpr
  intro h
  exact hP (proper_factor_zero W D f hd P h)

theorem proper_divisor_finprod (W : WeierstrassCurve.Affine K) (D : W.Point →₀ ℤ)
    (f : W.FunctionFieldˣ) (hd : Disjoint D.support (functionDivisor W f f.ne_zero).support) :
    evaluateDivisor W D f hd=∏ᶠ P,properFactor W D f hd P := by
  classical
  have hs : mulSupport (properFactor W D f hd) ⊆ D.support := by
    intro P hP
    apply Finsupp.mem_support_iff.mpr
    intro h
    exact hP (proper_factor_zero W D f hd P h)
  rw [finprod_eq_prod_of_mulSupport_subset _ hs]
  exact divisor_evaluation_on_apply W D.support D ⟨f,(disjoint_principal_support_iff W f D).mp hd⟩

/-- Exact classical order: finite product = f(div g) / g(div f), with both actual guards. -/
theorem global_disjoint_evaluation (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ)
    (hd : Disjoint (functionDivisor W f f.ne_zero).support (functionDivisor W g g.ne_zero).support) :
    globalTameProduct W f g=
      evaluateDivisor W (functionDivisor W g g.ne_zero) f hd.symm/
        evaluateDivisor W (functionDivisor W f f.ne_zero) g hd := by
  rw [proper_divisor_finprod,proper_divisor_finprod,←finprod_div_distrib
    (proper_factor_finite W _ f hd.symm) (proper_factor_finite W _ g hd)]
  apply finprod_congr
  intro P
  have hu := (disjoint_principal_support_iff W g (functionDivisor W f f.ne_zero)).mp hd
  by_cases hf : pointOrder W f P=0
  · rw [proper_factor_zero W _ g hd P hf,div_one,tame_first_unit W P f g hf]
    by_cases hg : pointOrder W g P=0
    · rw [hg,zpow_zero,proper_factor_zero W _ f hd.symm P hg]
    · rw [proper_factor_nonzero W _ f hd.symm P hg hf]
      rfl
  · have hg : pointOrder W g P=0 := (mem_support_units_iff W _ g).mp hu P
      (Finsupp.mem_support_iff.mpr hf)
    rw [proper_factor_zero W _ f hd.symm P hg,one_div,tame_second_unit W P f g hg,
      proper_factor_nonzero W _ g hd P hf hg]
    rfl

/-- The familiar disjoint evaluation identity is equivalent to product1 in this exact convention.
This theorem does not assert either side for arbitrary f,g. -/
theorem disjoint_reciprocity_iff (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ)
    (hd : Disjoint (functionDivisor W f f.ne_zero).support (functionDivisor W g g.ne_zero).support) :
    globalTameProduct W f g=1 ↔
      evaluateDivisor W (functionDivisor W g g.ne_zero) f hd.symm=
        evaluateDivisor W (functionDivisor W f f.ne_zero) g hd := by
  rw [global_disjoint_evaluation W f g hd,div_eq_one]

#print axioms proper_divisor_finprod
#print axioms global_disjoint_evaluation
#print axioms disjoint_reciprocity_iff
end
end EllipticTameSymbolAudit
