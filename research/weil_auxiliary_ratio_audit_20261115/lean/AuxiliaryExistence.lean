import AuxiliaryEvaluation

namespace WeilAuxiliaryRatioAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open MillerNormValuationAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
open NegationPullbackAudit FinitePointEvaluationAudit GeneralTranslationAudit
variable {K : Type*} [Field K] [DecidableEq K] [IsAlgClosed K]

def pointAbscissa (W : WeierstrassCurve.Affine K) : W.Point → K
  | .zero => 0
  | .some x _ _ => x

omit [DecidableEq K] in
/-- Every abscissa has an actual nonsingular point over algebraically closed coefficients. -/
theorem exists_point_over_x (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (x : K) :
    ∃ y : K, W.Nonsingular x y := by
  have hd : (fiberPolynomial W x).degree≠0 := by
    intro hz
    have hc : (fiberPolynomial W x).coeff 2=0 :=
      coeff_eq_zero_of_degree_lt (by rw [hz];norm_num)
    simp only [fiberPolynomial,coeff_sub,coeff_add,coeff_C_mul,coeff_X_pow,coeff_X,coeff_C] at hc
    norm_num at hc
  obtain ⟨y,hy⟩ := IsAlgClosed.exists_root (fiberPolynomial W x) hd
  exact ⟨y,(W.equation_iff_nonsingular_of_Δ_ne_zero hΔ).mp ((equation_fiber_root W x y).mpr hy)⟩

/-- An actual curve point exists outside any supplied finite set; no cardinality premise. -/
theorem exists_point_outside (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (T : Finset W.Point) :
    ∃ S : W.Point, S∉T := by
  obtain ⟨x,hx⟩ := (T.image (pointAbscissa W)).exists_notMem
  obtain ⟨y,hy⟩ := exists_point_over_x W hΔ x
  refine ⟨Point.some x y hy,?_⟩
  intro hS
  exact hx (Finset.mem_image.mpr ⟨Point.some x y hy,hS,rfl⟩)

/-- Every finite actual nonzero-function family has a common actual local-unit point. -/
theorem exists_common_unit_point (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (F : Finset W.FunctionFieldˣ) : ∃ S : W.Point, ∀ f∈F, pointOrder W f S=0 := by
  classical
  let T : Finset W.Point := F.biUnion (fun f => (functionDivisor W f f.ne_zero).support)
  obtain ⟨S,hS⟩ := exists_point_outside W hΔ T
  refine ⟨S,?_⟩
  intro f hf
  by_contra hz
  apply hS
  apply Finset.mem_biUnion.mpr
  exact ⟨f,hf,Finsupp.mem_support_iff.mpr hz⟩

/-- The actual four-factor admissible auxiliary domain is nonempty. -/
theorem exists_auxiliary_admissible (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0)
    (P Q : W.Point) (f g : W.FunctionFieldˣ) :
    ∃ S : W.Point, AuxiliaryAdmissible W P Q f g S := by
  classical
  let a := generalTranslatedUnit W hΔ Q f
  let b := negatedUnit W g
  let d := generalTranslatedUnit W hΔ (-P) b
  obtain ⟨S,hS⟩ := exists_common_unit_point W hΔ {a,b,f,d}
  have ha := hS a (by simp)
  have hb := hS b (by simp)
  have hf := hS f (by simp)
  have hd := hS d (by simp)
  change pointOrder W (generalTranslatedUnit W hΔ Q f) S=0 at ha
  rw [general_translated_unit_coe,general_translation_order] at ha
  change pointOrder W (fieldNeg W (g : W.FunctionField)) S=0 at hb
  rw [point_order_pullback] at hb
  change pointOrder W (generalTranslatedUnit W hΔ (-P) (negatedUnit W g)) S=0 at hd
  rw [general_translated_unit_coe,general_translation_order] at hd
  change pointOrder W (fieldNeg W (g : W.FunctionField)) (S+ -P)=0 at hd
  rw [point_order_pullback,show -(S+ -P)=P-S by abel] at hd
  exact ⟨S,ha,hb,hf,hd⟩

#print axioms exists_point_outside
#print axioms exists_common_unit_point
#print axioms exists_auxiliary_admissible
end
end WeilAuxiliaryRatioAudit
