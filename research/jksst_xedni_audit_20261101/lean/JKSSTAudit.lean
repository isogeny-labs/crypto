import Mathlib

/-!
# A skeptical formal audit of three JKSST proof obligations

This file audits three ingredients used by Jacobson--Koblitz--Silverman--
Stein--Teske, *Analysis of the Xedni Calculus Attack*, Designs, Codes and
Cryptography 20 (2000), 41--64, DOI 10.1023/A:1008312401197.

The results are deliberately stated with every hypothesis visible.  In
particular, the height argument below uses the *exact finite cardinality* of a
height ball.  It does not turn an asymptotic formula with an unspecified
`O`-constant into a strict finite bound.
-/

open scoped BigOperators

namespace JKSSTAudit

section UsefulRelation

variable {F ι : Type*} [Field F] [Fintype ι]

private theorem relation_expansion
    (x : F) (a b m : ι → F)
    (hrel : ∑ i, m i * (a i - x * b i) = 0) :
    (∑ i, m i * a i) - x * (∑ i, m i * b i) = 0 := by
  calc
    (∑ i, m i * a i) - x * (∑ i, m i * b i) =
        (∑ i, m i * a i) - ∑ i, x * (m i * b i) := by
          rw [Finset.mul_sum]
    _ = ∑ i, (m i * a i - x * (m i * b i)) := by
          rw [Finset.sum_sub_distrib]
    _ = ∑ i, m i * (a i - x * b i) := by
          apply Finset.sum_congr rfl
          intro i _
          ring
    _ = 0 := hrel

/-- **Obligation 1, useful case.**

Normalize a cyclic group of prime order by identifying its generator with
`1 : F`.  If `Q = xP`, then the sampled point `aᵢP - bᵢQ` has scalar
`aᵢ - x bᵢ`.  A relation recovers `x` exactly when its denominator is nonzero.
-/
theorem useful_relation_extract
    (x : F) (a b m : ι → F)
    (hrel : ∑ i, m i * (a i - x * b i) = 0)
    (useful : ∑ i, m i * b i ≠ 0) :
    x = (∑ i, m i * a i) / (∑ i, m i * b i) := by
  apply (eq_div_iff useful).2
  exact (sub_eq_zero.mp (relation_expansion x a b m hrel)).symm

/-- **Obligation 1, non-useful case.**

If the denominator vanishes, a valid lifted relation forces the numerator to
vanish as well.  It is then the identity `0 = 0` and reveals no discrete log.
-/
theorem zero_denominator_is_tautological
    (x : F) (a b m : ι → F)
    (hrel : ∑ i, m i * (a i - x * b i) = 0)
    (notUseful : ∑ i, m i * b i = 0) :
    ∑ i, m i * a i = 0 := by
  have h := sub_eq_zero.mp (relation_expansion x a b m hrel)
  simpa [notUseful] using h

end UsefulRelation

section ExactHeightBall

variable {G ι : Type*} [AddCommGroup G] [Fintype ι]

/-- The exact pigeonhole engine behind a bounded-coefficient relation.

All coefficient vectors in `{0, ..., C}^ι` map into the finite set `S`.  If
that box is strictly larger than `S`, two vectors collide.  Their difference
is a nonzero relation whose coefficients have absolute value at most `C`.

