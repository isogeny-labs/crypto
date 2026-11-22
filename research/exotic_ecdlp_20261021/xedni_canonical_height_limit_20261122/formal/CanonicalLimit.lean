import Mathlib.NumberTheory.Height.EllipticCurve
import Mathlib.NumberTheory.Height.NumberField
import Mathlib.Analysis.SpecificLimits.Basic
import Mathlib.Analysis.Normed.Group.Completeness
import Mathlib.Tactic

/-!
A uniform fourfold doubling defect yields a convergent canonical-height
renormalization. The finite-level statement keeps the defect hypothesis
explicit; the elliptic-point bridge is proved below.
-/

namespace XedniCanonicalLimit

noncomputable def normalized (f : ℕ → ℝ) (n : ℕ) : ℝ := f n / (4 : ℝ) ^ n

private theorem normalized_step (f : ℕ → ℝ) (n : ℕ) :
    normalized f (n + 1) - normalized f n =
      (f (n + 1) - 4 * f n) / (4 : ℝ) ^ (n + 1) := by
  unfold normalized
  rw [pow_succ]
  have hpow : (4 : ℝ) ^ n ≠ 0 := pow_ne_zero _ (by norm_num)
  field_simp

theorem normalized_step_bound
    (f : ℕ → ℝ) (C : ℝ) (hC : 0 ≤ C)
    (herror : ∀ n, |f (n + 1) - 4 * f n| ≤ C) (n : ℕ) :
    dist (normalized f n) (normalized f (n + 1)) ≤
      (C / 4) * (1 / 4 : ℝ) ^ n := by
  rw [Real.dist_eq, abs_sub_comm, normalized_step, abs_div]
  have hpowpos : (0 : ℝ) < 4 ^ (n + 1) := pow_pos (by norm_num) _
  rw [abs_of_pos hpowpos]
  have hbound := div_le_div_of_nonneg_right (herror n) hpowpos.le
  calc
    |f (n + 1) - 4 * f n| / (4 : ℝ) ^ (n + 1) ≤
        C / (4 : ℝ) ^ (n + 1) := hbound
    _ = (C / 4) * (1 / 4 : ℝ) ^ n := by
      rw [pow_succ, one_div, inv_pow]
      ring

theorem normalized_limit
    (f : ℕ → ℝ) (C : ℝ) (hC : 0 ≤ C)
    (herror : ∀ n, |f (n + 1) - 4 * f n| ≤ C) :
    ∃ L : ℝ, Filter.Tendsto (normalized f) Filter.atTop (nhds L) ∧
      |L - f 0| ≤ C / 3 := by
  have hr : (1 / 4 : ℝ) < 1 := by norm_num
  have hstep : ∀ n, dist (normalized f n) (normalized f (n + 1)) ≤
      (C / 4) * (1 / 4 : ℝ) ^ n :=
    normalized_step_bound f C hC herror
  obtain ⟨L, hlim⟩ := cauchySeq_tendsto_of_complete
    (cauchySeq_of_le_geometric (r := (1 / 4 : ℝ)) (C := C / 4) hr hstep)
  refine ⟨L, hlim, ?_⟩
  have hdist := dist_le_of_le_geometric_of_tendsto₀
    (r := (1 / 4 : ℝ)) (C := C / 4) hr hstep hlim
  convert hdist using 1 <;> norm_num [normalized, Real.dist_eq, abs_sub_comm] <;> ring

theorem doubling_defect_of_parallelogram
    {G : Type*} [AddCommGroup G] (h : G → ℝ) (C : ℝ)
    (hpar : ∀ P Q, |h (P + Q) + h (P - Q) -
      2 * (h P + h Q)| ≤ C) :
    ∃ D : ℝ, 0 ≤ D ∧ ∀ P, |h ((2 : ℕ) • P) - 4 * h P| ≤ D := by
  refine ⟨|C| + |h 0|, add_nonneg (abs_nonneg _) (abs_nonneg _), ?_⟩
  intro P
  have hp := hpar P P
  have hbound : |(h (P + P) - 4 * h P) + h 0| ≤ C := by
    convert hp using 1 <;> simp only [sub_self] <;> ring
  calc
    |h ((2 : ℕ) • P) - 4 * h P| =
        |((h (P + P) - 4 * h P) + h 0) - h 0| := by
          rw [two_nsmul]; ring
    _ ≤ |(h (P + P) - 4 * h P) + h 0| + |h 0| := abs_sub _ _
    _ ≤ C + |h 0| := by linarith [hbound]
    _ ≤ |C| + |h 0| := by linarith [le_abs_self C]

