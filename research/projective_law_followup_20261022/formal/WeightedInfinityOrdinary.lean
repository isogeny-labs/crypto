import WeightedRingGraded
import Common22ExplicitChart
import Common22WeightedProjKernel
import Mathlib.Tactic.Ring

set_option autoImplicit false
noncomputable section
namespace WeightedInfinityOrdinary
open MvPolynomial Common22WeightedProjHead Common22WeightedProjQuotient
variable {K : Type*} [Field K]

abbrev InfinityRing (a3 a2 a1 a0 : K) :=
  Common22ExplicitChart.ChartRing a3 a2 a1 a0

def specializeInfinity (a3 a2 a1 a0 : K) :
    Poly K →ₐ[K] InfinityRing a3 a2 a1 a0 :=
  MvPolynomial.aeval (fun i : Fin 3 =>
    if i = 0 then 1 else if i = 1 then
      Common22ExplicitChart.wbar a3 a2 a1 a0 else
      Common22ExplicitChart.zbar a3 a2 a1 a0)

theorem specializeInfinity_X (a3 a2 a1 a0 : K) :
    specializeInfinity a3 a2 a1 a0 (X 0) = 1 := by
  simp [specializeInfinity]

theorem specializeInfinity_Y (a3 a2 a1 a0 : K) :
    specializeInfinity a3 a2 a1 a0 (X 1) =
      Common22ExplicitChart.wbar a3 a2 a1 a0 := by
  simp [specializeInfinity]

theorem specializeInfinity_Z (a3 a2 a1 a0 : K) :
    specializeInfinity a3 a2 a1 a0 (X 2) =
      Common22ExplicitChart.zbar a3 a2 a1 a0 := by
  simp [specializeInfinity]

theorem specializeInfinity_equation (a3 a2 a1 a0 : K) :
    specializeInfinity a3 a2 a1 a0 (equation a3 a2 a1 a0) = 0 := by
  have hc := Common22ExplicitChart.quotient_chart_relation a3 a2 a1 a0
  simp only [equation, map_sub, map_add, map_mul, map_pow,
    specializeInfinity_X, specializeInfinity_Y, specializeInfinity_Z,
    map_one, one_pow, one_mul, map_C]
  -- The remaining equality is the explicit infinity relation, expanded in the quotient.
  simp [Common22ExplicitChart.ubar, Common22ExplicitChart.unitFactor,
    Common22ExplicitChart.zbar, Common22ExplicitChart.z, Common22ExplicitChart.wbar,
    Common22ExplicitChart.w] at hc ⊢
  have hC (a : K) :
      (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0))
        (C a) = algebraMap K (InfinityRing a3 a2 a1 a0) a := rfl
  simp only [hC] at hc
  rw [hc]
  ring

def toInfinity (a3 a2 a1 a0 : K) :
    WeightedRing a3 a2 a1 a0 →ₐ[K] InfinityRing a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0)
    (specializeInfinity a3 a2 a1 a0) (by
      intro p hp
      rw [Common22WeightedProjHead.relationIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨r, rfl⟩ := hp
      simp [specializeInfinity_equation])

def xbar (a3 a2 a1 a0 : K) : WeightedRing a3 a2 a1 a0 :=
  Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) (X 0)

def xMinusOneIdeal (a3 a2 a1 a0 : K) : Ideal (WeightedRing a3 a2 a1 a0) :=
  Ideal.span {xbar a3 a2 a1 a0 - 1}

abbrev XOneRing (a3 a2 a1 a0 : K) :=
  WeightedRing a3 a2 a1 a0 ⧸ xMinusOneIdeal a3 a2 a1 a0

def weightedToXOne (a3 a2 a1 a0 : K) :
    WeightedRing a3 a2 a1 a0 →ₐ[K] XOneRing a3 a2 a1 a0 :=
  Ideal.Quotient.mkₐ K (xMinusOneIdeal a3 a2 a1 a0)

def polyToXOne (a3 a2 a1 a0 : K) :
    Poly K →ₐ[K] XOneRing a3 a2 a1 a0 :=
  (weightedToXOne a3 a2 a1 a0).comp
    (Ideal.Quotient.mkₐ K (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0))