The strict cardinality threshold is exact; there is no asymptotic error term.
-/
theorem bounded_relation_of_finite_range
    (P : ι → G) (C : ℕ) (S : Finset G)
    (range_mem : ∀ c : ι → Fin (C + 1),
      (∑ i, (c i : ℤ) • P i) ∈ S)
    (card_lt : S.card < (C + 1) ^ Fintype.card ι) :
    ∃ m : ι → ℤ,
      m ≠ 0 ∧
      (∀ i, |m i| ≤ C) ∧
      ∑ i, m i • P i = 0 := by
  classical
  let boxSum : (ι → Fin (C + 1)) → S := fun c =>
    ⟨∑ i, (c i : ℤ) • P i, range_mem c⟩
  have boxSum_not_injective : ¬ Function.Injective boxSum := by
    intro boxSum_injective
    have card_le := Fintype.card_le_of_injective boxSum boxSum_injective
    simp only [Fintype.card_fun, Fintype.card_fin, Fintype.card_coe] at card_le
    omega
  simp only [Function.Injective, not_forall] at boxSum_not_injective
  push Not at boxSum_not_injective
  obtain ⟨c, d, sums_eq, cd_ne⟩ := boxSum_not_injective
  have value_sums_eq :
      (∑ i, (c i : ℤ) • P i) = ∑ i, (d i : ℤ) • P i :=
    congrArg Subtype.val sums_eq
  let m : ι → ℤ := fun i => (c i : ℤ) - (d i : ℤ)
  refine ⟨m, ?_, ?_, ?_⟩
  · intro m_zero
    apply cd_ne
    funext i
    apply Fin.ext
    have at_i := congrFun m_zero i
    simp only [m, Pi.zero_apply] at at_i
    omega
  · intro i
    have c_le : (c i : ℕ) ≤ C := Nat.le_of_lt_succ (c i).isLt
    have d_le : (d i : ℕ) ≤ C := Nat.le_of_lt_succ (d i).isLt
    simp only [m, abs_le]
    constructor <;> omega
  · calc
      ∑ i, m i • P i =
          ∑ i, ((c i : ℤ) • P i - (d i : ℤ) • P i) := by
            apply Finset.sum_congr rfl
            intro i _
            simp only [m, sub_smul]
      _ = (∑ i, (c i : ℤ) • P i) - ∑ i, (d i : ℤ) • P i := by
            rw [Finset.sum_sub_distrib]
      _ = 0 := sub_eq_zero.mpr value_sums_eq

/-- **Obligation 3.** An exact height-ball-to-coefficient theorem.

To use this for the Neron--Tate height, one must provide an actual finite set
`S` containing the indicated height ball and prove its exact cardinality is
strictly smaller than the coefficient box.  An asymptotic lattice-point count
with an unspecified `O`-constant is not enough for this theorem.
-/
theorem bounded_relation_of_exact_height_ball
    (P : ι → G) (height : G → ℝ) (B : ℝ) (C : ℕ) (S : Finset G)
    (partial_sum_height : ∀ c : ι → Fin (C + 1),
      height (∑ i, (c i : ℤ) • P i) ≤ B)
    (ball_mem : ∀ g, height g ≤ B → g ∈ S)
    (card_lt : S.card < (C + 1) ^ Fintype.card ι) :
    ∃ m : ι → ℤ,
      m ≠ 0 ∧
      (∀ i, |m i| ≤ C) ∧
      ∑ i, m i • P i = 0 := by
  apply bounded_relation_of_finite_range P C S
  · intro c
    exact ball_mem _ (partial_sum_height c)
  · exact card_lt

end ExactHeightBall

section HeightNormEstimate

variable {V ι : Type*} [NormedAddCommGroup V] [NormedSpace ℝ V] [Fintype ι]

/-- The rigorous form of JKSST's sentence "since the canonical height is a
metric".  The canonical height itself is quadratic and is not a metric;
its square root is the norm on the Mordell--Weil lattice modulo torsion.

This first inequality retains the individual coefficients and norms, so it
is tighter than replacing every term by its maximum.
-/
theorem coefficient_box_norm_bound_exact
    (P : ι → V) (C : ℕ) (c : ι → Fin (C + 1)) :
    ‖∑ i, (c i : ℤ) • P i‖ ≤ ∑ i, (c i : ℕ) * ‖P i‖ := by
  calc
    ‖∑ i, (c i : ℤ) • P i‖ ≤ ∑ i, ‖(c i : ℤ) • P i‖ := by
      simpa using
        (norm_sum_le Finset.univ (fun i => (c i : ℤ) • P i))
    _ = ∑ i, (c i : ℕ) * ‖P i‖ := by
      apply Finset.sum_congr rfl
      intro i _
      rw [norm_zsmul ℝ]
      simp

