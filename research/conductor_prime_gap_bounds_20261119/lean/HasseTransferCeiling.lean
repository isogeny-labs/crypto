import Mathlib.Tactic
import TransferBound
import DeepGap

/-!
# Hasse conductor bound and transfer ceilings

From a Frobenius discriminant identity `t² − 4q = D₀ · f₀²` with
`D₀ ≤ −3`, the Hasse bound forces `3 · f₀² ≤ 4 · q`. Composing with
`TransferBound` / `DeepGap` caps cross-stratum solver separation by the sum of
isogeny costs.

This file generalizes `DeepGap.Isogeny.comp` to an arbitrary-length
`IsogenyChain` with exact overhead `2 · cs.sum` on `TransferBound.Solver`.

**Not formalized here:** `End(E) ≃ O_f`, and Robert Remark 4.5 (poly(log ℓ)
volcano navigation) remain open documentation bridges, not axioms.
-/

set_option autoImplicit false

namespace HasseTransferCeiling

open TransferBound

/-! ## Hasse-style conductor bounds from discriminant hypotheses -/

/-- Every negative fundamental discriminant is `≤ −3` (equivalently: not
`-1` or `-2`, which are excluded as fundamental). -/
theorem fundamental_discriminant_le_neg_three {D₀ : ℤ}
    (hneg : D₀ < 0) (hne1 : D₀ ≠ -1) (hne2 : D₀ ≠ -2) : D₀ ≤ -3 := by
  omega

/-- From `D₀ ≤ −3` and `t² − 4q = D₀ · f₀²` with `f₀ > 0`, deduce
`3 · f₀² ≤ 4 · q`. -/
theorem three_mul_conductor_sq_le_four_q {q t D₀ f₀ : ℤ}
    (hdisc : t ^ 2 - 4 * q = D₀ * f₀ ^ 2) (hD : D₀ ≤ -3) (hf : 0 < f₀) :
    3 * f₀ ^ 2 ≤ 4 * q := by
  have ht2 : 0 ≤ t ^ 2 := sq_nonneg t
  have hf2 : 0 ≤ f₀ ^ 2 := sq_nonneg f₀
  have h4q : 4 * q = t ^ 2 - D₀ * f₀ ^ 2 := by
    linear_combination -hdisc
  have hge : -D₀ * f₀ ^ 2 ≤ 4 * q := by
    have : 0 ≤ t ^ 2 := ht2
    linarith
  -- `-D₀ ≥ 3`, so `3 f₀² ≤ (-D₀) f₀² ≤ 4q`
  have : 3 * f₀ ^ 2 ≤ -D₀ * f₀ ^ 2 := by nlinarith
  nlinarith

/-- Convenient form: `f₀² < 4q`. -/
theorem conductor_sq_lt_four_q {q t D₀ f₀ : ℤ}
    (hdisc : t ^ 2 - 4 * q = D₀ * f₀ ^ 2) (hD : D₀ ≤ -3) (hf : 0 < f₀) :
    f₀ ^ 2 < 4 * q := by
  have := three_mul_conductor_sq_le_four_q hdisc hD hf
  nlinarith [sq_nonneg f₀]

/-- Every positive divisor `c` of `f₀` also satisfies `c² < 4q`. -/
theorem conductor_divisor_bounds {q t D₀ f₀ c : ℤ}
    (hdisc : t ^ 2 - 4 * q = D₀ * f₀ ^ 2) (hD : D₀ ≤ -3)
    (hf : 0 < f₀) (hc : 0 < c) (hdiv : c ∣ f₀) :
    c ^ 2 < 4 * q := by
  obtain ⟨k, hk⟩ := hdiv
  have hkpos : 0 < k := by
    have : 0 < c * k := by simpa [hk] using hf
    nlinarith
  have h1 : 1 ≤ k := by omega
  have hc_le : c ≤ f₀ := by
    have : c * 1 ≤ c * k := by nlinarith
    simpa [hk, mul_one] using this
  have hc_sq : c ^ 2 ≤ f₀ ^ 2 := by
    have : 0 ≤ c := le_of_lt hc
    nlinarith
  have := conductor_sq_lt_four_q hdisc hD hf
  nlinarith

