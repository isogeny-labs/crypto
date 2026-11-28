module
public import TranslationSign

@[expose] public section
namespace LocalTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit
open FunctionDivisorFinitenessAudit FinitePointEvaluationAudit
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
attribute [local instance] signBaseCommRing signClosureCommRing
local instance : DecidableEq SignClosure := Classical.decEq _

/-- Actual elliptic translation local/divisor/evaluation compatibility, with an actual sign counter.
Neither local preservation nor valuation/residue invariance is a hypothesis. -/
theorem actual_local_translation_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      (∀ (P Q : W.Point) (f : W.FunctionField),
        localFunctionValuation W P (fieldTranslation W hΔ Q f)=localFunctionValuation W (P+Q) f) ∧
      (∀ (P Q : W.Point) (f : W.FunctionField),
        pointOrder W (fieldTranslation W hΔ Q f) P=pointOrder W f (P+Q)) ∧
      (∀ (P Q : W.Point) (r : RegularRing W (P+Q)),
        (regularTranslationEquiv W hΔ P Q r : W.FunctionField)=fieldTranslation W hΔ Q r) ∧
      (∀ (P Q : W.Point) (r : RegularRing W P),
        ((regularTranslationEquiv W hΔ P Q).symm r : W.FunctionField)=fieldTranslation W hΔ (-Q) r) ∧
      (∀ (P Q : W.Point) (r : RegularRing W (P+Q)),
        regularEvaluation W P (regularTranslation W hΔ P Q r)=regularEvaluation W (P+Q) r) ∧
      (∀ (Q : W.Point) (f : W.FunctionField) (hf : f≠0),
        functionDivisor W (fieldTranslation W hΔ Q f) (translation_nonzero W hΔ Q hf)=
          divisorTranslation W (-Q) (functionDivisor W f hf)) ∧
      (∀ (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ),
        Disjoint D.support
          (functionDivisor W (translatedUnit W hΔ Q f) (translatedUnit W hΔ Q f).ne_zero).support ↔
        Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support) ∧
      (∀ (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ)
        (hd : Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support),
        evaluateDivisor W D (translatedUnit W hΔ Q f)
          ((translation_disjoint_support W hΔ Q D f).mpr hd)=
        evaluateDivisor W (divisorTranslation W Q D) f hd)) ∧
    CharP SignClosure 5 ∧ signCurve.Δ≠0 ∧ signPoint≠0 ∧
    (3 : ℕ) • signPoint=0 ∧ signPoint≠-signPoint ∧ signFunction≠0 ∧
    pointOrder signCurve signFunction signPoint=3 ∧
    pointOrder signCurve signFunction (-signPoint)=0 ∧
    pointOrder signCurve (fieldTranslation signCurve sign_delta signPoint signFunction) 0=3 ∧
    functionDivisor signCurve (fieldTranslation signCurve sign_delta signPoint signFunction)
      (translation_nonzero signCurve sign_delta signPoint sign_function_nonzero)≠
      divisorTranslation signCurve signPoint (functionDivisor signCurve signFunction sign_function_nonzero) := by
  refine ⟨?_,sign_actual_char5,sign_delta,sign_point_nonzero,sign_three,sign_point_ne_negative,
    sign_function_nonzero,sign_point_order_three,sign_negative_order_zero,sign_pullback_order_three,
    wrong_divisor_translation_sign⟩
  intro K _ _ _ W hΔ
  refine ⟨translation_valuation W hΔ,translation_point_order W hΔ,?_,
    regular_translation_equiv_symm W hΔ,translation_residue W hΔ,?_,
    translation_disjoint_support W hΔ,translation_divisor_evaluation W hΔ⟩
  · intro P Q r
    exact regular_translation_coe W hΔ P Q r
  · intro Q f hf
    exact translation_function_divisor W hΔ Q hf

#print axioms actual_local_translation_certificate
end
end LocalTranslationAudit
