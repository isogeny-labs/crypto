import PrecisionHistory

/-!
# Uniform precision absorption

This module reuses the existing finite native precision-history theorem and
derives its two scalar absorption premises from the source quotient, threshold,
square-root bound, shrink history, and active cutoff. It does not extract a
history from Rust or prove graph admission.
-/

namespace Curve25519PrecisionAbsorption

open NativePrecisionHistory

/-- A quadratic is absorbed by the power of two once the active scale reaches
the source-history floor 32. -/
theorem inherited_quadratic_absorption
    (u : ℕ)
    (hu : 32 ≤ u) :
    41472 * u ^ 2 ≤ 2 ^ (u + 1) := by
  induction u, hu using Nat.le_induction with
  | base => norm_num
  | succ u hu ih =>
      have hsquare : (u + 1) ^ 2 ≤ 2 * u ^ 2 := by nlinarith
      calc
        41472 * (u + 1) ^ 2 ≤ 41472 * (2 * u ^ 2) :=
          Nat.mul_le_mul_left 41472 hsquare
        _ = 2 * (41472 * u ^ 2) := by ac_rfl
        _ ≤ 2 * 2 ^ (u + 1) := Nat.mul_le_mul_left 2 ih
        _ = 2 ^ ((u + 1) + 1) := by
          calc
            2 * 2 ^ (u + 1) = 2 ^ (u + 1) * 2 := Nat.mul_comm _ _
            _ = 2 ^ ((u + 1) + 1) := (pow_succ 2 (u + 1)).symm

/-- The inherited-history coefficient 81 is absorbed whenever the root leaf
factor is at most `512*u` and `u` is at least 32. -/
theorem inherited_history_absorption
    (h u : ℕ)
    (hu : 32 ≤ u)
    (hh : h ≤ 512 * u) :
    81 * h * u ≤ 2 ^ (u + 1) := by
  calc
    81 * h * u ≤ 81 * (512 * u) * u :=
      Nat.mul_le_mul_right u (Nat.mul_le_mul_left 81 hh)
    _ = 41472 * u ^ 2 := by norm_num [pow_two]; ac_rfl
    _ ≤ 2 ^ (u + 1) := inherited_quadratic_absorption u hu

/-- The source evaluation-loss polynomial at `A=0` is absorbed at scale 32. -/
theorem evaluation_zero_absorption
    (u : ℕ)
    (hu : 32 ≤ u) :
    8 * u * (2 * u + 1) ≤ 2 ^ (u + 1) := by
  calc
    8 * u * (2 * u + 1) ≤ 41472 * u ^ 2 := by nlinarith
    _ ≤ 2 ^ (u + 1) := inherited_quadratic_absorption u hu

/-- Increasing the prior packet exponent adds only one copy of `8*u` to the
left while doubling the right, so the evaluation loss is uniformly absorbed. -/
theorem evaluation_history_absorption
    (A u : ℕ)
    (hu : 32 ≤ u) :
    8 * u * (A + 2 * u + 1) ≤ 2 ^ (A + (u + 1)) := by
  induction A with
  | zero => simpa using evaluation_zero_absorption u hu
  | succ A ih =>
      have hone : 1 ≤ A + 2 * u + 1 := by omega
      have hsmall : 8 * u ≤ 8 * u * (A + 2 * u + 1) :=
        by simpa using Nat.mul_le_mul_left (8 * u) hone
      calc
        8 * u * (A + 1 + 2 * u + 1) =
            8 * u * (A + 2 * u + 1) + 8 * u := by
          rw [show A + 1 + 2 * u + 1 = (A + 2 * u + 1) + 1 by omega,
            Nat.mul_add]
          simp
        _ ≤ 8 * u * (A + 2 * u + 1) + 8 * u * (A + 2 * u + 1) :=
          Nat.add_le_add_left hsmall _
        _ = 2 * (8 * u * (A + 2 * u + 1)) := by omega
        _ ≤ 2 * 2 ^ (A + (u + 1)) := Nat.mul_le_mul_left 2 ih
        _ = 2 ^ (A + 1 + (u + 1)) := by
          rw [show A + 1 + (u + 1) = (A + (u + 1)) + 1 by omega, pow_succ]
          omega

/-- An active node strictly above the root-fixed leaf cutoff supplies the
root-to-current comparison used by the finite precision-history theorem. -/
theorem active_node_root_comparison
    (s M h : ℕ)
    (hh : 0 < h)
    (hactive : s / h < M) :
    s ≤ M * h := by
  exact ((Nat.div_lt_iff_lt_mul hh).mp hactive).le

/-- Eightfold shrink at every completed level accumulates to the exact
`8^d` root-to-current factor. -/
theorem cumulative_eightfold_shrink
    (M : ℕ → ℕ) (d : ℕ)
    (hshrink : ∀ k < d, 8 * M (k + 1) ≤ M k) :
    8 ^ d * M d ≤ M 0 := by
  induction d with
  | zero => simp
  | succ d ih =>
      have hprefix : ∀ k < d, 8 * M (k + 1) ≤ M k := by
        intro k hk
        exact hshrink k (by omega)
      calc
        8 ^ (d + 1) * M (d + 1) = 8 ^ d * (8 * M (d + 1)) := by
          rw [pow_succ]
          ac_rfl
        _ ≤ 8 ^ d * M d :=
          Nat.mul_le_mul_left (8 ^ d) (hshrink d (by omega))
        _ ≤ M 0 := ih hprefix

