import NormalizedCase11
namespace NonaryNormalizedCaseAudit
open NonaryNormalizedPredicateAudit
theorem case12 : NormalizedPredicate ⟨1,2⟩ := by
  unfold NormalizedPredicate
  decide +kernel
#print axioms case12
end NonaryNormalizedCaseAudit
