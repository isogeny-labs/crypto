import Common22Eisenstein
import Common22ExplicitChart
import Mathlib.RingTheory.Polynomial.UniqueFactorization
import Mathlib.Algebra.MvPolynomial.Equiv
import Mathlib.RingTheory.DiscreteValuationRing.TFAE

/-! Transport the nested Eisenstein polynomial to the explicit chart ring. -/

set_option autoImplicit false

noncomputable section

namespace Common22Domain

variable {K : Type*} [Field K]

/-- Separate `w` as the outer polynomial variable by exchanging the two
variables before the standard `Fin.succ` equivalence. -/
def nestedEquiv : MvPolynomial (Fin 2) K ≃ₐ[K]
    Polynomial (MvPolynomial (Fin 1) K) :=
  (MvPolynomial.renameEquiv K (Equiv.swap (0 : Fin 2) 1)).trans
    (MvPolynomial.finSuccEquiv K 1)

theorem nested_equation (a3 a2 a1 a0 : K) :
    nestedEquiv (Common22ExplicitChart.equation a3 a2 a1 a0) =
      Polynomial.X ^ 2 - Polynomial.C
        ((MvPolynomial.X 0 : MvPolynomial (Fin 1) K) *
          Common22Eisenstein.innerU a3 a2 a1 a0) := by
  have hX1 : MvPolynomial.finSuccEquiv K 1
      (MvPolynomial.X (1 : Fin 2)) =
      Polynomial.C (MvPolynomial.X (0 : Fin 1)) := by
    simpa using (MvPolynomial.finSuccEquiv_X_succ (R := K) (j := (0 : Fin 1)))
  have hC (a : K) : MvPolynomial.finSuccEquiv K 1
      (MvPolynomial.C a) = Polynomial.C (MvPolynomial.C a) := by
    simp [MvPolynomial.finSuccEquiv_apply]
  simp [nestedEquiv, Common22ExplicitChart.equation,
    Common22ExplicitChart.unitFactor, Common22ExplicitChart.z,
    Common22ExplicitChart.w, Common22Eisenstein.innerU,
    MvPolynomial.renameEquiv_apply,
    MvPolynomial.finSuccEquiv_X_zero, hX1, hC]

theorem chart_equation_irreducible (a3 a2 a1 a0 : K) :
    Irreducible (Common22ExplicitChart.equation a3 a2 a1 a0) := by
  apply Irreducible.of_map (f := nestedEquiv)
  rw [nested_equation]
  exact Common22Eisenstein.nested_chart_irreducible a3 a2 a1 a0

theorem chart_ring_domain (a3 a2 a1 a0 : K) :
    IsDomain (Common22ExplicitChart.ChartRing a3 a2 a1 a0) := by
  have hprime : Prime (Common22ExplicitChart.equation a3 a2 a1 a0) :=
    (chart_equation_irreducible a3 a2 a1 a0).prime
  exact (Ideal.Quotient.isDomain_iff_prime
    (Common22ExplicitChart.relationIdeal a3 a2 a1 a0)).mpr
      (Ideal.isPrime_span_singleton_of_prime hprime)

instance chart_ring_isDomain (a3 a2 a1 a0 : K) :
    IsDomain (Common22ExplicitChart.ChartRing a3 a2 a1 a0) :=
  chart_ring_domain a3 a2 a1 a0

theorem chart_wbar_ne_zero (a3 a2 a1 a0 : K) :
    Common22ExplicitChart.wbar a3 a2 a1 a0 ≠ 0 := by
  intro hw
  have hmem : Common22ExplicitChart.w ∈
      Common22ExplicitChart.relationIdeal a3 a2 a1 a0 := by
    exact (Ideal.Quotient.eq_zero_iff_mem).mp hw
  have hdvd : Common22ExplicitChart.equation a3 a2 a1 a0 ∣
      Common22ExplicitChart.w := by
    simpa only [Common22ExplicitChart.relationIdeal, Ideal.mem_span_singleton] using hmem
  have hdvd' : nestedEquiv (Common22ExplicitChart.equation a3 a2 a1 a0) ∣
      nestedEquiv (Common22ExplicitChart.w : MvPolynomial (Fin 2) K) := by
    obtain ⟨q, hq⟩ := hdvd
    exact ⟨nestedEquiv q, by simpa using congrArg nestedEquiv hq⟩
  have hmapw : nestedEquiv (Common22ExplicitChart.w : MvPolynomial (Fin 2) K) =
      Polynomial.X := by
    simp [nestedEquiv, Common22ExplicitChart.w,
      MvPolynomial.renameEquiv_apply, MvPolynomial.finSuccEquiv_X_zero]
  rw [nested_equation, hmapw] at hdvd'
  have hdeg := Polynomial.natDegree_le_of_dvd hdvd' Polynomial.X_ne_zero
  rw [Polynomial.natDegree_X_pow_sub_C,
    Polynomial.natDegree_X] at hdeg
  omega

