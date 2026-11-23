module
public import BasechangeCertificate

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerFinitePointAudit MillerPrincipalIdealAudit
variable {K : Type*} [Field K]

/-- Finite-point adaptation of the credited infinity-chart localization lift. -/
theorem point_residue_denominator (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (s : (evaluationIdeal W x y h).primeCompl) :
    IsUnit (pointEval W x y h s) := by
  apply isUnit_iff_ne_zero.mpr
  exact s.property

def localPointEval (W : WeierstrassCurve.Affine K) {x y : K} (h : W.Equation x y) :
    PointLocalRing W x y h →+* K :=
  IsLocalization.lift (point_residue_denominator W h)

theorem local_point_map (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : W.CoordinateRing) :
    localPointEval W h (algebraMap W.CoordinateRing (PointLocalRing W x y h) a) =
      pointEval W x y h a := IsLocalization.lift_eq _ a

theorem local_point_constant (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (c : K) :
    localPointEval W h (algebraMap W.CoordinateRing (PointLocalRing W x y h)
      (algebraMap K W.CoordinateRing c)) = c := by
  rw [local_point_map, point_eval_constant]

theorem local_point_surjective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Function.Surjective (localPointEval W h) :=
  fun c => ⟨algebraMap W.CoordinateRing (PointLocalRing W x y h)
    (algebraMap K W.CoordinateRing c), local_point_constant W h c⟩

theorem local_point_kernel (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) :
    RingHom.ker (localPointEval W h) = IsLocalRing.maximalIdeal (PointLocalRing W x y h) :=
  IsLocalRing.eq_maximalIdeal (RingHom.ker_isMaximal_of_surjective _ (local_point_surjective W h))

def pointResidueEquiv (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : IsLocalRing.ResidueField (PointLocalRing W x y h) ≃+* K :=
  (Ideal.quotEquivOfEq (local_point_kernel W h).symm).trans
    (RingHom.quotientKerEquivOfSurjective (local_point_surjective W h))

theorem point_residue_eval (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : PointLocalRing W x y h) :
    pointResidueEquiv W h (IsLocalRing.residue (PointLocalRing W x y h) a) =
      localPointEval W h a := rfl

theorem local_point_unit_iff (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) (a : PointLocalRing W x y h) :
    IsUnit a ↔ localPointEval W h a ≠ 0 := by
  have hz : localPointEval W h a = 0 ↔ ¬ IsUnit a := by
    change a ∈ RingHom.ker (localPointEval W h) ↔ ¬ IsUnit a
    rw [local_point_kernel, IsLocalRing.mem_maximalIdeal, mem_nonunits_iff]
  simp only [ne_eq, hz, not_not]

theorem local_point_u_zero (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : localPointEval W h (localU W h) = 0 := by
  rw [localU, local_point_map, point_eval_XClass, sub_self]

theorem local_point_v_zero (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : localPointEval W h (localV W h) = 0 := by
  rw [localV, local_point_map, point_eval_YClass, eval_C, sub_self]

#print axioms point_residue_eval
#print axioms local_point_unit_iff
end
end FinitePointEvaluationAudit
