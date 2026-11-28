import GreenKernel

namespace MarkovGreenBoundAudit
noncomputable section
open scoped BigOperators
variable {S : Type*} [Fintype S] [DecidableEq S] [Nonempty S]

def partialGreen (P : Matrix S S ℝ) (n : ℕ) (x y : S) : ℝ :=
  ∑ j ∈ Finset.range n, (P^j) x y

/-- Inclusive Green sum of the actual transition powers, including identity. -/
def green (P : Matrix S S ℝ) (T : ℕ) : Matrix S S ℝ := partialGreen P (T+1)
def rowEnergy (P : Matrix S S ℝ) (T : ℕ) (x : S) : ℝ := ∑ y, green P T x y ^ 2
def columnEnergy (P : Matrix S S ℝ) (T : ℕ) (x : S) : ℝ := ∑ y, green P T y x ^ 2
def rowMaximum (P : Matrix S S ℝ) (T : ℕ) : ℝ :=
  Finset.univ.sup' Finset.univ_nonempty (rowEnergy P T)
def columnMaximum (P : Matrix S S ℝ) (T : ℕ) : ℝ :=
  Finset.univ.sup' Finset.univ_nonempty (columnEnergy P T)
def maxEntry (P : Matrix S S ℝ) (j : ℕ) : ℝ :=
  Finset.univ.sup' Finset.univ_nonempty (fun xy : S × S => (P^j) xy.1 xy.2)
def sourceRHS (P : Matrix S S ℝ) (T : ℕ) : ℝ :=
  2 * ∑ j ∈ Finset.range (T+1), ((j : ℝ)+1) * maxEntry P j

theorem entry_le_max (P : Matrix S S ℝ) (j : ℕ) (x y : S) :
    (P^j) x y ≤ maxEntry P j :=
  Finset.le_sup' (fun uv : S×S => (P^j) uv.1 uv.2) (Finset.mem_univ (x,y))

theorem max_nonnegative (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (j : ℕ) : 0 ≤ maxEntry P j := by
  let x : S := Classical.choice inferInstance
  exact (power_nonnegative P hP j x x).trans (entry_le_max P j x x)

omit [Nonempty S] in
theorem partial_nonnegative (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (n : ℕ) (x y : S) : 0 ≤ partialGreen P n x y :=
  Finset.sum_nonneg (fun j _ => power_nonnegative P hP j x y)

omit [Nonempty S] in
theorem partial_row_sum (P : Matrix S S ℝ) (hP : P ∈ Matrix.rowStochastic ℝ S)
    (n : ℕ) (x : S) : (∑ y, partialGreen P n x y) = n := by
  unfold partialGreen
  rw [Finset.sum_comm]
  simp only [power_row_sum P hP, Finset.sum_const, Finset.card_range, nsmul_eq_mul, mul_one]

omit [Nonempty S] in
theorem green_transpose (P : Matrix S S ℝ) (T : ℕ) (x y : S) :
    green P.transpose T x y = green P T y x := by
  simp only [green, partialGreen, ← Matrix.transpose_pow, Matrix.transpose_apply]

theorem max_transpose (P : Matrix S S ℝ) (j : ℕ) :
    maxEntry P.transpose j = maxEntry P j := by
  apply le_antisymm
  · apply Finset.sup'_le
    intro xy _
    simpa only [← Matrix.transpose_pow, Matrix.transpose_apply] using entry_le_max P j xy.2 xy.1
  · apply Finset.sup'_le
    intro xy _
    simpa only [← Matrix.transpose_pow, Matrix.transpose_apply] using
      entry_le_max P.transpose j xy.2 xy.1

theorem sourceRHS_transpose (P : Matrix S S ℝ) (T : ℕ) :
    sourceRHS P.transpose T = sourceRHS P T := by
  simp only [sourceRHS, max_transpose]

#print axioms partial_row_sum
#print axioms green_transpose
#print axioms max_transpose
end
end MarkovGreenBoundAudit