theorem height_limit_of_parallelogram
    {G : Type*} [AddCommGroup G] (h : G → ℝ) (C : ℝ)
    (hpar : ∀ P Q, |h (P + Q) + h (P - Q) -
      2 * (h P + h Q)| ≤ C) :
    ∃ D : ℝ, 0 ≤ D ∧ ∀ P : G, ∃ L : ℝ,
      Filter.Tendsto
        (normalized (fun n => h ((2 ^ n : ℕ) • P))) Filter.atTop (nhds L) ∧
      |L - h P| ≤ D / 3 := by
  obtain ⟨D, hDnonneg, hD⟩ := doubling_defect_of_parallelogram h C hpar
  refine ⟨D, hDnonneg, ?_⟩
  intro P
  let f : ℕ → ℝ := fun n => h ((2 ^ n : ℕ) • P)
  have herror : ∀ n, |f (n + 1) - 4 * f n| ≤ D := by
    intro n
    have hnext : (2 ^ (n + 1) : ℕ) • P =
        (2 : ℕ) • ((2 ^ n : ℕ) • P) := by
      simp [pow_succ, mul_smul, mul_comm]
    simpa only [f, hnext] using hD ((2 ^ n : ℕ) • P)
  have hlim := normalized_limit f D hDnonneg herror
  simpa only [f, pow_zero, one_nsmul] using hlim

theorem limits_double
    {G : Type*} [AddCommGroup G] (h : G → ℝ) (P : G) (L L₂ : ℝ)
    (hP : Filter.Tendsto
      (normalized (fun n => h ((2 ^ n : ℕ) • P))) Filter.atTop (nhds L))
    (h₂ : Filter.Tendsto
      (normalized (fun n => h ((2 ^ n : ℕ) • ((2 : ℕ) • P))))
      Filter.atTop (nhds L₂)) :
    L₂ = 4 * L := by
  have hshift := hP.comp (Filter.tendsto_add_atTop_nat 1)
  have hscaled : Filter.Tendsto
      (fun n => 4 * normalized (fun m => h ((2 ^ m : ℕ) • P)) (n + 1))
      Filter.atTop (nhds (4 * L)) := hshift.const_mul 4
  have heq : ∀ n : ℕ,
      normalized (fun m => h ((2 ^ m : ℕ) • ((2 : ℕ) • P))) n =
      4 * normalized (fun m => h ((2 ^ m : ℕ) • P)) (n + 1) := by
    intro n
    unfold normalized
    have hsmul : (2 ^ n : ℕ) • ((2 : ℕ) • P) =
        (2 ^ (n + 1) : ℕ) • P := by
      rw [← mul_smul, pow_succ]
    change h ((2 ^ n : ℕ) • ((2 : ℕ) • P)) / (4 : ℝ) ^ n =
      4 * (h ((2 ^ (n + 1) : ℕ) • P) / (4 : ℝ) ^ (n + 1))
    rw [hsmul, pow_succ]
    have hpow : (4 : ℝ) ^ n ≠ 0 := pow_ne_zero _ (by norm_num)
    field_simp
    ring
  have hscaled' : Filter.Tendsto
      (normalized (fun n => h ((2 ^ n : ℕ) • ((2 : ℕ) • P))))
      Filter.atTop (nhds (4 * L)) := by
    convert hscaled using 1
    funext n
    exact heq n
  exact tendsto_nhds_unique h₂ hscaled'

theorem nsmul_square_of_exact_parallelogram
    {G : Type*} [AddCommGroup G] (H : G → ℝ)
    (hpar : ∀ P Q, H (P + Q) + H (P - Q) =
      2 * (H P + H Q)) (P : G) (n : ℕ) :
    H (n • P) = (n : ℝ) ^ 2 * H P := by
  have hzero : H 0 = 0 := by
    have hp := hpar 0 0
    simp only [add_zero, sub_self] at hp
    linarith
  induction n using Nat.twoStepInduction with
  | zero => simpa using hzero
  | one => simp
  | more n ih0 ih1 =>
    have hplus : ((n + 1 : ℕ) • P) + P = (n + 2 : ℕ) • P := by
      simp only [add_nsmul, one_nsmul, two_nsmul]
      abel
    have hminus : ((n + 1 : ℕ) • P) - P = n • P := by
      simp [add_nsmul]
    have hp := hpar ((n + 1 : ℕ) • P) P
    rw [hplus, hminus, ih0, ih1] at hp
    push_cast at hp ⊢
    nlinarith

