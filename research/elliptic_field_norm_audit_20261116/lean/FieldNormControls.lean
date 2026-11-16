import FieldNormGalois
import FieldNormWrongBase
import GeneralControls

namespace EllipticFieldNormAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine
open EllipticTranslationAudit MillerNormValuationAudit FunctionDivisorFinitenessAudit
open NegationPullbackAudit GeneralTranslationAudit WeilAuxiliaryRatioAudit
open scoped EllipticFieldNormAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩

/-- Actual singular binary cusp, not an elliptic curve. -/
def cuspCurve : WeierstrassCurve.Affine (ZMod 2) := ⟨0,0,0,0,0⟩

theorem cusp_delta_zero : cuspCurve.Δ=0 := by decide +kernel

theorem actual_cusp_not_separable : ¬Algebra.IsSeparable (RatFunc (ZMod 2)) cuspCurve.FunctionField := by
  rw [actual_field_separable_iff]
  push Not
  exact ⟨by decide +kernel,rfl,rfl⟩

theorem actual_cusp_not_galois : ¬IsGalois (RatFunc (ZMod 2)) cuspCurve.FunctionField := by
  rw [actual_galois_iff]
  push Not
  exact ⟨by decide +kernel,rfl,rfl⟩

private instance : CharP cuspCurve.FunctionField 2 := charP_of_injective_algebraMap' (ZMod 2) 2

/-- Actual characteristic2 cusp conjugation is identity, despite actual degree2/normality. -/
theorem actual_cusp_conjugation : (fieldNeg cuspCurve).toRingHom=RingHom.id cuspCurve.FunctionField := by
  apply function_hom_ext cuspCurve
  · intro c
    exact negation_constant cuspCurve c
  · exact field_neg_x cuspCurve
  · change fieldNeg cuspCurve (yFunction cuspCurve)=yFunction cuspCurve
    have hh : linearH cuspCurve=0 := by simp [linearH,cuspCurve]
    rw [field_neg_y,hh,map_zero,sub_zero]
    exact CharTwo.neg_eq (yFunction cuspCurve)

theorem actual_cusp_norm_square (f : cuspCurve.FunctionField) :
    algebraMap (RatFunc (ZMod 2)) cuspCurve.FunctionField (Algebra.norm (RatFunc (ZMod 2)) f)=f^2 := by
  rw [actual_field_norm_product]
  have he := RingHom.congr_fun actual_cusp_conjugation f
  change fieldNeg cuspCurve f=f at he
  rw [he,pow_two]

/-- Nonsingular binary model: separable/Galois degree2 and actual field-norm formula. -/
theorem actual_binary_norm_positive :
    GeneralTranslationAudit.binaryCurve.Δ≠0 ∧
      Module.finrank (RatFunc (ZMod 2)) GeneralTranslationAudit.binaryCurve.FunctionField=2 ∧
      IsGalois (RatFunc (ZMod 2)) GeneralTranslationAudit.binaryCurve.FunctionField :=
  ⟨GeneralTranslationAudit.binary_delta,function_field_degree_two _,
    nonsingular_function_field_galois _ GeneralTranslationAudit.binary_delta⟩

theorem actual_ternary_norm_positive :
    GeneralTranslationAudit.ternaryCurve.Δ≠0 ∧
      Module.finrank (RatFunc (ZMod 3)) GeneralTranslationAudit.ternaryCurve.FunctionField=2 ∧
      IsGalois (RatFunc (ZMod 3)) GeneralTranslationAudit.ternaryCurve.FunctionField :=
  ⟨GeneralTranslationAudit.ternary_delta,function_field_degree_two _,
    nonsingular_function_field_galois _ GeneralTranslationAudit.ternary_delta⟩

/-- Actual degree2/normality/norm identity survive; the deleted separability statement fails. -/
theorem actual_singular_binary_counter :
    cuspCurve.Δ=0 ∧ Module.finrank (RatFunc (ZMod 2)) cuspCurve.FunctionField=2 ∧
      Normal (RatFunc (ZMod 2)) cuspCurve.FunctionField ∧
      ¬Algebra.IsSeparable (RatFunc (ZMod 2)) cuspCurve.FunctionField ∧
      ¬IsGalois (RatFunc (ZMod 2)) cuspCurve.FunctionField ∧
      (fieldNeg cuspCurve).toRingHom=RingHom.id cuspCurve.FunctionField ∧
      ∀ f : cuspCurve.FunctionField,
        algebraMap (RatFunc (ZMod 2)) cuspCurve.FunctionField (Algebra.norm (RatFunc (ZMod 2)) f)=f^2 :=
  ⟨cusp_delta_zero,function_field_degree_two _,actual_quadratic_normal _,
    actual_cusp_not_separable,actual_cusp_not_galois,actual_cusp_conjugation,actual_cusp_norm_square⟩

#print axioms actual_binary_norm_positive
#print axioms actual_ternary_norm_positive
#print axioms actual_singular_binary_counter
end
end EllipticFieldNormAudit
