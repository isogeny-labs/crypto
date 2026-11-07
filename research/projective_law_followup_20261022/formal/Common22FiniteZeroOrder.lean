import Mathlib.RingTheory.DiscreteValuationRing.Basic
import Mathlib.RingTheory.LocalRing.Basic

/-!
# Nonbranch finite-zero multiplicity in a local DVR

This is the local step for `g = N - B*y`, `h = N + B*y` and the norm
factorization `g*h = u₁*u₂*U`.  It proves that if `g` vanishes at a
non-Weierstrass point and `2`, `B`, and `y` are units in its local DVR, then
the order of `g` is the sum of the three norm-factor orders.  It does not
construct the finite-point stalks, prove the branch-point case, or sum a
global principal divisor.
-/

set_option autoImplicit false
noncomputable section

namespace Common22FiniteZeroOrder

variable {R : Type*} [CommRing R] [IsDomain R] [IsDiscreteValuationRing R]

theorem other_factor_unit (N B y : R)
    (h2 : IsUnit (2 : R)) (hB : IsUnit B) (hy : IsUnit y)
    (hg : N - B * y ∈ nonunits R) :
    IsUnit (N + B * y) := by
  by_contra hh
  have hh' : N + B * y ∈ nonunits R := hh
  have hneg : -(N - B * y) ∈ nonunits R := by
    intro h
    exact hg (by simpa using h.neg)
  have hsum : (N + B * y) + -(N - B * y) ∈ nonunits R :=
    IsLocalRing.nonunits_add hh' hneg
  have heq : (N + B * y) + -(N - B * y) = (2 : R) * B * y := by ring
  rw [heq] at hsum
  exact hsum (h2.mul hB |>.mul hy)

theorem nonbranch_norm_order (N B y u₁ u₂ U : R)
    (h2 : IsUnit (2 : R)) (hB : IsUnit B) (hy : IsUnit y)
    (hg : N - B * y ∈ nonunits R)
    (hnorm : (N - B * y) * (N + B * y) = u₁ * u₂ * U) :
    IsDiscreteValuationRing.addVal R (N - B * y) =
      IsDiscreteValuationRing.addVal R u₁ +
        IsDiscreteValuationRing.addVal R u₂ +
          IsDiscreteValuationRing.addVal R U := by
  have hh := other_factor_unit N B y h2 hB hy hg
  have h := congrArg (IsDiscreteValuationRing.addVal R) hnorm
  rw [IsDiscreteValuationRing.addVal_mul,
    IsDiscreteValuationRing.addVal_eq_zero_iff.mpr hh,
    add_zero, IsDiscreteValuationRing.addVal_mul,
    IsDiscreteValuationRing.addVal_mul] at h
  exact h

#print axioms other_factor_unit
#print axioms nonbranch_norm_order

end Common22FiniteZeroOrder
