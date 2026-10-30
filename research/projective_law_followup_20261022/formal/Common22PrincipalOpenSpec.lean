import Common22SeparateChartMap
import Mathlib.AlgebraicGeometry.OpenImmersion

/-! Scheme-level consequence of the checked equivalence between the two
separately presented principal-open coordinate rings. The smooth projective
completion and its infinity stalk are outside this theorem. -/

set_option autoImplicit false

noncomputable section

namespace Common22PrincipalOpenSpec

open AlgebraicGeometry CategoryTheory

variable {K : Type*} [Field K]

def affinePrincipalOpenEmbedding (a3 a2 a1 a0 : K) :
    Spec ↧(Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) ⟶
      Spec ↧(Common22SeparateChartMap.AffineRing a3 a2 a1 a0) :=
  Spec.map (CommRingCat.ofHom
    (algebraMap (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)
      (Common22SeparateChartMap.AffineOpen a3 a2 a1 a0)))

def infinityPrincipalOpenEmbedding (a3 a2 a1 a0 : K) :
    Spec ↧(Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) ⟶
      Spec ↧(Common22ExplicitChart.ChartRing a3 a2 a1 a0) :=
  Spec.map (CommRingCat.ofHom
    (algebraMap (Common22ExplicitChart.ChartRing a3 a2 a1 a0)
      (Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0)))

instance affinePrincipalOpenEmbedding_isOpenImmersion (a3 a2 a1 a0 : K) :
    IsOpenImmersion (affinePrincipalOpenEmbedding a3 a2 a1 a0) := by
  unfold affinePrincipalOpenEmbedding
  infer_instance

instance infinityPrincipalOpenEmbedding_isOpenImmersion (a3 a2 a1 a0 : K) :
    IsOpenImmersion (infinityPrincipalOpenEmbedding a3 a2 a1 a0) := by
  unfold infinityPrincipalOpenEmbedding
  infer_instance

theorem affinePrincipalOpenEmbedding_opensRange (a3 a2 a1 a0 : K) :
    (affinePrincipalOpenEmbedding a3 a2 a1 a0).opensRange =
      PrimeSpectrum.basicOpen (Common22SeparateChartMap.xbar a3 a2 a1 a0) := by
  simpa only [affinePrincipalOpenEmbedding] using
    (Scheme.Hom.opensRange_localizationAway
      (R := ↧(Common22SeparateChartMap.AffineRing a3 a2 a1 a0))
      (Common22SeparateChartMap.xbar a3 a2 a1 a0))

theorem infinityPrincipalOpenEmbedding_opensRange (a3 a2 a1 a0 : K) :
    (infinityPrincipalOpenEmbedding a3 a2 a1 a0).opensRange =
      PrimeSpectrum.basicOpen (Common22ExplicitChart.zbar a3 a2 a1 a0) := by
  simpa only [infinityPrincipalOpenEmbedding] using
    (Scheme.Hom.opensRange_localizationAway
      (R := ↧(Common22ExplicitChart.ChartRing a3 a2 a1 a0))
      (Common22ExplicitChart.zbar a3 a2 a1 a0))

/-- An isomorphism of the two affine schemes representing the principal-open
overlap, induced contravariantly from the checked coordinate-ring equivalence. -/
def principalOpenSpecIso (a3 a2 a1 a0 : K) :
    Spec ↧(Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) ≅
      Spec ↧(Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) := by
  let e := RingEquiv.toCommRingCatIso
    ((Common22SeparateChartMap.separateChartOpenEquiv a3 a2 a1 a0).toRingEquiv)
  exact Scheme.Spec.mapIso e.symm.op

#print axioms affinePrincipalOpenEmbedding_isOpenImmersion
#print axioms infinityPrincipalOpenEmbedding_isOpenImmersion
#print axioms affinePrincipalOpenEmbedding_opensRange
#print axioms infinityPrincipalOpenEmbedding_opensRange
#print axioms principalOpenSpecIso

end Common22PrincipalOpenSpec
