import Mathlib

/-!
# Rank and minimum do not determine an exact lattice-ball count

An exact formula depending only on lattice rank, minimum separation, and
radius cannot exist: lattice shape still matters.  This file gives a concrete
rank-two counterexample in the sup norm.

The two lattices are the images of `ℤ²` under the diagonal embeddings

* `(x,y) ↦ (x,y)`, and
* `(x,y) ↦ (x,2y)`.

Both have least nonzero sup norm `1`.  Their closed radius-one balls have,
respectively, `9` and `3` points.  The underlying parameter group is `ℤ²`
in both cases, so both lattices have rank two.
-/

namespace JKSSTCountNonDeterminacy

/-- The integral sup norm on ambient coordinate pairs. -/
def supHeight (z : ℤ × ℤ) : ℕ :=
  max z.1.natAbs z.2.natAbs

/-- The standard full-rank embedding. -/
def squareEmbedding (z : ℤ × ℤ) : ℤ × ℤ := z

/-- The vertically stretched full-rank embedding. -/
def rectangleEmbedding (z : ℤ × ℤ) : ℤ × ℤ :=
  (z.1, 2 * z.2)

theorem squareEmbedding_injective : Function.Injective squareEmbedding := by
  exact fun _ _ h ↦ h

theorem rectangleEmbedding_injective : Function.Injective rectangleEmbedding := by
  rintro ⟨x, y⟩ ⟨x', y'⟩ h
  simp only [rectangleEmbedding, Prod.mk.injEq] at h
  apply Prod.ext h.1
  omega

/-- Sup norm of the standard rectangular embedding of `ℤ²`. -/
def squareHeight (z : ℤ × ℤ) : ℕ :=
  max z.1.natAbs z.2.natAbs

/-- Sup norm after the full-rank diagonal embedding `(x,y) ↦ (x,2y)`. -/
def rectangleHeight (z : ℤ × ℤ) : ℕ :=
  max z.1.natAbs (2 * z.2.natAbs)

@[simp] theorem supHeight_squareEmbedding (z : ℤ × ℤ) :
    supHeight (squareEmbedding z) = squareHeight z := by
  rfl

@[simp] theorem supHeight_rectangleEmbedding (z : ℤ × ℤ) :
    supHeight (rectangleEmbedding z) = rectangleHeight z := by
  simp [supHeight, rectangleEmbedding, rectangleHeight, Int.natAbs_mul]

/-- The standard lattice has minimum nonzero norm at least one. -/
theorem one_le_squareHeight {z : ℤ × ℤ} (hz : z ≠ 0) :
    1 ≤ squareHeight z := by
  have hcoord : z.1 ≠ 0 ∨ z.2 ≠ 0 := by
    by_cases hx : z.1 = 0
    · right
      intro hy
      apply hz
      exact Prod.ext hx hy
    · exact Or.inl hx
  rcases hcoord with hx | hy
  · have hxpos : 0 < z.1.natAbs := Int.natAbs_pos.mpr hx
    exact hxpos.trans_le (le_max_left _ _)
  · have hypos : 0 < z.2.natAbs := Int.natAbs_pos.mpr hy
    exact hypos.trans_le (le_max_right _ _)

/-- The stretched rectangular lattice also has minimum nonzero norm at least
one. -/
theorem one_le_rectangleHeight {z : ℤ × ℤ} (hz : z ≠ 0) :
    1 ≤ rectangleHeight z := by
  have hcoord : z.1 ≠ 0 ∨ z.2 ≠ 0 := by
    by_cases hx : z.1 = 0
    · right
      intro hy
      apply hz
      exact Prod.ext hx hy
    · exact Or.inl hx
  rcases hcoord with hx | hy
  · have hxpos : 0 < z.1.natAbs := Int.natAbs_pos.mpr hx
    exact hxpos.trans_le (le_max_left _ _)
  · have hypos : 0 < z.2.natAbs := Int.natAbs_pos.mpr hy
    have : 1 ≤ 2 * z.2.natAbs := by omega
    exact this.trans (le_max_right _ _)

/-- Both lower bounds are sharp, at the same vector `(1,0)`. -/
@[simp] theorem squareHeight_one_zero : squareHeight (1, 0) = 1 := by
  simp [squareHeight]

@[simp] theorem rectangleHeight_one_zero : rectangleHeight (1, 0) = 1 := by
  simp [rectangleHeight]

private theorem natAbs_le_one_iff (x : ℤ) :
    x.natAbs ≤ 1 ↔ -1 ≤ x ∧ x ≤ 1 := by
  omega

/-- The radius-one ball in the standard lattice. -/
noncomputable def squareBall : Finset (ℤ × ℤ) :=
  (Finset.Icc (-1 : ℤ) 1).product (Finset.Icc (-1 : ℤ) 1)

/-- The radius-one ball in the vertically stretched lattice. -/
noncomputable def rectangleBall : Finset (ℤ × ℤ) :=
  (Finset.Icc (-1 : ℤ) 1).product {0}

