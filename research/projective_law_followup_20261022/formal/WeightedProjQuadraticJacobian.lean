import WeightedProjQuadraticLocal
import Mathlib.LinearAlgebra.FiniteDimensional.Lemmas

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

/-- The one-row linearized affine equation on a pair of tangent coordinates. -/
def affineTangentRow {L : Type*} [Field L] (a b : L) : (L × L) →ₗ[L] L where
  toFun p := a * p.1 + b * p.2
  map_add' p q := by
    simp [mul_add, add_assoc, add_left_comm]
  map_smul' c p := by
    change a * (c * p.1) + b * (c * p.2) = c * (a * p.1 + b * p.2)
    ring

/-- A nonzero affine Jacobian row has a one-dimensional tangent kernel. -/
theorem affineTangentRow_ker_finrank_one {L : Type*} [Field L]
    (a b : L) (h : a ≠ 0 ∨ b ≠ 0) :
    Module.finrank L (LinearMap.ker (affineTangentRow a b)) = 1 := by
  have hsurj : Function.Surjective (affineTangentRow a b) := by
    rcases h with ha | hb
    · intro z
      refine ⟨(a⁻¹ * z, 0), ?_⟩
      simp [affineTangentRow, ha]
    · intro z
      refine ⟨(0, b⁻¹ * z), ?_⟩
      simp [affineTangentRow, hb]
  have hrange : LinearMap.range (affineTangentRow a b) = ⊤ :=
    LinearMap.range_eq_top.mpr hsurj
  have hdim := (affineTangentRow a b).finrank_range_add_finrank_ker
  rw [hrange] at hdim
  simp only [finrank_top, Module.finrank_prod, Module.finrank_self] at hdim
  omega

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

/-- The formal two-coordinate tangent equation at the residue-field point
has a one-dimensional kernel. Identifying this kernel with the cotangent dual
of the actual projective stalk is a separate, still-open theorem. -/
theorem root_affine_tangent_finrank_one (a3 a2 a1 a0 : K)
    (u v : Polynomial K) [Fact (Irreducible u)]
    (hdiv : u ∣ v ^ 2 - WeightedProjSmoothPoints.quintic a3 a2 a1 a0)
    (h2 : (2 : K) ≠ 0)
    (hsep : (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).Separable) :
    Module.finrank (AdjoinRoot u)
      (LinearMap.ker (affineTangentRow
        (-(Polynomial.aeval (AdjoinRoot.root u)
          (WeightedProjSmoothPoints.quintic a3 a2 a1 a0).derivative))
        ((2 : AdjoinRoot u) * Polynomial.aeval (AdjoinRoot.root u) v))) = 1 := by
  apply affineTangentRow_ker_finrank_one
  rcases root_affine_jacobian_nonzero a3 a2 a1 a0 u v hdiv h2 hsep with hy | hx
  · exact Or.inr hy
  · exact Or.inl (neg_ne_zero.mpr hx)

#print axioms affineTangentRow_ker_finrank_one
#print axioms root_affine_jacobian_nonzero
#print axioms root_affine_tangent_finrank_one

end WeightedProjQuadraticJacobian
