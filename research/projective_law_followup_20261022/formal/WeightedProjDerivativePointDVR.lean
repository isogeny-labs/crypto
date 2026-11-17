import WeightedProjNonbranchDVR

/-!
# A rational point with nonzero horizontal derivative

If `f'(a) ≠ 0`, the difference quotient `q_a` is a local unit. The curve
relation then makes `y-y₀` generate the point maximal ideal. With the
previous separability domain theorem, the actual weighted-Proj stalk is a
DVR. This covers the complementary smooth-point case in characteristic two.
It does not establish formula-specific norm transport or global Pic0.

The local DVR criterion follows Stacks Project Tag 00PD and pinned Mathlib's
`IsDiscreteValuationRing.TFAE`; the projective stalk identification is the
checked `WeightedProjNonbranchZeroOrder.pointRingEquiv` (Stacks Tag 01HD).
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjDerivativePointDVR

open MvPolynomial Common22SeparateChartMap Common22BranchStalk
  Common22BranchParameter WeightedProjNonbranchZeroOrder
  WeightedProjNonbranchDVR

variable {K : Type*} [Field K]

private abbrev Stalk (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :=
  Localization.AtPrime (pointIdeal a3 a2 a1 a0 a y hcurve)

theorem pointEval_qAt (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a) :
    pointEval a3 a2 a1 a0 a y hcurve
      (qAt a3 a2 a1 a0 a) = derivativeAt a3 a2 a1 a0 a := by
  have hc (b : K) :
      pointEval a3 a2 a1 a0 a y hcurve
        (algebraMap K (AffineRing a3 a2 a1 a0) b) = b := by
    change pointPreEval a y (C b) = b
    simp [pointPreEval]
  have hx : pointEval a3 a2 a1 a0 a y hcurve
      (xbar a3 a2 a1 a0) = a := by
    simp [pointEval, xbar, pointPreEval]
  simp only [qAt, map_add, map_mul, map_pow, hx, hc, derivativeAt]
  ring

theorem stalkQ_unit (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hderiv : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsUnit (stalkQ a3 a2 a1 a0 a y hcurve) := by
  apply (IsLocalization.AtPrime.isUnit_to_map_iff
    (Stalk a3 a2 a1 a0 a y hcurve)
    (pointIdeal a3 a2 a1 a0 a y hcurve) _).mpr
  simp only [Ideal.mem_primeCompl_iff]
  intro hmem
  have hz : pointEval a3 a2 a1 a0 a y hcurve
      (qAt a3 a2 a1 a0 a) = 0 := (RingHom.mem_ker).mp hmem
  exact hderiv ((pointEval_qAt a3 a2 a1 a0 a y hcurve).symm.trans hz)

theorem maximalIdeal_eq_span_y_sub_y (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hderiv : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsLocalRing.maximalIdeal (Stalk a3 a2 a1 a0 a y hcurve) =
      Ideal.span {stalkYSubY a3 a2 a1 a0 a y hcurve} := by
  rw [maximalIdeal_coordinates a3 a2 a1 a0 a y hcurve]
  apply le_antisymm
  · apply Ideal.span_le.mpr
    intro z hz
    simp only [Set.mem_insert_iff, Set.mem_singleton_iff] at hz
    rcases hz with rfl | rfl
    · let u := (stalkQ_unit a3 a2 a1 a0 a y hcurve hderiv).unit
      have hueq : (u : Stalk a3 a2 a1 a0 a y hcurve) =
          stalkQ a3 a2 a1 a0 a y hcurve :=
        (stalkQ_unit a3 a2 a1 a0 a y hcurve hderiv).unit_spec
      have hrel := stalk_relation a3 a2 a1 a0 a y hcurve
      rw [← hueq] at hrel
      have heq : stalkXSubA a3 a2 a1 a0 a y hcurve =
          stalkYSubY a3 a2 a1 a0 a y hcurve *
            (stalkYPlusY a3 a2 a1 a0 a y hcurve * (u⁻¹ : _)) := by
        calc
          _ = (stalkXSubA a3 a2 a1 a0 a y hcurve * (u : _)) * (u⁻¹ : _) := by simp [mul_assoc]
          _ = _ := by rw [← hrel]; ring
      exact Ideal.mem_span_singleton.mpr ⟨_, heq⟩
    · exact Ideal.mem_span_singleton_self _
  · exact Ideal.span_mono (by simp)

theorem point_local_dvr (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (hderiv : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing (Stalk a3 a2 a1 a0 a y hcurve) _
      (Common22SeparableAffineDomain.point_local_domain_of_separable
        a3 a2 a1 a0 a y hcurve hsep) := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    Common22SeparableAffineDomain.affine_ring_domain_of_separable a3 a2 a1 a0 hsep
  letI : IsDomain (Stalk a3 a2 a1 a0 a y hcurve) :=
    Common22SeparableAffineDomain.point_local_domain_of_separable
      a3 a2 a1 a0 a y hcurve hsep
  let R := Stalk a3 a2 a1 a0 a y hcurve
  have hnotfield : ¬IsField R := by
    intro hfield
    have hmzero : IsLocalRing.maximalIdeal R = ⊥ :=
      IsLocalRing.isField_iff_maximalIdeal_eq.mp hfield
    have hymem : stalkYSubY a3 a2 a1 a0 a y hcurve ∈
        IsLocalRing.maximalIdeal R := by
      rw [maximalIdeal_eq_span_y_sub_y a3 a2 a1 a0 a y hcurve hderiv]
      exact Ideal.mem_span_singleton_self _
    have hyzero : stalkYSubY a3 a2 a1 a0 a y hcurve = 0 := by
      simpa [hmzero] using hymem
    exact stalkYSubY_ne_zero a3 a2 a1 a0 a y hcurve hsep hyzero
  have hmprincipal : (IsLocalRing.maximalIdeal R).IsPrincipal := by
    rw [maximalIdeal_eq_span_y_sub_y a3 a2 a1 a0 a y hcurve hderiv]
    infer_instance
  exact ((IsDiscreteValuationRing.TFAE R hnotfield).out 5 1 rfl rfl).mp hmprincipal

theorem projective_point_dvr (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable)
    (hderiv : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing (ProjPointRing a3 a2 a1 a0 a y hcurve) _
      (Common22SeparableAffineDomain.projective_point_domain_of_separable
        a3 a2 a1 a0 a y hcurve hsep) := by
  letI : IsDomain (Stalk a3 a2 a1 a0 a y hcurve) :=
    Common22SeparableAffineDomain.point_local_domain_of_separable
      a3 a2 a1 a0 a y hcurve hsep
  letI : IsDiscreteValuationRing (Stalk a3 a2 a1 a0 a y hcurve) :=
    point_local_dvr a3 a2 a1 a0 a y hcurve hsep hderiv
  letI : IsDomain (ProjPointRing a3 a2 a1 a0 a y hcurve) :=
    Common22SeparableAffineDomain.projective_point_domain_of_separable
      a3 a2 a1 a0 a y hcurve hsep
  exact IsDiscreteValuationRing.RingEquivClass.isDiscreteValuationRing
    (pointRingEquiv a3 a2 a1 a0 a y hcurve).symm

#print axioms pointEval_qAt
#print axioms stalkQ_unit
#print axioms maximalIdeal_eq_span_y_sub_y
#print axioms point_local_dvr
#print axioms projective_point_dvr

end WeightedProjDerivativePointDVR
