import Common22BranchDomain
import WeightedProjNonbranchZeroOrder
import Mathlib.RingTheory.DiscreteValuationRing.TFAE

/-!
# Affine domain from a simple prime factor

The finite affine equation `y²-f(x)` is irreducible whenever `f` has a
prime factor of multiplicity exactly one. This is a sufficient algebraic
condition for the actual nonbranch stalk to be a domain; it is stated
explicitly rather than assuming the missing implication from separability.

The proof uses the checked Eisenstein lemma in `Common22Eisenstein` and the
checked nested-variable equivalence in `Common22BranchDomain`. The local
domain is transported through the actual weighted-Proj stalk isomorphism in
`WeightedProjNonbranchZeroOrder`. The remaining implication from a separable
monic quintic to the stated factor witness is not established here.
-/

set_option autoImplicit false
noncomputable section

namespace Common22AffineOddDomain

open MvPolynomial Common22SeparateChartMap Common22BranchStalk
  WeightedProjNonbranchZeroOrder

variable {K : Type*} [Field K]

private abbrev Inner (K : Type*) [Field K] := MvPolynomial (Fin 1) K

/-- A prime factor occurring only once in the quintic is a sufficient
condition for Eisenstein in the outer `y` variable. -/
theorem nested_equation_irreducible_of_prime_factor
    (a3 a2 a1 a0 : K) (t u : Inner K)
    (hfactor : Common22BranchDomain.innerF a3 a2 a1 a0 = t * u)
    (ht : Prime t) (hu : ¬t ∣ u) :
    Irreducible (Polynomial.X ^ 2 -
      Polynomial.C (Common22BranchDomain.innerF a3 a2 a1 a0)) := by
  rw [hfactor]
  exact Common22Eisenstein.outer_quadratic_irreducible t u ht hu

theorem affine_ring_domain_of_prime_factor
    (a3 a2 a1 a0 : K) (t u : Inner K)
    (hfactor : Common22BranchDomain.innerF a3 a2 a1 a0 = t * u)
    (ht : Prime t) (hu : ¬t ∣ u) :
    IsDomain (AffineRing a3 a2 a1 a0) := by
  have hirr : Irreducible (affineEquation a3 a2 a1 a0) := by
    apply Irreducible.of_map (f := Common22Domain.nestedEquiv)
    rw [Common22BranchDomain.nested_affine_equation]
    exact nested_equation_irreducible_of_prime_factor
      a3 a2 a1 a0 t u hfactor ht hu
  exact (Ideal.Quotient.isDomain_iff_prime (affineIdeal a3 a2 a1 a0)).mpr
    (Ideal.isPrime_span_singleton_of_prime hirr.prime)

theorem point_local_domain_of_prime_factor
    (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (t u : Inner K)
    (hfactor : Common22BranchDomain.innerF a3 a2 a1 a0 = t * u)
    (ht : Prime t) (hu : ¬t ∣ u) :
    IsDomain (Localization.AtPrime
      (pointIdeal a3 a2 a1 a0 a y hcurve)) := by
  letI : IsDomain (AffineRing a3 a2 a1 a0) :=
    affine_ring_domain_of_prime_factor a3 a2 a1 a0 t u hfactor ht hu
  exact IsLocalization.isDomain_of_atPrime _
    (pointIdeal a3 a2 a1 a0 a y hcurve)

theorem projective_point_domain_of_prime_factor
    (a3 a2 a1 a0 a y : K)
    (hcurve : y ^ 2 = fAt a3 a2 a1 a0 a)
    (t u : Inner K)
    (hfactor : Common22BranchDomain.innerF a3 a2 a1 a0 = t * u)
    (ht : Prime t) (hu : ¬t ∣ u) :
    IsDomain (ProjPointRing a3 a2 a1 a0 a y hcurve) := by
  letI : IsDomain (Localization.AtPrime
      (pointIdeal a3 a2 a1 a0 a y hcurve)) :=
    point_local_domain_of_prime_factor
      a3 a2 a1 a0 a y hcurve t u hfactor ht hu
  exact (pointRingEquiv a3 a2 a1 a0 a y hcurve).injective.isDomain
    (pointRingEquiv a3 a2 a1 a0 a y hcurve).toRingHom

#print axioms nested_equation_irreducible_of_prime_factor
#print axioms affine_ring_domain_of_prime_factor
#print axioms point_local_domain_of_prime_factor
#print axioms projective_point_domain_of_prime_factor

end Common22AffineOddDomain
