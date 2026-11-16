import Common22BranchLocalParameter
import Common22BranchValuation
import Common22Domain
import Common22Eisenstein
import Mathlib.Algebra.MvPolynomial.Equiv
import Mathlib.RingTheory.DiscreteValuationRing.TFAE

/-! A finite simple branch point in the actual affine quintic chart.

The simple-root condition permits Eisenstein at `X-a` in the coefficient
polynomial ring. The resulting affine domain and the already checked
principal maximal ideal yield a DVR at this actual branch localization.

Source check: `Common22Eisenstein.lean` and `Common22Domain.lean` establish
the identical algebraic pattern for the infinity chart; pinned Mathlib's
`RingTheory/DiscreteValuationRing/TFAE.lean` supplies the principal-maximal-
ideal criterion. Stacks Project, Tag 00PD
(https://stacks.math.columbia.edu/tag/00PD) gives the corresponding
Noetherian-local-domain characterization. None of these sources supplies a
global divisor-class or Rust-law translation.
-/

set_option autoImplicit false
noncomputable section

namespace Common22BranchDomain

open MvPolynomial Common22SeparateChartMap Common22BranchStalk
  Common22BranchParameter Common22BranchLocalParameter

variable {K : Type*} [Field K]

private abbrev Inner (K : Type*) [Field K] := MvPolynomial (Fin 1) K

def innerF (a3 a2 a1 a0 : K) : Inner K :=
  X 0 ^ 5 + C a3 * X 0 ^ 3 + C a2 * X 0 ^ 2 + C a1 * X 0 + C a0

def innerQ (a3 a2 a1 a0 a : K) : Inner K :=
  X 0 ^ 4 + C a * X 0 ^ 3 + C (a ^ 2) * X 0 ^ 2 + C (a ^ 3) * X 0 + C (a ^ 4) +
  C a3 * (X 0 ^ 2 + C a * X 0 + C (a ^ 2)) + C a2 * (X 0 + C a) + C a1

theorem inner_factor (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    innerF a3 a2 a1 a0 = (X 0 - C a) * innerQ a3 a2 a1 a0 a := by
  have hc : (C (fAt a3 a2 a1 a0 a) : Inner K) = 0 := by simp [hroot]
  simp only [fAt, map_add, map_mul, map_pow] at hc
  dsimp [innerF, innerQ]
  simp only [map_pow]
  linear_combination hc

def innerEval (a : K) : Inner K →+* K :=
  eval₂Hom (RingHom.id K) (fun _ => a)

theorem innerQ_eval (a3 a2 a1 a0 a : K) :
    innerEval a (innerQ a3 a2 a1 a0 a) = derivativeAt a3 a2 a1 a0 a := by
  simp [innerEval, innerQ, derivativeAt]
  ring

theorem inner_t_prime (a : K) : Prime (X 0 - C a : Inner K) := by
  have hirr : Irreducible (X 0 - C a : Inner K) := by
    apply Irreducible.of_map (f := MvPolynomial.uniqueAlgEquiv K (Fin 1))
    simpa [MvPolynomial.uniqueAlgEquiv] using (Polynomial.prime_X_sub_C a).irreducible
  exact hirr.prime

theorem inner_t_not_dvd_q (a3 a2 a1 a0 a : K)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    ¬(X 0 - C a : Inner K) ∣ innerQ a3 a2 a1 a0 a := by
  intro h
  have hev := map_dvd (innerEval a) h
  have hzero : innerEval a (X 0 - C a : Inner K) = 0 := by simp [innerEval]
  rw [hzero, zero_dvd_iff] at hev
  exact hsimple ((innerQ_eval a3 a2 a1 a0 a).symm.trans hev)

theorem inner_equation_irreducible (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    Irreducible (Polynomial.X ^ 2 - Polynomial.C (innerF a3 a2 a1 a0)) := by
  rw [inner_factor a3 a2 a1 a0 a hroot]
  exact Common22Eisenstein.outer_quadratic_irreducible
    (X 0 - C a) (innerQ a3 a2 a1 a0 a)
    (inner_t_prime a) (inner_t_not_dvd_q a3 a2 a1 a0 a hsimple)

theorem nested_affine_equation (a3 a2 a1 a0 : K) :
    Common22Domain.nestedEquiv (affineEquation a3 a2 a1 a0) =
      Polynomial.X ^ 2 - Polynomial.C (innerF a3 a2 a1 a0) := by
  have hX1 : MvPolynomial.finSuccEquiv K 1
      (MvPolynomial.X (1 : Fin 2)) =
      Polynomial.C (MvPolynomial.X (0 : Fin 1)) := by
    simpa using (MvPolynomial.finSuccEquiv_X_succ (R := K) (j := (0 : Fin 1)))
  have hC (b : K) : MvPolynomial.finSuccEquiv K 1
      (MvPolynomial.C b) = Polynomial.C (MvPolynomial.C b) := by
    simp [MvPolynomial.finSuccEquiv_apply]
  simp [Common22Domain.nestedEquiv, affineEquation, innerF,
    MvPolynomial.renameEquiv_apply, MvPolynomial.finSuccEquiv_X_zero,
    hX1, hC]

theorem affine_ring_domain (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsDomain (AffineRing a3 a2 a1 a0) := by
  have hirr : Irreducible (affineEquation a3 a2 a1 a0) := by
    apply Irreducible.of_map (f := Common22Domain.nestedEquiv)
    rw [nested_affine_equation]
    exact inner_equation_irreducible a3 a2 a1 a0 a hroot hsimple
  exact (Ideal.Quotient.isDomain_iff_prime (affineIdeal a3 a2 a1 a0)).mpr
    (Ideal.isPrime_span_singleton_of_prime hirr.prime)

theorem affine_ybar_ne_zero (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0) :
    ybar a3 a2 a1 a0 ≠ 0 := by
  intro hy
  have hmem : (X 1 : Poly K) ∈ affineIdeal a3 a2 a1 a0 :=
    (Ideal.Quotient.eq_zero_iff_mem).mp hy
  have hdvd : affineEquation a3 a2 a1 a0 ∣ (X 1 : Poly K) := by
    simpa only [affineIdeal, Ideal.mem_span_singleton] using hmem
  have hdvd' : Common22Domain.nestedEquiv (affineEquation a3 a2 a1 a0) ∣
      Common22Domain.nestedEquiv (X 1 : Poly K) := by
    obtain ⟨q, hq⟩ := hdvd
    exact ⟨Common22Domain.nestedEquiv q, by simpa using congrArg Common22Domain.nestedEquiv hq⟩
  have hmapy : Common22Domain.nestedEquiv (X 1 : Poly K) = Polynomial.X := by
    simp [Common22Domain.nestedEquiv, MvPolynomial.renameEquiv_apply,
      MvPolynomial.finSuccEquiv_X_zero]
  rw [nested_affine_equation, hmapy] at hdvd'
  have hdeg := Polynomial.natDegree_le_of_dvd hdvd' Polynomial.X_ne_zero
  rw [Polynomial.natDegree_X_pow_sub_C, Polynomial.natDegree_X] at hdeg
  omega

theorem branch_stalk_domain (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain a3 a2 a1 a0 a hroot hsimple
  exact IsLocalization.isDomain_of_atPrime
    (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot))
    (branchIdeal a3 a2 a1 a0 a hroot)

theorem branch_stalk_y_ne_zero (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    stalkY a3 a2 a1 a0 a hroot ≠ 0 := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain a3 a2 a1 a0 a hroot hsimple
  have hinj : Function.Injective
      (algebraMap (AffineRing a3 a2 a1 a0)
        (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot))) :=
    FaithfulSMul.algebraMap_injective _ _
  intro hy
  exact affine_ybar_ne_zero a3 a2 a1 a0 a hroot
    (hinj (by simpa [stalkY] using hy))

theorem branch_stalk_dvr (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    @IsDiscreteValuationRing
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) _
      (branch_stalk_domain a3 a2 a1 a0 a hroot hsimple) := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
  have hnotfield : ¬IsField R := by
    intro hfield
    have hmzero : IsLocalRing.maximalIdeal R = ⊥ :=
      IsLocalRing.isField_iff_maximalIdeal_eq.mp hfield
    have hymem : stalkY a3 a2 a1 a0 a hroot ∈ IsLocalRing.maximalIdeal R := by
      rw [maximalIdeal_eq_span_y a3 a2 a1 a0 a hroot hsimple]
      exact Ideal.subset_span (by simp)
    have hyzero : stalkY a3 a2 a1 a0 a hroot = 0 := by
      simpa [hmzero] using hymem
    exact branch_stalk_y_ne_zero a3 a2 a1 a0 a hroot hsimple hyzero
  have hmprincipal : (IsLocalRing.maximalIdeal R).IsPrincipal := by
    rw [maximalIdeal_eq_span_y a3 a2 a1 a0 a hroot hsimple]
    infer_instance
  exact ((IsDiscreteValuationRing.TFAE R hnotfield).out 5 1 rfl rfl).mp hmprincipal

theorem branch_stalk_y_irreducible (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    Irreducible (stalkY a3 a2 a1 a0 a hroot) := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple
  exact IsDiscreteValuationRing.irreducible_of_span_eq_maximalIdeal
    (stalkY a3 a2 a1 a0 a hroot)
    (branch_stalk_y_ne_zero a3 a2 a1 a0 a hroot hsimple)
    (maximalIdeal_eq_span_y a3 a2 a1 a0 a hroot hsimple)

theorem branch_order_y_eq_one (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
    @IsDiscreteValuationRing.addVal R _
      (branch_stalk_domain a3 a2 a1 a0 a hroot hsimple)
      (branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple)
      (stalkY a3 a2 a1 a0 a hroot) = 1 := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple
  exact IsDiscreteValuationRing.addVal_uniformizer
    (branch_stalk_y_irreducible a3 a2 a1 a0 a hroot hsimple)

theorem branch_order_x_sub_a_eq_two (a3 a2 a1 a0 a : K)
    (hroot : fAt a3 a2 a1 a0 a = 0)
    (hsimple : derivativeAt a3 a2 a1 a0 a ≠ 0) :
    let A := AffineRing a3 a2 a1 a0
    let R := Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)
    @IsDiscreteValuationRing.addVal R _
      (branch_stalk_domain a3 a2 a1 a0 a hroot hsimple)
      (branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple)
      ((algebraMap A R) (xbar a3 a2 a1 a0 - algebraMap K A a)) = 2 := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDomain (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_domain a3 a2 a1 a0 a hroot hsimple
  letI : IsDiscreteValuationRing
      (Localization.AtPrime (branchIdeal a3 a2 a1 a0 a hroot)) :=
    branch_stalk_dvr a3 a2 a1 a0 a hroot hsimple
  have hrelation := Common22BranchValuation.branch_order_relation
    a3 a2 a1 a0 a hroot hsimple
  have hy := branch_order_y_eq_one a3 a2 a1 a0 a hroot hsimple
  dsimp at hrelation hy ⊢
  dsimp [stalkY] at hy
  rw [hy] at hrelation
  simpa using hrelation

#print axioms inner_factor
#print axioms innerQ_eval
#print axioms inner_t_prime
#print axioms inner_t_not_dvd_q
#print axioms inner_equation_irreducible
#print axioms nested_affine_equation
#print axioms affine_ring_domain
#print axioms affine_ybar_ne_zero
#print axioms branch_stalk_domain
#print axioms branch_stalk_y_ne_zero
#print axioms branch_stalk_dvr
#print axioms branch_stalk_y_irreducible
#print axioms branch_order_y_eq_one
#print axioms branch_order_x_sub_a_eq_two

end Common22BranchDomain
