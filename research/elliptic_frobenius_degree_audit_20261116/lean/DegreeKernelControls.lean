import EllipticFrobeniusDegree

namespace EllipticFrobeniusDegreeAudit
noncomputable section
open IsogenyFactorizationGuardAudit GeneralTranslationAudit
variable {K : Type*} [Field K] [Fintype K]

theorem actual_degree_exceeds_one (W : WeierstrassCurve.Affine K) :
    1 < Module.finrank (frobeniusImage W) W.FunctionField := by
  rw [actual_elliptic_frobenius_degree]
  exact Fintype.one_lt_card

omit [Fintype K] in
theorem identity_field_degree (W : WeierstrassCurve.Affine K) :
    Module.finrank (AlgHom.id K W.FunctionField).fieldRange W.FunctionField = 1 := by
  have h : (AlgHom.id K W.FunctionField).fieldRange = ⊤ := by
    exact AlgHom.fieldRange_eq_top.mpr Function.surjective_id
  rw [h]
  exact IntermediateField.finrank_top

/-- Cardinality of the actual geometric point-set kernel, not its scheme length. -/
def geometricKernelCard (W : WeierstrassCurve.Affine K) : Nat :=
  Nat.card {P : (MillerBasechangeAudit.extendedCurve (L:=AlgebraicClosure K) W).Point //
    pointFrobenius W (AlgebraicClosure K) P = 0}

theorem actual_geometric_kernel_card (W : WeierstrassCurve.Affine K) :
    geometricKernelCard W = 1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨⟨0,map_zero _⟩,?_⟩
  intro P
  exact Subtype.ext ((point_frobenius_kernel W _ P.val).mp P.property)

theorem actual_degree_not_kernel_card (W : WeierstrassCurve.Affine K) :
    Module.finrank (frobeniusImage W) W.FunctionField ≠ geometricKernelCard W := by
  rw [actual_geometric_kernel_card]
  exact ne_of_gt (actual_degree_exceeds_one W)

def UnguardedFrobeniusKernelDegreeReading : Prop :=
  ∀ (A : Type) [Field A] [Fintype A] (W : WeierstrassCurve.Affine A), W.Δ ≠ 0 →
    Module.finrank (frobeniusImage W) W.FunctionField = geometricKernelCard W

private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] generalF2CommRing generalF3CommRing

theorem actual_binary_degree :
    binaryCurve.Δ ≠ 0 ∧ Module.finrank (frobeniusImage binaryCurve) binaryCurve.FunctionField = 2 := by
  exact ⟨binary_delta,actual_elliptic_frobenius_degree binaryCurve⟩

theorem actual_ternary_degree :
    ternaryCurve.Δ ≠ 0 ∧ Module.finrank (frobeniusImage ternaryCurve) ternaryCurve.FunctionField = 3 := by
  exact ⟨ternary_delta,actual_elliptic_frobenius_degree ternaryCurve⟩

def ActualConcreteDegreeControls : Prop :=
  (binaryCurve.Δ ≠ 0 ∧
    Module.finrank (frobeniusImage binaryCurve) binaryCurve.FunctionField = 2) ∧
  (ternaryCurve.Δ ≠ 0 ∧
    Module.finrank (frobeniusImage ternaryCurve) ternaryCurve.FunctionField = 3)

theorem actual_concrete_degree_controls : ActualConcreteDegreeControls :=
  ⟨actual_binary_degree,actual_ternary_degree⟩

theorem wrong_kernel_degree_reading_false : ¬ UnguardedFrobeniusKernelDegreeReading := by
  intro h
  exact actual_degree_not_kernel_card binaryCurve (h GeneralF2 binaryCurve binary_delta)

#print axioms actual_geometric_kernel_card
#print axioms wrong_kernel_degree_reading_false
#print axioms actual_binary_degree
#print axioms actual_ternary_degree
end
end EllipticFrobeniusDegreeAudit
