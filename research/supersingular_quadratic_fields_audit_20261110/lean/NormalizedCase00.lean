import NormalizedPredicate
namespace NonaryNormalizedCaseAudit
open NonaryNormalizedPredicateAudit
theorem case00 : NormalizedPredicate ⟨0,0⟩ := by
  unfold NormalizedPredicate
  decide +kernel
#print axioms case00
end NonaryNormalizedCaseAudit
