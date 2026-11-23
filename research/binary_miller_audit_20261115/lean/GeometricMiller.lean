module
public import BasechangeMiller

@[expose] public section
namespace MillerBasechangeAudit
noncomputable section
open WeierstrassCurve.Affine MillerRecursionAudit MillerFinitePointAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K]

local instance closureDecidableEq : DecidableEq (AlgebraicClosure K) := Classical.decEq _

theorem extended_delta_nonzero {L : Type*} [Field L] [Algebra K L]
    (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) : (extendedCurve (L := L) W).Δ ≠ 0 := by
  rw [WeierstrassCurve.map_Δ]
  intro hz
  apply hΔ
  apply (algebraMap K L).injective
  simpa only [map_zero] using hz

theorem field_base_nonzero {L : Type*} [Field L] [Algebra K L]
    (W : WeierstrassCurve.Affine K) {f : W.FunctionField} (hf : f ≠ 0) :
    fieldBaseChange (L := L) W f ≠ 0 :=
  fun he => hf ((field_base_injective (L := L) W) (he.trans (map_zero _).symm))

variable [DecidableEq K]

/-- Proper geometric condition: an actual elliptic model and the actual function/point
images in the actual algebraic closure, with all geometric point orders and normalization. -/
def GeometricMiller (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ)
    (f : W.FunctionField) : Prop :=
  W.Δ ≠ 0 ∧ IsMillerFunction (extendedCurve (L := AlgebraicClosure K) W)
    (pointBaseChange (L := AlgebraicClosure K) W P) n (fieldBaseChange (L := AlgebraicClosure K) W f)

theorem constructed_geometric_miller (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n : ℕ) : GeometricMiller W P n (millerFunction W P n) := by
  refine ⟨hΔ, ?_⟩
  rw [miller_function_basechange]
  exact constructed_is_miller (extendedCurve (L := AlgebraicClosure K) W)
    (pointBaseChange (L := AlgebraicClosure K) W P) n

/-- Uniqueness over every base field follows through the actual injective function-field map. -/
theorem geometric_miller_unique (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ)
    {f : W.FunctionField} (hf : GeometricMiller W P n f) : f = millerFunction W P n := by
  apply field_base_injective (L := AlgebraicClosure K) W
  rw [miller_function_basechange]
  exact GeometricPrincipalDivisorAudit.miller_function_unique
    (extendedCurve (L := AlgebraicClosure K) W) (extended_delta_nonzero W hf.1)
    (pointBaseChange (L := AlgebraicClosure K) W P) n hf.2

/-- Actual full geometric divisor of the actual base-field function, after its actual embedding. -/
theorem constructed_geometric_divisor (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (n : ℕ) :
    functionDivisor (extendedCurve (L := AlgebraicClosure K) W)
        (fieldBaseChange (L := AlgebraicClosure K) W (millerFunction W P n))
        (field_base_nonzero W (miller_function_nonzero W P n)) =
      millerDivisor (extendedCurve (L := AlgebraicClosure K) W)
        (pointBaseChange (L := AlgebraicClosure K) W P) n := by
  ext Q
  exact (constructed_geometric_miller W hΔ P n).2.1.2 Q

/-- Exact chosen addition descends to every base field; this does not assert arbitrary
rational-point-only normalized function uniqueness. -/
theorem chosen_miller_addition_all_fields (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0)
    (P : W.Point) (i j : ℕ) :
    millerFunction W P (i + j) =
      millerFunction W P i * millerFunction W P j * stepFunction W (i • P) (j • P) := by
  apply field_base_injective (L := AlgebraicClosure K) W
  simp only [miller_function_basechange, map_mul, field_base_step, point_base_nsmul]
  exact GeometricPrincipalDivisorAudit.chosen_miller_addition
    (extendedCurve (L := AlgebraicClosure K) W) (extended_delta_nonzero W hΔ)
    (pointBaseChange (L := AlgebraicClosure K) W P) i j

#print axioms geometric_miller_unique
#print axioms chosen_miller_addition_all_fields
end
end MillerBasechangeAudit