/-- Root admission, accumulated shrink, and the active leaf cutoff force the
recursion depth below the leaf factor. -/
theorem active_depth_of_shrink_history
    (s h d : ℕ) (M : ℕ → ℕ)
    (hroot : M 0 ≤ s)
    (hshrink : ∀ k < d, 8 * M (k + 1) ≤ M k)
    (hh : 0 < h)
    (hactive : s / h < M d) :
    8 ^ d < h := by
  have hactiveProduct : s < M d * h :=
    (Nat.div_lt_iff_lt_mul hh).mp hactive
  have hcumulative : 8 ^ d * M d ≤ M 0 :=
    cumulative_eightfold_shrink M d hshrink
  by_contra hnot
  have hleaf : h ≤ 8 ^ d := Nat.le_of_not_gt hnot
  have hcurrentRoot : M d * h ≤ M 0 := by
    calc
      M d * h ≤ M d * 8 ^ d := Nat.mul_le_mul_left (M d) hleaf
      _ = 8 ^ d * M d := Nat.mul_comm _ _
      _ ≤ M 0 := hcumulative
  omega

/-- The source quotient has scale at least 32 once the root logarithm covers
the exact `2^(22+d)` threshold at depth `d`. -/
theorem scale_floor_of_power_bound
    (lambdaRoot d u : ℕ)
    (hscale : u = lambdaRoot / 2 ^ (17 + d))
    (hpower : 2 ^ (22 + d) ≤ lambdaRoot) :
    32 ≤ u := by
  rw [hscale]
  apply (Nat.le_div_iff_mul_le (Nat.pow_pos (by decide))).2
  calc
    32 * 2 ^ (17 + d) = 2 ^ (22 + d) := by
      rw [show 32 = 2 ^ 5 by norm_num, ← pow_add]
      congr 1; omega
    _ ≤ lambdaRoot := hpower

/-- A parallel power bound makes the source scale dominate the recursion
depth factor. -/
theorem scale_dominates_depth
    (lambdaRoot d u : ℕ)
    (hscale : u = lambdaRoot / 2 ^ (17 + d))
    (hpower : 2 ^ (17 + 2 * d) ≤ lambdaRoot) :
    2 ^ d ≤ u := by
  rw [hscale]
  apply (Nat.le_div_iff_mul_le (Nat.pow_pos (by decide))).2
  calc
    2 ^ d * 2 ^ (17 + d) = 2 ^ (17 + 2 * d) := by
      rw [← pow_add]
      congr 1; omega
    _ ≤ lambdaRoot := hpower

/-- The square-root leaf divisor is at most `512*u` under the exact source
quotient once `u` dominates the depth factor. -/
theorem leaf_scale_of_source_quotient
    (lambdaRoot h d u : ℕ)
    (hsqrt : h ^ 2 ≤ lambdaRoot)
    (hscale : u = lambdaRoot / 2 ^ (17 + d))
    (hdepth : 2 ^ d ≤ u)
    (hu : 0 < u) :
    h ≤ 512 * u := by
  have hdenom : 0 < 2 ^ (17 + d) := Nat.pow_pos (by decide)
  have hlambda : lambdaRoot < 2 ^ (17 + d) * (u + 1) := by
    rw [hscale]
    exact Nat.lt_mul_div_succ lambdaRoot hdenom
  have huplus : u + 1 ≤ 2 * u := by omega
  have hlambda' : lambdaRoot < 262144 * u ^ 2 := by
    calc
      lambdaRoot < 2 ^ (17 + d) * (u + 1) := hlambda
      _ = 131072 * 2 ^ d * (u + 1) := by norm_num [pow_add]
      _ ≤ 131072 * u * (u + 1) :=
        Nat.mul_le_mul_right (u + 1) (Nat.mul_le_mul_left 131072 hdepth)
      _ ≤ 131072 * u * (2 * u) := Nat.mul_le_mul_left (131072 * u) huplus
      _ = 262144 * u ^ 2 := by norm_num [pow_two]; ac_rfl
  have hsquare : h ^ 2 < (512 * u) ^ 2 := by
    calc
      h ^ 2 ≤ lambdaRoot := hsqrt
      _ < 262144 * u ^ 2 := hlambda'
      _ = (512 * u) ^ 2 := by norm_num [pow_two]; ac_rfl
  nlinarith

/-- The exact native leaf divisor `floor(sqrt(lambda))` supplies the square
bound used by the schedule argument. -/
theorem leaf_square_bound
    (lambdaRoot h : ℕ)
    (hleaf : h = Nat.sqrt lambdaRoot) :
    h ^ 2 ≤ lambdaRoot := by
  rw [hleaf]
  simpa [pow_two] using Nat.sqrt_le lambdaRoot