@[simp] theorem mem_squareBall_iff (z : ℤ × ℤ) :
    z ∈ squareBall ↔ squareHeight z ≤ 1 := by
  rcases z with ⟨x, y⟩
  change (x, y) ∈ (Finset.Icc (-1 : ℤ) 1).product (Finset.Icc (-1 : ℤ) 1) ↔
    max x.natAbs y.natAbs ≤ 1
  constructor
  · intro hz
    have hprod := Finset.mem_product.mp hz
    exact max_le
      ((natAbs_le_one_iff x).2 (Finset.mem_Icc.mp hprod.1))
      ((natAbs_le_one_iff y).2 (Finset.mem_Icc.mp hprod.2))
  · intro hz
    have hmax := max_le_iff.mp hz
    exact Finset.mem_product.mpr ⟨
      Finset.mem_Icc.mpr ((natAbs_le_one_iff x).1 hmax.1),
      Finset.mem_Icc.mpr ((natAbs_le_one_iff y).1 hmax.2)⟩

@[simp] theorem mem_rectangleBall_iff (z : ℤ × ℤ) :
    z ∈ rectangleBall ↔ rectangleHeight z ≤ 1 := by
  rcases z with ⟨x, y⟩
  change (x, y) ∈ (Finset.Icc (-1 : ℤ) 1).product {0} ↔
    max x.natAbs (2 * y.natAbs) ≤ 1
  constructor
  · intro hz
    have hprod := Finset.mem_product.mp hz
    have hx := (natAbs_le_one_iff x).2 (Finset.mem_Icc.mp hprod.1)
    have hy : y = 0 := by simpa using hprod.2
    subst y
    simp [hx]
  · intro hz
    have hmax := max_le_iff.mp hz
    have hyAbs : y.natAbs = 0 := by omega
    have hy : y = 0 := Int.natAbs_eq_zero.mp hyAbs
    exact Finset.mem_product.mpr ⟨
      Finset.mem_Icc.mpr ((natAbs_le_one_iff x).1 hmax.1), by simpa⟩

/-- Exact count for the standard rank-two lattice. -/
@[simp] theorem card_squareBall : squareBall.card = 9 := by
  simp [squareBall, Int.card_Icc]

/-- Exact count for the stretched rank-two lattice. -/
@[simp] theorem card_rectangleBall : rectangleBall.card = 3 := by
  simp [rectangleBall, Int.card_Icc]

/-- Concrete non-determinacy: the two rank-two lattices have the same exact
minimum but different ball cardinalities at the same radius. -/
theorem same_minimum_different_radius_one_count :
    squareHeight (1, 0) = 1 ∧
    rectangleHeight (1, 0) = 1 ∧
    (∀ z ≠ 0, 1 ≤ squareHeight z) ∧
    (∀ z ≠ 0, 1 ≤ rectangleHeight z) ∧
    squareBall.card ≠ rectangleBall.card := by
  refine ⟨by simp, by simp, ?_, ?_, by simp⟩
  · exact fun _ hz ↦ one_le_squareHeight hz
  · exact fun _ hz ↦ one_le_rectangleHeight hz

/-! ## Euclidean (quadratic-height) version -/

/-- Squared Euclidean norm on the standard lattice. -/
def squareEuclideanHeightSq (z : ℤ × ℤ) : ℕ :=
  z.1.natAbs ^ 2 + z.2.natAbs ^ 2

/-- Squared Euclidean norm after the embedding `(x,y) ↦ (x,2y)`. -/
def rectangleEuclideanHeightSq (z : ℤ × ℤ) : ℕ :=
  z.1.natAbs ^ 2 + (2 * z.2.natAbs) ^ 2

theorem one_le_squareEuclideanHeightSq {z : ℤ × ℤ} (hz : z ≠ 0) :
    1 ≤ squareEuclideanHeightSq z := by
  have hcoord : z.1 ≠ 0 ∨ z.2 ≠ 0 := by
    by_cases hx : z.1 = 0
    · right
      intro hy
      exact hz (Prod.ext hx hy)
    · exact Or.inl hx
  rcases hcoord with hx | hy
  · have : 1 ≤ z.1.natAbs := Int.natAbs_pos.mpr hx
    simp only [squareEuclideanHeightSq]
    nlinarith
  · have : 1 ≤ z.2.natAbs := Int.natAbs_pos.mpr hy
    simp only [squareEuclideanHeightSq]
    nlinarith

theorem one_le_rectangleEuclideanHeightSq {z : ℤ × ℤ} (hz : z ≠ 0) :
    1 ≤ rectangleEuclideanHeightSq z := by
  have hcoord : z.1 ≠ 0 ∨ z.2 ≠ 0 := by
    by_cases hx : z.1 = 0
    · right
      intro hy
      exact hz (Prod.ext hx hy)
    · exact Or.inl hx
  rcases hcoord with hx | hy
  · have : 1 ≤ z.1.natAbs := Int.natAbs_pos.mpr hx
    simp only [rectangleEuclideanHeightSq]
    nlinarith
  · have : 1 ≤ z.2.natAbs := Int.natAbs_pos.mpr hy
    simp only [rectangleEuclideanHeightSq]
    nlinarith

