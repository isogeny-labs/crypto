import BinaryS3

/-! A domain negative control: the short-Weierstrass formula cannot be
substituted for the binary formula on an ordinary characteristic-two model.
This does not refute either formula on its stated curve model. -/
namespace BinaryS3Audit
open SemaevS3

def binaryCounterexampleCurve : WeierstrassCurve.Affine (ZMod 2) := ⟨1,0,0,0,1⟩

theorem binaryCounterexample_nonsingular : binaryCounterexampleCurve.Δ≠0 := by decide

theorem short_formula_binary_counterexample :
    ∃ P Q R : binaryCounterexampleCurve.Point,
      pointX P=some 1 ∧ pointX Q=some 1 ∧ pointX R=some 0 ∧ P+Q+R=0 ∧
        S3 binaryCounterexampleCurve.a₄ binaryCounterexampleCurve.a₆ (1:ZMod 2) 1 0≠0 ∧
        binaryS3 binaryCounterexampleCurve.a₆ (1:ZMod 2) 1 0=0 := by
  have h : binaryCounterexampleCurve.Nonsingular (1:ZMod 2) 0 := by
    rw [WeierstrassCurve.Affine.nonsingular_iff,WeierstrassCurve.Affine.equation_iff]
    decide
  let P : binaryCounterexampleCurve.Point := .some 1 0 h
  have hd : pointX (P+P)=some 0 := by
    exact (repeated_mathlib_iff binaryCounterexampleCurve rfl rfl rfl
      (by decide) h 0).mp (by decide)
  exact ⟨P,P,-(P+P),rfl,rfl,(TheoremAudit.pointX_neg _ _).trans hd,
    add_neg_cancel _,by decide,by decide⟩

#print axioms short_formula_binary_counterexample
end BinaryS3Audit
