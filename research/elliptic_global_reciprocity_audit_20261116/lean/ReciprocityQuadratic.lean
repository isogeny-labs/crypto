import ReciprocityProduct

namespace EllipticGlobalReciprocityAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerInfinityChartAudit MillerFinitePointAudit MillerRecursionAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit NegationPullbackAudit
open FinitePointEvaluationAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open EllipticTameSymbolAudit EllipticFieldNormAudit EllipticNormTransportAudit EllipticInfinityNormProjectionAudit
open EllipticSplitNormProjectionAudit EllipticRamifiedNormProjectionAudit EllipticFinitePlaceProjectionAudit
open P1TameReciprocityAudit
open scoped EllipticFieldNormAudit WithZero
variable {K : Type*} [Field K]
attribute [local instance] normFunctionCommRing

theorem actual_function_linear_expression (W : WeierstrassCurve.Affine K) (f : W.FunctionField) :
    ∃ p q : RatFunc K,f=algebraMap (RatFunc K) W.FunctionField p+
      algebraMap (RatFunc K) W.FunctionField q*yFunction W := by
  refine ⟨(functionBasis W).repr f 0,(functionBasis W).repr f 1,?_⟩
  have he:=(functionBasis W).sum_repr f
  simpa only [Fin.sum_univ_two,function_basis_zero,function_basis_one,Algebra.smul_def,mul_one] using he.symm

theorem actual_base_scalar_nonzero (W : WeierstrassCurve.Affine K) (c : RatFunc K) (hc : c≠0) :
    algebraMap (RatFunc K) W.FunctionField c≠0 := by
  intro he
  apply hc
  apply (algebraMap (RatFunc K) W.FunctionField).injective
  simpa only [map_zero] using he

/-- Derived from the actual basis; no abstract quadratic module is supplied. -/
theorem actual_nonbase_affine_relation (W : WeierstrassCurve.Affine K) (f g : W.FunctionField)
    (hn : ∀ c : RatFunc K,f≠algebraMap (RatFunc K) W.FunctionField c) :
    ∃ a b : RatFunc K,g=algebraMap (RatFunc K) W.FunctionField a*f+
      algebraMap (RatFunc K) W.FunctionField b := by
  obtain ⟨p,q,hf⟩:=actual_function_linear_expression W f
  obtain ⟨r,s,hg⟩:=actual_function_linear_expression W g
  have hq : q≠0 := by
    intro he
    rw [he,map_zero,zero_mul,add_zero] at hf
    exact hn p hf
  have hq':=actual_base_scalar_nonzero W q hq
  refine ⟨s/q,r-(s/q)*p,?_⟩
  rw [hg,hf,map_sub,map_mul,map_div₀]
  field_simp
  ring

theorem actual_base_unit_of_scalar (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ)
    (c : RatFunc K) (he : (f : W.FunctionField)=algebraMap (RatFunc K) W.FunctionField c) :
    ∃ u : (RatFunc K)ˣ,f=baseFieldUnit W u := by
  have hc : c≠0 := by
    intro hz
    rw [hz,map_zero] at he
    exact f.ne_zero he
  refine ⟨Units.mk0 c hc,?_⟩
  apply Units.ext
  exact he

def globalRightCharacter (W : WeierstrassCurve.Affine K) (f : W.FunctionFieldˣ) :
    W.FunctionFieldˣ →* Kˣ where
  toFun g:=globalTameProduct W f g
  map_one':=by
    have h:=global_mul_right W f 1 1
    simpa only [one_mul,mul_eq_left] using h.symm
  map_mul':=fun g h=>global_mul_right W f g h

theorem global_div_right (W : WeierstrassCurve.Affine K) (f g h : W.FunctionFieldˣ) :
    globalTameProduct W f (g/h)=globalTameProduct W f g/globalTameProduct W f h :=
  (globalRightCharacter W f).map_div g h

