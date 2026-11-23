import Mathlib

/-!
# Exact symmetric integer coefficient boxes

The paper's probability heuristic counts bounded coefficient vectors.  This
file records the exact finite count, with the zero vector removed.
-/

namespace JKSSTCoefficientBox

variable {ι : Type*} [Fintype ι] [DecidableEq ι]

/-- The symmetric integral box `[-C,C]^ι`. -/
noncomputable def coefficientBox (C : ℕ) : Finset (ι → ℤ) :=
  Finset.Icc (fun _ ↦ -(C : ℤ)) (fun _ ↦ (C : ℤ))

@[simp] theorem mem_coefficientBox_iff (C : ℕ) (m : ι → ℤ) :
    m ∈ coefficientBox C ↔ ∀ i, (|m i| : ℤ) ≤ C := by
  classical
  simp only [coefficientBox, Finset.mem_Icc, Pi.le_def]
  constructor
  · rintro ⟨lower, upper⟩ i
    rw [abs_le]
    exact ⟨lower i, upper i⟩
  · intro h
    exact ⟨fun i ↦ (abs_le.mp (h i)).1,
      fun i ↦ (abs_le.mp (h i)).2⟩

@[simp] theorem card_coefficientBox (C : ℕ) :
    (coefficientBox (ι := ι) C).card =
      (2 * C + 1) ^ Fintype.card ι := by
  classical
  rw [coefficientBox, Pi.card_Icc]
  simp only [Int.card_Icc, Finset.prod_const, Finset.card_univ]
  congr 1
  omega

@[simp] theorem zero_mem_coefficientBox (C : ℕ) :
    (0 : ι → ℤ) ∈ coefficientBox C := by
  rw [mem_coefficientBox_iff]
  simp

/-- The candidate relation vectors: the symmetric box with the tautological
zero vector deleted. -/
noncomputable def nonzeroCoefficientBox (C : ℕ) : Finset (ι → ℤ) :=
  (coefficientBox C).erase 0

@[simp] theorem mem_nonzeroCoefficientBox_iff (C : ℕ) (m : ι → ℤ) :
    m ∈ nonzeroCoefficientBox C ↔
      m ≠ 0 ∧ ∀ i, (|m i| : ℤ) ≤ C := by
  classical
  simp [nonzeroCoefficientBox]

/-- Exact candidate count: `(2C+1)^r - 1`, not `(2C)^r`, and not including
the zero vector. -/
@[simp] theorem card_nonzeroCoefficientBox (C : ℕ) :
    (nonzeroCoefficientBox (ι := ι) C).card =
      (2 * C + 1) ^ Fintype.card ι - 1 := by
  classical
  rw [nonzeroCoefficientBox, Finset.card_erase_of_mem]
  · rw [card_coefficientBox]
  · exact zero_mem_coefficientBox C

end JKSSTCoefficientBox
