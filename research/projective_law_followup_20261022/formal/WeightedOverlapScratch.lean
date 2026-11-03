import WeightedProjInfinityChart
import WeightedProjSchemeChart
import WeightedProjCover
import Common22SeparateChartMap

set_option autoImplicit false
noncomputable section
namespace WeightedOverlapScratch
variable {A σ : Type*} [CommRing A] [SetLike σ A] [AddSubgroupClass σ A]
variable (𝒜 : ℕ → σ) [GradedRing 𝒜] (f g : A)


private theorem hfg (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) : f * g ∈ 𝒜 2 := by simpa using SetLike.mul_mem_graded hf hg
private theorem hff (hf : f ∈ 𝒜 1) : f * f ∈ 𝒜 (1 • (2 : ℕ)) := by simpa using SetLike.mul_mem_graded hf hf
private theorem hgg (hg : g ∈ 𝒜 1) : g * g ∈ 𝒜 (1 • (2 : ℕ)) := by simpa using SetLike.mul_mem_graded hg hg

def fOverG (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) : HomogeneousLocalization.Away 𝒜 (f*g) :=
  HomogeneousLocalization.Away.mk 𝒜 (hfg 𝒜 f g hf hg) 1 (f*f) (hff 𝒜 f hf)

def gOverF (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) : HomogeneousLocalization.Away 𝒜 (f*g) :=
  HomogeneousLocalization.Away.mk 𝒜 (hfg 𝒜 f g hf hg) 1 (g*g) (hgg 𝒜 g hg)

theorem zChart_x_to_common (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) :
    HomogeneousLocalization.awayMap 𝒜 hf (show f*g = g*f from mul_comm f g)
      (HomogeneousLocalization.Away.mk 𝒜 hg 1 f hf) =
        fOverG 𝒜 f g hf hg := by
  simpa [fOverG, hfg, hff] using
    (HomogeneousLocalization.awayMap_mk 𝒜 hf
      (show f*g = g*f from mul_comm f g) 1 hg f hf)

theorem xChart_z_to_common (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) :
    HomogeneousLocalization.awayMap 𝒜 hg (rfl : f*g = f*g)
      (HomogeneousLocalization.Away.mk 𝒜 hf 1 g hg) =
        gOverF 𝒜 f g hf hg := by
  simpa [gOverF, hfg, hgg, mul_comm] using
    (HomogeneousLocalization.awayMap_mk 𝒜 hg
      (rfl : f*g = f*g) 1 hf g hg)

def yOverG (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) (h : A) (hh : h ∈ 𝒜 3) :
    HomogeneousLocalization.Away 𝒜 (f*g) :=
  HomogeneousLocalization.Away.mk 𝒜 (hfg 𝒜 f g hf hg) 3 (h*f^3) (by
    simpa using SetLike.mul_mem_graded hh (SetLike.pow_mem_graded 3 hf))

def wOverF (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) (h : A) (hh : h ∈ 𝒜 3) :
    HomogeneousLocalization.Away 𝒜 (f*g) :=
  HomogeneousLocalization.Away.mk 𝒜 (hfg 𝒜 f g hf hg) 3 (h*g^3) (by
    simpa using SetLike.mul_mem_graded hh (SetLike.pow_mem_graded 3 hg))

theorem zChart_y_to_common (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1)
    (h : A) (hh : h ∈ 𝒜 3) :
    HomogeneousLocalization.awayMap 𝒜 hf (show f*g = g*f from mul_comm f g)
      (HomogeneousLocalization.Away.mk 𝒜 hg 3 h hh) =
        yOverG 𝒜 f g hf hg h hh := by
  simpa [yOverG, hfg, mul_comm] using
    (HomogeneousLocalization.awayMap_mk 𝒜 hf
      (show f*g = g*f from mul_comm f g) 3 hg h hh)

theorem xChart_w_to_common (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1)
    (h : A) (hh : h ∈ 𝒜 3) :
    HomogeneousLocalization.awayMap 𝒜 hg (rfl : f*g = f*g)
      (HomogeneousLocalization.Away.mk 𝒜 hf 3 h hh) =
        wOverF 𝒜 f g hf hg h hh := by
  simpa [wOverF, hfg] using
    (HomogeneousLocalization.awayMap_mk 𝒜 hg
      (rfl : f*g = f*g) 3 hf h hh)

