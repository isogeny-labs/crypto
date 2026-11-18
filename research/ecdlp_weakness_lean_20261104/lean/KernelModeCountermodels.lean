import Mathlib
import LeftKernel

/-!
# Countermodels separating left- and right-kernel obligations

Row surplus always supplies a nonzero left-kernel vector, but it does not make
the row action injective on factor-base logarithms.  An unused column gives an
explicit nonzero right-kernel vector; the zero matrix shows that both kernels
can be nontrivial even with one more row than column.
-/

open scoped BigOperators

namespace KernelModeCountermodels

variable {K : Type*} [Field K] {n B : ℕ}

/-- A vector annihilated by every row of `M`. -/
def InRightKernel (M : Fin n → Fin B → K) (v : Fin B → K) : Prop :=
  ∀ i, ∑ j, M i j * v j = 0

/-- Any column missing from every relation supplies a nonzero right-kernel
vector, so full reusable column rank is impossible. -/
theorem unused_column_gives_nonzero_right_kernel
    (M : Fin n → Fin B → K) (j : Fin B) (hunused : ∀ i, M i j = 0) :
    ∃ v : Fin B → K, v ≠ 0 ∧ InRightKernel M v := by
  classical
  refine ⟨Pi.single j 1, ?_, ?_⟩
  · intro hzero
    have := congrFun hzero j
    simp at this
  · intro i
    simpa [Pi.single_apply] using hunused i

/-- With `B+1` zero rows, the existing left-kernel theorem applies while a
nonzero right-kernel vector also exists.  Thus row surplus alone does not imply
the reusable-table rank condition. -/
theorem surplus_can_have_both_nonzero_kernels (hB : 0 < B) :
    ∃ M : Fin (B + 1) → Fin B → K,
      (∃ lam : Fin (B + 1) → K, lam ≠ 0 ∧
        ∀ j, ∑ i, lam i * M i j = 0) ∧
      (∃ v : Fin B → K, v ≠ 0 ∧ InRightKernel M v) := by
  let M : Fin (B + 1) → Fin B → K := fun _ _ => 0
  refine ⟨M, LeftKernel.exists_left_kernel M (Nat.lt_succ_self B), ?_⟩
  let j : Fin B := ⟨0, hB⟩
  exact unused_column_gives_nonzero_right_kernel M j (fun _ => rfl)

end KernelModeCountermodels
