import DoublingField

namespace EllipticDoublingDegreeAudit
noncomputable section
open GeneralTranslationAudit EllipticTranslationAudit MillerBasechangeAudit
open WeierstrassCurve.Affine EllipticFrobeniusDegreeAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] generalF2CommRing generalF3CommRing

theorem actual_binary_doubling_degree :
    Module.finrank (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField=4 :=
  actual_doubling_degree _ _

theorem actual_ternary_doubling_degree :
    Module.finrank (doublingImage ternaryCurve ternary_delta) ternaryCurve.FunctionField=4 :=
  actual_doubling_degree _ _

/-- Actual geometric point-set kernel of actual point doubling. -/
def doublingGeometricKernelCard {K : Type*} [Field K] (W : WeierstrassCurve.Affine K) : Nat :=
  Nat.card {P : (extendedCurve (L:=AlgebraicClosure K) W).Point // P+P=0}

theorem actual_binary_double_kernel (L : Type*) [Field L] [Algebra GeneralF2 L] [DecidableEq L]
    (P : (extendedCurve (L:=L) binaryCurve).Point) : P+P=0 ↔ P=0 := by
  have ht : (2 : L)=0 := by
    have hz : (2 : GeneralF2)=0 := by decide +kernel
    simpa only [map_ofNat,map_zero] using congrArg (algebraMap GeneralF2 L) hz
  cases P with
  | zero => exact ⟨fun _ => rfl,fun _ => zero_add _⟩
  | some x y h =>
    have hy : y≠(extendedCurve (L:=L) binaryCurve).negY x y := by
      have hy' : y≠ -y-1 := by
        intro he
        have he' : y+y= -1 := by linear_combination he
        rw [←two_mul,ht,zero_mul] at he'
        exact one_ne_zero (neg_eq_zero.mp he'.symm)
      simpa only [negY,extendedCurve,WeierstrassCurve.map,binaryCurve,map_zero,map_one,
        zero_mul,sub_zero] using hy'
    constructor
    · intro he
      rw [Point.add_self_of_Y_ne hy] at he
      exact (Point.some_ne_zero _ he).elim
    · intro he
      exact (Point.some_ne_zero _ he).elim

theorem actual_binary_kernel_card : doublingGeometricKernelCard binaryCurve=1 := by
  apply Nat.card_eq_one_iff_exists.mpr
  refine ⟨⟨0,by simp⟩,?_⟩
  intro P
  exact Subtype.ext ((actual_binary_double_kernel _ P.val).mp P.property)

theorem actual_binary_degree_not_kernel :
    Module.finrank (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField≠
      doublingGeometricKernelCard binaryCurve := by
  rw [actual_binary_doubling_degree,actual_binary_kernel_card]
  decide

def UnguardedDoublingKernelDegreeReading : Prop :=
  ∀ (K : Type) [Field K] (W : WeierstrassCurve.Affine K) (hΔ : W.Δ≠0),
    Module.finrank (doublingImage W hΔ) W.FunctionField=doublingGeometricKernelCard W

theorem wrong_doubling_kernel_reading_false : ¬ UnguardedDoublingKernelDegreeReading := by
  intro h
  exact actual_binary_degree_not_kernel (h GeneralF2 binaryCurve binary_delta)

def invalidBinaryShort : WeierstrassCurve.Affine GeneralF2 := ⟨0,0,0,1,0⟩

theorem actual_short_char_two_guard :
    invalidBinaryShort.Δ=0 ∧ invalidBinaryShort.Equation 1 0 ∧
      ¬ invalidBinaryShort.Nonsingular 1 0 := by
  refine ⟨by decide +kernel,?_,?_⟩
  · rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel
  · rw [WeierstrassCurve.Affine.nonsingular_iff',WeierstrassCurve.Affine.equation_iff]
    decide +kernel

theorem coefficient_point_map_is_not_doubling :
    ¬ ∀ P : (genericCurve binaryCurve).Point,
      coefficientPointMap binaryCurve (doublingAlgHom binaryCurve binary_delta) P=P+P := by
  intro h
  let P := constantPoint binaryCurve binaryPoint
  have hp : P≠0 := by
    intro he
    apply binary_point_nonzero
    exact (point_base_injective binaryCurve) (he.trans (map_zero _).symm)
  have hfix : coefficientPointMap binaryCurve (doublingAlgHom binaryCurve binary_delta) P=P :=
    Point.map_baseChange (W':=binaryCurve) (doublingAlgHom binaryCurve binary_delta) binaryPoint
  have he : P+P=P+0 := by rw [←h P,hfix,add_zero]
  exact hp (add_left_cancel he)

def ActualDoublingControls : Prop :=
  (Module.finrank (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField=4) ∧
  (Module.finrank (doublingImage ternaryCurve ternary_delta) ternaryCurve.FunctionField=4) ∧
  doublingGeometricKernelCard binaryCurve=1 ∧
  ¬ UnguardedDoublingKernelDegreeReading ∧
  invalidBinaryShort.Δ=0 ∧
  (¬ ∀ P : (genericCurve binaryCurve).Point,
    coefficientPointMap binaryCurve (doublingAlgHom binaryCurve binary_delta) P=P+P)

theorem actual_doubling_controls : ActualDoublingControls :=
  ⟨actual_binary_doubling_degree,actual_ternary_doubling_degree,actual_binary_kernel_card,
    wrong_doubling_kernel_reading_false,actual_short_char_two_guard.1,
    coefficient_point_map_is_not_doubling⟩

#print axioms actual_binary_kernel_card
#print axioms coefficient_point_map_is_not_doubling
#print axioms actual_binary_doubling_degree
#print axioms actual_ternary_doubling_degree
end
end EllipticDoublingDegreeAudit