theorem w_mul_x_cube_eq_y (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1)
    (h : A) (hh : h ∈ 𝒜 3) :
    wOverF 𝒜 f g hf hg h hh * (fOverG 𝒜 f g hf hg)^3 =
      yOverG 𝒜 f g hf hg h hh := by
  apply (HomogeneousLocalization.ext_iff_val _ _).2
  simp only [HomogeneousLocalization.val_mul, HomogeneousLocalization.val_pow,
    wOverF, fOverG, yOverG, HomogeneousLocalization.Away.val_mk]
  rw [Localization.mk_pow, Localization.mk_mul]
  rw [Localization.mk_eq_mk_iff]
  apply Localization.r_of_eq
  dsimp
  ring

theorem fOverG_mul_gOverF (hf : f ∈ 𝒜 1) (hg : g ∈ 𝒜 1) : fOverG 𝒜 f g hf hg * gOverF 𝒜 f g hf hg = 1 := by
  apply (HomogeneousLocalization.ext_iff_val _ _).2
  simp only [HomogeneousLocalization.val_mul, fOverG, gOverF,
    HomogeneousLocalization.Away.val_mk, map_one, HomogeneousLocalization.val_one]
  rw [Localization.mk_mul]
  simpa [pow_one, mul_comm, mul_left_comm, mul_assoc] using
    (Localization.mk_self (M := Submonoid.powers (f*g)) ⟨(f*g)^2, ⟨2, rfl⟩⟩)
end WeightedOverlapScratch

namespace WeightedOverlapSpecialized
open Common22WeightedProjHead Common22WeightedProjQuotient WeightedRingGraded WeightedProjCover
variable {K : Type*} [Field K]

def Xc (a3 a2 a1 a0 : K) : WeightedRing a3 a2 a1 a0 := gen a3 a2 a1 a0 0
def Yc (a3 a2 a1 a0 : K) : WeightedRing a3 a2 a1 a0 := gen a3 a2 a1 a0 1
def Zc (a3 a2 a1 a0 : K) : WeightedRing a3 a2 a1 a0 := gen a3 a2 a1 a0 2

theorem Xc_mem (a3 a2 a1 a0 : K) : Xc a3 a2 a1 a0 ∈ degreeSubmodule a3 a2 a1 a0 1 := by
  simpa [Xc, weights] using gen_mem a3 a2 a1 a0 (0 : Fin 3)
theorem Yc_mem (a3 a2 a1 a0 : K) : Yc a3 a2 a1 a0 ∈ degreeSubmodule a3 a2 a1 a0 3 := by
  simpa [Yc, weights] using gen_mem a3 a2 a1 a0 (1 : Fin 3)
theorem Zc_mem (a3 a2 a1 a0 : K) : Zc a3 a2 a1 a0 ∈ degreeSubmodule a3 a2 a1 a0 1 := by
  simpa [Zc, weights] using gen_mem a3 a2 a1 a0 (2 : Fin 3)

theorem weighted_overlap_inverse (a3 a2 a1 a0 : K) :
    WeightedOverlapScratch.fOverG (degreeSubmodule a3 a2 a1 a0)
      (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) *
    WeightedOverlapScratch.gOverF (degreeSubmodule a3 a2 a1 a0)
      (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0) = 1 :=
  WeightedOverlapScratch.fOverG_mul_gOverF (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)

