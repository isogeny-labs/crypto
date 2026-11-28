import ConcreteRelation

/-! A coefficient-box radius from individual point norm bounds. -/

namespace XedniHeightRadius

open XedniUniformQuotient XedniFreeTorsionQuotient XedniConcreteRelation

variable {H A : Type*} [AddCommGroup H]

/-- A subadditive integer-homogeneous seminorm bounds every coefficient-box
sum by the number of points times the coefficient radius times the maximum
point norm. -/
theorem coefficient_box_sum_radius (r K : ℕ) (points : Fin r → H)
    (norm : H → ℝ) (U : ℝ)
    (hzero : norm 0 = 0)
    (hadd : ∀ x y, norm (x + y) ≤ norm x + norm y)
    (hsmul : ∀ (a : ℤ) x, norm (a • x) ≤ (a.natAbs : ℝ) * norm x)
    (hpoint : ∀ i, norm (points i) ≤ U)
    (hU : 0 ≤ U) :
    ∀ coeff ∈ coefficientBox r K,
      norm (eval points coeff) ≤ (r : ℝ) * (K : ℝ) * U := by
  intro coeff hbox
  have hsum : ∀ s : Finset (Fin r),
      norm (∑ i ∈ s, coeff i • points i) ≤
        ∑ i ∈ s, ((coeff i).natAbs : ℝ) * U := by
    intro s
    induction s using Finset.induction_on with
    | empty => simp [hzero]
    | @insert i s hi ih =>
      simp only [Finset.sum_insert hi]
      calc
        norm (coeff i • points i + ∑ j ∈ s, coeff j • points j) ≤
            norm (coeff i • points i) + norm (∑ j ∈ s, coeff j • points j) :=
          hadd _ _
        _ ≤ ((coeff i).natAbs : ℝ) * norm (points i) +
            ∑ j ∈ s, ((coeff j).natAbs : ℝ) * U :=
          add_le_add (hsmul _ _) ih
        _ ≤ ((coeff i).natAbs : ℝ) * U +
            ∑ j ∈ s, ((coeff j).natAbs : ℝ) * U := by
          gcongr
          exact hpoint i
  have hcoeff : ∀ i : Fin r, ((coeff i).natAbs : ℝ) ≤ K := by
    intro i
    exact_mod_cast coefficientBox_bounded hbox i
  calc
    norm (eval points coeff) ≤ ∑ i : Fin r, ((coeff i).natAbs : ℝ) * U := by
      simpa [eval] using hsum Finset.univ
    _ ≤ ∑ _i : Fin r, (K : ℝ) * U := by
      apply Finset.sum_le_sum
      intro i _
      exact mul_le_mul_of_nonneg_right (hcoeff i) hU
    _ = (r : ℝ) * (K : ℝ) * U := by simp [mul_assoc]

/-- The concrete free-plus-torsion theorem needs only individual point
norm bounds once the free-coordinate seminorm laws are supplied. -/
theorem bounded_relation_from_point_heights [AddCommGroup A] [Fintype A]
    (d r c K N : ℕ) [NeZero N]
    (points : Fin r → LiftGroup (A := A) d)
    (freeNorm : (Fin d → ℤ) → ℝ) (m U : ℝ)
    (hdegree : d < r) (hN : N ≤ c * K)
    (hlarge : Fintype.card A * c ^ d < K)
    (hzero : freeNorm 0 = 0)
    (hadd : ∀ x y, freeNorm (x + y) ≤ freeNorm x + freeNorm y)
    (hsmul : ∀ (a : ℤ) x,
      freeNorm (a • x) ≤ (a.natAbs : ℝ) * freeNorm x)
    (hpoint : ∀ i, freeNorm (points i).1 ≤ U)
    (hU : 0 ≤ U)
    (hscale : ∀ u, freeNorm (N • u) = (N : ℝ) * freeNorm u)
    (hkill : ∀ a : A, N • a = 0)
    (hmin : ∀ u, u ≠ 0 → m ≤ freeNorm u)
    (hgap : ((r : ℝ) * K * U) + ((r : ℝ) * K * U) < (N : ℝ) * m) :
    ∃ coeff, (∃ i, coeff i ≠ 0) ∧ Bounded (2 * K) coeff ∧
      eval points coeff = 0 := by
  let liftNorm : LiftGroup (A := A) d → ℝ := fun z => freeNorm z.1
  have htriangle : ∀ x y, freeNorm (x - y) ≤ freeNorm x + freeNorm y := by
    intro x y
    have hneg : freeNorm (-y) ≤ freeNorm y := by
      simpa using hsmul (-1) y
    calc
      freeNorm (x - y) ≤ freeNorm x + freeNorm (-y) := by
        simpa [sub_eq_add_neg] using hadd x (-y)
      _ ≤ freeNorm x + freeNorm y := add_le_add_right hneg _
  have hbox : ∀ coeff ∈ coefficientBox r K,
      freeNorm (eval points coeff).1 ≤ (r : ℝ) * K * U := by
    apply coefficient_box_sum_radius r K points liftNorm U
    · exact hzero
    · intro x y; exact hadd x.1 y.1
    · intro a x; exact hsmul a x.1
    · exact hpoint
    · exact hU
  exact bounded_relation_of_free_torsion_rank d r c K N points freeNorm m
    ((r : ℝ) * K * U) hdegree hN hlarge htriangle hbox hscale hkill
    hmin hgap

#print axioms XedniHeightRadius.coefficient_box_sum_radius
#print axioms XedniHeightRadius.bounded_relation_from_point_heights

end XedniHeightRadius
