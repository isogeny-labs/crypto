import BinaryFourthPoints

namespace SemaevBinaryFourthCertificateAudit
noncomputable section
open SemaevBinaryFourthAudit SemaevBinaryFourthControlAudit SemaevBinaryFourthPointsAudit
  GeneralS4Audit
local instance : DecidableEq F := Classical.decEq _
private instance primeTwo : Fact (Nat.Prime 2) := ⟨by decide⟩

theorem all_F2_tuples_agree :
    ∀ x y z w : ZMod 2,correctedFourth x y z w=literalPrintedFourth x y z w :=
  by decide +kernel

theorem prime_field_control (a x y z w : ZMod 2) :
    generalS4 (binaryCurve a) x y z w=literalPrintedFourth x y z w := by
  rw [actual_resultant_corrected]
  exact all_F2_tuples_agree x y z w

theorem literal_printed_recursion_claim_false :
    ¬∀ a x y z w : F,
      literalPrintedFourth x y z w=generalS4 (binaryCurve a) x y z w := by
  intro h
  have hh := h 0 1 1 alpha (alpha^2)
  rw [literal_value_one,actual_resultant_zero] at hh
  exact one_ne_zero hh

theorem literal_printed_vanishing_claim_false :
    ¬∀ x y z w : F,fourPoints curve x y z w→literalPrintedFourth x y z w=0 := by
  intro h
  have hh := h 1 1 alpha (alpha^2) actual_point_witness
  rw [literal_value_one] at hh
  exact one_ne_zero hh

theorem binary_fourth_source_audit_certificate :
    (∀ (K : Type) [Field K] [CharP K 2] (a x y z w : K),
      generalS4 (binaryCurve a) x y z w=correctedFourth x y z w) ∧
    Nat.card F=4 ∧ curve.Δ≠0 ∧ alpha^2+alpha+1=0 ∧
    generalS4 curve 1 1 alpha (alpha^2)=0 ∧
    literalPrintedFourth (1:F) 1 alpha (alpha^2)=1 ∧
    fourPoints curve 1 1 alpha (alpha^2) ∧
    (∀ x y z w : ZMod 2,correctedFourth x y z w=literalPrintedFourth x y z w) ∧
    (¬∀ a x y z w : F,
      literalPrintedFourth x y z w=generalS4 (binaryCurve a) x y z w) ∧
    (¬∀ x y z w : F,fourPoints curve x y z w→literalPrintedFourth x y z w=0) := by
  exact ⟨fun K _ _ a x y z w => actual_resultant_corrected a x y z w,
    DiemSmallFieldAudit.field_cardinality,curve_nonsingular,
    DiemSmallFieldAudit.alpha_relation,actual_resultant_zero,literal_value_one,
    actual_point_witness,all_F2_tuples_agree,literal_printed_recursion_claim_false,
    literal_printed_vanishing_claim_false⟩

#print axioms all_F2_tuples_agree
#print axioms literal_printed_recursion_claim_false
#print axioms literal_printed_vanishing_claim_false
#print axioms binary_fourth_source_audit_certificate
end
end SemaevBinaryFourthCertificateAudit
