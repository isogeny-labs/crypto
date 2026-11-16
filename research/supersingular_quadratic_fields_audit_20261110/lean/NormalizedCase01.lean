import NormalizedCase00
namespace NonaryNormalizedCaseAudit
open NonaryNormalizedPredicateAudit
theorem case01 : NormalizedPredicate ⟨0,1⟩ := by
  unfold NormalizedPredicate
  decide +kernel
#print axioms case01
end NonaryNormalizedCaseAudit
