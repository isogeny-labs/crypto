import Mathlib.Algebra.Field.Basic
import Mathlib.Tactic.Ring

/-!
The author-code N6 term is `(2R - 3Q² - a₃ Z⁴) W`; the implemented variant
uses `W²`.  Here `d` stands for the parenthesized coefficient and `h` for
the other W²-weighted coefficient of N5.  These are literal polynomial
claims about the W-coordinate action, not a Jacobian group-law theorem.
-/

set_option autoImplicit false

namespace N6Homogeneity

section WeightedRepresentation
variable {K : Type*} [Field K]

/-- Scaling numerator and W by the same nonzero μ preserves each affine
ordinate coefficient, for the Z³W and Z⁵W denominators alike. -/
theorem ordinate_scale_invariant
    (n : ℕ) (a z w μ : K) (_hz : z ≠ 0) (_hw : w ≠ 0) (hμ : μ ≠ 0) :
    (μ * a) / (z ^ n * (μ * w)) = a / (z ^ n * w) := by
  have hden : z ^ n * (μ * w) = μ * (z ^ n * w) := by ring
  rw [hden]
  exact mul_div_mul_left a (z ^ n * w) hμ

end WeightedRepresentation

section GeneralRing
variable {R : Type*} [CommRing R]

/-- With the implemented W² term, the runtime `AA` intermediate has W-weight 3. -/
theorem aa_squared_w_is_homogeneous
    (q r s t h d w μ : R) :
    (q * (μ * s) - μ * t) * ((μ * s) ^ 2 - h * (μ * w) ^ 2) -
      (r * (μ * s)) * (d * (μ * w) ^ 2) =
    μ ^ 3 * ((q * s - t) * (s ^ 2 - h * w ^ 2) -
      (r * s) * (d * w ^ 2)) := by
  ring

/-- The corresponding `CC` intermediate also has W-weight 3. -/
theorem cc_squared_w_is_homogeneous
    (s t h d w μ : R) :
    (μ * s) * ((μ * s) ^ 2 - h * (μ * w) ^ 2) -
      (μ * t) * (d * (μ * w) ^ 2) =
    μ ^ 3 * (s * (s ^ 2 - h * w ^ 2) - t * (d * w ^ 2)) := by
  ring

/-- The literal W¹ term leaves this exact defect in `AA`. -/
theorem aa_linear_w_defect
    (q r s t h d w μ : R) :
    ((q * (μ * s) - μ * t) * ((μ * s) ^ 2 - h * (μ * w) ^ 2) -
      (r * (μ * s)) * (d * (μ * w))) -
    μ ^ 3 * ((q * s - t) * (s ^ 2 - h * w ^ 2) -
      (r * s) * (d * w)) =
    μ ^ 2 * (μ - 1) * r * s * d * w := by
  ring

/-- The literal W¹ term leaves this exact defect in `CC`. -/
theorem cc_linear_w_defect
    (s t h d w μ : R) :
    ((μ * s) * ((μ * s) ^ 2 - h * (μ * w) ^ 2) -
      (μ * t) * (d * (μ * w))) -
    μ ^ 3 * (s * (s ^ 2 - h * w ^ 2) - t * (d * w)) =
    μ ^ 2 * (μ - 1) * t * d * w := by
  ring

end GeneralRing

/-- A concrete integer witness: at μ=3, the literal AA gives -9, not -27. -/
theorem literal_aa_counterexample :
    (((0 : ℤ) * (3 * 1) - 3 * 0) * ((3 * 1) ^ 2 - 0 * (3 * 1) ^ 2) -
      (1 * (3 * 1)) * (1 * (3 * 1))) ≠
    3 ^ 3 * ((0 * 1 - 0) * (1 ^ 2 - 0 * 1 ^ 2) - (1 * 1) * (1 * 1)) := by
  decide

end N6Homogeneity

#print axioms N6Homogeneity.aa_squared_w_is_homogeneous
#print axioms N6Homogeneity.ordinate_scale_invariant
#print axioms N6Homogeneity.cc_squared_w_is_homogeneous
#print axioms N6Homogeneity.aa_linear_w_defect
#print axioms N6Homogeneity.cc_linear_w_defect
#print axioms N6Homogeneity.literal_aa_counterexample