/-- When `D₀ ≤ −4`, the sharper bound `f₀² ≤ q` holds. -/
theorem conductor_le_sqrt_q_of_le_neg_four {q t D₀ f₀ : ℤ}
    (hdisc : t ^ 2 - 4 * q = D₀ * f₀ ^ 2) (hD : D₀ ≤ -4) (hf : 0 < f₀) :
    f₀ ^ 2 ≤ q := by
  have ht2 : 0 ≤ t ^ 2 := sq_nonneg t
  have h4q : 4 * q = t ^ 2 - D₀ * f₀ ^ 2 := by
    linear_combination -hdisc
  have : 4 * f₀ ^ 2 ≤ -D₀ * f₀ ^ 2 := by nlinarith [sq_nonneg f₀]
  have : -D₀ * f₀ ^ 2 ≤ 4 * q := by nlinarith
  nlinarith

/-! ## Arbitrary-length isogeny chains -/

variable {α : Type}

/-- A same-carrier chain of evaluable isogenies. -/
structure IsogenyChain (ops : MonoidOps α) where
  steps : List (Isogeny ops ops)

/-- Total evaluation cost of a chain. -/
def IsogenyChain.costSum {ops : MonoidOps α} (cs : IsogenyChain ops) : ℕ :=
  (cs.steps.map (·.cost)).sum

/-- Fold `DeepGap.Isogeny.comp` along a list (identity isogeny if empty). -/
def IsogenyChain.compose {ops : MonoidOps α} : List (Isogeny ops ops) → Isogeny ops ops
  | [] =>
    { toFun := id
      inj := fun _ _ h => h
      map_one := rfl
      map_mul := fun _ _ => rfl
      cost := 0 }
  | φ :: rest => DeepGap.Isogeny.comp (IsogenyChain.compose rest) φ

theorem IsogenyChain.compose_cost {ops : MonoidOps α} :
    ∀ steps : List (Isogeny ops ops),
      (IsogenyChain.compose steps).cost = (steps.map (·.cost)).sum
  | [] => by simp [IsogenyChain.compose]
  | φ :: rest => by
      simp [IsogenyChain.compose, DeepGap.comp_cost, IsogenyChain.compose_cost rest]

/-- Transfer along a chain: overhead is exactly `2 · cs.sum`. -/
theorem chain_solver_transfer {ops : MonoidOps α}
    (cs : IsogenyChain ops) (S : Solver ops) (g h : α) :
    ((transfer (IsogenyChain.compose cs.steps) S).run g h).1 =
      (S.run
          ((IsogenyChain.compose cs.steps).toFun g)
          ((IsogenyChain.compose cs.steps).toFun h)).1 +
        2 * cs.costSum := by
  rw [transfer_cost, IsogenyChain.costSum, IsogenyChain.compose_cost]

/-- Vertical prime-gap DLP ceiling: degree-`ℓ` edge plus a walk separates
always-correct solvers by at most `2 · (φ.cost + walk.costSum)`. -/
theorem vertical_prime_gap_dlp_ceiling {ops : MonoidOps α}
    (φ : Isogeny ops ops) (walk : IsogenyChain ops)
    (S : Solver ops) (g h : α) :
    ((transfer (DeepGap.Isogeny.comp (IsogenyChain.compose walk.steps) φ) S).run g h).1 =
      (S.run
          ((DeepGap.Isogeny.comp (IsogenyChain.compose walk.steps) φ).toFun g)
          ((DeepGap.Isogeny.comp (IsogenyChain.compose walk.steps) φ).toFun h)).1 +
        2 * (φ.cost + walk.costSum) := by
  have hc := IsogenyChain.compose_cost walk.steps
  simp [transfer_cost, DeepGap.comp_cost, IsogenyChain.costSum, hc, Nat.add_comm,
    Nat.add_left_comm]

/-- Linear Vélu cost of a gap prime `ℓ` is on the Pollard-rho scale of a
group of size `ℓ²`: `ℓ ≤ 2 · √(ℓ²)` in the squared form `ℓ² ≤ (2ℓ) · ℓ`. -/
theorem linear_velu_gap_is_rho_scale {ℓ : ℕ} (_hℓ : 0 < ℓ) :
    ℓ * ℓ ≤ (2 * ℓ) * ℓ := by
  nlinarith

end HasseTransferCeiling
