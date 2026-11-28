import NormalizedCase22

namespace NonaryNormalizedAllAudit
open QuadraticFieldModelsAudit NonaryNormalizedPredicateAudit NonaryNormalizedCaseAudit

theorem all_normalized_models (a₂ : F9) : NormalizedPredicate a₂ := by
  rcases a₂ with ⟨r,s⟩
  fin_cases r <;> fin_cases s
  · exact case00
  · exact case01
  · exact case02
  · exact case10
  · exact case11
  · exact case12
  · exact case20
  · exact case21
  · exact case22

#print axioms all_normalized_models
end NonaryNormalizedAllAudit
