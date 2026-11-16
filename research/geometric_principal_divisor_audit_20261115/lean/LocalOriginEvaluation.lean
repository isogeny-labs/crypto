module
public import DVRIdentification
public import Mathlib.RingTheory.Ideal.Quotient.Operations

@[expose] public section
namespace MillerInfinityChartAudit
noncomputable section
variable {K : Type*} [Field K]

theorem origin_denominator_nonzero (W : WeierstrassCurve.Affine K)
    (s : (originIdeal W).primeCompl) : IsUnit (originEval W s) := by
  apply isUnit_iff_ne_zero.mpr
  exact s.property

def localOriginEval (W : WeierstrassCurve.Affine K) : ChartLocalRing W →+* K :=
  IsLocalization.lift (origin_denominator_nonzero W)

theorem local_origin_map (W : WeierstrassCurve.Affine K) (b : ChartRing W) :
    localOriginEval W (algebraMap (ChartRing W) (ChartLocalRing W) b) = originEval W b :=
  IsLocalization.lift_eq _ b

theorem local_origin_constant (W : WeierstrassCurve.Affine K) (c : K) :
    localOriginEval W (algebraMap (ChartRing W) (ChartLocalRing W)
      (algebraMap K (ChartRing W) c)) = c := by
  rw [local_origin_map, origin_eval_constant]

theorem local_origin_surjective (W : WeierstrassCurve.Affine K) :
    Function.Surjective (localOriginEval W) :=
  fun c => ⟨algebraMap (ChartRing W) (ChartLocalRing W) (algebraMap K (ChartRing W) c),
    local_origin_constant W c⟩

theorem local_origin_kernel (W : WeierstrassCurve.Affine K) :
    RingHom.ker (localOriginEval W) = IsLocalRing.maximalIdeal (ChartLocalRing W) :=
  IsLocalRing.eq_maximalIdeal (RingHom.ker_isMaximal_of_surjective _ (local_origin_surjective W))

/-- The actual local residue field is the original coefficient field, with an explicit comparison. -/
def chartResidueEquiv (W : WeierstrassCurve.Affine K) :
    IsLocalRing.ResidueField (ChartLocalRing W) ≃+* K :=
  (Ideal.quotEquivOfEq (local_origin_kernel W).symm).trans
    (RingHom.quotientKerEquivOfSurjective (local_origin_surjective W))

theorem chart_residue_eval (W : WeierstrassCurve.Affine K) (a : ChartLocalRing W) :
    chartResidueEquiv W (IsLocalRing.residue (ChartLocalRing W) a) = localOriginEval W a := by
  rfl

#print axioms chart_residue_eval
end
end MillerInfinityChartAudit