@[simp] theorem squareEuclideanHeightSq_one_zero :
    squareEuclideanHeightSq (1, 0) = 1 := by
  simp [squareEuclideanHeightSq]

@[simp] theorem rectangleEuclideanHeightSq_one_zero :
    rectangleEuclideanHeightSq (1, 0) = 1 := by
  simp [rectangleEuclideanHeightSq]

/-- Closed Euclidean radius-one ball in the standard lattice, represented by
the equivalent squared-norm inequality. -/
def squareEuclideanBall : Finset (ℤ × ℤ) :=
  ((Finset.Icc (-1 : ℤ) 1).product (Finset.Icc (-1 : ℤ) 1)).filter
    (fun z ↦ squareEuclideanHeightSq z ≤ 1)

/-- Closed Euclidean radius-one ball in the stretched lattice. -/
def rectangleEuclideanBall : Finset (ℤ × ℤ) :=
  (Finset.Icc (-1 : ℤ) 1).product {0}

@[simp] theorem mem_squareEuclideanBall_iff (z : ℤ × ℤ) :
    z ∈ squareEuclideanBall ↔ squareEuclideanHeightSq z ≤ 1 := by
  rcases z with ⟨x, y⟩
  rw [squareEuclideanBall, Finset.mem_filter]
  have hmem : (x, y) ∈
      (Finset.Icc (-1 : ℤ) 1).product (Finset.Icc (-1 : ℤ) 1) ↔
      x ∈ Finset.Icc (-1 : ℤ) 1 ∧ y ∈ Finset.Icc (-1 : ℤ) 1 :=
    Finset.mem_product
  rw [hmem]
  constructor
  · exact fun h ↦ h.2
  · intro h
    have hx : x.natAbs ≤ 1 := by
      simp only [squareEuclideanHeightSq] at h
      nlinarith
    have hy : y.natAbs ≤ 1 := by
      simp only [squareEuclideanHeightSq] at h
      nlinarith
    exact ⟨⟨Finset.mem_Icc.mpr ((natAbs_le_one_iff x).1 hx),
      Finset.mem_Icc.mpr ((natAbs_le_one_iff y).1 hy)⟩, h⟩

@[simp] theorem mem_rectangleEuclideanBall_iff (z : ℤ × ℤ) :
    z ∈ rectangleEuclideanBall ↔ rectangleEuclideanHeightSq z ≤ 1 := by
  rcases z with ⟨x, y⟩
  rw [rectangleEuclideanBall]
  have hmem : (x, y) ∈ (Finset.Icc (-1 : ℤ) 1).product {0} ↔
      x ∈ Finset.Icc (-1 : ℤ) 1 ∧ y ∈ ({0} : Finset ℤ) :=
    Finset.mem_product
  rw [hmem]
  constructor
  · rintro ⟨hx, hy⟩
    have xbound : x.natAbs ≤ 1 :=
      (natAbs_le_one_iff x).2 (Finset.mem_Icc.mp hx)
    have yzero : y = 0 := by simpa using hy
    subst y
    simp only [rectangleEuclideanHeightSq, Int.natAbs_zero, mul_zero,
      zero_pow (by norm_num : (2 : ℕ) ≠ 0), add_zero]
    have hx_cases : x.natAbs = 0 ∨ x.natAbs = 1 := by omega
    rcases hx_cases with hx0 | hx1
    · simp [hx0]
    · simp [hx1]
  · intro h
    have xbound : x.natAbs ≤ 1 := by
      simp only [rectangleEuclideanHeightSq] at h
      nlinarith
    have yzeroAbs : y.natAbs = 0 := by
      simp only [rectangleEuclideanHeightSq] at h
      nlinarith
    have yzero : y = 0 := Int.natAbs_eq_zero.mp yzeroAbs
    exact ⟨Finset.mem_Icc.mpr ((natAbs_le_one_iff x).1 xbound), by simpa⟩

@[simp] theorem card_squareEuclideanBall : squareEuclideanBall.card = 5 := by
  decide

@[simp] theorem card_rectangleEuclideanBall : rectangleEuclideanBall.card = 3 := by
  simp [rectangleEuclideanBall, Int.card_Icc]

/-- In the inner-product-relevant Euclidean norm, rank, minimum, and radius
still do not determine the exact ball count: the counts are `5` and `3`. -/
theorem same_euclidean_minimum_different_radius_one_count :
    squareEuclideanHeightSq (1, 0) = 1 ∧
    rectangleEuclideanHeightSq (1, 0) = 1 ∧
    (∀ z ≠ 0, 1 ≤ squareEuclideanHeightSq z) ∧
    (∀ z ≠ 0, 1 ≤ rectangleEuclideanHeightSq z) ∧
    squareEuclideanBall.card ≠ rectangleEuclideanBall.card := by
  refine ⟨by simp, by simp, ?_, ?_, by simp⟩
  · exact fun _ hz ↦ one_le_squareEuclideanHeightSq hz
  · exact fun _ hz ↦ one_le_rectangleEuclideanHeightSq hz

end JKSSTCountNonDeterminacy
