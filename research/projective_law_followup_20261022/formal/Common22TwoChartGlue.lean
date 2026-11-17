import Common22PrincipalOpenSpec
import Mathlib.AlgebraicGeometry.Gluing
import Mathlib.AlgebraicGeometry.Pullbacks

set_option autoImplicit false
noncomputable section

namespace Common22TwoChartGlue

open AlgebraicGeometry CategoryTheory CategoryTheory.Limits

variable {K : Type} [Field K]

def chart (a3 a2 a1 a0 : K) : Bool → Scheme
  | false => Spec ↧(Common22SeparateChartMap.AffineRing a3 a2 a1 a0)
  | true => Spec ↧(Common22ExplicitChart.ChartRing a3 a2 a1 a0)

def overlap (a3 a2 a1 a0 : K) : Bool → Scheme
  | false => Spec ↧(Common22SeparateChartMap.AffineOpen a3 a2 a1 a0)
  | true => Spec ↧(Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0)

def embed (a3 a2 a1 a0 : K) (i : Bool) :
    overlap a3 a2 a1 a0 i ⟶ chart a3 a2 a1 a0 i := by
  cases i
  · exact Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding a3 a2 a1 a0
  · exact Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding a3 a2 a1 a0

instance embed_open (a3 a2 a1 a0 : K) (i : Bool) :
    IsOpenImmersion (embed a3 a2 a1 a0 i) := by
  cases i
  · exact Common22PrincipalOpenSpec.affinePrincipalOpenEmbedding_isOpenImmersion a3 a2 a1 a0
  · exact Common22PrincipalOpenSpec.infinityPrincipalOpenEmbedding_isOpenImmersion a3 a2 a1 a0

def transition (a3 a2 a1 a0 : K) (i j : Bool) (h : i ≠ j) :
    overlap a3 a2 a1 a0 i ⟶ overlap a3 a2 a1 a0 j := by
  cases i <;> cases j
  · exact False.elim (h rfl)
  · exact (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).hom
  · exact (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).inv
  · exact False.elim (h rfl)

def preData (a3 a2 a1 a0 : K) : CategoryTheory.GlueData'.{0, 1} Scheme.{0} where
  J := Bool
  U := chart a3 a2 a1 a0
  V i _ _ := overlap a3 a2 a1 a0 i
  f i _ _ := embed a3 a2 a1 a0 i
  f_mono i j h := by
    infer_instance
  f_hasPullback i j k hij hik := by infer_instance
  t i j h := transition a3 a2 a1 a0 i j h
  t' i j k hij hik hjk := by
    cases i <;> cases j <;> cases k <;>
      first | exact False.elim (hij rfl) | exact False.elim (hik rfl) |
        exact False.elim (hjk rfl)
  t_fac i j k hij hik hjk := by
    cases i <;> cases j <;> cases k <;>
      first | exact False.elim (hij rfl) | exact False.elim (hik rfl) |
        exact False.elim (hjk rfl)
  t_inv i j hij := by
    cases i <;> cases j
    · exact False.elim (hij rfl)
    · change (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).hom ≫
        (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).inv = 𝟙 _
      exact (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).hom_inv_id
    · change (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).inv ≫
        (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).hom = 𝟙 _
      exact (Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0).inv_hom_id
    · exact False.elim (hij rfl)
  cocycle i j k hij hik hjk := by
    cases i <;> cases j <;> cases k <;>
      first | exact False.elim (hij rfl) | exact False.elim (hik rfl) |
        exact False.elim (hjk rfl)

def glueData (a3 a2 a1 a0 : K) : Scheme.GlueData where
  toGlueData := CategoryTheory.GlueData.ofGlueData' (preData a3 a2 a1 a0)
  f_open i j := by
    dsimp [CategoryTheory.GlueData.ofGlueData', CategoryTheory.GlueData'.f']
    split_ifs with h
    · infer_instance
    · letI : IsOpenImmersion ((preData a3 a2 a1 a0).f i j h) := by
        change IsOpenImmersion (embed a3 a2 a1 a0 i)
        exact embed_open a3 a2 a1 a0 i
      infer_instance

def gluedCurve (a3 a2 a1 a0 : K) : Scheme :=
  (glueData a3 a2 a1 a0).glued

def chartIntoGlued (a3 a2 a1 a0 : K) (i : Bool) :
    chart a3 a2 a1 a0 i ⟶ gluedCurve a3 a2 a1 a0 :=
  (glueData a3 a2 a1 a0).ι i

instance chartIntoGlued_open (a3 a2 a1 a0 : K) (i : Bool) :
    IsOpenImmersion (chartIntoGlued a3 a2 a1 a0 i) :=
  (glueData a3 a2 a1 a0).ι_isOpenImmersion i

theorem chartIntoGlued_jointly_surjective (a3 a2 a1 a0 : K)
    (x : (gluedCurve a3 a2 a1 a0).carrier) :
    ∃ (i : Bool) (y : (chart a3 a2 a1 a0 i).carrier),
      chartIntoGlued a3 a2 a1 a0 i y = x :=
  (glueData a3 a2 a1 a0).ι_jointly_surjective x

#print axioms preData
#print axioms glueData
#print axioms gluedCurve
#print axioms chartIntoGlued_open
#print axioms chartIntoGlued_jointly_surjective

end Common22TwoChartGlue
