module
public import NormEvaluation
public import Mathlib.Data.Set.Finite.Lattice

@[expose] public section
namespace FunctionDivisorFinitenessAudit
noncomputable section
open Polynomial WeierstrassCurve.Affine WeierstrassCurve.Affine.CoordinateRing
open MillerPrincipalIdealAudit MillerFinitePointAudit
variable {K : Type*} [Field K]

def fiberPolynomial (W : WeierstrassCurve.Affine K) (x : K) : K[X] :=
  X ^ 2 + C (W.a₁ * x + W.a₃) * X - C (x ^ 3 + W.a₂ * x ^ 2 + W.a₄ * x + W.a₆)

theorem fiber_polynomial_nonzero (W : WeierstrassCurve.Affine K) (x : K) :
    fiberPolynomial W x ≠ 0 := by
  intro h
  have hc := congrArg (fun p : K[X] => p.coeff 2) h
  simp only [fiberPolynomial, coeff_sub, coeff_add, coeff_C_mul, coeff_X_pow, coeff_X, coeff_C] at hc
  norm_num at hc

theorem equation_fiber_root (W : WeierstrassCurve.Affine K) (x y : K) :
    W.Equation x y ↔ IsRoot (fiberPolynomial W x) y := by
  have he : (fiberPolynomial W x).eval y =
      y ^ 2 + W.a₁ * x * y + W.a₃ * y - (x ^ 3 + W.a₂ * x ^ 2 + W.a₄ * x + W.a₆) := by
    simp only [fiberPolynomial, eval_sub, eval_add, eval_pow, eval_mul, eval_C, eval_X]
    ring
  rw [W.equation_iff', Polynomial.IsRoot.def, he]

theorem equation_fiber_finite (W : WeierstrassCurve.Affine K) (x : K) :
    Set.Finite {y : K | W.Equation x y} := by
  simpa only [equation_fiber_root] using
    Polynomial.finite_setOfPred_isRoot (fiber_polynomial_nonzero W x)

def zeroPairs (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) : Set (K × K) :=
  {xy | ∃ h : W.Equation xy.1 xy.2, pointEval W xy.1 xy.2 h a = 0}

/-- Nonzero actual coordinate functions have finitely many actual affine zeros over every field. -/
theorem zero_pairs_finite (W : WeierstrassCurve.Affine K) {a : W.CoordinateRing} (ha : a ≠ 0) :
    (zeroPairs W a).Finite := by
  let roots : Set K := {x | IsRoot (Algebra.norm K[X] a) x}
  have hroots : roots.Finite := Polynomial.finite_setOfPred_isRoot (norm_polynomial_nonzero W ha)
  let fiber (x : K) : Set (K × K) := (fun y : K => (x, y)) '' {y | W.Equation x y}
  have hfiber (x : K) : (fiber x).Finite := (equation_fiber_finite W x).image _
  have hall : (⋃ x ∈ roots, fiber x).Finite := hroots.biUnion (fun x _ => hfiber x)
  apply hall.subset
  intro xy hxy
  obtain ⟨h, hz⟩ := hxy
  apply Set.mem_iUnion.mpr
  refine ⟨xy.1, ?_⟩
  apply Set.mem_iUnion.mpr
  refine ⟨zero_eval_norm_root W h a hz, ?_⟩
  exact ⟨xy.2, h, rfl⟩

def affineCoordinates (W : WeierstrassCurve.Affine K) : W.Point → Option (K × K)
  | .zero => none
  | .some x y _ => some (x, y)

theorem affine_coordinates_injective (W : WeierstrassCurve.Affine K) :
    Function.Injective (affineCoordinates W) := by
  intro P Q h
  cases P <;> cases Q <;> simp_all [affineCoordinates]

def affineZeroSet (W : WeierstrassCurve.Affine K) (a : W.CoordinateRing) : Set W.Point :=
  {P | match P with
    | .zero => False
    | .some x y h => pointEval W x y h.1 a = 0}

theorem affine_zero_set_finite (W : WeierstrassCurve.Affine K) {a : W.CoordinateRing} (ha : a ≠ 0) :
    (affineZeroSet W a).Finite := by
  have hp : ((affineCoordinates W) ⁻¹' (Option.some '' zeroPairs W a)).Finite :=
    Set.Finite.preimage (fun _ _ _ _ h => affine_coordinates_injective W h)
      ((zero_pairs_finite W ha).image Option.some)
  apply hp.subset
  intro P hP
  cases P with
  | zero => exact False.elim hP
  | some x y h =>
    change some (x, y) ∈ Option.some '' zeroPairs W a
    exact ⟨(x, y), ⟨h.1, hP⟩, rfl⟩

#print axioms zero_pairs_finite
#print axioms affine_zero_set_finite
end
end FunctionDivisorFinitenessAudit
