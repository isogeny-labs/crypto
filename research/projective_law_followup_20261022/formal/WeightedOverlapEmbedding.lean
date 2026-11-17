import WeightedOverlapEquality
import Mathlib.AlgebraicGeometry.ProjectiveSpectrum.Basic

/-! Canonical embeddings of the two weighted-Proj chart overlaps.
These statements concern the actual open immersions into `Proj`; they do not
identify the separately glued two-chart curve with `Proj`. -/

set_option autoImplicit false
set_option maxHeartbeats 400000
noncomputable section

namespace WeightedOverlapEmbedding

open AlgebraicGeometry CategoryTheory WeightedOverlapFull WeightedOverlapSpecialized
  Common22WeightedProjQuotient WeightedRingGraded

variable {K : Type*} [Field K]

theorem zCommon_symm_awayMap (a3 a2 a1 a0 : K)
    (t : ZAway a3 a2 a1 a0) :
    (zCommonEquiv a3 a2 a1 a0).symm
      ((HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0)
        (show Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
          Zc a3 a2 a1 a0 * Xc a3 a2 a1 a0 from mul_comm _ _)) t) =
      algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0) t := by
  apply (RingEquiv.symm_apply_eq (zCommonEquiv a3 a2 a1 a0)).2
  exact (WeightedOverlapEquality.zCommon_algebraMap a3 a2 a1 a0 t).symm

#print axioms zCommon_symm_awayMap

theorem xCommon_symm_awayMap (a3 a2 a1 a0 : K)
    (t : XAway a3 a2 a1 a0) :
    (xCommonEquiv a3 a2 a1 a0).symm
      ((HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
        (Zc_mem a3 a2 a1 a0)
        (rfl : Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
          Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)) t) =
      algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0) t := by
  apply (RingEquiv.symm_apply_eq (xCommonEquiv a3 a2 a1 a0)).2
  exact (WeightedOverlapEquality.xCommon_algebraMap a3 a2 a1 a0 t).symm

#print axioms xCommon_symm_awayMap

theorem zLocalized_awayι (a3 a2 a1 a0 : K) :
    Spec.map (CommRingCat.ofHom
      (algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0))) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0) (Zc a3 a2 a1 a0)
        (Zc_mem a3 a2 a1 a0) (by decide) =
    Spec.map (CommRingCat.ofHom
      (zCommonEquiv a3 a2 a1 a0).symm.toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
        (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0)) (by decide) := by
  have hr : (zCommonEquiv a3 a2 a1 a0).symm.toRingHom.comp
      (HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0)
        (show Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
          Zc a3 a2 a1 a0 * Xc a3 a2 a1 a0 from mul_comm _ _)) =
      algebraMap (ZAway a3 a2 a1 a0) (ZOpen a3 a2 a1 a0) := by
    ext t
    exact zCommon_symm_awayMap a3 a2 a1 a0 t
  rw [← hr, CommRingCat.ofHom_comp, Spec.map_comp, Category.assoc]
  rw [Proj.SpecMap_awayMap_awayι]

#print axioms zLocalized_awayι

theorem xLocalized_awayι (a3 a2 a1 a0 : K) :
    Spec.map (CommRingCat.ofHom
      (algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0))) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0) (Xc a3 a2 a1 a0)
        (Xc_mem a3 a2 a1 a0) (by decide) =
    Spec.map (CommRingCat.ofHom
      (xCommonEquiv a3 a2 a1 a0).symm.toRingHom) ≫
      Proj.awayι (degreeSubmodule a3 a2 a1 a0)
        (Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)
        (SetLike.mul_mem_graded (Xc_mem a3 a2 a1 a0)
          (Zc_mem a3 a2 a1 a0)) (by decide) := by
  have hr : (xCommonEquiv a3 a2 a1 a0).symm.toRingHom.comp
      (HomogeneousLocalization.awayMap (degreeSubmodule a3 a2 a1 a0)
        (Zc_mem a3 a2 a1 a0)
        (rfl : Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0 =
          Xc a3 a2 a1 a0 * Zc a3 a2 a1 a0)) =
      algebraMap (XAway a3 a2 a1 a0) (XOpen a3 a2 a1 a0) := by
    ext t
    exact xCommon_symm_awayMap a3 a2 a1 a0 t
  rw [← hr, CommRingCat.ofHom_comp, Spec.map_comp, Category.assoc]
  rw [Proj.SpecMap_awayMap_awayι]

#print axioms xLocalized_awayι

end WeightedOverlapEmbedding