/-- The uniform version used to choose a single height-ball radius. -/
theorem coefficient_box_norm_bound
    (P : ι → V) (C : ℕ) (H : ℝ)
    (point_norm_le : ∀ i, ‖P i‖ ≤ H)
    (c : ι → Fin (C + 1)) :
    ‖∑ i, (c i : ℤ) • P i‖ ≤ Fintype.card ι * C * H := by
  calc
    ‖∑ i, (c i : ℤ) • P i‖ ≤ ∑ i, (c i : ℕ) * ‖P i‖ :=
      coefficient_box_norm_bound_exact P C c
    _ ≤ ∑ _i : ι, (C : ℝ) * H := by
      apply Finset.sum_le_sum
      intro i _
      have ci_le : (c i : ℕ) ≤ C := Nat.le_of_lt_succ (c i).isLt
      exact mul_le_mul (by exact_mod_cast ci_le) (point_norm_le i)
        (norm_nonneg _) (by positivity)
    _ = Fintype.card ι * C * H := by simp [mul_assoc]

/-- A directly usable exact norm-ball version of Obligation 3 for a discrete
group mapped into its ambient real normed space.

It is essential that `S` is a finite set of elements of `G`, not a finite set
purporting to contain an entire real ball in `V`.  In the Mordell--Weil
application, `G` is the finitely generated group and `π` is its projection to
the real vector space modulo torsion.  Finiteness, torsion fibers, and the
exact cardinality estimate remain explicit obligations.
-/
theorem bounded_relation_of_exact_norm_ball
    {G : Type*} [AddCommGroup G]
    (P : ι → G) (π : G →+ V) (C : ℕ) (H : ℝ)
    (point_norm_le : ∀ i, ‖π (P i)‖ ≤ H)
    (S : Finset G)
    (ball_mem : ∀ g, ‖π g‖ ≤ Fintype.card ι * C * H → g ∈ S)
    (card_lt : S.card < (C + 1) ^ Fintype.card ι) :
    ∃ m : ι → ℤ,
      m ≠ 0 ∧
      (∀ i, |m i| ≤ C) ∧
      ∑ i, m i • P i = 0 := by
  apply bounded_relation_of_exact_height_ball P (fun g => ‖π g‖)
    (Fintype.card ι * C * H) C S
  · intro c
    simpa using
      (coefficient_box_norm_bound (fun i => π (P i)) C H point_norm_le c)
  · exact ball_mem
  · exact card_lt

end HeightNormEstimate

section CubicParameterCount

variable {F ι : Type*} [Field F] [Fintype ι]

/-- The ten coefficients of a homogeneous ternary cubic. -/
abbrev TernaryCubicCoefficients (F : Type*) := Fin 10 → F

/-- **Obligation 5.** A nonzero ternary cubic can satisfy at most nine
independent homogeneous linear point conditions.

`evaluation` sends a coefficient vector to its values at the prescribed
points.  Surjectivity says the point conditions are independent.  A nonzero
cubic through every point is a nonzero element of its kernel.  Rank-nullity
then gives `#points ≤ 9`.
-/
theorem independent_cubic_conditions_le_nine
    (evaluation : TernaryCubicCoefficients F →ₗ[F] (ι → F))
    (independent_conditions : Function.Surjective evaluation)
    (cubic : TernaryCubicCoefficients F)
    (cubic_ne_zero : cubic ≠ 0)
    (vanishes : evaluation cubic = 0) :
    Fintype.card ι ≤ 9 := by
  have kernel_nontrivial : 0 < Module.finrank F (LinearMap.ker evaluation) := by
    apply Module.finrank_pos_iff_exists_ne_zero.mpr
    exact ⟨⟨cubic, vanishes⟩, by
      intro h
      apply cubic_ne_zero
      exact congrArg Subtype.val h⟩
  have range_eq_top : LinearMap.range evaluation = ⊤ :=
    LinearMap.range_eq_top.mpr independent_conditions
  have rank_nullity := LinearMap.finrank_range_add_finrank_ker evaluation
  rw [range_eq_top, finrank_top] at rank_nullity
  have exact_dimension :
      Fintype.card ι + Module.finrank F (LinearMap.ker evaluation) = 10 := by
    simpa [TernaryCubicCoefficients] using rank_nullity
  omega

end CubicParameterCount

end JKSSTAudit
