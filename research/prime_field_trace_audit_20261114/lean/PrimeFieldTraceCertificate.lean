import FiveSeven

namespace PrimeFieldTraceAudit
noncomputable section
open SmallPrimeTraceGeometryAudit SupersingularTraceClassesAudit
private instance : Fact (Nat.Prime 5) := ⟨by decide⟩
private instance : Fact (Nat.Prime 7) := ⟨by decide⟩

/-- Same F5 model as the earlier rational S3/r=2 controls; credited reuse. -/
def sixControl : WeierstrassCurve.Affine F5 := shortModel 0 1
def eightControl : WeierstrassCurve.Affine F7 := shortModel 1 0

theorem six_delta : sixControl.Δ≠0 := by decide +kernel
theorem eight_delta : eightControl.Δ≠0 := by decide +kernel

def sixPoint : sixControl.Point := .some 2 2
  ((sixControl.equation_iff_nonsingular_of_Δ_ne_zero six_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))
def eightPoint : eightControl.Point := .some 3 3
  ((eightControl.equation_iff_nonsingular_of_Δ_ne_zero eight_delta).mp (by
    rw [WeierstrassCurve.Affine.equation_iff]
    decide +kernel))

theorem six_point_order : addOrderOf sixPoint=6 := by
  apply (addOrderOf_eq_iff (by decide : 0<(6:ℕ))).mpr
  refine ⟨by decide +kernel,?_⟩
  intro m hm hp
  have h : ∀ n : Fin 6,0<n.val → n.val • sixPoint≠0 := by decide +kernel
  exact h ⟨m,hm⟩ hp

theorem eight_point_order : addOrderOf eightPoint=8 := by
  apply (addOrderOf_eq_iff (by decide : 0<(8:ℕ))).mpr
  refine ⟨by decide +kernel,?_⟩
  intro m hm hp
  have h : ∀ n : Fin 8,0<n.val → n.val • eightPoint≠0 := by decide +kernel
  exact h ⟨m,hm⟩ hp

theorem six_actual_control :
    sixControl.Δ≠0 ∧ sourceSupersingular sixControl ∧ Nat.card sixControl.Point=6 ∧
    addOrderOf sixPoint=6 ∧ orderOf (5:ZMod 6)=2 ∧ orderOf (5:ZMod 2)=1 := by
  have hc : Nat.card sixControl.Point=6 := by
    rw [actual_point_count sixControl six_delta]
    decide +kernel
  refine ⟨six_delta,(five_all_models sixControl six_delta).2.mpr hc,hc,
    six_point_order,?_,by rw [show (5:ZMod 2)=1 by decide +kernel,orderOf_one]⟩
  exact composite_trace_zero_order 5 6 (by decide) (by decide)

theorem eight_actual_control :
    eightControl.Δ≠0 ∧ sourceSupersingular eightControl ∧ Nat.card eightControl.Point=8 ∧
    addOrderOf eightPoint=8 ∧ orderOf (7:ZMod 8)=2 ∧ orderOf (7:ZMod 2)=1 := by
  have hc : Nat.card eightControl.Point=8 := by
    rw [actual_point_count eightControl eight_delta]
    decide +kernel
  refine ⟨eight_delta,(seven_all_models eightControl eight_delta).2.mpr hc,hc,
    eight_point_order,?_,by rw [show (7:ZMod 2)=1 by decide +kernel,orderOf_one]⟩
  exact composite_trace_zero_order 7 8 (by decide) (by decide)

def singularControl : WeierstrassCurve.Affine F5 := shortModel 0 0

