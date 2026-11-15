import Mathlib.NumberTheory.Height.EllipticCurve

/-!
# Canonicalizing an approximately quadratic height

Mathlib currently proves the approximate parallelogram law for the naïve
height of an elliptic curve, but does not yet package the associated
canonical-height limit.  This file supplies the general analytic
construction.

For an additive commutative group `G` and `h : G → ℝ`, assume `h 0 = 0`
and

`|h(x+y) + h(x-y) - 2(h x + h y)| ≤ C`.

The sequence `h(2^n x)/4^n` is Cauchy.  Its limit is within `C/3` of `h`,
satisfies exact doubling, and satisfies the exact parallelogram law.  These
are the kernel-checked analytic properties needed to construct the canonical
height from Mathlib's naïve elliptic-curve height.

Mathlib's naïve height is the projective height of the `x`-coordinate, hence
is a Weil height for `2[O]`.  The direct limit below is the canonical height
in that `x`-height normalization.  With the common convention that the
Néron--Tate height is attached to `[O]`, it is one half of this limit; both
normalizations are defined below so the factor of two is never implicit.

The constant supplied by Mathlib's approximate parallelogram theorem is
existential and model-dependent.  Nothing here bounds it by a minimal
discriminant, makes it uniform over the xedni lift family, or proves the
JKSST height/discriminant premise.
-/

open Filter
open scoped Topology

namespace JKSSTCanonicalHeight

variable {G : Type*} [AddCommGroup G]

/-- Iterated doubling in an additive group. -/
def doubleIter (x : G) : ℕ → G
  | 0 => x
  | n + 1 => 2 • doubleIter x n

@[simp] theorem doubleIter_zero (x : G) : doubleIter x 0 = x := rfl

@[simp] theorem doubleIter_succ (x : G) (n : ℕ) :
    doubleIter x (n + 1) = 2 • doubleIter x n := rfl

@[simp] theorem doubleIter_add (x y : G) (n : ℕ) :
    doubleIter (x + y) n = doubleIter x n + doubleIter y n := by
  induction n with
  | zero => rfl
  | succ n ih => simp [doubleIter, ih]

@[simp] theorem doubleIter_sub (x y : G) (n : ℕ) :
    doubleIter (x - y) n = doubleIter x n - doubleIter y n := by
  induction n with
  | zero => rfl
  | succ n ih => simp [doubleIter, ih, nsmul_sub]

@[simp] theorem doubleIter_two_nsmul (x : G) (n : ℕ) :
    doubleIter (2 • x) n = doubleIter x (n + 1) := by
  induction n with
  | zero => rfl
  | succ n ih => simp [doubleIter, ih]

/-- The normalized doubling sequence. -/
noncomputable def scaledSequence (h : G → ℝ) (x : G) (n : ℕ) : ℝ :=
  h (doubleIter x n) / (4 : ℝ) ^ n

/-- The canonical limit attached to `h`.  Convergence is proved below from
the approximate parallelogram law. -/
noncomputable def canonicalize (h : G → ℝ) (x : G) : ℝ :=
  limUnder atTop (scaledSequence h x)

/-- Explicit approximate-parallelogram hypothesis. -/
def ApproxParallelogram (h : G → ℝ) (C : ℝ) : Prop :=
  h 0 = 0 ∧ 0 ≤ C ∧
    ∀ x y, |h (x + y) + h (x - y) - 2 * (h x + h y)| ≤ C

theorem ApproxParallelogram.doubling_error
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) :
    |h (2 • x) - 4 * h x| ≤ C := by
  have happ := H.2.2 x x
  have hfour : 2 * (h x + h x) = 4 * h x := by ring
  rw [two_nsmul]
  simpa [H.1, hfour] using happ

