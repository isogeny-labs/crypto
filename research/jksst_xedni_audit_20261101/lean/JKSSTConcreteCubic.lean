import JKSSTAudit

/-!
# Concrete ternary-cubic evaluation for the JKSST dimension count

The original audit stated the `r ≤ 9` result for an arbitrary linear map from
ten coefficients.  This file constructs the actual evaluation map of a
homogeneous ternary cubic at projective-coordinate triples and restates the
rank argument with that concrete map.
-/

open scoped BigOperators

namespace JKSSTConcreteCubic

variable {F ι : Type*} [Field F] [Fintype ι]

/-- A coordinate triple.  Projective equivalence is not needed for the linear
dimension count, so points are represented by triples directly. -/
structure Point3 (F : Type*) where
  x : F
  y : F
  z : F

/-- Coefficients of a homogeneous ternary cubic, in the monomial order

`X³, X²Y, X²Z, XY², XYZ, XZ², Y³, Y²Z, YZ², Z³`.
-/
abbrev Coefficients (F : Type*) := Fin 10 → F

/-- The ten degree-three monomials evaluated at a coordinate triple. -/
def cubicMonomials (p : Point3 F) : Fin 10 → F :=
  ![p.x ^ 3,
    p.x ^ 2 * p.y,
    p.x ^ 2 * p.z,
    p.x * p.y ^ 2,
    p.x * p.y * p.z,
    p.x * p.z ^ 2,
    p.y ^ 3,
    p.y ^ 2 * p.z,
    p.y * p.z ^ 2,
    p.z ^ 3]

/-- Coordinate scaling used to expose homogeneity. -/
def scalePoint (a : F) (p : Point3 F) : Point3 F :=
  ⟨a * p.x, a * p.y, a * p.z⟩

@[simp] theorem cubicMonomials_scale
    (a : F) (p : Point3 F) (j : Fin 10) :
    cubicMonomials (scalePoint a p) j = a ^ 3 * cubicMonomials p j := by
  fin_cases j <;> simp [cubicMonomials, scalePoint] <;> ring

/-- Value of the concrete cubic at `p`. -/
def cubicValue (c : Coefficients F) (p : Point3 F) : F :=
  ∑ j, c j * cubicMonomials p j

/-- The concrete polynomial is homogeneous of degree three. -/
theorem cubicValue_scale (a : F) (c : Coefficients F) (p : Point3 F) :
    cubicValue c (scalePoint a p) = a ^ 3 * cubicValue c p := by
  simp only [cubicValue, cubicMonomials_scale]
  calc
    ∑ j, c j * (a ^ 3 * cubicMonomials p j) =
        ∑ j, a ^ 3 * (c j * cubicMonomials p j) := by
      apply Finset.sum_congr rfl
      intro j _
      ring
    _ = a ^ 3 * ∑ j, c j * cubicMonomials p j := by
      rw [Finset.mul_sum]

/-- Evaluation at one point is linear in the ten coefficients. -/
noncomputable def evalAt (p : Point3 F) : Coefficients F →ₗ[F] F :=
  Fintype.linearCombination F (cubicMonomials p)

@[simp] theorem evalAt_apply (p : Point3 F) (c : Coefficients F) :
    evalAt p c = cubicValue c p := by
  rfl

/-- A coordinate triple lies on the cubic precisely when its concrete
evaluation vanishes. -/
def LiesOn (c : Coefficients F) (p : Point3 F) : Prop :=
  cubicValue c p = 0

@[simp] theorem evalAt_eq_zero_iff_liesOn (p : Point3 F) (c : Coefficients F) :
    evalAt p c = 0 ↔ LiesOn c p := by
  rfl

/-- Simultaneous evaluation at a finite indexed family of points. -/
noncomputable def evaluation (points : ι → Point3 F) :
    Coefficients F →ₗ[F] (ι → F) :=
  LinearMap.pi fun i ↦ evalAt (points i)

omit [Fintype ι] in
@[simp] theorem evaluation_apply
    (points : ι → Point3 F) (c : Coefficients F) (i : ι) :
    evaluation points c i = cubicValue c (points i) := by
  rfl

/-- The concrete point conditions are independent when their evaluation map
has full row rank. -/
def FullRowRank (points : ι → Point3 F) : Prop :=
  Module.finrank F (LinearMap.range (evaluation points)) = Fintype.card ι

/-- Full row rank is equivalent to independent/surjective point conditions. -/
theorem fullRowRank_iff_surjective (points : ι → Point3 F) :
    FullRowRank points ↔ Function.Surjective (evaluation points) := by
  constructor
  · intro h
    apply LinearMap.range_eq_top.mp
    apply Submodule.eq_top_of_finrank_eq
    calc
      Module.finrank F (LinearMap.range (evaluation points)) =
          Fintype.card ι := h
      _ = Module.finrank F (ι → F) := by simp
  · intro h
    unfold FullRowRank
    rw [LinearMap.range_eq_top.mpr h]
    simp

/-- A nonzero concrete homogeneous ternary cubic through a family of points
whose evaluation conditions have full row rank forces at most nine points. -/
theorem fullRowRank_points_on_nonzero_cubic_le_nine
    (points : ι → Point3 F)
    (fullRank : FullRowRank points)
    (c : Coefficients F) (c_ne_zero : c ≠ 0)
    (onCubic : ∀ i, LiesOn c (points i)) :
    Fintype.card ι ≤ 9 := by
  apply JKSSTAudit.independent_cubic_conditions_le_nine
    (evaluation points) (fullRowRank_iff_surjective points |>.mp fullRank)
    c c_ne_zero
  funext i
  exact onCubic i

/-- Equivalent formulation using surjectivity directly. -/
theorem independent_points_on_nonzero_cubic_le_nine
    (points : ι → Point3 F)
    (independent : Function.Surjective (evaluation points))
    (c : Coefficients F) (c_ne_zero : c ≠ 0)
    (onCubic : ∀ i, LiesOn c (points i)) :
    Fintype.card ι ≤ 9 := by
  exact fullRowRank_points_on_nonzero_cubic_le_nine points
    ((fullRowRank_iff_surjective points).2 independent) c c_ne_zero onCubic

end JKSSTConcreteCubic
