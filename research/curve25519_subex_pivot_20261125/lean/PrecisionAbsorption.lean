import PrecisionHistory

/-!
# Uniform precision absorption

This module reuses the existing finite native precision-history theorem and
discharges its two scalar absorption premises from the source-history bounds
`32 ≤ u` and `h ≤ 512*u`. It does not extract a history from Rust or prove
graph admission.
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

/-- Compose the existing finite precision-history theorem with the uniform
source-history absorption bounds. The remaining hypotheses are precisely the
numeric history and next-call formula that a native path witness must bind. -/
theorem next_precision_mul_le_of_uniform_history
    (s A d h : ℕ) (M u a i p : ℕ → ℕ)
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
    (hleafPositive : 0 < h)
    (haboveLeaf : s / h < M d)
    (hscaleFloor : 32 ≤ u d)
    (hleafScale : h ≤ 512 * u d) :
    p (d + 1) * u d ≤ 34 * M d * 2 ^ (A + 2 * u d - i d) := by
  exact next_precision_mul_le s A d h M u a i p hroot hpzero hshrink hscale
    hexponent hprecision hnext hcurrent hactive
    (active_node_root_comparison s (M d) h hleafPositive haboveLeaf)
    (inherited_history_absorption h (u d) hscaleFloor hleafScale)
    (evaluation_history_absorption A (u d) hscaleFloor)

end Curve25519PrecisionAbsorption
