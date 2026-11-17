import NormalizedCase01
namespace NonaryNormalizedCaseAudit
open NonaryNormalizedPredicateAudit
theorem case02 : NormalizedPredicate ⟨0,2⟩ := by
  unfold NormalizedPredicate
  decide +kernel
#print axioms case02
end NonaryNormalizedCaseAudit