theorem zsmul_square_of_exact_parallelogram
    {G : Type*} [AddCommGroup G] (H : G → ℝ)
    (hpar : ∀ P Q, H (P + Q) + H (P - Q) =
      2 * (H P + H Q)) (P : G) (n : ℤ) :
    H (n • P) = (n : ℝ) ^ 2 * H P := by
  have hzero : H 0 = 0 := by
    have hp := hpar 0 0
    simp only [add_zero, sub_self] at hp
    linarith
  have hneg (R : G) : H (-R) = H R := by
    have hp := hpar 0 R
    simp only [zero_add, zero_sub, hzero, zero_add] at hp
    linarith
  cases n with
  | ofNat k =>
      simpa only [Int.ofNat_eq_natCast, natCast_zsmul, Int.cast_natCast] using
        nsmul_square_of_exact_parallelogram H hpar P k
  | negSucc k =>
      simp only [negSucc_zsmul, hneg]
      rw [nsmul_square_of_exact_parallelogram H hpar P (k + 1)]
      push_cast
      ring

theorem limit_exact_parallelogram
    {G : Type*} [AddCommGroup G] (h H : G → ℝ) (C : ℝ)
    (hpar : ∀ P Q, |h (P + Q) + h (P - Q) -
      2 * (h P + h Q)| ≤ C)
    (hH : ∀ R : G, Filter.Tendsto
      (normalized (fun n => h ((2 ^ n : ℕ) • R)))
      Filter.atTop (nhds (H R))) (P Q : G) :
    H (P + Q) + H (P - Q) = 2 * (H P + H Q) := by
  let s : G → ℕ → ℝ := fun R n =>
    normalized (fun m => h ((2 ^ m : ℕ) • R)) n
  let d : ℕ → ℝ := fun n =>
    s (P + Q) n + s (P - Q) n - 2 * (s P n + s Q n)
  have hs (R : G) : Filter.Tendsto (s R) Filter.atTop (nhds (H R)) := hH R
  have hdlim : Filter.Tendsto d Filter.atTop
      (nhds (H (P + Q) + H (P - Q) - 2 * (H P + H Q))) :=
    ((hs (P + Q)).add (hs (P - Q))).sub (((hs P).add (hs Q)).const_mul 2)
  have heq (n : ℕ) : d n =
      (h (((2 ^ n : ℕ) • P) + ((2 ^ n : ℕ) • Q)) +
        h (((2 ^ n : ℕ) • P) - ((2 ^ n : ℕ) • Q)) -
        2 * (h ((2 ^ n : ℕ) • P) + h ((2 ^ n : ℕ) • Q))) /
        (4 : ℝ) ^ n := by
    dsimp [d, s, normalized]
    rw [nsmul_add, nsmul_sub]
    ring
  have hbound (n : ℕ) : |d n| ≤ C / (4 : ℝ) ^ n := by
    have hp : (0 : ℝ) < (4 : ℝ) ^ n := pow_pos (by norm_num) n
    rw [heq, abs_div, abs_of_pos hp]
    exact div_le_div_of_nonneg_right
      (hpar ((2 ^ n : ℕ) • P) ((2 ^ n : ℕ) • Q))
      hp.le
  have hsmall : Filter.Tendsto (fun n : ℕ => C / (4 : ℝ) ^ n)
      Filter.atTop (nhds 0) := by
    have hp := tendsto_pow_atTop_nhds_zero_of_lt_one
      (by norm_num : (0 : ℝ) ≤ 1 / 4)
      (by norm_num : (1 / 4 : ℝ) < 1)
    simpa [div_eq_mul_inv, inv_pow] using hp.const_mul C
  have habs : Filter.Tendsto (fun n => |d n|) Filter.atTop (nhds 0) :=
    squeeze_zero (fun n => abs_nonneg (d n)) hbound hsmall
  have hd0 : Filter.Tendsto d Filter.atTop (nhds 0) :=
    (tendsto_zero_iff_abs_tendsto_zero d).mpr habs
  have hEq := tendsto_nhds_unique hdlim hd0
  linarith

