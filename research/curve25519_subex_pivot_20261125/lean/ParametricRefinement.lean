module

public import Mathlib

@[expose] public section

/-!
# Parametric source-guard and cost-ledger implications

These theorems start from the variance, source-guard, precision, and mode
bounds used by the native certificate. They do not prove those hypotheses for
an arbitrary graph and do not import any contraction theorem.
-/

namespace Curve25519ParametricRefinement

/-- A single scaled class deviation cannot exceed the source class cap once
the complete variance budget and the strongest source guard hold. -/
theorem class_cap_of_scaled_variance
    (M D δ c : ℝ)
    (hM : 0 ≤ M)
    (hvar : (c * D - M) ^ 2 ≤ 2 * M * δ * D ^ 2)
    (hguard : 2 * D ^ 2 * δ ≤ M) :
    c * D ≤ 2 * M := by
  have hbudget : 2 * M * δ * D ^ 2 ≤ M ^ 2 := by
    nlinarith [mul_nonneg hM (sub_nonneg.mpr hguard)]
  by_contra hcap
  have hgt : 2 * M < c * D := lt_of_not_ge hcap
  have hdev : M < c * D - M := by linarith
  have hdevNonneg : 0 ≤ c * D - M := le_trans hM hdev.le
  have hsq : M ^ 2 < (c * D - M) ^ 2 := by
    exact (sq_lt_sq₀ hM hdevNonneg).2 hdev
  linarith

/-- The class cap, the length-array guard, and the padding ceiling give the
coefficient-three packet-capacity bound. -/
theorem padded_capacity
    (M D L c height : ℝ)
    (hD : 0 ≤ D)
    (hclass : c * D ≤ 2 * M)
    (hlength : L * D ≤ M)
    (hheight : height ≤ c + L) :
    height * D ≤ 3 * M := by
  have hscaled := mul_le_mul_of_nonneg_right hheight hD
  nlinarith

/-- The history's coarser `6M/64` child bound still implies eightfold shrink. -/
theorem history_child_strict_shrink
    (M child : ℝ)
    (hM : 0 ≤ M)
    (hchild : child * 64 ≤ 6 * M) :
    8 * child ≤ M := by
  nlinarith

/-- The stronger source-guard child numerator also implies eightfold shrink. -/
theorem derived_child_strict_shrink
    (M child : ℝ)
    (hM : 0 ≤ M)
    (hchild : child * 64 ≤ 4 * M) :
    8 * child ≤ M := by
  nlinarith

/-- Old vertices keep the parent cap and packet vertices keep it through
`2L≤δ`; hence either kind satisfies the same degree cap. -/
theorem child_degree_cap
    (δ L oldDegree packetDegree : ℝ)
    (hold : oldDegree ≤ δ)
    (hpacket : packetDegree ≤ 2 * L)
    (hguard : 2 * L ≤ δ) :
    max oldDegree packetDegree ≤ δ := by
  exact max_le hold (hpacket.trans hguard)

/-- The mode and normalized-precision bounds compose to coefficient 204 after
cancelling the positive dimension parameter. -/
theorem mode_precision_product
    (M D n p u : ℝ)
    (hM : 0 ≤ M)
    (hD : 0 < D)
    (hp : 0 ≤ p)
    (hu : 0 ≤ u)
    (hmode : n * D ≤ 6 * M)
    (hprecision : p * u ≤ 34 * M * D) :
    n * p * u ≤ 204 * M ^ 2 := by
  have hpu : 0 ≤ p * u := mul_nonneg hp hu
  apply le_of_mul_le_mul_right ?_ hD
  calc
    n * p * u * D = (n * D) * (p * u) := by ring
    _ ≤ (6 * M) * (p * u) := mul_le_mul_of_nonneg_right hmode hpu
    _ ≤ (6 * M) * (34 * M * D) :=
      mul_le_mul_of_nonneg_left hprecision (mul_nonneg (by norm_num) hM)
    _ = 204 * M ^ 2 * D := by ring

/-- Exact arithmetic for the tuple, local, root-u, and root-log constants. -/
theorem cost_ledger_constants :
    14 ^ 2 < (204 : ℕ) ∧
    (204 : ℕ) ≤ 15 ^ 2 ∧
    5 * 15 + 5 = 80 ∧
    80 + 1 + 4 + 5 + 6 = 96 ∧
    96 * 3 + 1 + 1 = 290 ∧
    290 * 512 = 148480 := by
  norm_num

/-- One conjunction exposing the kernel-checked boundary to downstream audits. -/
theorem parametric_refinement_certificate :
    (∀ M D δ c : ℝ,
      0 ≤ M →
      (c * D - M) ^ 2 ≤ 2 * M * δ * D ^ 2 →
      2 * D ^ 2 * δ ≤ M →
      c * D ≤ 2 * M) ∧
    (∀ M D L c height : ℝ,
      0 ≤ D →
      c * D ≤ 2 * M →
      L * D ≤ M →
      height ≤ c + L →
      height * D ≤ 3 * M) ∧
    (∀ M child : ℝ, 0 ≤ M → child * 64 ≤ 6 * M → 8 * child ≤ M) ∧
    (∀ M child : ℝ, 0 ≤ M → child * 64 ≤ 4 * M → 8 * child ≤ M) ∧
    (∀ δ L oldDegree packetDegree : ℝ,
      oldDegree ≤ δ →
      packetDegree ≤ 2 * L →
      2 * L ≤ δ →
      max oldDegree packetDegree ≤ δ) ∧
    (∀ M D n p u : ℝ,
      0 ≤ M →
      0 < D →
      0 ≤ p →
      0 ≤ u →
      n * D ≤ 6 * M →
      p * u ≤ 34 * M * D →
      n * p * u ≤ 204 * M ^ 2) ∧
    (14 ^ 2 < (204 : ℕ) ∧
      (204 : ℕ) ≤ 15 ^ 2 ∧
      5 * 15 + 5 = 80 ∧
      80 + 1 + 4 + 5 + 6 = 96 ∧
      96 * 3 + 1 + 1 = 290 ∧
      290 * 512 = 148480) := by
  exact ⟨class_cap_of_scaled_variance,
    padded_capacity,
    history_child_strict_shrink,
    derived_child_strict_shrink,
    child_degree_cap,
    mode_precision_product,
    cost_ledger_constants⟩

end Curve25519ParametricRefinement
