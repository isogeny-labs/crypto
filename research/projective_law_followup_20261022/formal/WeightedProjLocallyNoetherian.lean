import WeightedProjIsIntegral
import Mathlib.AlgebraicGeometry.Noetherian
import Mathlib.RingTheory.Polynomial.Basic
import Mathlib.RingTheory.Ideal.Quotient.Noetherian

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
set_option backward.isDefEq.respectTransparency false
noncomputable section

namespace WeightedProjLocallyNoetherian

open AlgebraicGeometry CategoryTheory WeightedRingGraded
  Common22WeightedProjQuotient WeightedOverlapSpecialized

variable {K : Type} [Field K]

theorem infinity_chart_noetherian (a3 a2 a1 a0 : K) :
    IsNoetherianRing (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0) := by
  infer_instance

theorem affine_chart_noetherian (a3 a2 a1 a0 : K) :
    IsNoetherianRing (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) := by
  infer_instance

theorem whole_curve_locally_noetherian (a3 a2 a1 a0 : K) :
    IsLocallyNoetherian (Proj (degreeSubmodule a3 a2 a1 a0)) := by
  let X := Proj (degreeSubmodule a3 a2 a1 a0)
  let Ux : X.Opens := Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)
  let Uz : X.Opens := Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)
  have hx : IsLocallyNoetherian Ux.toScheme := by
    letI : IsNoetherianRing (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0) :=
      infinity_chart_noetherian a3 a2 a1 a0
    change IsLocallyNoetherian ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).toScheme)
    exact isLocallyNoetherian_of_isOpenImmersion
      (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).hom
  have hz : IsLocallyNoetherian Uz.toScheme := by
    letI : IsNoetherianRing (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) :=
      affine_chart_noetherian a3 a2 a1 a0
    change IsLocallyNoetherian ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (Common22WeightedProjKernel.zbar a3 a2 a1 a0)).toScheme)
    exact isLocallyNoetherian_of_isOpenImmersion
      (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).hom
  let U : Bool → X.Opens := fun b => if b then Uz else Ux
  have hcover : TopologicalSpace.IsOpenCover U := by
    apply TopologicalSpace.IsOpenCover.mk
    rw [iSup_bool_eq]
    change Uz ⊔ Ux = ⊤
    rw [sup_comm]
    exact WeightedProjCover.two_basic_opens_cover a3 a2 a1 a0
  exact (isLocallyNoetherian_iff_openCover (X.openCoverOfIsOpenCover U hcover)).mpr (by
    intro b
    cases b
    · change IsLocallyNoetherian Ux.toScheme
      exact hx
    · change IsLocallyNoetherian Uz.toScheme
      exact hz)

#print axioms infinity_chart_noetherian
#print axioms affine_chart_noetherian
#print axioms whole_curve_locally_noetherian
end WeightedProjLocallyNoetherian
