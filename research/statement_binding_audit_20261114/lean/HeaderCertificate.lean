module

public import HeaderControls

public section

namespace StatementBindingAudit

/-- Genuine semantic controls plus noninjectivity of the explicitly defined
simple delimiter model. Native parser observations and the Python verifier
are not asserted to be formally refined by this theorem. -/
theorem header_binding_controls_certificate :
    (∀ n : ℤ, 0 ≤ abs n) ∧
    (∀ n : ℕ, namedArgument n=n) ∧
    (¬ Function.Injective (prefixBefore barDelimiter)) ∧
    (¬ Function.Injective (prefixBefore namedDelimiter)) ∧
    (¬ Function.Injective (prefixBefore proofDelimiter)) ∧
    (barSourceA ≠ barSourceB ∧
      prefixBefore barDelimiter barSourceA=prefixBefore barDelimiter barSourceB) ∧
    (namedSourceA ≠ namedSourceB ∧
      prefixBefore namedDelimiter namedSourceA=prefixBefore namedDelimiter namedSourceB) ∧
    (proofSourceA ≠ proofSourceB ∧
      prefixBefore proofDelimiter proofSourceA=prefixBefore proofDelimiter proofSourceB) := by
  obtain ⟨hn₁,he₁,hn₂,he₂,hn₃,he₃⟩ := delimiter_model_not_injective
  refine ⟨bar_control,named_argument_control,?_,?_,?_,⟨hn₁,he₁⟩,⟨hn₂,he₂⟩,⟨hn₃,he₃⟩⟩
  · intro h
    exact hn₁ (h he₁)
  · intro h
    exact hn₂ (h he₂)
  · intro h
    exact hn₃ (h he₃)

#print axioms header_binding_controls_certificate

end StatementBindingAudit
