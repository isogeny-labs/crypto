import WeightedProjOverlapNonempty
import Common22SeparableAffineDomain
import Mathlib.AlgebraicGeometry.Properties

/-! Domain-to-scheme transport for the two actual weighted-Proj basic opens.
The whole-curve `IsIntegral` instance is a separate gluing obligation. -/

set_option autoImplicit false
set_option maxHeartbeats 600000
set_option backward.isDefEq.respectTransparency.types false
set_option backward.isDefEq.respectTransparency false
noncomputable section

namespace WeightedProjIsIntegral

open AlgebraicGeometry CategoryTheory WeightedRingGraded
  Common22WeightedProjQuotient WeightedOverlapSpecialized

variable {K : Type} [Field K]

theorem x_basic_open_integral (a3 a2 a1 a0 : K) :
    IsIntegral ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).toScheme) := by
  letI : IsDomain (WeightedInfinityOrdinary.InfinityRing a3 a2 a1 a0) :=
    Common22Domain.chart_ring_domain a3 a2 a1 a0
  exact IsIntegral.of_isIso
    (WeightedProjInfinityChart.xBasicOpenIsoInfinity a3 a2 a1 a0).inv

theorem z_basic_open_integral (a3 a2 a1 a0 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    IsIntegral ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (Common22WeightedProjKernel.zbar a3 a2 a1 a0)).toScheme) := by
  letI : IsDomain (Common22WeightedProjQuotient.AffineRing a3 a2 a1 a0) := by
    change IsDomain (Common22SeparateChartMap.AffineRing a3 a2 a1 a0)
    exact Common22SeparableAffineDomain.affine_ring_domain_of_separable
      a3 a2 a1 a0 hsep
  exact IsIntegral.of_isIso
    (WeightedProjSchemeChart.zBasicOpenIsoAffine a3 a2 a1 a0).inv

#print axioms x_basic_open_integral
#print axioms z_basic_open_integral

theorem whole_curve_integral (a3 a2 a1 a0 : K)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    IsIntegral (Proj (degreeSubmodule a3 a2 a1 a0)) := by
  let X := Proj (degreeSubmodule a3 a2 a1 a0)
  let Ux : X.Opens := Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedOverlapSpecialized.Xc a3 a2 a1 a0)
  let Uz : X.Opens := Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
    (WeightedOverlapSpecialized.Zc a3 a2 a1 a0)
  have hx : IsIntegral Ux.toScheme := by
    change IsIntegral ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (WeightedInfinityOrdinary.xbar a3 a2 a1 a0)).toScheme)
    exact x_basic_open_integral a3 a2 a1 a0
  have hz : IsIntegral Uz.toScheme := by
    change IsIntegral ((Proj.basicOpen (degreeSubmodule a3 a2 a1 a0)
      (Common22WeightedProjKernel.zbar a3 a2 a1 a0)).toScheme)
    exact z_basic_open_integral a3 a2 a1 a0 hsep
  let U : Bool → X.Opens := fun b => if b then Uz else Ux
  have hcover : TopologicalSpace.IsOpenCover U := by
    apply TopologicalSpace.IsOpenCover.mk
    rw [iSup_bool_eq]
    change Uz ⊔ Ux = ⊤
    rw [sup_comm]
    exact WeightedProjCover.two_basic_opens_cover a3 a2 a1 a0
  obtain ⟨p, hpx, hpz⟩ :=
    WeightedProjOverlapNonempty.actual_x_z_overlap_nonempty a3 a2 a1 a0
  have hdis : ¬ Disjoint Ux Uz := by
    intro hd
    have hmem : p ∈ Ux ⊓ Uz := ⟨hpx, hpz⟩
    have hbot : Ux ⊓ Uz = ⊥ := disjoint_iff.mp hd
    rw [hbot] at hmem
    exact hmem
  have hpair : Pairwise (fun i j => ¬ Disjoint (U i) (U j)) := by
    intro i j hij
    cases i <;> cases j
    · exact (hij rfl).elim
    · simpa [U] using hdis
    · simpa [U, disjoint_comm] using hdis
    · exact (hij rfl).elim
  letI : PreirreducibleSpace X :=
    PreirreducibleSpace.of_isOpenCover hpair hcover (by
      intro b
      cases b
      · change PreirreducibleSpace Ux.toScheme
        letI : IsIntegral Ux.toScheme := hx
        infer_instance
      · change PreirreducibleSpace Uz.toScheme
        letI : IsIntegral Uz.toScheme := hz
        infer_instance)
  let C : X.OpenCover := Scheme.Cover.mkOfCovers Bool
    (fun b => (U b).toScheme) (fun b => (U b).ι) (by
      intro x
      obtain ⟨b, hb⟩ := hcover.exists_mem x
      exact ⟨b, ⟨x, hb⟩, rfl⟩)
  have hloc (b : Bool) : IsReduced (C.X b) := by
    cases b
    · change IsReduced Ux.toScheme
      letI : IsIntegral Ux.toScheme := hx
      infer_instance
    · change IsReduced Uz.toScheme
      letI : IsIntegral Uz.toScheme := hz
      infer_instance
  have hred : IsReduced X := by
    letI : ∀ b : Bool, IsReduced (C.X b) := hloc
    exact IsReduced.of_openCover X C
  letI : IsReduced X := hred
  letI : Nonempty X := ⟨p⟩
  letI : IrreducibleSpace X :=
    { toPreirreducibleSpace := inferInstance, toNonempty := inferInstance }
  exact isIntegral_of_irreducibleSpace_of_isReduced X

#print axioms whole_curve_integral

end WeightedProjIsIntegral
