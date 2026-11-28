module
public import GeneralControls

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit MillerFinitePointAudit MillerBasechangeAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit FinitePointEvaluationAudit

section ExplicitExtensionInterfaces
variable {K : Type*} [Field K]

/-- Explicit-field interface to the actual mapped coefficient model. -/
abbrev extensionCurve (L : Type*) [Field L] [Algebra K L] (W : WeierstrassCurve.Affine K) :=
  extendedCurve (L := L) W

/-- Explicit-field interface to the actual full function-field embedding. -/
abbrev extensionField (L : Type*) [Field L] [Algebra K L] (W : WeierstrassCurve.Affine K) :
    W.FunctionField →+* (extensionCurve L W).FunctionField := fieldBaseChange (L := L) W

theorem extension_delta (L : Type*) [Field L] [Algebra K L] (W : WeierstrassCurve.Affine K)
    (hΔ : W.Δ≠0) : (extensionCurve L W).Δ≠0 := extended_delta_nonzero W hΔ

variable [DecidableEq K]

/-- Body is the actual official coefficient-point map. -/
abbrev extensionPoint (L : Type*) [Field L] [Algebra K L] [DecidableEq L]
    (W : WeierstrassCurve.Affine K) : W.Point →+ (extensionCurve L W).Point :=
  pointBaseChange (L := L) W

/-- Body is the actual regular-ring coefficient-extension map, with preservation derived. -/
abbrev extensionRegular (L : Type*) [Field L] [Algebra K L] [DecidableEq L]
    (W : WeierstrassCurve.Affine K) (P : W.Point) :
    RegularRing W P →+* RegularRing (extensionCurve L W) (extensionPoint L W P) :=
  regularBasechange (L := L) W P
end ExplicitExtensionInterfaces

local instance : Fact (Nat.Prime 2) := ⟨by decide⟩
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
attribute [local instance] generalF2CommRing generalF3CommRing generalF5CommRing