theorem actual_base_factor_invariance [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (a b : (RatFunc K)ˣ) (f g : W.FunctionFieldˣ) :
    globalTameProduct W (baseFieldUnit W a*f) (g/baseFieldUnit W b)=globalTameProduct W f g := by
  rw [global_mul_left]
  have hl : globalTameProduct W (baseFieldUnit W a) (g/baseFieldUnit W b)=1 := by
    rw [global_skew,actual_mixed_reciprocity W hΔ (g/baseFieldUnit W b) a,inv_one]
  rw [hl,one_mul,global_div_right,actual_mixed_reciprocity W hΔ f b,div_one]

/-- Full geometric reciprocity, using actual basis coefficients and the actual Steinberg relation. -/
theorem actual_geometric_reciprocity [IsAlgClosed K] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) (f g : W.FunctionFieldˣ) : globalTameProduct W f g=1 := by
  by_cases hf : ∃ c : RatFunc K,(f : W.FunctionField)=algebraMap (RatFunc K) W.FunctionField c
  · obtain ⟨c,hc⟩:=hf
    obtain ⟨u,hu⟩:=actual_base_unit_of_scalar W f c hc
    rw [hu,global_skew,actual_mixed_reciprocity W hΔ g u,inv_one]
  · have hn : ∀ c : RatFunc K,(f : W.FunctionField)≠algebraMap (RatFunc K) W.FunctionField c :=
      fun c hc=>hf ⟨c,hc⟩
    obtain ⟨a,b,hg⟩:=actual_nonbase_affine_relation W f g hn
    by_cases ha : a=0
    · rw [ha,map_zero,zero_mul,zero_add] at hg
      obtain ⟨u,hu⟩:=actual_base_unit_of_scalar W g b hg
      rw [hu,actual_mixed_reciprocity W hΔ f u]
    · by_cases hb : b=0
      · rw [hb,map_zero,add_zero] at hg
        let au : (RatFunc K)ˣ:=Units.mk0 a ha
        have he : g=baseFieldUnit W au*f := by
          apply Units.ext
          exact hg
        rw [he,global_mul_right,actual_mixed_reciprocity W hΔ f au,geometric_diagonal_product W hΔ f,mul_one]
      · let au : (RatFunc K)ˣ:=Units.mk0 a ha
        let bu : (RatFunc K)ˣ:=Units.mk0 b hb
        let U : W.FunctionFieldˣ:=baseFieldUnit W (-au/bu)*f
        let V : W.FunctionFieldˣ:=g/baseFieldUnit W bu
        have hb':=actual_base_scalar_nonzero W b hb
        have hs : (U : W.FunctionField)+(V : W.FunctionField)=1 := by
          change ((baseFieldUnit W (-au/bu)*f : W.FunctionFieldˣ) : W.FunctionField)+
            ((g/baseFieldUnit W bu : W.FunctionFieldˣ) : W.FunctionField)=1
          rw [Units.val_mul,Units.val_div_eq_div_val]
          change algebraMap (RatFunc K) W.FunctionField ((-au/bu : (RatFunc K)ˣ) : RatFunc K)*(f : W.FunctionField)+
            (g : W.FunctionField)/algebraMap (RatFunc K) W.FunctionField (bu : RatFunc K)=1
          rw [Units.val_div_eq_div_val]
          change algebraMap (RatFunc K) W.FunctionField (-a/b)*(f : W.FunctionField)+
            (g : W.FunctionField)/algebraMap (RatFunc K) W.FunctionField b=1
          rw [hg,map_div₀,map_neg]
          field_simp
          ring
        have hstein:=global_steinberg W U V hs
        change globalTameProduct W (baseFieldUnit W (-au/bu)*f) (g/baseFieldUnit W bu)=1 at hstein
        rw [actual_base_factor_invariance W hΔ] at hstein
        exact hstein

#print axioms actual_function_linear_expression
#print axioms actual_nonbase_affine_relation
#print axioms actual_base_unit_of_scalar
#print axioms actual_base_factor_invariance
#print axioms actual_geometric_reciprocity
end
end EllipticGlobalReciprocityAudit