/-- Consecutive normalized values differ by a geometric error. -/
theorem scaledSequence_step
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C)
    (x : G) (n : ℕ) :
    dist (scaledSequence h x n) (scaledSequence h x (n + 1)) ≤
      (C / 4) * (1 / 4 : ℝ) ^ n := by
  have herr := H.doubling_error (doubleIter x n)
  have hpow : (4 : ℝ) ^ n ≠ 0 := pow_ne_zero _ (by norm_num)
  have hpow' : (4 : ℝ) ^ (n + 1) ≠ 0 := pow_ne_zero _ (by norm_num)
  rw [Real.dist_eq]
  calc
    |scaledSequence h x n - scaledSequence h x (n + 1)| =
        |(4 * h (doubleIter x n) - h (2 • doubleIter x n)) /
          (4 : ℝ) ^ (n + 1)| := by
      congr 1
      rw [scaledSequence, scaledSequence, doubleIter_succ, pow_succ]
      field_simp
    _ =
        |h (2 • doubleIter x n) - 4 * h (doubleIter x n)| /
          (4 : ℝ) ^ (n + 1) := by
      rw [abs_div, abs_of_pos (by positivity : (0 : ℝ) < 4 ^ (n + 1))]
      congr 1
      exact abs_sub_comm _ _
    _ ≤ C / (4 : ℝ) ^ (n + 1) := by
      exact div_le_div_of_nonneg_right herr (by positivity)
    _ = (C / 4) * (1 / 4 : ℝ) ^ n := by
      rw [one_div_pow, pow_succ]
      field_simp

/-- The normalized sequence is Cauchy. -/
theorem scaledSequence_cauchy
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) :
    CauchySeq (scaledSequence h x) := by
  apply cauchySeq_of_le_geometric (r := (1 / 4 : ℝ)) (C := C / 4)
  · norm_num
  · exact scaledSequence_step H x

/-- The normalized sequence tends to the declared canonical limit. -/
theorem tendsto_canonicalize
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) :
    Tendsto (scaledSequence h x) atTop (𝓝 (canonicalize h x)) :=
  (scaledSequence_cauchy H x).tendsto_limUnder

/-- The canonical limit differs from the original approximate height by at
most `C/3`. -/
theorem abs_canonicalize_sub_le
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) :
    |canonicalize h x - h x| ≤ C / 3 := by
  have hd := dist_le_of_le_geometric_of_tendsto₀
    (f := scaledSequence h x) (1 / 4 : ℝ) (C / 4) (by norm_num)
    (scaledSequence_step H x) (tendsto_canonicalize H x)
  have hconst : (C / 4) / (1 - (1 / 4 : ℝ)) = C / 3 := by ring
  rw [hconst] at hd
  simpa [scaledSequence, Real.dist_eq, abs_sub_comm] using hd

@[simp] theorem doubleIter_group_zero (n : ℕ) :
    doubleIter (0 : G) n = 0 := by
  induction n with
  | zero => rfl
  | succ n ih => simp [doubleIter, ih]

/-- The canonicalized height vanishes at zero. -/
theorem canonicalize_zero
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) :
    canonicalize h 0 = 0 := by
  have hseq : scaledSequence h (0 : G) = fun _ => 0 := by
    funext n
    simp [scaledSequence, H.1]
  have ht := tendsto_canonicalize H (0 : G)
  have hz : Tendsto (scaledSequence h (0 : G)) atTop (𝓝 0) := by
    rw [hseq]
    exact tendsto_const_nhds
  exact tendsto_nhds_unique ht hz

theorem scaledSequence_two_nsmul
    (h : G → ℝ) (x : G) (n : ℕ) :
    scaledSequence h (2 • x) n = 4 * scaledSequence h x (n + 1) := by
  rw [scaledSequence, scaledSequence, doubleIter_two_nsmul, pow_succ]
  field_simp