theorem height_function_of_parallelogram
    {G : Type*} [AddCommGroup G] (h : G → ℝ) (C : ℝ)
    (hpar : ∀ P Q, |h (P + Q) + h (P - Q) -
      2 * (h P + h Q)| ≤ C) :
    ∃ D : ℝ, 0 ≤ D ∧ ∃ H : G → ℝ, ∀ P : G,
      Filter.Tendsto
        (normalized (fun n => h ((2 ^ n : ℕ) • P))) Filter.atTop (nhds (H P)) ∧
      |H P - h P| ≤ D / 3 ∧ H ((2 : ℕ) • P) = 4 * H P ∧
      ∀ n : ℤ, H (n • P) = (n : ℝ) ^ 2 * H P := by
  classical
  obtain ⟨D, hD, hlimits⟩ := height_limit_of_parallelogram h C hpar
  choose H hH using hlimits
  have hexact : ∀ P Q, H (P + Q) + H (P - Q) = 2 * (H P + H Q) :=
    limit_exact_parallelogram h H C hpar (fun R => (hH R).1)
  refine ⟨D, hD, H, ?_⟩
  intro P
  obtain ⟨hlim, hclose⟩ := hH P
  exact ⟨hlim, hclose,
    limits_double h P (H P) (H ((2 : ℕ) • P)) hlim (hH _).1,
    zsmul_square_of_exact_parallelogram H hexact P⟩

theorem elliptic_naive_height_limit
    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic] :
    ∃ D : ℝ, 0 ≤ D ∧ ∀ P : W.Point, ∃ L : ℝ,
      Filter.Tendsto
        (normalized (fun n => (((2 ^ n : ℕ) • P).naiveHeight)))
        Filter.atTop (nhds L) ∧
      |L - P.naiveHeight| ≤ D / 3 := by
  classical
  obtain ⟨C, hpar⟩ := WeierstrassCurve.Affine.approx_parallelogram_law W
  exact height_limit_of_parallelogram
    (fun P : W.Point => P.naiveHeight) C hpar

theorem elliptic_naive_height_function
    (W : WeierstrassCurve.Affine ℚ) [W.IsElliptic] :
    ∃ D : ℝ, 0 ≤ D ∧ ∃ H : W.Point → ℝ, ∀ P : W.Point,
      Filter.Tendsto
        (normalized (fun n => (((2 ^ n : ℕ) • P).naiveHeight)))
        Filter.atTop (nhds (H P)) ∧
      |H P - P.naiveHeight| ≤ D / 3 ∧
      H ((2 : ℕ) • P) = 4 * H P ∧
      ∀ n : ℤ, H (n • P) = (n : ℝ) ^ 2 * H P := by
  classical
  obtain ⟨C, hpar⟩ := WeierstrassCurve.Affine.approx_parallelogram_law W
  exact height_function_of_parallelogram
    (fun P : W.Point => P.naiveHeight) C hpar

def FixedCurve : WeierstrassCurve.Affine ℚ := ⟨0, -47, 0, 751, -3997⟩

instance : FixedCurve.IsElliptic := ⟨by
  apply isUnit_iff_ne_zero.mpr
  norm_num [FixedCurve, WeierstrassCurve.Δ, WeierstrassCurve.b₂,
    WeierstrassCurve.b₄, WeierstrassCurve.b₆, WeierstrassCurve.b₈]⟩

theorem fixed_curve_naive_height_function :
    ∃ D : ℝ, 0 ≤ D ∧ ∃ H : FixedCurve.Point → ℝ, ∀ P : FixedCurve.Point,
      Filter.Tendsto
        (normalized (fun n => (((2 ^ n : ℕ) • P).naiveHeight)))
        Filter.atTop (nhds (H P)) ∧
      |H P - P.naiveHeight| ≤ D / 3 ∧
      H ((2 : ℕ) • P) = 4 * H P ∧
      ∀ n : ℤ, H (n • P) = (n : ℝ) ^ 2 * H P :=
  elliptic_naive_height_function FixedCurve

#print axioms normalized_step_bound
#print axioms normalized_limit
#print axioms doubling_defect_of_parallelogram
#print axioms height_limit_of_parallelogram
#print axioms limits_double
#print axioms nsmul_square_of_exact_parallelogram
#print axioms zsmul_square_of_exact_parallelogram
#print axioms limit_exact_parallelogram
#print axioms height_function_of_parallelogram
#print axioms elliptic_naive_height_limit
#print axioms elliptic_naive_height_function
#print axioms fixed_curve_naive_height_function

end XedniCanonicalLimit
