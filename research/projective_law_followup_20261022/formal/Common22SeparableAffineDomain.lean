import Common22AffineOddDomain
import WeightedProjQuinticHead
import Mathlib.RingTheory.Polynomial.UniqueFactorization

/-!
# Affine quintic domain from separability

A separable monic quintic has a prime factor occurring once. Transport that
factor through the one-variable MvPolynomial equivalence, then apply the
checked Eisenstein domain theorem to the actual affine quotient and the
actual weighted-Proj point stalk. This proves the domain gate, not the
principal-maximal-ideal or DVR gate.
-/

set_option autoImplicit false
noncomputable section

namespace Common22SeparableAffineDomain

open MvPolynomial Polynomial Common22SeparateChartMap Common22BranchStalk
  WeightedProjNonbranchZeroOrder

variable {K : Type*} [Field K]

private abbrev Inner (K : Type*) [Field K] := MvPolynomial (Fin 1) K

private abbrev q (a3 a2 a1 a0 : K) : Polynomial K :=
  WeightedProjSmoothPoints.quintic a3 a2 a1 a0

private def e : Inner K ≃ₐ[K] Polynomial K :=
  MvPolynomial.uniqueAlgEquiv K (Fin 1)

theorem innerF_equiv (a3 a2 a1 a0 : K) :
    e (Common22BranchDomain.innerF a3 a2 a1 a0) =
      q a3 a2 a1 a0 := by
  simp [e, q, Common22BranchDomain.innerF,
    WeightedProjSmoothPoints.quintic, MvPolynomial.uniqueAlgEquiv_apply]

theorem quintic_natDegree (a3 a2 a1 a0 : K) :
    (q a3 a2 a1 a0).natDegree = 5 := by
  apply Polynomial.natDegree_eq_of_le_of_coeff_ne_zero
    (WeightedProjQuinticHead.quintic_natDegree_le_five a3 a2 a1 a0)
  rw [WeightedProjQuinticHead.quintic_coeff_five]
  exact one_ne_zero

theorem quintic_not_unit (a3 a2 a1 a0 : K) :
    ¬IsUnit (q a3 a2 a1 a0) := by
  intro hu
  have hd : (q a3 a2 a1 a0).degree = 0 :=
    Polynomial.isUnit_iff_degree_eq_zero.mp hu
  have hn : (q a3 a2 a1 a0).natDegree = 0 := by
    simp [Polynomial.natDegree, hd]
  have hfive := quintic_natDegree a3 a2 a1 a0
  omega

/-- The witness is extracted from a separable quintic, with no supplied
rational root or factorization. -/
theorem exists_simple_prime_factor (a3 a2 a1 a0 : K)
    (hsep : (q a3 a2 a1 a0).Separable) :
    ∃ t u : Inner K,
      Common22BranchDomain.innerF a3 a2 a1 a0 = t * u ∧
      Prime t ∧ ¬t ∣ u := by
  obtain ⟨g, _, hg, hdiv⟩ :=
    Polynomial.exists_monic_irreducible_factor
      (q a3 a2 a1 a0) (quintic_not_unit a3 a2 a1 a0)
  obtain ⟨v, hv⟩ := hdiv
  let t : Inner K := (e (K := K)).symm g
  let u : Inner K := (e (K := K)).symm v
  have hfactor : Common22BranchDomain.innerF a3 a2 a1 a0 = t * u := by
    apply (e (K := K)).injective
    rw [map_mul, (e (K := K)).apply_symm_apply,
      (e (K := K)).apply_symm_apply, innerF_equiv]
    exact hv
  have ht : Prime t := by
    have hi : Irreducible t := by
      apply Irreducible.of_map (f := e (K := K))
      simpa [t] using hg
    exact hi.prime
  have hu : ¬t ∣ u := by
    intro htu
    have hdu : g ∣ (e (K := K)) u := by
      obtain ⟨w, hw⟩ := htu
      refine ⟨(e (K := K)) w, ?_⟩
      simpa [t, map_mul] using congrArg (e (K := K)) hw
    have hqfactor : q a3 a2 a1 a0 = g * (e (K := K)) u := by
      calc
        q a3 a2 a1 a0 = (e (K := K)) (Common22BranchDomain.innerF a3 a2 a1 a0) :=
          (innerF_equiv a3 a2 a1 a0).symm
        _ = (e (K := K)) (t * u) := by rw [hfactor]
        _ = g * (e (K := K)) u := by simp [t]
    have hsqdiv : g * g ∣ q a3 a2 a1 a0 := by
      obtain ⟨w, hw⟩ := hdu
      refine ⟨w, ?_⟩
      rw [hqfactor, hw]
      ring
    exact hg.not_isUnit (hsep.squarefree g hsqdiv)
  exact ⟨t, u, hfactor, ht, hu⟩

theorem affine_ring_domain_of_separable (a3 a2 a1 a0 : K)
    (hsep : (q a3 a2 a1 a0).Separable) :
    IsDomain (AffineRing a3 a2 a1 a0) := by
  obtain ⟨t, u, hfactor, ht, hu⟩ :=
    exists_simple_prime_factor a3 a2 a1 a0 hsep
  exact Common22AffineOddDomain.affine_ring_domain_of_prime_factor
    a3 a2 a1 a0 t u hfactor ht hu

theorem point_local_domain_of_separable (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (q a3 a2 a1 a0).Separable) :
    IsDomain (Localization.AtPrime
      (pointIdeal a3 a2 a1 a0 a y hcurve)) := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain_of_separable a3 a2 a1 a0 hsep
  exact IsLocalization.isDomain_of_atPrime _
    (pointIdeal a3 a2 a1 a0 a y hcurve)

theorem projective_point_domain_of_separable (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (hsep : (q a3 a2 a1 a0).Separable) :
    IsDomain (ProjPointRing a3 a2 a1 a0 a y hcurve) := by
  letI : IsDomain (Localization.AtPrime
      (pointIdeal a3 a2 a1 a0 a y hcurve)) :=
    point_local_domain_of_separable a3 a2 a1 a0 a y hcurve hsep
  exact (pointRingEquiv a3 a2 a1 a0 a y hcurve).injective.isDomain
    (pointRingEquiv a3 a2 a1 a0 a y hcurve).toRingHom

#print axioms innerF_equiv
#print axioms quintic_natDegree
#print axioms quintic_not_unit
#print axioms exists_simple_prime_factor
#print axioms affine_ring_domain_of_separable
#print axioms point_local_domain_of_separable
#print axioms projective_point_domain_of_separable

end Common22SeparableAffineDomain
