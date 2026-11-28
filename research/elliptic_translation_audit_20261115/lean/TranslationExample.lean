module
public import TranslationPicard

@[expose] public section
namespace EllipticTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerNormValuationAudit MillerBasechangeAudit

/-- The credited curve of Galbraith Example9.2.4, over actual coefficient fields. -/
def translationExampleCurve (K : Type*) [Field K] : WeierstrassCurve.Affine K := ⟨0,0,0,1,0⟩

theorem translation_example_nonsingular_zero (K : Type*) [Field K] :
    (translationExampleCurve K).Nonsingular 0 0 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
  norm_num [translationExampleCurve]

def translationExampleQ (K : Type*) [Field K] : (translationExampleCurve K).Point :=
  Point.some 0 0 (translation_example_nonsingular_zero K)

theorem translation_example_relation (K : Type*) [Field K] :
    yFunction (translationExampleCurve K)^2=xFunction (translationExampleCurve K)^3+
      xFunction (translationExampleCurve K) := by
  have he := function_curve_relation (translationExampleCurve K)
  have h0 : constantHom (translationExampleCurve K) (0 : K)=0 := map_zero _
  have h1 : constantHom (translationExampleCurve K) (1 : K)=1 := map_one _
  change yFunction (translationExampleCurve K)^2+
    (constantHom (translationExampleCurve K) 0*xFunction (translationExampleCurve K)+
      constantHom (translationExampleCurve K) 0)*yFunction (translationExampleCurve K)=
    xFunction (translationExampleCurve K)^3+constantHom (translationExampleCurve K) 0*
      xFunction (translationExampleCurve K)^2+constantHom (translationExampleCurve K) 1*
      xFunction (translationExampleCurve K)+constantHom (translationExampleCurve K) 0 at he
  simp only [h0,h1,zero_mul,one_mul,add_zero] at he
  exact he

abbrev TranslationBase5 := ZMod 5
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev TranslationClosure5 := AlgebraicClosure TranslationBase5
local instance : CommRing TranslationBase5 := (inferInstance : Field TranslationBase5).toCommRing
local instance : CommRing TranslationClosure5 := (inferInstance : Field TranslationClosure5).toCommRing

theorem translation_example_delta_five : (translationExampleCurve TranslationBase5).Δ ≠ 0 := by
  decide +kernel

theorem translation_example_delta_closure : (translationExampleCurve TranslationClosure5).Δ ≠ 0 := by
  have hm : translationExampleCurve TranslationClosure5 =
      (translationExampleCurve TranslationBase5).map (algebraMap TranslationBase5 TranslationClosure5) := by
    ext <;> simp [translationExampleCurve]
  rw [hm,WeierstrassCurve.map_Δ]
  simpa only [map_zero] using
    (algebraMap TranslationBase5 TranslationClosure5).injective.ne translation_example_delta_five

theorem translation_example_q_double :
    translationExampleQ TranslationBase5+translationExampleQ TranslationBase5=0 := by decide +kernel

/-- Raw affine substitution at the explicitly excluded point Q, on the actual curve. -/
def rawExampleAtQ : (translationExampleCurve TranslationBase5).Point :=
  Point.some ((0 : TranslationBase5)⁻¹) (-(0 : TranslationBase5)/(0 : TranslationBase5)^2)
    (by simpa using translation_example_nonsingular_zero TranslationBase5)

theorem raw_example_at_q_wrong :
    rawExampleAtQ ≠ translationExampleQ TranslationBase5+translationExampleQ TranslationBase5 := by
  decide +kernel

def translationExampleP5 : (translationExampleCurve TranslationBase5).Point :=
  Point.some 2 0 (by
    rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
    decide +kernel)

def translationExampleS5 : (translationExampleCurve TranslationBase5).Point :=
  Point.some 3 0 (by
    rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
    decide +kernel)

theorem translation_example_guarded_positive :
    (2 : TranslationBase5) ≠ 0 ∧
    translationExampleP5+translationExampleQ TranslationBase5=translationExampleS5 ∧
    (((2 : TranslationBase5)⁻¹),-(0 : TranslationBase5)/(2 : TranslationBase5)^2)=(3,0) := by
  decide +kernel

#print axioms raw_example_at_q_wrong
#print axioms translation_example_delta_closure
end
end EllipticTranslationAudit