/-- The canonicalized height satisfies exact quadratic doubling. -/
theorem canonicalize_two_nsmul
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) :
    canonicalize h (2 • x) = 4 * canonicalize h x := by
  have ht := tendsto_canonicalize H (2 • x)
  have hxshift :=
    (tendsto_canonicalize H x).comp (tendsto_add_atTop_nat 1)
  have hscaled : Tendsto (fun n => 4 * scaledSequence h x (n + 1)) atTop
      (𝓝 (4 * canonicalize h x)) :=
    tendsto_const_nhds.mul hxshift
  have ht' : Tendsto (scaledSequence h (2 • x)) atTop
      (𝓝 (4 * canonicalize h x)) := by
    convert hscaled using 1
    funext n
    exact scaledSequence_two_nsmul h x n
  exact tendsto_nhds_unique ht ht'

/-- The normalized parallelogram error at level `n`. -/
noncomputable def scaledParallelogramError (h : G → ℝ) (x y : G) (n : ℕ) : ℝ :=
  scaledSequence h (x + y) n + scaledSequence h (x - y) n -
    2 * (scaledSequence h x n + scaledSequence h y n)

theorem scaledParallelogramError_eq
    (h : G → ℝ) (x y : G) (n : ℕ) :
    scaledParallelogramError h x y n =
      (h (doubleIter x n + doubleIter y n) +
        h (doubleIter x n - doubleIter y n) -
          2 * (h (doubleIter x n) + h (doubleIter y n))) /
            (4 : ℝ) ^ n := by
  simp only [scaledParallelogramError, scaledSequence,
    doubleIter_add, doubleIter_sub]
  field_simp

theorem abs_scaledParallelogramError_le
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C)
    (x y : G) (n : ℕ) :
    |scaledParallelogramError h x y n| ≤ C / (4 : ℝ) ^ n := by
  rw [scaledParallelogramError_eq, abs_div,
    abs_of_pos (by positivity : (0 : ℝ) < 4 ^ n)]
  exact div_le_div_of_nonneg_right (H.2.2 (doubleIter x n) (doubleIter y n))
    (by positivity)

/-- The bounded error disappears in the limit, giving the exact
parallelogram law. -/
theorem canonicalize_parallelogram
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x y : G) :
    canonicalize h (x + y) + canonicalize h (x - y) =
      2 * (canonicalize h x + canonicalize h y) := by
  have hpow : Tendsto (fun n : ℕ => (1 / 4 : ℝ) ^ n) atTop (𝓝 0) :=
    tendsto_pow_atTop_nhds_zero_of_abs_lt_one (by norm_num)
  have hright : Tendsto (fun n : ℕ => C / (4 : ℝ) ^ n) atTop (𝓝 0) := by
    have hmul : Tendsto (fun n : ℕ => C * (1 / 4 : ℝ) ^ n) atTop
        (𝓝 (C * 0)) := tendsto_const_nhds.mul hpow
    convert hmul using 1
    · funext n
      rw [one_div_pow]
      ring
    · ring_nf
  have habs : Tendsto (fun n => |scaledParallelogramError h x y n|)
      atTop (𝓝 0) :=
    squeeze_zero' (Eventually.of_forall fun _ => abs_nonneg _)
      (Eventually.of_forall fun n => abs_scaledParallelogramError_le H x y n)
      hright
  have herr : Tendsto (scaledParallelogramError h x y) atTop (𝓝 0) :=
    (tendsto_zero_iff_abs_tendsto_zero _).2 habs
  have hxy := (tendsto_canonicalize H (x + y)).add
    (tendsto_canonicalize H (x - y))
  have hsum := (tendsto_canonicalize H x).add (tendsto_canonicalize H y)
  have htwo : Tendsto (fun n => 2 *
      (scaledSequence h x n + scaledSequence h y n)) atTop
      (𝓝 (2 * (canonicalize h x + canonicalize h y))) :=
    tendsto_const_nhds.mul hsum
  have hcombined : Tendsto (scaledParallelogramError h x y) atTop
      (𝓝 (canonicalize h (x + y) + canonicalize h (x - y) -
        2 * (canonicalize h x + canonicalize h y))) := by
    exact hxy.sub htwo
  have hz := tendsto_nhds_unique hcombined herr
  linarith

