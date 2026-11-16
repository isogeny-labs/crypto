import JKSSTAudit
import Mathlib.Data.Pi.Interval

/-!
# Exact finite lattice boxes for the JKSST height argument

JKSST use an asymptotic Euclidean-lattice point count with an unspecified
error term.  This file gives a completely finite replacement in a concrete
coordinate lattice: the sup-norm ball in `ℤ^d` has exactly `(2 R + 1)^d`
points.  Any height ball whose coordinates are bounded by `R` therefore has
at most that many points.
-/

open scoped BigOperators

namespace JKSSTHeightCount

/-- The integral sup-norm ball (equivalently, the coordinate box)
`[-R,R]^d ∩ ℤ^d`. -/
noncomputable def integerBox (d R : ℕ) : Finset (Fin d → ℤ) :=
  Finset.Icc (fun _ => -(R : ℤ)) (fun _ => (R : ℤ))

@[simp]
theorem mem_integerBox_iff {d R : ℕ} {z : Fin d → ℤ} :
    z ∈ integerBox d R ↔ ∀ i, |z i| ≤ R := by
  classical
  simp only [integerBox, Finset.mem_Icc, Pi.le_def]
  constructor
  · rintro ⟨lower, upper⟩ i
    rw [abs_le]
    exact ⟨lower i, upper i⟩
  · intro h
    constructor
    · intro i
      exact (abs_le.mp (h i)).1
    · intro i
      exact (abs_le.mp (h i)).2

/-- Exact cardinality, with no asymptotic term and no hidden constant. -/
@[simp]
theorem card_integerBox (d R : ℕ) :
    (integerBox d R).card = (2 * R + 1) ^ d := by
  classical
  rw [integerBox, Pi.card_Icc]
  simp only [Int.card_Icc, Finset.prod_const, Finset.card_univ,
    Fintype.card_fin]
  congr 1
  omega

/-- Every finite collection of integral lattice points with coordinate bound
`R` has cardinality at most `(2R+1)^d`. -/
theorem card_le_of_coordinate_bound
    {d R : ℕ} (S : Finset (Fin d → ℤ))
    (coordinate_bound : ∀ z ∈ S, ∀ i, |z i| ≤ R) :
    S.card ≤ (2 * R + 1) ^ d := by
  classical
  calc
    S.card ≤ (integerBox d R).card := by
      apply Finset.card_le_card
      intro z hz
      rw [mem_integerBox_iff]
      exact coordinate_bound z hz
    _ = (2 * R + 1) ^ d := card_integerBox d R

/-- Exact height-ball bound after a coordinate estimate.  This formulation
separates the arithmetic-height obligation (`coordinate_bound`) from the
finite counting argument. -/
theorem exact_height_ball_card_bound
    {d R : ℕ} (height : (Fin d → ℤ) → ℝ) (B : ℝ)
    (S : Finset (Fin d → ℤ))
    (is_ball : ∀ z, z ∈ S ↔ height z ≤ B)
    (coordinate_bound : ∀ z, height z ≤ B → ∀ i, |z i| ≤ R) :
    S.card ≤ (2 * R + 1) ^ d := by
  apply card_le_of_coordinate_bound S
  intro z hz
  exact coordinate_bound z ((is_ball z).mp hz)

/-- A fully finite bounded-relation theorem in the concrete lattice `ℤ^d`.

If all coefficient-box partial sums land in `[-R,R]^d` and the coefficient
box is larger than that lattice box, then a nonzero relation with
`|mᵢ| ≤ C` exists.  The threshold is the exact inequality
`(2R+1)^d < (C+1)^r`.
-/
theorem bounded_relation_of_integer_box
    {ι : Type*} [Fintype ι]
    (d R C : ℕ) (P : ι → (Fin d → ℤ))
    (partial_sum_bound : ∀ c : ι → Fin (C + 1), ∀ j,
      |(∑ i, (c i : ℤ) • P i) j| ≤ R)
    (box_lt : (2 * R + 1) ^ d < (C + 1) ^ Fintype.card ι) :
    ∃ m : ι → ℤ,
      m ≠ 0 ∧
      (∀ i, |m i| ≤ C) ∧
      ∑ i, m i • P i = 0 := by
  classical
  apply JKSSTAudit.bounded_relation_of_finite_range P C (integerBox d R)
  · intro c
    rw [mem_integerBox_iff]
    exact partial_sum_bound c
  · simpa using box_lt

end JKSSTHeightCount
