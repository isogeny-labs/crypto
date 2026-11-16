module
public import PointResidue

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open MillerFinitePointAudit
open scoped WithZero
variable {K : Type*} [Field K]

local instance pointMonoid (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : Monoid (PointLocalRing W x y h) :=
  (inferInstance : CommRing (PointLocalRing W x y h)).toMonoid

local instance pointFractionRing (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Equation x y) : IsFractionRing (PointLocalRing W x y h) W.FunctionField :=
  pointFunctionFractionRing W h

theorem finite_local_integral (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : PointLocalRing W x y h.1) :
    finiteAdic W h (pointToField W h.1 a) ≤ 1 := by
  let := pointLocalDVR W h
  change (IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1)).valuation
    W.FunctionField (algebraMap (PointLocalRing W x y h.1) W.FunctionField a) ≤ 1
  exact (IsDiscreteValuationRing.maximalIdeal (PointLocalRing W x y h.1)).valuation_le_one a

def localToFiniteRing (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    PointLocalRing W x y h.1 →+* (finiteAdic W h).valuationSubring where
  toFun a := ⟨pointToField W h.1 a, finite_local_integral W h a⟩
  map_zero' := Subtype.ext (map_zero _)
  map_one' := Subtype.ext (map_one _)
  map_add' a b := Subtype.ext (map_add _ a b)
  map_mul' a b := Subtype.ext (map_mul _ a b)

theorem local_to_finite_injective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : Function.Injective (localToFiniteRing W h) := by
  intro a b he
  exact point_to_field_injective W h.1 (congrArg Subtype.val he)

/-- Application of the existing official DVR integer-lift theorem to the actual point ring. -/
theorem local_to_finite_surjective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : Function.Surjective (localToFiniteRing W h) := by
  let := pointLocalDVR W h
  intro f
  obtain ⟨a, ha⟩ := IsDiscreteValuationRing.exists_lift_of_le_one
    (A := PointLocalRing W x y h.1) (K := W.FunctionField) f.property
  exact ⟨a, Subtype.ext ha⟩

def finiteLocalEquiv (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    PointLocalRing W x y h.1 ≃+* (finiteAdic W h).valuationSubring :=
  RingEquiv.ofBijective (localToFiniteRing W h)
    ⟨local_to_finite_injective W h, local_to_finite_surjective W h⟩

theorem finite_local_equiv_coe (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : PointLocalRing W x y h.1) :
    (finiteLocalEquiv W h a : W.FunctionField) = pointToField W h.1 a := rfl

def finiteRegularEval (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : (finiteAdic W h).valuationSubring →+* K :=
  (localPointEval W h.1).comp (finiteLocalEquiv W h).symm.toRingHom

theorem finite_regular_eval_local (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : PointLocalRing W x y h.1) :
    finiteRegularEval W h (finiteLocalEquiv W h a) = localPointEval W h.1 a := by
  change localPointEval W h.1 ((finiteLocalEquiv W h).symm (finiteLocalEquiv W h a)) = _
  rw [RingEquiv.symm_apply_apply]

theorem finite_regular_surjective (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) : Function.Surjective (finiteRegularEval W h) :=
  (local_point_surjective W h.1).comp (finiteLocalEquiv W h).symm.surjective

theorem finite_regular_kernel (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) :
    RingHom.ker (finiteRegularEval W h) =
      IsLocalRing.maximalIdeal ((finiteAdic W h).valuationSubring) :=
  IsLocalRing.eq_maximalIdeal
    (RingHom.ker_isMaximal_of_surjective _ (finite_regular_surjective W h))

theorem finite_regular_value_iff (W : WeierstrassCurve.Affine K) {x y : K}
    (h : W.Nonsingular x y) (a : (finiteAdic W h).valuationSubring) :
    finiteRegularEval W h a ≠ 0 ↔ finiteAdic W h a = 1 := by
  have hz : finiteRegularEval W h a = 0 ↔
      a ∈ IsLocalRing.maximalIdeal ((finiteAdic W h).valuationSubring) := by
    change a ∈ RingHom.ker (finiteRegularEval W h) ↔ _
    rw [finite_regular_kernel]
  rw [ne_eq, hz, Valuation.mem_maximalIdeal_iff]
  constructor
  · intro hn
    exact le_antisymm a.property (le_of_not_gt hn)
  · intro he
    simp only [he, lt_self_iff_false, not_false_eq_true]

#print axioms local_to_finite_surjective
#print axioms finite_regular_value_iff
end
end FinitePointEvaluationAudit
