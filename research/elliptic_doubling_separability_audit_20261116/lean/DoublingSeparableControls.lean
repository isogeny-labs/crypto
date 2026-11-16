import BinaryDoublingInseparable

namespace EllipticDoublingSeparabilityAudit
noncomputable section
open EllipticDoublingDegreeAudit GeneralTranslationAudit
private instance : Fact (Nat.Prime 2) := ⟨by decide⟩
private instance : Fact (Nat.Prime 3) := ⟨by decide⟩
attribute [local instance] generalF2CommRing generalF3CommRing

theorem actual_ternary_doubling_separable :
    Algebra.IsSeparable (doublingImage ternaryCurve ternary_delta) ternaryCurve.FunctionField :=
  actual_odd_doubling_separable _ _ (by decide +kernel)

def rationalOddCurve : WeierstrassCurve.Affine ℚ := ⟨0,0,0,1,0⟩

theorem rational_odd_delta : rationalOddCurve.Δ≠0 := by
  norm_num [rationalOddCurve,WeierstrassCurve.Δ,WeierstrassCurve.b₂,
    WeierstrassCurve.b₄,WeierstrassCurve.b₆,WeierstrassCurve.b₈]

theorem actual_rational_doubling_separable :
    Algebra.IsSeparable (doublingImage rationalOddCurve rational_odd_delta) rationalOddCurve.FunctionField :=
  actual_odd_doubling_separable _ _ (by norm_num)

def ActualDoublingSeparabilityControls : Prop :=
  Algebra.IsSeparable (doublingImage ternaryCurve ternary_delta) ternaryCurve.FunctionField ∧
  Algebra.IsSeparable (doublingImage rationalOddCurve rational_odd_delta) rationalOddCurve.FunctionField ∧
  Module.finrank (doublingImage binaryCurve binary_delta) binaryCurve.FunctionField=4 ∧
  doublingGeometricKernelCard binaryCurve=1

theorem actual_doubling_separability_controls : ActualDoublingSeparabilityControls :=
  ⟨actual_ternary_doubling_separable,actual_rational_doubling_separable,
    actual_binary_doubling_degree,actual_binary_kernel_card⟩

#print axioms actual_ternary_doubling_separable
#print axioms actual_rational_doubling_separable
end
end EllipticDoublingSeparabilityAudit
