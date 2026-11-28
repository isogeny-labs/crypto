import QuadraticModels

namespace NonaryNormalizedPredicateAudit
open SmallCharacteristicCensusAudit QuadraticFieldModelsAudit

def NormalizedPredicate (a₂ : F9) : Prop := ∀ a₄ a₆ : F9,
  let E : WeierstrassCurve.Affine F9 := ⟨0,a₂,0,a₄,a₆⟩
  Int.gcd ((10:ℤ)-(counted E:ℤ)) 9≠1 →
  counted E=4 ∨ counted E=7 ∨ counted E=10 ∨ counted E=13 ∨ counted E=16

end NonaryNormalizedPredicateAudit
