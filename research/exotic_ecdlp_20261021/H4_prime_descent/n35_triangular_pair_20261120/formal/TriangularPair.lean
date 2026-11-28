import Mathlib

/-! Every ordered pair has a commutatively equivalent triangular pair whose
row-major packed index is no larger. -/

namespace TriangularPair

def packed (B i j : ℕ) : ℕ := i * B + j

theorem swapped_packed_lt {B i j : ℕ} (hi : i < B) (hji : j < i) :
    packed B j i < packed B i j := by
  have hB : 1 < B := by omega
  have hsub : i - j + j = i := by omega
  have hpos : 0 < (i - j) * (B - 1) := Nat.mul_pos (by omega) (by omega)
  dsimp [packed]
  nlinarith

theorem triangular_pair_dominates {B : ℕ} {X Y : Type*}
    (base : Fin B → X) (op : X → X → Y)
    (hop : ∀ x y, op x y = op y x) (i j : Fin B) :
    ∃ a b : Fin B, a.val ≤ b.val ∧
      op (base a) (base b) = op (base i) (base j) ∧
      packed B a.val b.val ≤ packed B i.val j.val := by
  by_cases hij : i.val ≤ j.val
  · exact ⟨i, j, hij, rfl, le_refl _⟩
  · have hji : j.val < i.val := by omega
    refine ⟨j, i, Nat.le_of_lt hji, hop _ _, ?_⟩
    exact Nat.le_of_lt (swapped_packed_lt i.isLt hji)

#print axioms TriangularPair.swapped_packed_lt
#print axioms TriangularPair.triangular_pair_dominates

end TriangularPair