theorem x_eq_one (a3 a2 a1 a0 : K) :
    polyToXOne a3 a2 a1 a0 (X 0) = 1 := by
  have h : (Ideal.Quotient.mk (xMinusOneIdeal a3 a2 a1 a0))
      (xbar a3 a2 a1 a0 - 1) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  have h' : (weightedToXOne a3 a2 a1 a0) (xbar a3 a2 a1 a0) - 1 = 0 := by
    simpa [weightedToXOne, xbar] using h
  simpa [polyToXOne, weightedToXOne, xbar] using sub_eq_zero.mp h'

theorem weighted_relation_zero (a3 a2 a1 a0 : K) :
    polyToXOne a3 a2 a1 a0 (Common22WeightedProjHead.equation a3 a2 a1 a0) = 0 := by
  have h : (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0))
      (Common22WeightedProjHead.equation a3 a2 a1 a0) = 0 :=
    Ideal.Quotient.eq_zero_iff_mem.mpr (Ideal.subset_span (by simp))
  simpa [polyToXOne, weightedToXOne] using
    congrArg (weightedToXOne a3 a2 a1 a0) h

def infinityToXOnePoly (a3 a2 a1 a0 : K) :
    Common22ExplicitChart.ChartPoly K →ₐ[K] XOneRing a3 a2 a1 a0 :=
  MvPolynomial.aeval (fun i : Fin 2 =>
    if i = 0 then polyToXOne a3 a2 a1 a0 (X 2)
    else polyToXOne a3 a2 a1 a0 (X 1))

theorem infinity_relation_zero (a3 a2 a1 a0 : K) :
    infinityToXOnePoly a3 a2 a1 a0
      (Common22ExplicitChart.equation a3 a2 a1 a0) = 0 := by
  have hf := weighted_relation_zero a3 a2 a1 a0
  have hx := x_eq_one a3 a2 a1 a0
  simp only [Common22WeightedProjHead.equation, map_sub, map_add,
    map_mul, map_pow] at hf
  rw [hx] at hf
  have hz : infinityToXOnePoly a3 a2 a1 a0 (X 0) =
      polyToXOne a3 a2 a1 a0 (X 2) := by simp [infinityToXOnePoly]
  have hw : infinityToXOnePoly a3 a2 a1 a0 (X 1) =
      polyToXOne a3 a2 a1 a0 (X 1) := by simp [infinityToXOnePoly]
  have hC (a : K) : infinityToXOnePoly a3 a2 a1 a0 (C a) =
      polyToXOne a3 a2 a1 a0 (C a) := by
    simp [infinityToXOnePoly, polyToXOne, weightedToXOne]
  simp only [Common22ExplicitChart.equation, Common22ExplicitChart.unitFactor,
    Common22ExplicitChart.z, Common22ExplicitChart.w,
    map_sub, map_add, map_mul, map_pow]
  rw [hz, hw, hC a3, hC a2, hC a1, hC a0]
  simp only [map_one, mul_one, one_mul] at hf ⊢
  ring_nf at hf ⊢
  assumption

def infinityToXOne (a3 a2 a1 a0 : K) :
    InfinityRing a3 a2 a1 a0 →ₐ[K] XOneRing a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (Common22ExplicitChart.relationIdeal a3 a2 a1 a0)
    (infinityToXOnePoly a3 a2 a1 a0) (by
      intro p hp
      rw [Common22ExplicitChart.relationIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨r, rfl⟩ := hp
      rw [map_mul, infinity_relation_zero]
      exact zero_mul ((infinityToXOnePoly a3 a2 a1 a0) r))

theorem toInfinity_X (a3 a2 a1 a0 : K) :
    toInfinity a3 a2 a1 a0 (xbar a3 a2 a1 a0) = 1 := by
  change specializeInfinity a3 a2 a1 a0 (X 0) = 1
  exact specializeInfinity_X a3 a2 a1 a0

def xOneToInfinity (a3 a2 a1 a0 : K) :
    XOneRing a3 a2 a1 a0 →ₐ[K] InfinityRing a3 a2 a1 a0 :=
  Ideal.Quotient.liftₐ (xMinusOneIdeal a3 a2 a1 a0)
    (toInfinity a3 a2 a1 a0) (by
      intro p hp
      rw [xMinusOneIdeal, Ideal.mem_span_singleton] at hp
      obtain ⟨r, rfl⟩ := hp
      simp [toInfinity_X])

theorem infinityToXOne_mk (a3 a2 a1 a0 : K)
    (p : Common22ExplicitChart.ChartPoly K) :
    infinityToXOne a3 a2 a1 a0
      (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) p) =
    infinityToXOnePoly a3 a2 a1 a0 p := rfl