/-- The global threshold makes the exact native square-root leaf divisor
strictly positive. -/
theorem leaf_positive_of_threshold
    (lambdaRoot h : ℕ)
    (hthreshold : 2 ^ 26 ≤ lambdaRoot)
    (hleaf : h = Nat.sqrt lambdaRoot) :
    0 < h := by
  rw [hleaf]
  exact Nat.sqrt_pos.2 (by omega)

/-- The global threshold covers shallow depths directly; at deeper active
depths, `8^d<h` and `h^2≤lambda` supply both power bounds needed by the source
quotient lemmas. -/
theorem active_schedule_power_bounds
    (lambdaRoot h d : ℕ)
    (hthreshold : 2 ^ 26 ≤ lambdaRoot)
    (hsqrt : h ^ 2 ≤ lambdaRoot)
    (hactiveDepth : 8 ^ d < h) :
    2 ^ (22 + d) ≤ lambdaRoot ∧
      2 ^ (17 + 2 * d) ≤ lambdaRoot := by
  by_cases hd : d ≤ 4
  · constructor
    · exact (Nat.pow_le_pow_right (by decide) (by omega)).trans hthreshold
    · exact (Nat.pow_le_pow_right (by decide) (by omega)).trans hthreshold
  · have hd5 : 5 ≤ d := by omega
    have hactiveSquare : (8 ^ d) ^ 2 ≤ h ^ 2 :=
      pow_le_pow_left' hactiveDepth.le 2
    have hpowerIdentity : 2 ^ (6 * d) = (8 ^ d) ^ 2 := by
      symm
      calc
        (8 ^ d) ^ 2 = (((2 : ℕ) ^ 3) ^ d) ^ 2 := by norm_num
        _ = (2 ^ (3 * d)) ^ 2 :=
          congrArg (fun value : ℕ => value ^ 2) (pow_mul 2 3 d).symm
        _ = 2 ^ ((3 * d) * 2) := (pow_mul 2 (3 * d) 2).symm
        _ = 2 ^ (6 * d) := by congr 1; omega
    have hsix : 2 ^ (6 * d) ≤ lambdaRoot := by
      calc
        2 ^ (6 * d) = (8 ^ d) ^ 2 := hpowerIdentity
        _ ≤ h ^ 2 := hactiveSquare
        _ ≤ lambdaRoot := hsqrt
    constructor
    · exact (Nat.pow_le_pow_right (by decide) (by omega)).trans hsix
    · exact (Nat.pow_le_pow_right (by decide) (by omega)).trans hsix

/-- Compose the existing finite precision-history theorem with the uniform
source-history absorption bounds. The remaining hypotheses are the typed
history, source-quotient, square-root, and next-call facts that a native path
witness must bind. -/
theorem next_precision_mul_le_of_uniform_history
    (s lambdaRoot A d h : ℕ) (M u a i p : ℕ → ℕ)
    (hroot : M 0 ≤ s) (hpzero : p 0 ≤ s)
    (hshrink : ∀ k < d, 8 * M (k + 1) ≤ M k)
    (hscale : ∀ k < d, 0 < u k ∧ u k ≤ M k)
    (hexponent : ∀ k < d, a k ≤ A)
    (hprecision : ∀ k < d,
      p (k + 1) = CryptoLeanCore.TaperedPrecision.childPrecision
        (p k) (M k) (u k) (a k) (i k))
    (hnext : p (d + 1) = CryptoLeanCore.TaperedPrecision.childPrecision
      (p d) (M d) (u d) (A + 2 * u d) (i d))
    (hcurrent : u d ≤ M d) (hactive : i d < u d)
    (haboveLeaf : s / h < M d)
    (hthreshold : 2 ^ 26 ≤ lambdaRoot)
    (hleaf : h = Nat.sqrt lambdaRoot)
    (hsourceScale : u d = lambdaRoot / 2 ^ (17 + d))
    :
    p (d + 1) * u d ≤ 34 * M d * 2 ^ (A + 2 * u d - i d) := by
  have hsqrt := leaf_square_bound lambdaRoot h hleaf
  have hleafPositive := leaf_positive_of_threshold lambdaRoot h hthreshold hleaf
  have hactiveDepth := active_depth_of_shrink_history s h d M hroot hshrink
    hleafPositive haboveLeaf
  have hpowers := active_schedule_power_bounds lambdaRoot h d hthreshold hsqrt hactiveDepth
  have hscaleFloor := scale_floor_of_power_bound lambdaRoot d (u d) hsourceScale hpowers.1
  have hdepth := scale_dominates_depth lambdaRoot d (u d) hsourceScale hpowers.2
  have hleafScale := leaf_scale_of_source_quotient lambdaRoot h d (u d) hsqrt hsourceScale
    hdepth (by omega)
  exact next_precision_mul_le s A d h M u a i p hroot hpzero hshrink hscale
    hexponent hprecision hnext hcurrent hactive
    (active_node_root_comparison s (M d) h hleafPositive haboveLeaf)
    (inherited_history_absorption h (u d) hscaleFloor hleafScale)
    (evaluation_history_absorption A (u d) hscaleFloor)

end Curve25519PrecisionAbsorption
