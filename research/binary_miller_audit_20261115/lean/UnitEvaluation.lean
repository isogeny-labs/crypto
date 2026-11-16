module
public import EvaluationConstants

@[expose] public section
namespace FinitePointEvaluationAudit
noncomputable section
open MillerFinitePointAudit MillerNormValuationAudit
variable {K : Type*} [Field K]

/-- Reuse the official equivalence of field-unit and regular-ring-unit subgroups. -/
def pointUnitEvaluation (W : WeierstrassCurve.Affine K) (P : W.Point) :
    (RegularRing W P).unitGroup →* Kˣ :=
  (Units.map (regularEvaluation W P).toMonoidHom).comp
    (RegularRing W P).unitGroupMulEquiv.toMonoidHom

theorem mem_point_units_iff (W : WeierstrassCurve.Affine K) (P : W.Point)
    (f : W.FunctionFieldˣ) : f ∈ (RegularRing W P).unitGroup ↔ pointOrder W f P = 0 := by
  rw [Valuation.mem_unitGroup_iff]
  exact (point_order_zero_iff_value_one W f f.ne_zero P).symm

/-- Actual field units regular and nonvanishing at every specified support point. -/
def supportUnits (W : WeierstrassCurve.Affine K) (T : Finset W.Point) :
    Subgroup W.FunctionFieldˣ := ⨅ (P : W.Point) (_ : P ∈ T), (RegularRing W P).unitGroup

theorem mem_support_units_iff (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (f : W.FunctionFieldˣ) : f ∈ supportUnits W T ↔ ∀ P ∈ T, pointOrder W f P = 0 := by
  simp only [supportUnits, Subgroup.mem_iInf, mem_point_units_iff]

def supportPointMap (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (P : W.Point) (hP : P ∈ T) : supportUnits W T →* (RegularRing W P).unitGroup where
  toFun f := ⟨f.val, (mem_point_units_iff W P f.val).mpr
    ((mem_support_units_iff W T f.val).mp f.property P hP)⟩
  map_one' := rfl
  map_mul' _ _ := rfl

/-- Outside T the helper is 1; divisor evaluation requires its support to be contained in T. -/
def supportPointEvaluation (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (P : W.Point) : supportUnits W T →* Kˣ := by
  classical
  exact if hP : P ∈ T then (pointUnitEvaluation W P).comp (supportPointMap W T P hP) else 1

def supportConstantUnits (W : WeierstrassCurve.Affine K) (T : Finset W.Point) :
    Kˣ →* supportUnits W T where
  toFun c := ⟨Units.map (constantHom W).toMonoidHom c, by
    apply (mem_support_units_iff W T _).mpr
    intro P _
    apply (point_order_zero_iff_value_one W _ (Units.ne_zero _) P).mpr
    exact local_constant_value W P c.ne_zero⟩
  map_one' := Subtype.ext (map_one _)
  map_mul' a b := Subtype.ext (map_mul _ a b)

theorem support_point_constant (W : WeierstrassCurve.Affine K) (T : Finset W.Point)
    (P : W.Point) (hP : P ∈ T) (c : Kˣ) :
    supportPointEvaluation W T P (supportConstantUnits W T c) = c := by
  classical
  simp only [supportPointEvaluation, dite_eq_left hP]
  apply Units.ext
  change regularEvaluation W P (regularConstant W P (c : K)) = (c : K)
  exact regular_eval_constant W P c

#print axioms mem_support_units_iff
#print axioms support_point_constant
end
end FinitePointEvaluationAudit
