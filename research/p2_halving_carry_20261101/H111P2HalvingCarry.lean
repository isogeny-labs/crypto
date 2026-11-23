import Mathlib.Algebra.Group.Hom.Basic
import Mathlib.Tactic

/-!
# H111: first-layer halving-carry interfaces

These are the uniform algebraic and reconstruction obligations behind H111.
They do not assert that any of the 24 empirical predicates is exact.
-/

namespace H111P2HalvingCarry

section Carry

variable {Q A : Type*}
variable [AddCommGroup Q] [AddCommGroup A]

/-- The binary section carry tested by H111.  `section` is intentionally only
a set-theoretic function. -/
def carry (liftSection : Q → A) (half target : Q) : A :=
  2 • liftSection half - liftSection target

/-- Reduction sends the carry to zero whenever `half` really doubles to the
target.  No homomorphism assumption is made about the section. -/
theorem carry_mem_reduction_kernel
    (reduction : A →+ Q) (liftSection : Q → A) (half target : Q)
    (hsection : ∀ q, reduction (liftSection q) = q)
    (hhalf : 2 • half = target) :
    reduction (carry liftSection half target) = 0 := by
  simp [carry, hsection, hhalf]

/-- If the section itself is additive, its binary carry vanishes.  This is the
formal reason for H111's torsion-section zero control. -/
theorem homomorphic_section_carry_zero
    (liftSection : Q →+ A) (half target : Q) (hhalf : 2 • half = target) :
    carry liftSection half target = 0 := by
  simp [carry, ← liftSection.map_nsmul, hhalf]

end Carry

section Oracle

variable {Point Value : Type*}

/-- Exactness of a public predicate supplies the canonical parity bit queried
by the repeated-halving algorithm. -/
theorem exact_predicate_supplies_parity
    (observable : Point → Value) (predicate : Value → ℕ)
    (scalar : Point → ℕ)
    (hexact : ∀ q, predicate (observable q) = scalar q % 2) (q : Point) :
    predicate (observable q) = scalar q % 2 := by
  exact hexact q

/-- Reconstruct an integer from successive public parity queries. -/
def reconstruct (oracle : ℕ → ℕ) : ℕ → ℕ
  | 0 => 0
  | k + 1 => oracle (k + 1) + 2 * reconstruct oracle ((k + 1) / 2)
termination_by k => k
decreasing_by omega

/-- An exact parity oracle makes repeated halving recover every natural
number; this is the integer core of H111's scalar decoder. -/
theorem reconstruct_eq
    (oracle : ℕ → ℕ) (hexact : ∀ k, oracle k = k % 2) :
    ∀ k, reconstruct oracle k = k := by
  intro k
  induction k using Nat.strong_induction_on with
  | h k ih =>
      cases k with
      | zero => simp [reconstruct]
      | succ n =>
          rw [reconstruct, hexact, ih ((n + 1) / 2) (by omega)]
          omega

/-- Direct EC verification makes a reconstructed scalar conclusive whenever
the scalar-action map is injective on the admitted domain. -/
theorem verified_scalar_unique
    {Scalar Point : Type*} (mul : Scalar → Point) (target : Point)
    (candidate trueScalar : Scalar) (hcandidate : mul candidate = target)
    (htrue : mul trueScalar = target) (hinjective : Function.Injective mul) :
    candidate = trueScalar := by
  apply hinjective
  rw [hcandidate, htrue]

end Oracle

section Cost

/-- Complete charged H111 path: setup, carry queries, predicates, halving,
verification, controls, and proof/checking overhead. -/
def totalCost
    (setup carryQueries predicates halving verify controls checks : ℕ) : ℕ :=
  setup + carryQueries + predicates + halving + verify + controls + checks

theorem totalCost_bound
    (setup carryQueries predicates halving verify controls checks budget : ℕ)
    (h : setup + carryQueries + predicates + halving + verify + controls + checks ≤ budget) :
    totalCost setup carryQueries predicates halving verify controls checks ≤ budget := by
  exact h

end Cost

#print axioms carry_mem_reduction_kernel
#print axioms homomorphic_section_carry_zero
#print axioms exact_predicate_supplies_parity
#print axioms reconstruct_eq
#print axioms verified_scalar_unique
#print axioms totalCost_bound

end H111P2HalvingCarry