theorem origin_stalk_domain (a3 a2 a1 a0 : K) :
    IsDomain (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) := by
  exact IsLocalization.isDomain_of_atPrime
    (Common22ExplicitChart.OriginStalk a3 a2 a1 a0)
    (Common22ExplicitChart.originIdeal a3 a2 a1 a0)

instance origin_stalk_isDomain (a3 a2 a1 a0 : K) :
    IsDomain (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) :=
  origin_stalk_domain a3 a2 a1 a0

theorem stalkW_ne_zero (a3 a2 a1 a0 : K) :
    Common22ExplicitChart.stalkW a3 a2 a1 a0 ≠ 0 := by
  have hinj : Function.Injective
      (algebraMap (Common22ExplicitChart.ChartRing a3 a2 a1 a0)
        (Common22ExplicitChart.OriginStalk a3 a2 a1 a0)) :=
    FaithfulSMul.algebraMap_injective _ _
  intro hw
  exact chart_wbar_ne_zero a3 a2 a1 a0 (hinj (by simpa [Common22ExplicitChart.stalkW] using hw))

theorem origin_stalk_dvr (a3 a2 a1 a0 : K) :
    IsDiscreteValuationRing (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) := by
  have hnotfield : ¬IsField (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) := by
    intro hfield
    have hmzero : IsLocalRing.maximalIdeal
        (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) = ⊥ :=
      IsLocalRing.isField_iff_maximalIdeal_eq.mp hfield
    have hwmem : Common22ExplicitChart.stalkW a3 a2 a1 a0 ∈
        IsLocalRing.maximalIdeal (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) := by
      rw [Common22ExplicitChart.originStalk_maximalIdeal_eq_span_w]
      exact Ideal.subset_span (by simp)
    have hwzero : Common22ExplicitChart.stalkW a3 a2 a1 a0 = 0 := by
      simpa [hmzero] using hwmem
    exact stalkW_ne_zero a3 a2 a1 a0 hwzero
  have hmprincipal : (IsLocalRing.maximalIdeal
      (Common22ExplicitChart.OriginStalk a3 a2 a1 a0)).IsPrincipal := by
    rw [Common22ExplicitChart.originStalk_maximalIdeal_eq_span_w]
    infer_instance
  exact ((IsDiscreteValuationRing.TFAE
    (Common22ExplicitChart.OriginStalk a3 a2 a1 a0) hnotfield).out 5 1 rfl rfl).mp hmprincipal

/-- `w` is an irreducible uniformizer of the explicitly constructed affine
chart localization. Identifying this ring with the projective infinity stalk
is a separate geometric obligation. -/
theorem stalkW_irreducible (a3 a2 a1 a0 : K) :
    Irreducible (Common22ExplicitChart.stalkW a3 a2 a1 a0) := by
  exact IsDiscreteValuationRing.irreducible_of_span_eq_maximalIdeal
    (Common22ExplicitChart.stalkW a3 a2 a1 a0)
    (stalkW_ne_zero a3 a2 a1 a0)
    (Common22ExplicitChart.originStalk_maximalIdeal_eq_span_w a3 a2 a1 a0)

end Common22Domain

#print axioms Common22Domain.nested_equation
#print axioms Common22Domain.chart_equation_irreducible
#print axioms Common22Domain.chart_ring_domain
#print axioms Common22Domain.chart_wbar_ne_zero
#print axioms Common22Domain.origin_stalk_domain
#print axioms Common22Domain.stalkW_ne_zero
#print axioms Common22Domain.origin_stalk_dvr
#print axioms Common22Domain.stalkW_irreducible
