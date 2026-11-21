import QuadraticModels

namespace CharacteristicThreeNormalizationAudit
open SmallCharacteristicCensusAudit
variable {K : Type*} [Field K] [CharP K 3]

def normalize (E : WeierstrassCurve.Affine K) : WeierstrassCurve.Affine K :=
  ⟨0,E.a₂+E.a₁^2,0,E.a₄-E.a₁*E.a₃,E.a₆+E.a₃^2⟩

theorem normalized_equation (E : WeierstrassCurve.Affine K) (x y : K) :
    equationTest E (x,y) ↔ equationTest (normalize E) (x,y-E.a₁*x-E.a₃) := by
  have h3 : (3:K)=0 := CharP.cast_eq_zero K 3
  dsimp only [equationTest,normalize]
  constructor
  · intro h
    linear_combination h+(E.a₁*E.a₃*x-E.a₁*x*y-E.a₃*y)*h3
  · intro h
    linear_combination h-(E.a₁*E.a₃*x-E.a₁*x*y-E.a₃*y)*h3

def equationEquiv (E : WeierstrassCurve.Affine K) :
    {xy : K×K // equationTest E xy}≃{xy : K×K // equationTest (normalize E) xy} where
  toFun xy := ⟨(xy.val.1,xy.val.2-E.a₁*xy.val.1-E.a₃),
    (normalized_equation E xy.val.1 xy.val.2).mp xy.property⟩
  invFun xy := ⟨(xy.val.1,xy.val.2+E.a₁*xy.val.1+E.a₃),by
    apply (normalized_equation E xy.val.1 (xy.val.2+E.a₁*xy.val.1+E.a₃)).mpr
    have h : xy.val.2+E.a₁*xy.val.1+E.a₃-E.a₁*xy.val.1-E.a₃=xy.val.2 := by ring
    rw [h]
    exact xy.property⟩
  left_inv xy := by apply Subtype.ext;apply Prod.ext;rfl;dsimp;ring
  right_inv xy := by apply Subtype.ext;apply Prod.ext;rfl;dsimp;ring

theorem counted_normalize [Fintype K] [DecidableEq K] (E : WeierstrassCurve.Affine K) :
    counted E=counted (normalize E) :=
  Fintype.card_congr (Equiv.optionCongr (equationEquiv E))

#print axioms normalized_equation
#print axioms counted_normalize
end CharacteristicThreeNormalizationAudit
