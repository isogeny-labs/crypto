module
public import SmallControls

@[expose] public section
namespace PrimeTraceWeakBoundAudit
noncomputable section

local instance : Fact (Nat.Prime 3) := ⟨by decide⟩
local instance : Fact (Nat.Prime 5) := ⟨by decide⟩
local instance : Fact (Nat.Prime 7) := ⟨by decide⟩

/-- Actual source prime-field clause with no assumed Hasse/count/trace-zero
or modular-order/field-degree identification, and actual omission controls. -/
theorem prime_trace_weak_bound_certificate :
    (∀ (K : Type) [Field K] [Fintype K],
      3 < Fintype.card K → ringChar K ≠ 2 →
      ∀ (W : WeierstrassCurve.Affine K), W.Δ ≠ 0 →
        1 < Nat.card W.Point ∧ Nat.card W.Point < 2 * Fintype.card K + 1) ∧
    (∀ (p : ℕ) [Fact p.Prime], 5 ≤ p →
      ∀ (W : WeierstrassCurve.Affine (ZMod p)), W.Δ ≠ 0 →
        (sourceSupersingular p W ↔ actualTrace W = 0) ∧
        (sourceSupersingular p W → Nat.card W.Point = p + 1) ∧
        (∀ (r : ℕ), 2 < r → r ∣ Nat.card W.Point → sourceSupersingular p W →
          orderOf (p : ZMod r) = 2 ∧ Module.finrank (ZMod p) (rootsField p r) = 2)) ∧
    lowerThree.Δ ≠ 0 ∧ upperThree.Δ ≠ 0 ∧
    Nat.card lowerThree.Point = 1 ∧ Nat.card upperThree.Point = 7 ∧
    sourceSupersingular 3 lowerThree ∧ sourceSupersingular 3 upperThree ∧
    actualTrace lowerThree = 3 ∧ actualTrace upperThree = -3 ∧
    twoControl.Δ ≠ 0 ∧ sourceSupersingular 5 twoControl ∧
    Nat.card twoControl.Point = 6 ∧ addOrderOf twoPoint = 2 ∧
    Module.finrank (ZMod 5) (rootsField 5 (addOrderOf twoPoint)) = 1 ∧
    ¬ Module.finrank (ZMod 5) (rootsField 5 (addOrderOf twoPoint)) = 2 ∧
    eightControl.Δ ≠ 0 ∧ sourceSupersingular 7 eightControl ∧
    Nat.card eightControl.Point = 8 ∧ addOrderOf eightPoint = 8 ∧
    Module.finrank (ZMod 7) (rootsField 7 (addOrderOf eightPoint)) = 2 := by
  refine ⟨?_, ?_, lower_three_delta, upper_three_delta, lower_three_count,
    upper_three_count, three_source_controls.1, three_source_controls.2.1,
    three_source_controls.2.2.1, three_source_controls.2.2.2,
    two_control_delta, two_control_source, two_control_count, two_point_order,
    ?_, literal_prime_field_degree_two_false, eight_control_delta, eight_control_source,
    eight_control_count, eight_point_order, ?_⟩
  · intro K _ _ hq hF W hΔ
    exact actual_weak_point_bound hq hF W hΔ
  · intro p _ hp W hΔ
    refine ⟨prime_source_iff_trace_zero p hp W hΔ, prime_source_count p hp W hΔ, ?_⟩
    intro r hr hd hss
    exact ⟨prime_source_modular_order p hp W hΔ hss r hr hd,
      prime_source_cyclotomic_degree p hp W hΔ hss r hr hd⟩
  · rw [two_point_order]
    exact two_control_actual_degree
  · rw [eight_point_order]
    exact eight_control_actual_degree

#print axioms prime_trace_weak_bound_certificate
end
end PrimeTraceWeakBoundAudit
