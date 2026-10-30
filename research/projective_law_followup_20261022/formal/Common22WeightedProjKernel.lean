import Common22WeightedProjQuotient
import Mathlib.Tactic.Ring

set_option autoImplicit false
noncomputable section
namespace Common22WeightedProjKernel
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
variable {K : Type*} [Field K]

def zbar (a3 a2 a1 a0 : K) : WeightedRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 2)

def zMinusOneIdeal (a3 a2 a1 a0 : K) : Ideal (WeightedRing a3 a2 a1 a0) :=
  Ideal.span {zbar a3 a2 a1 a0 - 1}

abbrev ChartRing (a3 a2 a1 a0 : K) :=
  WeightedRing a3 a2 a1 a0 ⧸ zMinusOneIdeal a3 a2 a1 a0

def weightedToChart (a3 a2 a1 a0 : K) :
    WeightedRing a3 a2 a1 a0 →ₐ[K] ChartRing a3 a2 a1 a0 :=
  Ideal.Quotient.mkₐ K (zMinusOneIdeal a3 a2 a1 a0)

def polyToChart (a3 a2 a1 a0 : K) :
    Poly K →ₐ[K] ChartRing a3 a2 a1 a0 :=
  (weightedToChart a3 a2 a1 a0).comp
    (Ideal.Quotient.mkₐ K (relationIdeal a3 a2 a1 a0))

theorem z_eq_one (a3 a2 a1 a0 : K) :
    polyToChart a3 a2 a1 a0 (X 2) = 1 := by
  have h : (Ideal.Quotient.mk (zMinusOneIdeal a3 a2 a1 a0))
      (zbar a3 a2 a1 a0 - 1) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  have h' : (weightedToChart a3 a2 a1 a0) (zbar a3 a2 a1 a0) - 1 = 0 := by
    simpa [weightedToChart, zbar] using h
  simpa [polyToChart, weightedToChart, zbar] using sub_eq_zero.mp h'

theorem relation_zero (a3 a2 a1 a0 : K) :
    polyToChart a3 a2 a1 a0 (equation a3 a2 a1 a0) = 0 := by
  have h : (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0))
      (equation a3 a2 a1 a0) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  simpa [polyToChart, weightedToChart] using
    congrArg (weightedToChart a3 a2 a1 a0) h

def affineToChartPoly (a3 a2 a1 a0 : K) :
    AffinePoly K →ₐ[K] ChartRing a3 a2 a1 a0 :=
  MvPolynomial.aeval (fun i : Fin 2 =>
    if i = 0 then polyToChart a3 a2 a1 a0 (X 0)
    else polyToChart a3 a2 a1 a0 (X 1))

theorem affine_relation_zero (a3 a2 a1 a0 : K) :
    affineToChartPoly a3 a2 a1 a0
      (affineEquation a3 a2 a1 a0) = 0 := by
  have hf := relation_zero a3 a2 a1 a0
  have hz := z_eq_one a3 a2 a1 a0
  simp only [equation, map_sub, map_add, map_mul, map_pow] at hf
  rw [hz] at hf
  have hx : affineToChartPoly a3 a2 a1 a0 (X 0) =
      polyToChart a3 a2 a1 a0 (X 0) := by simp [affineToChartPoly]
  have hy : affineToChartPoly a3 a2 a1 a0 (X 1) =
      polyToChart a3 a2 a1 a0 (X 1) := by simp [affineToChartPoly]
  have hc (a : K) : affineToChartPoly a3 a2 a1 a0 (C a) =
      polyToChart a3 a2 a1 a0 (C a)  := by simp [affineToChartPoly, polyToChart, weightedToChart]
  simp only [affineEquation, map_sub, map_add, map_mul, map_pow]
  rw [hx, hy, hc a3, hc a2, hc a1, hc a0]
  ring_nf at hf ⊢
  assumption

def affineToChart (a3 a2 a1 a0 : K) :
    AffineRing a3 a2 a1 a0 →ₐ[K] ChartRing a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (affineIdeal a3 a2 a1 a0)
    (affineToChartPoly a3 a2 a1 a0) (by
      intro p hp
      rw [affineIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨q, rfl⟩ := hp
      rw [map_mul, affine_relation_zero]
      exact zero_mul ((affineToChartPoly a3 a2 a1 a0) q))

def chartToAffine (a3 a2 a1 a0 : K) :
    ChartRing a3 a2 a1 a0 →ₐ[K] AffineRing a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (zMinusOneIdeal a3 a2 a1 a0)
    (toAffine a3 a2 a1 a0) (by
      intro p hp
      rw [zMinusOneIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨q, rfl⟩ := hp
      simp only [map_mul, map_sub, map_one, zbar, toAffine_Z, sub_self, zero_mul])


theorem affineToChart_mk (a3 a2 a1 a0 : K) (p : AffinePoly K) :
    affineToChart a3 a2 a1 a0
      (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) p) =
      affineToChartPoly a3 a2 a1 a0 p := rfl

theorem chartToAffine_mk (a3 a2 a1 a0 : K)
    (w : WeightedRing a3 a2 a1 a0) :
    chartToAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (zMinusOneIdeal a3 a2 a1 a0) w) =
      toAffine a3 a2 a1 a0 w := rfl

