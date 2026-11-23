import RelationDegree
import Mathlib.Algebra.Field.ZMod
import Mathlib.Tactic.ReduceModChar
import Mathlib.RingTheory.MvPolynomial.Basic

/-! The exact F17 graph/base in the recorded experiment.
    The degree is the specified homogeneous quotient invariant, not an F4 bound. -/
noncomputable section
open MvPolynomial
namespace SecantFixture
local instance : Fact (Nat.Prime 17) := ⟨by decide⟩
abbrev F := ZMod 17

def baseX : Finset F := {5, 6, 10, 16}
def graph (t : F) : F := 4*t^2+9*t+9
def curve (t : F) : F := t^3+2*t+2

theorem base_cardinality : baseX.card = 4 := by decide

theorem nonsingular : (4*(2:F)^3+27*(2:F)^2) ≠ 0 := by decide

theorem exact_split (t : F) :
    graph t ^ 2 - curve t = 16*(t-5)*(t-6)*(t-10)*(t-16) := by
  unfold graph curve
  ring_nf
  have : CharP F 17 := inferInstance
  reduce_mod_char!
  ring_nf
  reduce_mod_char!

theorem exact_membership_factorization (t : F) :
    (t-5)*(t-6)*(t-10)*(t-16) = t^4+14*t^3+10*t+6 := by
  ring_nf
  have : CharP F 17 := inferInstance
  reduce_mod_char!
  ring_nf
  reduce_mod_char!

/-- All graph/curve intersections, not just the four exhibited witnesses. -/
theorem graph_intersection_iff (t : F) : graph t ^ 2 = curve t ↔ t ∈ baseX := by
  rw [← sub_eq_zero, exact_split]
  have h16 : (16 : F) ≠ 0 := by decide
  simp [baseX, mul_eq_zero, sub_eq_zero, h16, or_assoc]

theorem signed_lift_on_curve (t e : F) (ht : t ∈ baseX)
    (he : e = 1 ∨ e = -1) : (e*graph t)^2 = curve t := by
  rcases he with rfl | rfl <;> simpa using (graph_intersection_iff t).mpr ht

def graphLower (i : Fin 3) (e : F) : MvPolynomial (Fin 3) F :=
  C (e*9)*X i+C (e*9)

theorem graphLower_degree (i : Fin 3) (e : F) : (graphLower i e).totalDegree < 2 := by
  unfold graphLower
  apply lt_of_le_of_lt (totalDegree_add _ _)
  apply max_lt_iff.mpr
  constructor
  · exact lt_of_le_of_lt (isHomogeneous_C_mul_X (e*9) i).totalDegree_le (by decide)
  · exact lt_of_le_of_lt (isHomogeneous_C (Fin 3) (e*9)).totalDegree_le (by decide)

def denseLower (i : Fin 2) : MvPolynomial (Fin 2) F :=
  C 14*X i^3+C 10*X i+C 6

/-- Polynomial equality, not only equality of functions on the finite field. -/
theorem membership_polynomial (i : Fin 2) :
    ((X i : MvPolynomial (Fin 2) F)-C 5)*(X i-C 6)*(X i-C 10)*(X i-C 16) =
      X i^4+denseLower i := by
  unfold denseLower
  simp only [map_ofNat]
  ring_nf
  have : CharP (MvPolynomial (Fin 2) F) 17 := inferInstance
  reduce_mod_char!
  ring_nf
  reduce_mod_char!

theorem signed_graph_polynomial (i : Fin 3) (e r s : F) :
    SecantTopForms.q i 2 (e*4) r s (graphLower i e) =
      C e*(C 4*(X i : MvPolynomial (Fin 3) F)^2+C 9*X i+C 9)-
        X 2*(X i-C r)+C s := by
  simp only [SecantTopForms.q, graphLower, map_mul, map_ofNat]
  ring

theorem denseLower_degree (i : Fin 2) : (denseLower i).totalDegree < 4 := by
  unfold denseLower
  apply lt_of_le_of_lt (totalDegree_add _ _)
  apply max_lt_iff.mpr
  constructor
  · apply lt_of_le_of_lt (totalDegree_add _ _)
    apply max_lt_iff.mpr
    constructor
    · exact lt_of_le_of_lt (isHomogeneous_C_mul_X_pow (14:F) i 3).totalDegree_le (by decide)
    · exact lt_of_le_of_lt (isHomogeneous_C_mul_X (10:F) i).totalDegree_le (by decide)
  · exact lt_of_le_of_lt (isHomogeneous_C (Fin 2) (6:F)).totalDegree_le (by decide)

theorem sign_nonzero (e : F) (he : e = 1 ∨ e = -1) : e*4 ≠ 0 := by
  rcases he with rfl | rfl <;> decide

/-- All four signs and every target share the actual first degrees3 and5. -/
theorem four_branch_degree_reduction (r s e f : F)
    (he : e = 1 ∨ e = -1) (hf : f = 1 ∨ f = -1) :
    SecantIdeal.FirstVanishing
      (SecantTopForms.leadingIdeal 2 (e*4) (f*4) r s 2
        (graphLower 0 e) (graphLower 1 f)) 3 ∧
    SecantIdeal.FirstVanishing
      (DenseTopForms.leadingIdeal 4 2 2 r (denseLower 0) (denseLower 1)) 5 := by
  have h := RelationDegree.strict_reduction (K := F) 2 (by decide)
    (e*4) (f*4) r s 2 2 (sign_nonzero e he) (sign_nonzero f hf) (by decide)
    (graphLower 0 e) (graphLower 1 f) (graphLower_degree 0 e) (graphLower_degree 1 f)
    (denseLower 0) (denseLower 1) (denseLower_degree 0) (denseLower_degree 1)
  exact ⟨h.1, h.2.1⟩

end SecantFixture

#print axioms SecantFixture.exact_split
#print axioms SecantFixture.graph_intersection_iff
#print axioms SecantFixture.membership_polynomial
#print axioms SecantFixture.signed_graph_polynomial
#print axioms SecantFixture.four_branch_degree_reduction
