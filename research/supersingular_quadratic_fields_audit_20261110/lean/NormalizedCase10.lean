import NormalizedCase02
namespace NonaryNormalizedCaseAudit
open NonaryNormalizedPredicateAudit
theorem case10 : NormalizedPredicate ⟨1,0⟩ := by
  unfold NormalizedPredicate
  decide +kernel
#print axioms case10
end NonaryNormalizedCaseAudit