theorem weighted_overlap_coordinates (a3 a2 a1 a0 : K) :
    WeightedOverlapScratch.wOverF (degreeSubmodule a3 a2 a1 a0)
      (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
      (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0) *
    (WeightedOverlapScratch.fOverG (degreeSubmodule a3 a2 a1 a0)
      (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0))^3 =
    WeightedOverlapScratch.yOverG (degreeSubmodule a3 a2 a1 a0)
      (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
      (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
      (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0) :=
  WeightedOverlapScratch.w_mul_x_cube_eq_y (degreeSubmodule a3 a2 a1 a0)
    (Xc a3 a2 a1 a0) (Zc a3 a2 a1 a0)
    (Xc_mem a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)
    (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)

theorem zChart_x_image (a3 a2 a1 a0 : K) :
    WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0
      (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
        (WeightedChartIso.z_homogeneous a3 a2 a1 a0) 1
        (Xc a3 a2 a1 a0) (Xc_mem a3 a2 a1 a0)) =
      Common22WeightedProjQuotient.xbar a3 a2 a1 a0 := by
  change WeightedChartIso.weightedAwayToAffine a3 a2 a1 a0 _ = _
  rw [WeightedChartIso.weightedAwayToAffine, WeightedChartRoute.awayToOne_mk]
  exact toAffine_X a3 a2 a1 a0

theorem zChart_y_image (a3 a2 a1 a0 : K) :
    WeightedChartIso.weightedAwayEquivAffine a3 a2 a1 a0
      (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
        (WeightedChartIso.z_homogeneous a3 a2 a1 a0) 3
        (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)) =
      Common22WeightedProjQuotient.ybar a3 a2 a1 a0 := by
  change WeightedChartIso.weightedAwayToAffine a3 a2 a1 a0 _ = _
  rw [WeightedChartIso.weightedAwayToAffine, WeightedChartRoute.awayToOne_mk]
  exact toAffine_Y a3 a2 a1 a0

theorem xChart_z_image (a3 a2 a1 a0 : K) :
    WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0
      (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
        (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) 1
        (Zc a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0)) =
      Common22ExplicitChart.zbar a3 a2 a1 a0 := by
  change (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0)
    (WeightedChartRoute.awayToOne (degreeSubmodule a3 a2 a1 a0)
      (WeightedChartDehomog.quotientMap (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))
      (WeightedChartDehomog.quotientMap_f (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))
      (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
        (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) 1
        (Zc a3 a2 a1 a0) (Zc_mem a3 a2 a1 a0))) = _
  rw [WeightedChartRoute.awayToOne_mk]
  change WeightedInfinityOrdinary.toInfinity a3 a2 a1 a0 (Zc a3 a2 a1 a0) = _
  change WeightedInfinityOrdinary.specializeInfinity a3 a2 a1 a0 (MvPolynomial.X 2) = _
  exact WeightedInfinityOrdinary.specializeInfinity_Z a3 a2 a1 a0

theorem xChart_w_image (a3 a2 a1 a0 : K) :
    WeightedProjInfinityChart.xAwayEquivInfinity a3 a2 a1 a0
      (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
        (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) 3
        (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0)) =
      Common22ExplicitChart.wbar a3 a2 a1 a0 := by
  change (WeightedInfinityOrdinary.xOneEquivInfinity a3 a2 a1 a0)
    (WeightedChartRoute.awayToOne (degreeSubmodule a3 a2 a1 a0)
      (WeightedChartDehomog.quotientMap (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))
      (WeightedChartDehomog.quotientMap_f (WeightedInfinityOrdinary.xbar a3 a2 a1 a0))
      (HomogeneousLocalization.Away.mk (degreeSubmodule a3 a2 a1 a0)
        (WeightedProjInfinityChart.x_homogeneous a3 a2 a1 a0) 3
        (Yc a3 a2 a1 a0) (Yc_mem a3 a2 a1 a0))) = _
  rw [WeightedChartRoute.awayToOne_mk]
  change WeightedInfinityOrdinary.toInfinity a3 a2 a1 a0 (Yc a3 a2 a1 a0) = _
  change WeightedInfinityOrdinary.specializeInfinity a3 a2 a1 a0 (MvPolynomial.X 1) = _
  exact WeightedInfinityOrdinary.specializeInfinity_Y a3 a2 a1 a0

#print axioms zChart_x_image
#print axioms zChart_y_image
#print axioms xChart_z_image
#print axioms xChart_w_image
#print axioms WeightedOverlapScratch.zChart_x_to_common
#print axioms WeightedOverlapScratch.xChart_z_to_common
#print axioms WeightedOverlapScratch.zChart_y_to_common
#print axioms WeightedOverlapScratch.xChart_w_to_common
#print axioms WeightedOverlapScratch.fOverG_mul_gOverF
#print axioms WeightedOverlapScratch.w_mul_x_cube_eq_y
#print axioms weighted_overlap_inverse
#print axioms weighted_overlap_coordinates
end WeightedOverlapSpecialized
