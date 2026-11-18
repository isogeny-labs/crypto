module
public import PicardControls

@[expose] public section
namespace EllipticPicardBridgeAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerFinitePointAudit FunctionDivisorFinitenessAudit GeometricPrincipalDivisorAudit
local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : CommRing F3 := (inferInstance : Field F3).toCommRing

/-- Actual elliptic Abel-Jacobi and principal-divisor criterion, with a concrete domain counter. -/
theorem actual_elliptic_picard_certificate :
    (∀ (K : Type) [Field K] [IsAlgClosed K] [DecidableEq K]
      (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0),
      Function.Bijective (pointClass W hΔ) ∧
      (∀ P : W.Point, ellipticPicardEquiv W hΔ P=divisorClass W hΔ (pointDivisor W P)) ∧
      (∀ D : W.Point →₀ ℤ, D ∈ principalPointDivisors W ↔
        divisorDegree W D=0 ∧ divisorPointSum W D=0) ∧
      (∀ P Q : W.Point, (∃ (f : W.FunctionField) (hf : f ≠ 0),
        functionDivisor W f hf=Finsupp.single P 1-Finsupp.single Q 1) ↔ P=Q) ∧
      (∀ (f : W.FunctionField) (_ : f ≠ 0),
        FiniteRegular W f ↔ ∃ a : W.CoordinateRing,
          f=algebraMap W.CoordinateRing W.FunctionField a) ∧
      (∀ (f : W.FunctionField) (_ : f ≠ 0),
        FiniteRegular W f → pointOrder W f 0 ≠ -1)) ∧
    rationalCurve.Δ ≠ 0 ∧
    divisorDegree rationalCurve (principalDivisorHom rationalCurve rationalBadUnit) = -4 ∧
    (¬ ∃ h : Additive rationalCurve.FunctionFieldˣ → ZeroDivisors rationalCurve,
      ∀ u, (h u).val=principalDivisorHom rationalCurve u) ∧
    geometricControlCurve.Δ ≠ 0 ∧
    Function.Bijective geometricControlMap ∧
    (∃ P, geometricControlMap P≠0) := by
  refine ⟨?_,rational_delta,rational_principal_degree_negative_four,
    no_rational_principal_restriction,geometric_control_delta,
    geometric_control_bijective,geometric_control_nonzero_class⟩
  intro K _ _ _ W hΔ
  exact ⟨point_class_bijective W hΔ,elliptic_picard_equiv_apply W hΔ,
    principal_iff_degree_and_point_sum W hΔ,point_difference_principal_iff W hΔ,
    fun _ hf => finite_regular_iff_coordinate W hΔ hf,
    fun _ hf hr => no_simple_infinity_pole W hΔ hf hr⟩

#print axioms actual_elliptic_picard_certificate
end
end EllipticPicardBridgeAudit