/-- Exact evenness follows from the parallelogram identity and vanishing at
zero. -/
theorem canonicalize_neg
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) :
    canonicalize h (-x) = canonicalize h x := by
  have hp := canonicalize_parallelogram H (0 : G) x
  rw [zero_add, zero_sub, canonicalize_zero H] at hp
  linarith

/-- The canonicalized height has the expected square law for every natural
multiple.  This is a convenient scalar form of quadraticity in addition to
the exact parallelogram law. -/
theorem canonicalize_nsmul
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C) (x : G) (n : ℕ) :
    canonicalize h (n • x) = (n : ℝ) ^ 2 * canonicalize h x := by
  induction n using Nat.twoStepInduction with
  | zero => simp [canonicalize_zero H]
  | one => simp
  | more n hn hn1 =>
      have hp := canonicalize_parallelogram H ((n + 1) • x) x
      have hadd : (n + 1) • x + x = (n + 2) • x := by
        simp [add_nsmul]
        abel
      have hsub : (n + 1) • x - x = n • x := by
        simp [add_nsmul]
      rw [hadd, hsub, hn, hn1] at hp
      calc
        canonicalize h ((n + 2) • x) =
            2 * (((n + 1 : ℕ) : ℝ) ^ 2 * canonicalize h x +
              canonicalize h x) - (n : ℝ) ^ 2 * canonicalize h x := by
          linarith
        _ = ((n + 2 : ℕ) : ℝ) ^ 2 * canonicalize h x := by
          push_cast
          ring

/-- A nonnegative approximately quadratic function has a nonnegative
canonicalization, since every term in the normalized sequence is
nonnegative. -/
theorem canonicalize_nonneg
    {h : G → ℝ} {C : ℝ} (H : ApproxParallelogram h C)
    (h_nonneg : ∀ x, 0 ≤ h x) (x : G) :
    0 ≤ canonicalize h x := by
  apply ge_of_tendsto (tendsto_canonicalize H x)
  exact Eventually.of_forall fun n =>
    div_nonneg (h_nonneg (doubleIter x n)) (by positivity)

end JKSSTCanonicalHeight

namespace WeierstrassCurve.Affine

open Height Filter
open scoped Topology
open JKSSTCanonicalHeight

variable {F : Type*} [Field F] [AdmissibleAbsValues F]
variable {W : Affine F}

@[simp] theorem Point.naiveHeight_zero :
    (0 : W.Point).naiveHeight = 0 := by
  rw [Point.naiveHeight_eq_logHeight₁]
  simp

theorem Point.naiveHeight_nonneg (P : W.Point) :
    0 ≤ P.naiveHeight := by
  rw [Point.naiveHeight_eq_logHeight]
  exact Height.logHeight_nonneg _

variable [W.IsElliptic] [DecidableEq F]

/-- Mathlib's approximate parallelogram theorem, packaged with a
nonnegative constant and the normalization at the identity needed by the
canonicalization construction. -/
theorem exists_naiveHeight_approxParallelogram :
    ∃ C : ℝ, ApproxParallelogram (fun P : W.Point => P.naiveHeight) C := by
  obtain ⟨C, hC⟩ := approx_parallelogram_law W
  refine ⟨max C 0, Point.naiveHeight_zero, le_max_right C 0, ?_⟩
  intro P Q
  exact (hC P Q).trans (le_max_left C 0)

/-- The canonical `x`-height obtained as the normalized doubling limit of
Mathlib's naïve height.  This is the canonical height for `2[O]`; see
`Point.neronTateHeight` for the common `[O]` normalization. -/
noncomputable def Point.canonicalHeight (P : W.Point) : ℝ :=
  canonicalize (fun Q : W.Point => Q.naiveHeight) P