theorem singular_point_count : Nat.card singularControl.Point=5 := by
  rw [Nat.card_congr singularControl.nonsingularPointEquiv]
  change Nat.card (Option {xy : F5×F5 // singularControl.Nonsingular xy.1 xy.2})=5
  let : DecidablePred (fun xy : F5×F5 => singularControl.Nonsingular xy.1 xy.2) := fun xy =>
    decidable_of_iff (xy.2^2=xy.1^3 ∧ ((0:F5)≠3*xy.1^2 ∨ xy.2≠ -xy.2)) (by
      change (xy.2^2=xy.1^3 ∧ ((0:F5)≠3*xy.1^2 ∨ xy.2≠ -xy.2)) ↔
        singularControl.Nonsingular xy.1 xy.2
      rw [WeierstrassCurve.Affine.nonsingular_iff,WeierstrassCurve.Affine.equation_iff]
      simp [singularControl,shortModel])
  rw [Nat.card_eq_fintype_card]
  decide +kernel

theorem singular_negative_control :
    singularControl.Δ=0 ∧ Nat.card singularControl.Point=5 ∧
    equationCount singularControl=6 ∧
    ¬sourceSupersingular singularControl ∧ countedSupersingular singularControl := by
  refine ⟨by decide +kernel,singular_point_count,by decide +kernel,?_,by decide +kernel⟩
  simp only [sourceSupersingular,fieldTrace,ZMod.card,singular_point_count]
  decide +kernel

/-- Deleting nonsingularity from the equation-count bridge is false. -/
theorem deleting_nonsingularity_false :
    ¬∀ W : WeierstrassCurve.Affine F5,Nat.card W.Point=equationCount W := by
  intro h
  have hc := h singularControl
  rw [singular_negative_control.2.1,singular_negative_control.2.2.1] at hc
  contradiction

/-- Actual all-model prime-field statements, explicit cyclic composite-order
controls and a direct universal negation for the nonsingularity hypothesis. -/
theorem prime_field_trace_certificate :
    (∀ (K : Type) [Field K] [DecidableEq K] (W : WeierstrassCurve.Affine K),W.Δ≠0 →
      (normalized W).Δ=W.Δ ∧ Nonempty ((normalized W).Point ≃+ W.Point) ∧
      ((2:K)≠0 → (3:K)≠0 → normalized W=shortModel (normalized W).a₄ (normalized W).a₆)) ∧
    (∀ (K : Type) [Field K] [Fintype K] [DecidableEq K]
      (W : WeierstrassCurve.Affine K),W.Δ≠0 →
      Nat.card W.Point=equationCount (normalized W) ∧
      fieldTrace W=countedTrace (normalized W) ∧
      (sourceSupersingular W ↔ countedSupersingular (normalized W))) ∧
    (∀ W : WeierstrassCurve.Affine F5,W.Δ≠0 →
      (fieldTrace W)^2≤20 ∧ (sourceSupersingular W ↔ fieldTrace W=0) ∧
      (sourceSupersingular W ↔ Nat.card W.Point=6) ∧
      (sourceSupersingular W → ∀ r : ℕ,r∣Nat.card W.Point →
        Nat.Coprime r 5 ∧ orderOf (5:ZMod r)∣2 ∧ (2<r → orderOf (5:ZMod r)=2))) ∧
    (∀ W : WeierstrassCurve.Affine F7,W.Δ≠0 →
      (fieldTrace W)^2≤28 ∧ (sourceSupersingular W ↔ fieldTrace W=0) ∧
      (sourceSupersingular W ↔ Nat.card W.Point=8) ∧
      (sourceSupersingular W → ∀ r : ℕ,r∣Nat.card W.Point →
        Nat.Coprime r 7 ∧ orderOf (7:ZMod r)∣2 ∧ (2<r → orderOf (7:ZMod r)=2))) ∧
    (sixControl.Δ≠0 ∧ sourceSupersingular sixControl ∧ Nat.card sixControl.Point=6 ∧
      addOrderOf sixPoint=6 ∧ orderOf (5:ZMod 6)=2 ∧ orderOf (5:ZMod 2)=1) ∧
    (eightControl.Δ≠0 ∧ sourceSupersingular eightControl ∧ Nat.card eightControl.Point=8 ∧
      addOrderOf eightPoint=8 ∧ orderOf (7:ZMod 8)=2 ∧ orderOf (7:ZMod 2)=1) ∧
    (singularControl.Δ=0 ∧ Nat.card singularControl.Point=5 ∧ equationCount singularControl=6 ∧
      ¬sourceSupersingular singularControl ∧ countedSupersingular singularControl) ∧
    (¬∀ W : WeierstrassCurve.Affine F5,Nat.card W.Point=equationCount W) := by
  refine ⟨?_,?_,?_,?_,six_actual_control,eight_actual_control,singular_negative_control,
    deleting_nonsingularity_false⟩
  · intro K _ _ W hΔ
    exact ⟨normalized_delta W,⟨normalizationEquiv W hΔ⟩,normalized_shape W⟩
  · intro K _ _ _ W hΔ
    exact normalization_semantics W hΔ
  · intro W hΔ
    exact ⟨(five_all_models W hΔ).1,five_source_iff_trace_zero W hΔ,
      (five_all_models W hΔ).2,five_all_count_divisors W hΔ⟩
  · intro W hΔ
    exact ⟨(seven_all_models W hΔ).1,seven_source_iff_trace_zero W hΔ,
      (seven_all_models W hΔ).2,seven_all_count_divisors W hΔ⟩

#print axioms six_actual_control
#print axioms eight_actual_control
#print axioms deleting_nonsingularity_false
#print axioms prime_field_trace_certificate
end
end PrimeFieldTraceAudit