theorem xOneToInfinity_mk (a3 a2 a1 a0 : K)
    (p : WeightedRing a3 a2 a1 a0) :
    xOneToInfinity a3 a2 a1 a0
      (Ideal.Quotient.mk (xMinusOneIdeal a3 a2 a1 a0) p) =
    toInfinity a3 a2 a1 a0 p := rfl

theorem toInfinity_mk (a3 a2 a1 a0 : K) (p : Poly K) :
    toInfinity a3 a2 a1 a0
      (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) p) =
    specializeInfinity a3 a2 a1 a0 p := rfl

theorem infinity_roundtrip (a3 a2 a1 a0 : K) :
    (xOneToInfinity a3 a2 a1 a0).comp (infinityToXOne a3 a2 a1 a0) =
      AlgHom.id K (InfinityRing a3 a2 a1 a0) := by
  apply Ideal.Quotient.algHom_ext
  apply MvPolynomial.algHom_ext
  intro i
  fin_cases i
  · change xOneToInfinity a3 a2 a1 a0
      (infinityToXOne a3 a2 a1 a0
        (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) (X 0))) =
      Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) (X 0)
    rw [infinityToXOne_mk]
    simp only [infinityToXOnePoly, MvPolynomial.aeval_X, ite_true]
    change xOneToInfinity a3 a2 a1 a0
      (polyToXOne a3 a2 a1 a0 (X 2)) = _
    change toInfinity a3 a2 a1 a0
      (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) (X 2)) = _
    rw [toInfinity_mk, specializeInfinity_Z]
    rfl
  · change xOneToInfinity a3 a2 a1 a0
      (infinityToXOne a3 a2 a1 a0
        (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) (X 1))) =
      Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) (X 1)
    rw [infinityToXOne_mk]
    simp only [infinityToXOnePoly, MvPolynomial.aeval_X]
    change xOneToInfinity a3 a2 a1 a0
      (polyToXOne a3 a2 a1 a0 (X 1)) = _
    change toInfinity a3 a2 a1 a0
      (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) (X 1)) = _
    rw [toInfinity_mk, specializeInfinity_Y]
    rfl

theorem xOne_roundtrip (a3 a2 a1 a0 : K) :
    (infinityToXOne a3 a2 a1 a0).comp (xOneToInfinity a3 a2 a1 a0) =
      AlgHom.id K (XOneRing a3 a2 a1 a0) := by
  apply Ideal.Quotient.algHom_ext
  apply Ideal.Quotient.algHom_ext
  apply MvPolynomial.algHom_ext
  intro i
  fin_cases i
  · change infinityToXOne a3 a2 a1 a0
      (toInfinity a3 a2 a1 a0
        (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) (X 0))) =
      polyToXOne a3 a2 a1 a0 (X 0)
    rw [toInfinity_mk, specializeInfinity_X, map_one, x_eq_one]
  · change infinityToXOne a3 a2 a1 a0
      (toInfinity a3 a2 a1 a0
        (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) (X 1))) =
      polyToXOne a3 a2 a1 a0 (X 1)
    rw [toInfinity_mk, specializeInfinity_Y]
    change infinityToXOne a3 a2 a1 a0
      (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) (X 1)) = _
    rw [infinityToXOne_mk]
    simp [infinityToXOnePoly]
  · change infinityToXOne a3 a2 a1 a0
      (toInfinity a3 a2 a1 a0
        (Ideal.Quotient.mk (Common22WeightedProjHead.relationIdeal a3 a2 a1 a0) (X 2))) =
      polyToXOne a3 a2 a1 a0 (X 2)
    rw [toInfinity_mk, specializeInfinity_Z]
    change infinityToXOne a3 a2 a1 a0
      (Ideal.Quotient.mk (Common22ExplicitChart.relationIdeal a3 a2 a1 a0) (X 0)) = _
    rw [infinityToXOne_mk]
    simp [infinityToXOnePoly]

def xOneEquivInfinity (a3 a2 a1 a0 : K) :
    XOneRing a3 a2 a1 a0 ≃ₐ[K] InfinityRing a3 a2 a1 a0 :=
  AlgEquiv.ofAlgHom (xOneToInfinity a3 a2 a1 a0)
    (infinityToXOne a3 a2 a1 a0)
    (infinity_roundtrip a3 a2 a1 a0) (xOne_roundtrip a3 a2 a1 a0)

#print axioms xOneEquivInfinity

#print axioms specializeInfinity_equation
end WeightedInfinityOrdinary