/-- The common Néron--Tate normalization attached to `[O]`.  Since the naïve
`x`-height corresponds to `2[O]`, this is half of `canonicalHeight`. -/
noncomputable def Point.neronTateHeight (P : W.Point) : ℝ :=
  P.canonicalHeight / 2

/-- The defining normalized sequence converges to `canonicalHeight`. -/
theorem Point.tendsto_scaledSequence_naiveHeight (P : W.Point) :
    Tendsto
      (scaledSequence (fun Q : W.Point => Q.naiveHeight) P)
      atTop (𝓝 P.canonicalHeight) := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  exact tendsto_canonicalize H P

/-- A fully explicit uniform comparison: for some nonnegative
approximate-parallelogram constant `C`, the canonical and naïve heights
differ everywhere by at most `C/3`. -/
theorem exists_abs_canonicalHeight_sub_naiveHeight_le :
    ∃ C : ℝ, 0 ≤ C ∧
      (∀ P : W.Point, |P.canonicalHeight - P.naiveHeight| ≤ C / 3) := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  exact ⟨C, H.2.1, fun P => abs_canonicalize_sub_le H P⟩

/-- The corresponding comparison in the `[O]` Néron--Tate normalization.
The `C/6` is exactly the preceding `C/3` geometric-series bound divided by
two. -/
theorem exists_abs_neronTateHeight_sub_half_naiveHeight_le :
    ∃ C : ℝ, 0 ≤ C ∧
      (∀ P : W.Point,
        |P.neronTateHeight - P.naiveHeight / 2| ≤ C / 6) := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  refine ⟨C, H.2.1, fun P => ?_⟩
  have hb := abs_canonicalize_sub_le H P
  calc
    |P.neronTateHeight - P.naiveHeight / 2| =
        |P.canonicalHeight - P.naiveHeight| / 2 := by
      rw [Point.neronTateHeight, ← sub_div, abs_div,
        abs_of_pos (by norm_num : (0 : ℝ) < 2)]
    _ ≤ (C / 3) / 2 := div_le_div_of_nonneg_right hb (by norm_num)
    _ = C / 6 := by ring

@[simp] theorem Point.canonicalHeight_zero :
    (0 : W.Point).canonicalHeight = 0 := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  exact canonicalize_zero H

theorem Point.canonicalHeight_nonneg (P : W.Point) :
    0 ≤ P.canonicalHeight := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  exact canonicalize_nonneg H Point.naiveHeight_nonneg P

@[simp] theorem Point.neronTateHeight_zero :
    (0 : W.Point).neronTateHeight = 0 := by
  simp [Point.neronTateHeight]

theorem Point.neronTateHeight_nonneg (P : W.Point) :
    0 ≤ P.neronTateHeight := by
  exact div_nonneg P.canonicalHeight_nonneg (by norm_num)

theorem Point.canonicalHeight_parallelogram (P Q : W.Point) :
    (P + Q).canonicalHeight + (P - Q).canonicalHeight =
      2 * (P.canonicalHeight + Q.canonicalHeight) := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  exact canonicalize_parallelogram H P Q

theorem Point.neronTateHeight_parallelogram (P Q : W.Point) :
    (P + Q).neronTateHeight + (P - Q).neronTateHeight =
      2 * (P.neronTateHeight + Q.neronTateHeight) := by
  rw [Point.neronTateHeight, Point.neronTateHeight,
    Point.neronTateHeight, Point.neronTateHeight]
  have hp := P.canonicalHeight_parallelogram Q
  linarith

@[simp] theorem Point.canonicalHeight_nsmul (P : W.Point) (n : ℕ) :
    (n • P).canonicalHeight = (n : ℝ) ^ 2 * P.canonicalHeight := by
  obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
  exact canonicalize_nsmul H P n