/-- Actual arbitrary-field elliptic translations, extension/local compatibility, and finite controls.
No algebraic-closure, transcendence, local-preservation or valuation-invariance premise is supplied.
Rational-point divisors over nonclosed fields are not asserted to have geometric degree zero. -/
theorem actual_general_translation_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      (∀ (Q : W.Point) (c : K),
        generalTranslationEquiv W hΔ Q (constantHom W c)=constantHom W c) ∧
      (∀ Q R : W.Point, (generalTranslation W hΔ Q).comp (generalTranslation W hΔ R)=
        generalTranslation W hΔ (Q+R)) ∧
      generalTranslation W hΔ 0=RingHom.id W.FunctionField ∧
      (∀ (Q : W.Point) (f : W.FunctionField),
        (generalTranslationEquiv W hΔ Q).symm f=generalTranslation W hΔ (-Q) f) ∧
      Function.Injective (generalTranslation W hΔ) ∧
      (∀ (P Q : W.Point) (f : W.FunctionField),
        localFunctionValuation W P (generalTranslation W hΔ Q f)=localFunctionValuation W (P+Q) f) ∧
      (∀ (P Q : W.Point) (f : W.FunctionField),
        pointOrder W (generalTranslation W hΔ Q f) P=pointOrder W f (P+Q)) ∧
      (∀ (P Q : W.Point) (r : RegularRing W (P+Q)),
        (generalRegularEquiv W hΔ P Q r : W.FunctionField)=generalTranslation W hΔ Q r) ∧
      (∀ (P Q : W.Point) (r : RegularRing W P),
        ((generalRegularEquiv W hΔ P Q).symm r : W.FunctionField)=generalTranslation W hΔ (-Q) r) ∧
      (∀ (P Q : W.Point) (r : RegularRing W (P+Q)),
        regularEvaluation W P (generalRegularTranslation W hΔ P Q r)=regularEvaluation W (P+Q) r) ∧
      (∀ (Q : W.Point) (f : W.FunctionField) (hf : f≠0),
        functionDivisor W (generalTranslation W hΔ Q f) (general_translation_nonzero W hΔ Q hf)=
          divisorTranslation W (-Q) (functionDivisor W f hf)) ∧
      (∀ (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ),
        Disjoint D.support
          (functionDivisor W (generalTranslatedUnit W hΔ Q f) (generalTranslatedUnit W hΔ Q f).ne_zero).support ↔
        Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support) ∧
      (∀ (Q : W.Point) (D : W.Point →₀ ℤ) (f : W.FunctionFieldˣ)
        (hd : Disjoint (divisorTranslation W Q D).support (functionDivisor W f f.ne_zero).support),
        evaluateDivisor W D (generalTranslatedUnit W hΔ Q f)
          ((general_translation_disjoint_support W hΔ Q D f).mpr hd)=
        evaluateDivisor W (divisorTranslation W Q D) f hd)) ∧
    (∀ (K L : Type) [Field K] [Field L] [Algebra K L] [DecidableEq K] [DecidableEq L]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
      (∀ Q : W.Point, (extensionField L W).comp (generalTranslation W hΔ Q)=
        (generalTranslation (extensionCurve L W) (extension_delta L W hΔ) (extensionPoint L W Q)).comp
          (extensionField L W)) ∧
      (∀ (P : W.Point) (f : W.FunctionField),
        localFunctionValuation (extensionCurve L W) (extensionPoint L W P) (extensionField L W f)=
          localFunctionValuation W P f) ∧
      (∀ (P : W.Point) (f : W.FunctionField),
        pointOrder (extensionCurve L W) (extensionField L W f) (extensionPoint L W P)=pointOrder W f P) ∧
      (∀ P : W.Point, Function.Injective (extensionRegular L W P)) ∧
      (∀ (P : W.Point) (r : RegularRing W P),
        regularEvaluation (extensionCurve L W) (extensionPoint L W P) (extensionRegular L W P r)=
          algebraMap K L (regularEvaluation W P r))) ∧
    (∀ (K : Type) [Field K] [DecidableEq K] [IsAlgClosed K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0) (Q : W.Point),
      generalTranslation W hΔ Q=fieldTranslation W hΔ Q) ∧
    CharP GeneralF2 2 ∧ binaryCurve.Δ≠0 ∧ binaryPoint≠0 ∧
    generalTranslation binaryCurve binary_delta binaryPoint≠RingHom.id binaryCurve.FunctionField ∧
    CharP GeneralF3 3 ∧ ternaryCurve.Δ≠0 ∧ ternaryPoint≠0 ∧
    generalTranslation ternaryCurve ternary_delta ternaryPoint≠RingHom.id ternaryCurve.FunctionField ∧
    CharP GeneralF5 5 ∧ (translationExampleCurve GeneralF5).Δ≠0 ∧
    (¬ ∃ a : (translationExampleCurve GeneralF5).CoordinateRing,
      generalTranslation (translationExampleCurve GeneralF5) finite_example_delta
        (translationExampleQ GeneralF5) (xFunction (translationExampleCurve GeneralF5))=
      algebraMap (translationExampleCurve GeneralF5).CoordinateRing
        (translationExampleCurve GeneralF5).FunctionField a) ∧
    (∀ Q : rationalCurve.Point,
      divisorDegree rationalCurve
        (functionDivisor rationalCurve (generalTranslation rationalCurve rational_delta Q numeratorFunction)
          (general_translation_nonzero rationalCurve rational_delta Q numerator_function_nonzero))=(-4 : ℤ)) := by
  refine ⟨?_,?_,?_,actual_binary_characteristic,binary_delta,binary_point_nonzero,
    binary_translation_nontrivial,actual_ternary_characteristic,ternary_delta,ternary_point_nonzero,
    ternary_translation_nontrivial,actual_five_characteristic,finite_example_delta,
    finite_example_not_affine,rational_bad_degree_survives⟩
  · intro K _ _ W hΔ
    refine ⟨?_,general_translation_comp W hΔ,general_translation_zero W hΔ,
      general_translation_equiv_symm W hΔ,general_translation_faithful W hΔ,
      general_translation_valuation W hΔ,general_translation_order W hΔ,?_,
      general_regular_equiv_symm W hΔ,general_translation_residue W hΔ,?_,
      general_translation_disjoint_support W hΔ,general_translation_divisor_evaluation W hΔ⟩
    · intro Q c
      rw [general_translation_equiv_apply,general_translation_constant]
    · intro P Q r
      exact general_regular_translation_coe W hΔ P Q r
    · intro Q f hf
      exact general_translation_function_divisor W hΔ Q hf
  · intro K L _ _ _ _ _ W hΔ
    exact ⟨general_translation_basechange W hΔ,local_valuation_basechange W,local_order_basechange W,
      regular_basechange_injective W,regular_basechange_residue W⟩
  · intro K _ _ _ W hΔ Q
    exact general_translation_agrees W hΔ Q

#print axioms actual_general_translation_certificate
end
end GeneralTranslationAudit
