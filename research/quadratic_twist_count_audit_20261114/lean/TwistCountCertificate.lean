import LiteralTwistControl

namespace QuadraticTwistCountAudit
noncomputable section
open SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit

/-- Actual point-count/trace theorem, a literal primary-exercise coordinate
counterexample and exact hypothesis-deletion controls. -/
theorem twist_count_trace_certificate :
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K],ringChar K≠2 →
      ∀ a b c d : K,(cubicModel a b c).Δ≠0 → ¬IsSquare d →
      Nat.card (cubicModel a b c).Point+Nat.card (twistModel a b c d).Point=2*(Fintype.card K+1) ∧
      fieldTrace (twistModel a b c d)= -fieldTrace (cubicModel a b c) ∧
      (sourceSupersingular (twistModel a b c d) ↔ sourceSupersingular (cubicModel a b c))) ∧
    (sourceCurve.Δ≠0 ∧ scaledCurve.Δ≠0 ∧ sourceCurve.j=(1728:F7) ∧ ¬IsSquare (3:F7) ∧
      specialChange • sourceCurve=scaledCurve ∧
      Nonempty (sourceCurve.Point ≃+ scaledCurve.Point) ∧
      Function.LeftInverse backwardPoint forwardPoint ∧ Function.RightInverse backwardPoint forwardPoint ∧
      (∀ P Q : sourceCurve.Point,forwardPoint (P+Q)=forwardPoint P+forwardPoint Q) ∧
      Nat.card sourceCurve.Point=8 ∧ Nat.card scaledCurve.Point=8) ∧
    (∀ (R : Type) [CommRing R] [CharP R 7],∀ x y z : R,
      2*(4*x)=x ∧ 4*(2*x)=x ∧
      y^2*z-(4*x)^3-2*(4*x)*z^2=y^2*z-x^3-x*z^2 ∧
      y^2*z-(2*x)^3-(2*x)*z^2=y^2*z-x^3-2*x*z^2) ∧
    (¬∀ a b d : F7,(cubicModel 0 a b).Δ≠0 → d≠0 → ¬IsSquare d →
      ¬∃ C : WeierstrassCurve.VariableChange F7,C • cubicModel 0 a b=twistModel 0 a b d) ∧
    (¬∀ a b c d : ZMod 3,(cubicModel a b c).Δ≠0 → d≠0 →
      Nat.card (cubicModel a b c).Point+Nat.card (twistModel a b c d).Point=
        2*(Fintype.card (ZMod 3)+1)) ∧
    (¬∀ a b c d : ZMod 5,¬IsSquare d →
      Nat.card (cubicModel a b c).Point+Nat.card (twistModel a b c d).Point=
        2*(Fintype.card (ZMod 5)+1)) := by
  refine ⟨?_,?_,?_,literal_nonsquare_no_change_false,deleting_nonsquare_false,deleting_nonsingularity_false⟩
  · intro K _ _ _ hF a b c d hΔ hd
    exact ⟨actual_twist_count_sum hF a b c d hΔ hd,
      actual_twist_trace_neg hF a b c d hΔ hd,actual_twist_source_iff hF a b c d hΔ hd⟩
  · exact ⟨source_delta,scaled_delta,source_j,three_nonsquare,special_change,⟨specialAddEquiv⟩,
      forward_left_inverse,forward_right_inverse,forward_add,source_count,scaled_count⟩
  · intro R _ _ x y z
    exact universal_projective_identity x y z

#print axioms twist_count_trace_certificate
end
end QuadraticTwistCountAudit
