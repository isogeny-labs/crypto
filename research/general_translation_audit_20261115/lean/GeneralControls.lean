module
public import GeneralEvaluation

@[expose] public section
namespace GeneralTranslationAudit
noncomputable section
open WeierstrassCurve.Affine
open EllipticTranslationAudit LocalTranslationAudit
open MillerNormValuationAudit MillerFinitePointAudit MillerBasechangeAudit
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit

/-- The credited source example formula, now over its actual arbitrary coefficient field. -/
theorem general_source_example_formula (K : Type*) [Field K] [DecidableEq K]
    (hΔ : (translationExampleCurve K).Δ≠0) :
    generalTranslation (translationExampleCurve K) hΔ (translationExampleQ K)
      (xFunction (translationExampleCurve K))=(xFunction (translationExampleCurve K))⁻¹ ∧
    generalTranslation (translationExampleCurve K) hΔ (translationExampleQ K)
      (yFunction (translationExampleCurve K))=
      -yFunction (translationExampleCurve K)/(xFunction (translationExampleCurve K))^2 := by
  have hxy : ¬ (xFunction (translationExampleCurve K)=0 ∧
      yFunction (translationExampleCurve K)=(genericCurve (translationExampleCurve K)).negY 0 0) :=
    fun h => x_function_nonzero (translationExampleCurve K) h.1
  constructor
  · rw [general_translation_x,translatedX,translatedGeneric,EllipticTranslationAudit.genericPoint,
      example_constant_point,generic_add_some (translationExampleCurve K)
        (generic_nonsingular _ hΔ) (example_generic_nonsingular_zero K) hxy]
    exact example_add_x_formula K
  · rw [general_translation_y,translatedY,translatedGeneric,EllipticTranslationAudit.genericPoint,
      example_constant_point,generic_add_some (translationExampleCurve K)
        (generic_nonsingular _ hΔ) (example_generic_nonsingular_zero K) hxy]
    exact example_add_y_formula K

local instance : Fact (Nat.Prime 2) := ⟨by decide⟩
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
abbrev GeneralF2 := ZMod 2
abbrev GeneralF3 := ZMod 3
abbrev GeneralF5 := ZMod 5
noncomputable abbrev generalF2CommRing : CommRing GeneralF2 := (inferInstance : Field GeneralF2).toCommRing
noncomputable abbrev generalF3CommRing : CommRing GeneralF3 := (inferInstance : Field GeneralF3).toCommRing
noncomputable abbrev generalF5CommRing : CommRing GeneralF5 := (inferInstance : Field GeneralF5).toCommRing
attribute [local instance] generalF2CommRing generalF3CommRing generalF5CommRing

/-- Actual nonsingular binary model, rather than a short-form characteristic-two substitution. -/
def binaryCurve : WeierstrassCurve.Affine GeneralF2 := ⟨0,0,1,0,0⟩
theorem binary_delta : binaryCurve.Δ≠0 := by decide +kernel
theorem binary_nonsingular : binaryCurve.Nonsingular 0 0 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
  decide +kernel
def binaryPoint : binaryCurve.Point := .some 0 0 binary_nonsingular
theorem binary_point_nonzero : binaryPoint≠0 := Point.some_ne_zero binary_nonsingular
theorem binary_translation_nontrivial :
    generalTranslation binaryCurve binary_delta binaryPoint≠RingHom.id binaryCurve.FunctionField := by
  intro he
  exact binary_point_nonzero (general_translation_faithful binaryCurve binary_delta
    (he.trans (general_translation_zero binaryCurve binary_delta).symm))

/-- The credited actual F3 curve, now used with the unrestricted construction. -/
abbrev ternaryCurve : WeierstrassCurve.Affine GeneralF3 := rationalCurve
theorem ternary_delta : ternaryCurve.Δ≠0 := by decide +kernel
theorem ternary_nonsingular : ternaryCurve.Nonsingular 0 1 := by
  rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
  decide +kernel
def ternaryPoint : ternaryCurve.Point := .some 0 1 ternary_nonsingular
theorem ternary_point_nonzero : ternaryPoint≠0 := Point.some_ne_zero ternary_nonsingular
theorem ternary_translation_nontrivial :
    generalTranslation ternaryCurve ternary_delta ternaryPoint≠RingHom.id ternaryCurve.FunctionField := by
  intro he
  exact ternary_point_nonzero (general_translation_faithful ternaryCurve ternary_delta
    (he.trans (general_translation_zero ternaryCurve ternary_delta).symm))

theorem finite_example_delta : (translationExampleCurve GeneralF5).Δ≠0 := by decide +kernel

/-- Concrete F5 function-field translation is valid but does not preserve the affine coordinate ring. -/
theorem finite_example_not_affine :
    ¬ ∃ a : (translationExampleCurve GeneralF5).CoordinateRing,
      generalTranslation (translationExampleCurve GeneralF5) finite_example_delta
        (translationExampleQ GeneralF5) (xFunction (translationExampleCurve GeneralF5))=
      algebraMap (translationExampleCurve GeneralF5).CoordinateRing
        (translationExampleCurve GeneralF5).FunctionField a := by
  rw [(general_source_example_formula GeneralF5 finite_example_delta).1]
  exact reciprocal_x_not_coordinate_image (translationExampleCurve GeneralF5)

/-- Known rational-point-only degree defect persists under every actual F3 translation.
This prevents conflating all-field point transport with geometric degree-zero principal divisors. -/
theorem rational_bad_degree_survives (Q : rationalCurve.Point) :
    divisorDegree rationalCurve
      (functionDivisor rationalCurve (generalTranslation rationalCurve rational_delta Q numeratorFunction)
        (general_translation_nonzero rationalCurve rational_delta Q numerator_function_nonzero))=(-4 : ℤ) := by
  rw [general_translation_function_divisor,divisor_translation_degree]
  exact rational_only_degree_counter

theorem actual_binary_characteristic : CharP GeneralF2 2 := inferInstance
theorem actual_ternary_characteristic : CharP GeneralF3 3 := inferInstance
theorem actual_five_characteristic : CharP GeneralF5 5 := inferInstance

#print axioms binary_translation_nontrivial
#print axioms ternary_translation_nontrivial
#print axioms finite_example_not_affine
#print axioms rational_bad_degree_survives
end
end GeneralTranslationAudit
