import TameParameter

namespace EllipticTameSymbolAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open Function
variable {K : Type*} [Field K]

/-- Honest finite multiplicative support, derived before using finprod's convention. -/
theorem tame_support_finite (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ) :
    HasFiniteMulSupport (fun P => tameSymbol W P f g) := by
  apply ((function_order_support_finite W f f.ne_zero).union
    (function_order_support_finite W g g.ne_zero)).subset
  intro P hP
  by_contra hn
  have hf : pointOrder W f P=0 := by
    by_contra h
    exact hn (Or.inl h)
  have hg : pointOrder W g P=0 := by
    by_contra h
    exact hn (Or.inr h)
  exact hP (tame_units W P f g hf hg)

def globalTameProduct (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ) : Kˣ :=
  ∏ᶠ P, tameSymbol W P f g

theorem global_mul_left (W : WeierstrassCurve.Affine K) (f g h : W.FunctionFieldˣ) :
    globalTameProduct W (f*g) h=globalTameProduct W f h*globalTameProduct W g h := by
  simp only [globalTameProduct,tame_mul_left]
  exact finprod_mul_distrib (tame_support_finite W f h) (tame_support_finite W g h)

theorem global_mul_right (W : WeierstrassCurve.Affine K) (f g h : W.FunctionFieldˣ) :
    globalTameProduct W f (g*h)=globalTameProduct W f g*globalTameProduct W f h := by
  simp only [globalTameProduct,tame_mul_right]
  exact finprod_mul_distrib (tame_support_finite W f g) (tame_support_finite W f h)

theorem global_skew (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ) :
    globalTameProduct W g f=(globalTameProduct W f g)⁻¹ := by
  unfold globalTameProduct
  calc
    _ = ∏ᶠ P,(tameSymbol W P f g)⁻¹ := finprod_congr (fun P => tame_skew W P f g)
    _ = _ := finprod_inv_distrib _

theorem global_steinberg (W : WeierstrassCurve.Affine K) (f g : W.FunctionFieldˣ)
    (hs : (f : W.FunctionField)+(g : W.FunctionField)=1) : globalTameProduct W f g=1 := by
  simp only [globalTameProduct,tame_steinberg W _ f g hs,finprod_one]

def coefficientUnit (W : WeierstrassCurve.Affine K) : Kˣ →* W.FunctionFieldˣ :=
  Units.map (constantHom W).toMonoidHom

theorem coefficient_order (W : WeierstrassCurve.Affine K) (c : Kˣ) (P : W.Point) :
    pointOrder W (coefficientUnit W c) P=0 := constant_point_order W c.ne_zero P

theorem tame_constant (W : WeierstrassCurve.Affine K) (P : W.Point)
    (c : Kˣ) (g : W.FunctionFieldˣ) : tameSymbol W P (coefficientUnit W c) g=c^pointOrder W g P := by
  have hb : balancedLocalUnit W P (coefficientUnit W c) g=
      (⟨coefficientUnit W c,(mem_point_units_iff W P _).mpr (coefficient_order W c P)⟩ :
        (RegularRing W P).unitGroup)^pointOrder W g P := by
    apply Subtype.ext
    simp only [balancedLocalUnit,balancedUnit,coefficient_order,zpow_zero,div_one,Subgroup.coe_zpow]
  rw [tameSymbol,coefficient_order,zero_mul,zpow_zero,one_mul,hb,map_zpow]
  change (localUnitValue W P (coefficientUnit W c) (coefficient_order W c P))^pointOrder W g P=_
  exact congrArg (fun z : Kˣ => z^pointOrder W g P) (local_value_constant W P c)

theorem product_orders (W : WeierstrassCurve.Affine K) (c : Kˣ) (f : W.FunctionFieldˣ) :
    (∏ᶠ P, c^pointOrder W f P)=c^divisorDegree W (functionDivisor W f f.ne_zero) := by
  classical
  have hs : mulSupport (fun P => c^pointOrder W f P) ⊆
      (functionDivisor W f f.ne_zero).support := by
    intro P hP
    apply Finsupp.mem_support_iff.mpr
    intro h
    exact hP (by
      change c^pointOrder W f P=1
      rw [function_divisor_apply] at h
      rw [h,zpow_zero])
  rw [finprod_eq_prod_of_mulSupport_subset _ hs]
  change (∏ P ∈ (functionDivisor W f f.ne_zero).support,c^pointOrder W f P)=
    c^(∑ P ∈ (functionDivisor W f f.ne_zero).support,pointOrder W f P)
  have he (S : Finset W.Point) : (∏ P ∈ S,c^pointOrder W f P)=c^(∑ P ∈ S,pointOrder W f P) := by
    induction S using Finset.induction_on with
    | empty => simp
    | @insert P S hP ih => rw [Finset.prod_insert hP,Finset.sum_insert hP,ih,zpow_add]
  exact he _

/-- Exact formula over the actual coefficient field; the unweighted rational divisor may
have nonzero degree if the field is not algebraically closed. -/
theorem global_constant_degree (W : WeierstrassCurve.Affine K) (c : Kˣ) (g : W.FunctionFieldˣ) :
    globalTameProduct W (coefficientUnit W c) g=c^divisorDegree W (functionDivisor W g g.ne_zero) := by
  simp only [globalTameProduct,tame_constant]
  exact product_orders W c g

theorem global_diagonal_degree (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    globalTameProduct W f f=(-1 : Kˣ)^divisorDegree W (functionDivisor W f f.ne_zero) := by
  simp only [globalTameProduct,tame_diagonal]
  exact product_orders W (-1) f

variable [IsAlgClosed K]

/-- Geometric constant-function reciprocity, derived from actual degreezero. -/
theorem geometric_constant_product (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (c : Kˣ) (g : W.FunctionFieldˣ) : globalTameProduct W (coefficientUnit W c) g=1 := by
  rw [global_constant_degree,function_divisor_degree_zero W hΔ g.ne_zero,zpow_zero]

theorem geometric_diagonal_product (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (f : W.FunctionFieldˣ) : globalTameProduct W f f=1 := by
  rw [global_diagonal_degree,function_divisor_degree_zero W hΔ f.ne_zero,zpow_zero]

#print axioms tame_support_finite
#print axioms global_steinberg
#print axioms global_constant_degree
#print axioms geometric_constant_product
#print axioms geometric_diagonal_product
end
end EllipticTameSymbolAudit
