import WeightedProjQuadraticLocal

/-!
# Affine Jacobian at an irreducible support point

At the extension-valued point supplied by an irreducible Mumford support,
separability of the quintic and invertibility of two ensure that the two
affine partial derivatives cannot both vanish. This is the residue-field
Jacobian input for a future cotangent-space computation. It does not itself
prove regularity or a DVR structure on the projective stalk.

Mathematical reference: Stacks Project Tag 04QM (smooth schemes over fields
and regularity). Formal input: pinned Mathlib `Polynomial.Separable` and
`WeightedProjQuadraticPoint.root_curve_of_dvd`.
-/

set_option autoImplicit false
noncomputable section

namespace WeightedProjQuadraticJacobian

open Polynomial

variable {K : Type*} [Field K]

theorem root_affine_jacobian_nonzero (a3 a2 a1 a0 : K)
    (u v : Polynomial K) [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0)
    (h2 : (2 : K) ≠ 0)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    (2 : AdjoinRoot u) * Polynomial.aeval (AdjoinRoot.root u) v ≠ 0 ∨
      Polynomial.aeval (AdjoinRoot.root u)
        (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).derivative ≠ 0 := by
  let x : AdjoinRoot u := AdjoinRoot.root u
  let y : AdjoinRoot u := Polynomial.aeval x v
  have hcurve : y ^ 2 =
      Polynomial.aeval x (WeightedProjSmoothPoints.quintic a3 a2 a1 a0) := by
    have h := WeightedProjQuadraticPoint.root_curve_of_dvd a3 a2 a1 a0 u v hdiv
    simpa [x, y, WeightedProjQuadraticPoint.fAtExtension,
      WeightedProjSmoothPoints.quintic] using h
  by_cases hy : y = 0
  · right
    have hx : Polynomial.aeval x
        (WeightedProjSmoothPoints.quintic a3 a2 a1 a0) = 0 := by
      simpa [hy] using hcurve.symm
    exact hsep.aeval_derivative_ne_zero hx
  · left
    have h2' : (2 : AdjoinRoot u) ≠ 0 := by
      intro hz
      apply h2
      have hmap : algebraMap K (AdjoinRoot u) (2 : K) =
          algebraMap K (AdjoinRoot u) 0 := by
        simpa only [map_ofNat, map_zero] using hz
      exact (FaithfulSMul.algebraMap_injective K (AdjoinRoot u)) hmap
    exact mul_ne_zero h2' hy

#print axioms root_affine_jacobian_nonzero

end WeightedProjQuadraticJacobian
