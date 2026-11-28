import F4SaturationBoundary
import Mathlib.Data.ZMod.Basic

/-! Literal expected-output interpretation for the preregistered F101 fixture.
This does not formalize the C parser or the native solver's execution. -/
namespace F4ExpectedOutputsAudit
noncomputable section
open MvPolynomial SolvingDegreeBound F4SaturationBoundaryAudit

instance : Fact (Nat.Prime 101) := ⟨by decide +kernel⟩
abbrev P := F4SaturationBoundaryAudit.Poly (ZMod 101)

def ordinaryRows : Set P := {x*y,x^2+C 100*x,y^2+C 100*y}
def saturatedRows : Set P := {x+C 100,y}

theorem coefficient_100 : C (100 : ZMod 101) = (-1 : P) := by
  have hc : (100 : ZMod 101) = -1 := by decide +kernel
  rw [hc,map_neg,map_one]

theorem ordinaryRows_eq : ordinaryRows=booleanGenerators (K:=ZMod 101) := by
  simp only [ordinaryRows,booleanGenerators,coefficient_100,neg_one_mul,sub_eq_add_neg]

theorem saturatedRows_eq : saturatedRows={x (K:=ZMod 101)-1,y} := by
  simp only [saturatedRows,coefficient_100,sub_eq_add_neg]

theorem ordinary_rows_in_ideal : ordinaryRows⊆Ideal.span (booleanGenerators (K:=ZMod 101)) := by
  rw [ordinaryRows_eq]
  exact Ideal.subset_span

theorem saturated_rows_in_saturation : ∀ p∈saturatedRows,
    SaturationMember (Ideal.span (booleanGenerators (K:=ZMod 101))) x p := by
  intro p hp
  rw [saturatedRows_eq] at hp
  rcases hp with rfl | rfl
  · exact x_minus_one_in_boolean_saturation
  · exact y_in_boolean_saturation

theorem saturated_rows_outside_model (D t : ℕ) (G : ℕ→Set P)
    (hrows : saturatedRows⊆G t) : ¬F4Run D booleanGenerators G := by
  apply boolean_saturation_output_cannot_be_F4Run D t G
  apply hrows
  simp [saturatedRows]

theorem expected_output_audit_certificate :
    (∀ (V A : Type*) [Field A] (D : ℕ) (F : Set (MvPolynomial V A))
      (G : ℕ→Set (MvPolynomial V A)),F4Run D F G → ∀ t,G t⊆Ideal.span F) ∧
    (¬ ∀ (F : Set P) (h p : P),
      SaturationMember (Ideal.span F) h p →
        ∃ (D t : ℕ) (G : ℕ→Set P),F4Run D F G ∧ p∈G t) ∧
    ordinaryRows=booleanGenerators (K:=ZMod 101) ∧
    ordinaryRows⊆Ideal.span (booleanGenerators (K:=ZMod 101)) ∧
    (∀ p∈saturatedRows,
      SaturationMember (Ideal.span (booleanGenerators (K:=ZMod 101))) x p) ∧
    y (K:=ZMod 101)∉Ideal.span booleanGenerators ∧
    (∀ (D t : ℕ) (G : ℕ→Set P),saturatedRows⊆G t → ¬F4Run D booleanGenerators G) := by
  refine ⟨?_,unrestricted_saturation_model_claim_false,ordinaryRows_eq,
    ordinary_rows_in_ideal,saturated_rows_in_saturation,
    y_not_in_boolean_ideal,saturated_rows_outside_model⟩
  intro V A _ D F G hrun
  exact every_run_in_input_ideal D F G hrun

#print axioms expected_output_audit_certificate
end
end F4ExpectedOutputsAudit
