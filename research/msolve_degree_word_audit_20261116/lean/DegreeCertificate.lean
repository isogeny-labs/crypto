import NativeWitness

namespace MsolveDegreeWordAudit
noncomputable section
open MvPolynomial SolvingDegreeBound

def ActualWordBounds : Prop :=
  (∀ xs : List ℕ, (degreeWord xs).toNat = xs.sum % 65536) ∧
  (∀ xs : List ℕ, (degreeWord xs).toNat = xs.sum ↔ xs.sum < 65536) ∧
  (lcmDegreeWord 65535 0 65534 1).toNat = 0 ∧
  max 65535 65534 + max 0 1 = 65536 ∧
  (¬ ∀ a b c d : ℕ, a+b<65536 → c+d<65536 →
    (lcmDegreeWord a b c d).toNat = max a c + max b d)

def ActualPolynomialBoundary : Prop :=
  ∀ (K : Type) [Field K], ∀ n : ℕ,
    (firstPoly n : Poly K).totalDegree = n+2 ∧
    (secondPoly n : Poly K).totalDegree = n+2 ∧
    DClosed (n+2) (Submodule.span K (inputs n)) ∧
    (∀ G : ℕ → Set (Poly K), F4Run (n+2) (inputs n) G → ∀ t, linearY ∉ G t) ∧
    F4Run (n+3) (inputs n) (healthyRun n : ℕ → Set (Poly K)) ∧
    (linearY : Poly K) ∈ healthyRun n 1

/-- The FULL inherited theorem, including the named-argument suffix omitted in its old plan. -/
def LegacyFullF4Statement : Prop :=
  (∀ (V A : Type) [Field A] (D : ℕ) (F : Set (MvPolynomial V A))
    (G : ℕ → Set (MvPolynomial V A)), F4Run D F G → ∀ t, G t ⊆ Ideal.span F) ∧
  (¬ ∀ (F : Set F4ExpectedOutputsAudit.P) (h p : F4ExpectedOutputsAudit.P),
    F4SaturationBoundaryAudit.SaturationMember (Ideal.span F) h p →
      ∃ (D t : ℕ) (G : ℕ → Set F4ExpectedOutputsAudit.P), F4Run D F G ∧ p ∈ G t) ∧
  F4ExpectedOutputsAudit.ordinaryRows = F4SaturationBoundaryAudit.booleanGenerators (K:=ZMod 101) ∧
  F4ExpectedOutputsAudit.ordinaryRows ⊆ Ideal.span (F4SaturationBoundaryAudit.booleanGenerators (K:=ZMod 101)) ∧
  (∀ p ∈ F4ExpectedOutputsAudit.saturatedRows,
    F4SaturationBoundaryAudit.SaturationMember
      (Ideal.span (F4SaturationBoundaryAudit.booleanGenerators (K:=ZMod 101)))
      F4SaturationBoundaryAudit.x p) ∧
  F4SaturationBoundaryAudit.y (K:=ZMod 101) ∉ Ideal.span F4SaturationBoundaryAudit.booleanGenerators ∧
  (∀ (D t : ℕ) (G : ℕ → Set F4ExpectedOutputsAudit.P),
    F4ExpectedOutputsAudit.saturatedRows ⊆ G t →
      ¬ F4Run D F4SaturationBoundaryAudit.booleanGenerators G)

theorem actual_degree_word_mathematical_certificate :
    ActualWordBounds ∧ ActualPolynomialBoundary ∧ LegacyFullF4Statement := by
  refine ⟨?_, ?_, ?_⟩
  · exact ⟨degree_word_mod, degree_word_exact_iff, boundary_lcm_word,
      boundary_lcm_natural, unrestricted_lcm_degree_reading_false⟩
  · intro K _ n
    exact ⟨first_degree n, second_degree n, input_span_closed n,
      fun G hG => no_lower_run n G hG, healthy_run n, healthy_run_contains_y n⟩
  · exact F4ExpectedOutputsAudit.expected_output_audit_certificate

def ActualRecordedPrefix : Prop :=
  decodeSet nativeControlInputs = (inputs 0 : Set (Poly (ZMod 101))) ∧
  decodeSet nativeBoundaryInputs = (inputs 65533 : Set (Poly (ZMod 101))) ∧
  (linearY : Poly (ZMod 101)) ∈ decodeSet nativeBoundaryBasis ∧
  (∀ r ∈ nativeControlLCM, ValidRecordedLCM r) ∧
  (∀ r ∈ nativeBoundaryLCM, ValidRecordedLCM r) ∧
  recordedBound 2 nativeControlRows nativeControlSelections = 3 ∧
  recordedBound 65535 nativeBoundaryRows nativeBoundarySelections = 65535 ∧
  ([65534,1,65535,0,65535,1,0,65536] ∈ nativeBoundaryLCM ∧
    [0,0,65536,1] ∈ nativeBoundaryRows ∧
    [0,2147483647] ∈ nativeBoundarySelections) ∧
  (¬ ∃ (G : Nat → Set (Poly (ZMod 101))) (t : Nat),
    F4Run nativeBoundaryStoredBound (decodeSet nativeBoundaryInputs) G ∧
    decodeSet nativeBoundaryBasis ⊆ G t)

/-- Recorded-prefix facts do not assert successful completion of the native call. -/
theorem actual_degree_word_certificate :
    ActualWordBounds ∧ ActualPolynomialBoundary ∧ ActualRecordedPrefix ∧ LegacyFullF4Statement := by
  obtain ⟨hw,hp,hl⟩ := actual_degree_word_mathematical_certificate
  refine ⟨hw,hp,?_,hl⟩
  exact ⟨native_control_inputs,native_boundary_inputs,written_boundary_contains_y,
    recorded_lcm_arithmetic.1,recorded_lcm_arithmetic.2,
    recorded_bounds.1,recorded_bounds.2.1,recorded_boundary_events,
    written_boundary_not_degree_word_run⟩

#print axioms actual_degree_word_mathematical_certificate
#print axioms actual_degree_word_certificate
end
end MsolveDegreeWordAudit
