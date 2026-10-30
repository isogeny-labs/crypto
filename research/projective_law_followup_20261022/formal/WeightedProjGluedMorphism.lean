import WeightedProjGlueCompatibility
import WeightedProjGlobalGlue

/-! The actual `Proj` morphism on the common open is preserved by the
separately glued transition. The global isomorphism and Pic⁰ comparison remain open. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
set_option maxRecDepth 4096
noncomputable section

namespace WeightedProjGluedMorphism

open AlgebraicGeometry CategoryTheory CategoryTheory.Limits WeightedOverlapFull WeightedOverlapSpecialized
  WeightedRingGraded Common22WeightedProjQuotient

variable {K : Type} [Field K]

theorem overlapCommonIntoProj_compat (a3 a2 a1 a0 : K) :
    Common22TwoChartGlue.transition a3 a2 a1 a0 false true (by decide) ≫
      (Spec.map (CommRingCat.ofHom
        (xLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
        Spec.map (CommRingCat.ofHom
          (xCommonEquiv a3 a2 a1 a0).symm.toRingHom) ≫
          Proj.awayι (degreeSubmodule a3 a2 a1 a0)
            (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
            (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
              (Zc_mem a3 a2 a1 a0)) (by decide)) =
    Spec.map (CommRingCat.ofHom
      (zLocalizedEquiv a3 a2 a1 a0).toRingHom) ≫
      Spec.map (CommRingCat.ofHom
        (zCommonEquiv a3 a2 a1 a0).symm.toRingHom) ≫
        Proj.awayι (degreeSubmodule a3 a2 a1 a0)
          (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
          (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
            (Zc_mem a3 a2 a1 a0)) (by decide) := by
  have h := congrArg
    (fun f : Spec ↧(Common22SeparateChartMap.AffineOpen a3 a2 a1 a0) ⟶
        Spec ↧(CommonAway a3 a2 a1 a0) =>
      f ≫ Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
        (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0)) (by decide))
    (WeightedProjGlueCompatibility.overlapSpec_compat a3 a2 a1 a0)
  rw [← Category.assoc]
  exact h

#print axioms overlapCommonIntoProj_compat

theorem chartCompat_false_true (a3 a2 a1 a0 : K) :
    Common22TwoChartGlue.transition a3 a2 a1 a0 false true (by decide) ≫
      Common22TwoChartGlue.embed a3 a2 a1 a0 true ≫
        WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 true =
    Common22TwoChartGlue.embed a3 a2 a1 a0 false ≫
      WeightedProjGlobalGlue.chartIntoProj a3 a2 a1 a0 false := by
  let p := Common22PrincipalOpenSpec.principalOpenSpecIso a3 a2 a1 a0
  have hx := congrArg
    (fun f : Spec ↧(Common22SeparateChartMap.InfinityOpen a3 a2 a1 a0) ⟶
        Proj (degreeSubmodule a3 a2 a1 a0) => p.hom ≫ f)
    (WeightedProjChartIntoProj.xOverlap_common a3 a2 a1 a0)
  have hc := overlapCommonIntoProj_compat a3 a2 a1 a0
  have hz := WeightedProjChartIntoProj.zOverlap_common a3 a2 a1 a0
  exact hx.trans (hc.trans hz.symm)

#print axioms chartCompat_false_true


end WeightedProjGluedMorphism