@[simp] theorem Point.neronTateHeight_nsmul (P : W.Point) (n : ℕ) :
    (n • P).neronTateHeight = (n : ℝ) ^ 2 * P.neronTateHeight := by
  rw [Point.neronTateHeight, Point.canonicalHeight_nsmul,
    Point.neronTateHeight]
  ring

/-- Torsion points have canonical height zero.  This direction uses only the
quadratic scalar law, not Northcott. -/
theorem Point.canonicalHeight_eq_zero_of_mem_torsion
    (P : W.Point) (hP : P ∈ AddCommGroup.torsion W.Point) :
    P.canonicalHeight = 0 := by
  have hfin : IsOfFinAddOrder P := (AddCommGroup.mem_torsion P).mp hP
  obtain ⟨n, hn_pos, hnP⟩ := hfin.exists_nsmul_eq_zero
  have hquad := P.canonicalHeight_nsmul n
  rw [hnP, Point.canonicalHeight_zero] at hquad
  have hn_ne : (n : ℝ) ^ 2 ≠ 0 := by
    positivity
  exact (mul_eq_zero.mp hquad.symm).resolve_left hn_ne

theorem Point.neronTateHeight_eq_zero_of_mem_torsion
    (P : W.Point) (hP : P ∈ AddCommGroup.torsion W.Point) :
    P.neronTateHeight = 0 := by
  rw [Point.neronTateHeight,
    Point.canonicalHeight_eq_zero_of_mem_torsion P hP, zero_div]

section Northcott

variable [Northcott (logHeight₁ (K := F))]

/-- With Northcott, height zero characterizes torsion.  The reverse direction
uses the uniform `C/3` comparison to put all multiples into one finite naïve
height ball, followed by the infinite pigeonhole principle. -/
theorem Point.canonicalHeight_eq_zero_iff_mem_torsion (P : W.Point) :
    P.canonicalHeight = 0 ↔ P ∈ AddCommGroup.torsion W.Point := by
  constructor
  · intro hP
    change canonicalize (fun Q : W.Point => Q.naiveHeight) P = 0 at hP
    obtain ⟨C, H⟩ := exists_naiveHeight_approxParallelogram (W := W)
    let S : Set W.Point := {Q | Q.naiveHeight ≤ C / 3}
    have hmem : ∀ n : ℕ, n • P ∈ S := by
      intro n
      have hbound := abs_canonicalize_sub_le H (n • P)
      have hzero : canonicalize (fun Q : W.Point => Q.naiveHeight) (n • P) = 0 := by
        rw [canonicalize_nsmul H P n, hP]
        ring
      rw [hzero, zero_sub, abs_neg] at hbound
      exact (le_abs_self ((n • P).naiveHeight)).trans hbound
    have hSfinite : S.Finite := finite_naiveHeight_le W (C / 3)
    obtain ⟨m, -, n, -, hmn, hmnP⟩ :=
      Set.infinite_univ.exists_ne_map_eq_of_mapsTo
        (fun k _ => hmem k) hSfinite
    apply (AddCommGroup.mem_torsion P).mpr
    rw [isOfFinAddOrder_iff_nsmul_eq_zero]
    rcases lt_or_gt_of_ne hmn with hlt | hgt
    · refine ⟨n - m, Nat.sub_pos_of_lt hlt, ?_⟩
      rw [sub_nsmul P hlt.le, hmnP]
      simp
    · refine ⟨m - n, Nat.sub_pos_of_lt hgt, ?_⟩
      rw [sub_nsmul P hgt.le, hmnP]
      simp
  · exact Point.canonicalHeight_eq_zero_of_mem_torsion P

theorem Point.neronTateHeight_eq_zero_iff_mem_torsion (P : W.Point) :
    P.neronTateHeight = 0 ↔ P ∈ AddCommGroup.torsion W.Point := by
  simpa [Point.neronTateHeight] using
    P.canonicalHeight_eq_zero_iff_mem_torsion

end Northcott

end WeierstrassCurve.Affine
