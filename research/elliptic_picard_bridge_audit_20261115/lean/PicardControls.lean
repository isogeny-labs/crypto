module
public import PicardCriterion

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open WeierstrassCurve.Affine
open FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

/-- The credited actual F3 polynomial control, as an actual function-field unit. -/
def rationalBadUnit : Additive rationalCurve.FunctionFieldˣ :=
  Additive.ofMul (Units.mk0 numeratorFunction numerator_function_nonzero)

theorem rational_principal_degree_negative_four :
    divisorDegree rationalCurve (principalDivisorHom rationalCurve rationalBadUnit) = -4 :=
  rational_only_degree_counter

/-- Dropping geometric scope prevents the actual principal map from landing in degree zero.
The old actual F3 control is credited; this is not a new curve population. -/
theorem no_rational_principal_restriction :
    ¬ ∃ h : Additive rationalCurve.FunctionFieldˣ → ZeroDivisors rationalCurve,
      ∀ u, (h u).val=principalDivisorHom rationalCurve u := by
  rintro ⟨h,he⟩
  have hz := (h rationalBadUnit).property
  change divisorDegree rationalCurve (h rationalBadUnit).val=0 at hz
  rw [he,rational_principal_degree_negative_four] at hz
  norm_num at hz

/-- Actual positive map on the credited nonsingular curve over the actual algebraic closure of F3. -/
def geometricControlMap : geometricControlCurve.Point →
    PicardZero geometricControlCurve geometric_control_delta := by
  classical
  exact fun P => pointClass geometricControlCurve geometric_control_delta P

theorem geometric_control_bijective : Function.Bijective geometricControlMap := by
  classical
  exact point_class_bijective geometricControlCurve geometric_control_delta

theorem geometric_control_nonzero_class : ∃ P, geometricControlMap P≠0 := by
  classical
  exact actual_picard_nontrivial geometricControlCurve geometric_control_delta

#print axioms no_rational_principal_restriction
#print axioms geometric_control_bijective
end
end EllipticPicardBridgeAudit
