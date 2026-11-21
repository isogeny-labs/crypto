import QuadraticModels
import SourceTrace

namespace QuadraticExtensionControlsAudit
open SmallCharacteristicCensusAudit QuadraticFieldModelsAudit FiniteFieldSourceTraceAudit
  SupersingularTraceClassesAudit TorsionBoundaryAudit

def alpha : F4 := ⟨0,1⟩
def beta : F9 := ⟨0,1⟩
def E4 : WeierstrassCurve.Affine F4 := ⟨0,0,alpha,alpha^2,0⟩
def E9 : WeierstrassCurve.Affine F9 := ⟨0,0,0,beta,1⟩

theorem E4_nonsingular : E4.Δ≠0 := by
  unfold E4 WeierstrassCurve.Δ WeierstrassCurve.b₂ WeierstrassCurve.b₄
    WeierstrassCurve.b₆ WeierstrassCurve.b₈
  decide +kernel

theorem E9_nonsingular : E9.Δ≠0 := by
  unfold E9 WeierstrassCurve.Δ WeierstrassCurve.b₂ WeierstrassCurve.b₄
    WeierstrassCurve.b₆ WeierstrassCurve.b₈
  decide +kernel

theorem E4_counted : counted E4=7 := by unfold counted E4;decide +kernel
theorem E9_counted : counted E9=13 := by unfold counted E9;decide +kernel

theorem E4_cardinality : Nat.card E4.Point=7 :=
  (actual_cardinality_eq_counted E4 E4_nonsingular).trans E4_counted
theorem E9_cardinality : Nat.card E9.Point=13 :=
  (actual_cardinality_eq_counted E9 E9_nonsingular).trans E9_counted

theorem E4_supersingular : SourceSupersingular E4 := by
  norm_num [SourceSupersingular,fieldTrace,field_cardinalities.1,E4_cardinality]
theorem E9_supersingular : SourceSupersingular E9 := by
  norm_num [SourceSupersingular,fieldTrace,field_cardinalities.2,E9_cardinality]

theorem E4_prime_order_point : ∃ P : E4.Point,addOrderOf P=7 := by
  let : Fact (Nat.Prime 7) := ⟨by decide⟩
  apply SmallCharacteristicPrimeAudit.actual_prime_order_point E4 7
  rw [E4_cardinality]
theorem E9_prime_order_point : ∃ P : E9.Point,addOrderOf P=13 := by
  let : Fact (Nat.Prime 13) := ⟨by decide⟩
  apply SmallCharacteristicPrimeAudit.actual_prime_order_point E9 13
  rw [E9_cardinality]

theorem E4_embedding_degree : embeddingDegree 4 7=3 := by
  apply (orderOf_eq_iff (by decide : 0<3)).mpr
  constructor
  · decide +kernel
  · intro m hm hp
    interval_cases m <;> decide +kernel

theorem E9_embedding_degree : embeddingDegree 9 13=3 := by
  apply (orderOf_eq_iff (by decide : 0<3)).mpr
  constructor
  · decide +kernel
  · intro m hm hp
    interval_cases m <;> decide +kernel

#print axioms E4_cardinality
#print axioms E9_cardinality
#print axioms E4_prime_order_point
#print axioms E9_prime_order_point
end QuadraticExtensionControlsAudit