theorem affine_roundtrip (a3 a2 a1 a0 : K) :
    (chartToAffine a3 a2 a1 a0).comp (affineToChart a3 a2 a1 a0) =
      AlgHom.id K (AffineRing a3 a2 a1 a0) := by
  apply Ideal.Quotient.algHom_ext
  apply MvPolynomial.algHom_ext
  intro i
  fin_cases i
  · change chartToAffine a3 a2 a1 a0
      (affineToChart a3 a2 a1 a0
        (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 0))) =
      Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 0)
    rw [affineToChart_mk]
    simp only [affineToChartPoly, MvPolynomial.aeval_X, ite_eq_left]
    change toAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 0)) = _
    exact toAffine_X a3 a2 a1 a0
  · change chartToAffine a3 a2 a1 a0
      (affineToChart a3 a2 a1 a0
        (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 1))) =
      Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 1)
    rw [affineToChart_mk]
    simp only [affineToChartPoly, MvPolynomial.aeval_X]
    change toAffine a3 a2 a1 a0
      (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 1)) = _
    exact toAffine_Y a3 a2 a1 a0


theorem chart_roundtrip (a3 a2 a1 a0 : K) :
    (affineToChart a3 a2 a1 a0).comp (chartToAffine a3 a2 a1 a0) =
      AlgHom.id K (ChartRing a3 a2 a1 a0) := by
  apply Ideal.Quotient.algHom_ext
  apply Ideal.Quotient.algHom_ext
  apply MvPolynomial.algHom_ext
  intro i
  fin_cases i
  · change affineToChart a3 a2 a1 a0
      (toAffine a3 a2 a1 a0
        (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 0))) =
      polyToChart a3 a2 a1 a0 (X 0)
    rw [toAffine_X]
    change affineToChart a3 a2 a1 a0
      (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 0)) = _
    rw [affineToChart_mk]
    simp [affineToChartPoly, polyToChart]
  · change affineToChart a3 a2 a1 a0
      (toAffine a3 a2 a1 a0
        (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 1))) =
      polyToChart a3 a2 a1 a0 (X 1)
    rw [toAffine_Y]
    change affineToChart a3 a2 a1 a0
      (Ideal.Quotient.mk (affineIdeal a3 a2 a1 a0) (X 1)) = _
    rw [affineToChart_mk]
    simp [affineToChartPoly, polyToChart]
  · change affineToChart a3 a2 a1 a0
      (toAffine a3 a2 a1 a0
        (Ideal.Quotient.mk (relationIdeal a3 a2 a1 a0) (X 2))) =
      polyToChart a3 a2 a1 a0 (X 2)
    rw [toAffine_Z, map_one, z_eq_one]


def chartEquivAffine (a3 a2 a1 a0 : K) :
    ChartRing a3 a2 a1 a0 ≃ₐ[K] AffineRing a3 a2 a1 a0 :=
  AlgEquiv.ofAlgHom (chartToAffine a3 a2 a1 a0)
    (affineToChart a3 a2 a1 a0)
    (affine_roundtrip a3 a2 a1 a0) (chart_roundtrip a3 a2 a1 a0)

theorem chartToAffine_injective (a3 a2 a1 a0 : K) :
    Function.Injective (chartToAffine a3 a2 a1 a0) := by
  intro x y h
  have hh := congrArg (affineToChart a3 a2 a1 a0) h
  simpa only [← AlgHom.comp_apply, chart_roundtrip, AlgHom.id_apply] using hh

theorem exact_kernel (a3 a2 a1 a0 : K) :
    RingHom.ker (toAffine a3 a2 a1 a0).toRingHom =
      zMinusOneIdeal a3 a2 a1 a0 := by
  apply le_antisymm
  · intro w hw
    rw [RingHom.mem_ker] at hw
    have h : chartToAffine a3 a2 a1 a0
        (Ideal.Quotient.mk (zMinusOneIdeal a3 a2 a1 a0) w) = 0 := by
      change (toAffine a3 a2 a1 a0) w = 0
      exact hw
    have hz : (Ideal.Quotient.mk (zMinusOneIdeal a3 a2 a1 a0)) w = 0 :=
      (chartToAffine_injective a3 a2 a1 a0) h
    exact Ideal.Quotient.eq_zero_iff_mem.mp hz
  · intro w hw
    rw [zMinusOneIdeal, Ideal.mem_span_singleton] at hw
    obtain ⟨q, rfl⟩ := hw
    simp [RingHom.mem_ker, zbar, toAffine_Z]

#print axioms affine_relation_zero
#print axioms affine_roundtrip
#print axioms chart_roundtrip
#print axioms exact_kernel

end Common22WeightedProjKernel
