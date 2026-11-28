import IcNegative.YieldVisibility

/-!
# Exact boundary for cycle closure in two-summand relation graphs

A two-summand relation contributes an unsigned row with two nonzero
factor-base coefficients.  Over an odd scalar field this is the signless
incidence matrix, not the oriented incidence matrix.  Consequently, the first
graph-theoretic cycle need not supply a left-kernel vector: an odd unicyclic
component can still have full row rank.  The finite countermodels below isolate
that distinction, and `recover_target_of_projected_dependency` records the
second obligation needed after factor-base cancellation.
-/

namespace IcNegative.CycleClosure

open scoped BigOperators

/-- A left dependency among relation rows cancels every factor-base column. -/
def RowDependency {E V R : Type*} [Fintype E] [Fintype V]
    [CommRing R] (incidence : E → V → R) (weight : E → R) : Prop :=
  ∀ v, ∑ e, weight e * incidence e v = 0

/-- The unsigned incidence rows of a triangle over the odd prime field F₃. -/
def triangleIncidence : Fin 3 → Fin 3 → ZMod 3
  | 0, 0 => 1
  | 0, 1 => 1
  | 1, 1 => 1
  | 1, 2 => 1
  | 2, 0 => 1
  | 2, 2 => 1
  | _, _ => 0

/-- A triangle is already a graph-theoretic cycle, but its unsigned incidence
rows have no nonzero dependency over F₃.  This refutes the unqualified rule
"the first cycle closes elimination" for the odd-order ECDLP scalar model. -/
theorem triangle_cycle_has_no_row_dependency :
    ∀ weight : Fin 3 → ZMod 3,
      RowDependency triangleIncidence weight → ∀ e, weight e = 0 := by
  simp only [RowDependency]
  decide

/-- A self-pair `P + P` is a loop row with coefficient two.  It is nonzero and
independent over an odd scalar field, so a graph loop is not automatically an
eliminable relation circuit either. -/
def loopIncidence : Fin 1 → Fin 1 → ZMod 3
  | _, _ => 2

theorem loop_has_no_row_dependency :
    ∀ weight : Fin 1 → ZMod 3,
      RowDependency loopIncidence weight → ∀ e, weight e = 0 := by
  simp only [RowDependency]
  decide

/-- The unsigned incidence rows of a square over F₃. -/
def squareIncidence : Fin 4 → Fin 4 → ZMod 3
  | 0, 0 => 1
  | 0, 1 => 1
  | 1, 1 => 1
  | 1, 2 => 1
  | 2, 2 => 1
  | 2, 3 => 1
  | 3, 3 => 1
  | 3, 0 => 1
  | _, _ => 0

def squareWeight : Fin 4 → ZMod 3
  | 0 => 1
  | 1 => -1
  | 2 => 1
  | 3 => -1

/-- In contrast, the alternating weights on a square cancel every column. -/
theorem square_cycle_has_nonzero_row_dependency :
    ∃ weight : Fin 4 → ZMod 3,
      RowDependency squareIncidence weight ∧ ∃ e, weight e ≠ 0 := by
  refine ⟨squareWeight, ?_, 0, ?_⟩
  · simp only [RowDependency]
    decide
  · decide

/-- Two parallel unsigned edges form a length-two cycle and cancel with
opposite weights.  This is why a multigraph can close earlier than a simple
graph, while an odd simple cycle need not close at all. -/
def parallelIncidence : Fin 2 → Fin 2 → ZMod 3
  | _, _ => 1

def parallelWeight : Fin 2 → ZMod 3
  | 0 => 1
  | 1 => -1

theorem parallel_edges_have_nonzero_row_dependency :
    ∃ weight : Fin 2 → ZMod 3,
      RowDependency parallelIncidence weight ∧ ∃ e, weight e ≠ 0 := by
  refine ⟨parallelWeight, ?_, 0, ?_⟩
  · simp only [RowDependency]
    decide
  · decide

/-- Cancelling factor-base columns is not yet target recovery.  If the weighted
target coefficient `B` is nonzero, the scalar equation `A + B*d = 0` determines
the target logarithm.  If `B = 0`, the same graph dependency may be useless. -/
theorem recover_target_of_projected_dependency {K : Type*} [Field K]
    (targetLog A B : K) (equation : A + B * targetLog = 0) (hB : B ≠ 0) :
    targetLog = -A / B := by
  calc
    targetLog = (targetLog * B) / B := by field_simp
    _ = -A / B := by
      congr 1
      rw [mul_comm]
      exact eq_neg_of_add_eq_zero_right equation

/-- A genuine factor-base dependency can project to zero on the target and
therefore reveal no target logarithm. -/
theorem zero_target_projection_is_uninformative {K : Type*} [Field K]
    (targetLog : K) : (0 : K) + 0 * targetLog = 0 := by
  ring

end IcNegative.CycleClosure
