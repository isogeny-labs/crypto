import Mathlib.Data.Nat.Prime.Basic
import Mathlib.NumberTheory.Padics.PadicVal.Basic
import Mathlib.Tactic
import TransferBound
import ConductorOrthogonality
import HasseTransferCeiling

/-!
# Floor vs crater ECDLP cost difference across a prime conductor gap

For crater conductor `1` and floor conductor `ℓ` (prime), a mutual
degree-`ℓ` isogeny step is a `ValidConductorStep`, forces a valuation gap,
and therefore sits under `TransferBound.within_isogeny_cost`: always-correct
solvers on the two levels differ by at most twice the isogeny evaluation
cost.

This is the Lean half of
`research/floor_crater_ecdlp_gap_20261120/PROTOCOL.md`. It does **not**
claim that Pollard rho (or any representation-hashing walk) is harder on
the floor. Unformalized bridges: `End(E) ≃ O_f`; Robert Remark 4.5.
-/

set_option autoImplicit false

namespace FloorCraterEcdlp

open TransferBound ConductorOrthogonality HasseTransferCeiling

/-- Crater `f = 1` and floor `f = ℓ` are joined by a mutual degree-`ℓ` step. -/
theorem floor_crater_valid_step {ℓ : ℕ} (_hℓ : Nat.Prime ℓ) :
    ValidConductorStep 1 ℓ ℓ where
  forward := one_dvd _
  backward := by simp

/-- The one-step walk from crater to floor has a `v_ℓ` gap. -/
theorem floor_crater_valuation_gap {ℓ : ℕ} [hp : Fact ℓ.Prime] :
    padicValNat ℓ 1 ≠ padicValNat ℓ ℓ := by
  have hnot : ¬ ℓ ∣ 1 :=
    Nat.not_dvd_of_pos_of_lt (Nat.succ_pos 0) hp.out.one_lt
  have h0 : padicValNat ℓ 1 = 0 := padicValNat.eq_zero_of_not_dvd hnot
  have h1 : padicValNat ℓ ℓ = 1 := padicValNat_self
  omega

/-- Packaging: the crater→floor walk forces a step with `ℓ ∣ d` and `ℓ ≤ d`. -/
theorem floor_crater_forces_large_step {ℓ : ℕ} (hℓ : Nat.Prime ℓ) :
    ∃ d ∈ ([ℓ] : List ℕ), ℓ ∣ d ∧ ℓ ≤ d := by
  have : Fact ℓ.Prime := ⟨hℓ⟩
  let w : ConductorWalk 1 [ℓ] :=
    ConductorWalk.cons (floor_crater_valid_step hℓ) hℓ.ne_zero
      (Nat.one_ne_zero) hℓ.ne_zero (ConductorWalk.singleton ℓ hℓ.ne_zero)
  have hend : w.endpoint = ℓ := rfl
  have hgap : padicValNat ℓ 1 ≠ padicValNat ℓ w.endpoint := by
    rw [hend]; exact floor_crater_valuation_gap
  exact conductor_gap_forces_large_step w hgap

/-- **Floor/crater ECDLP ceiling (always-correct solvers).**
With evaluable degree-`ℓ` isogenies both ways, every always-correct solver
on either level has a partner on the other whose cost is the original cost
on the transported instance plus twice one evaluation. Hence the best costs
differ by at most `2 · max(cost φ, cost ψ)`. -/
theorem floor_crater_dlp_within_isogeny_cost {α : Type} {ops : MonoidOps α}
    (φ ψ : Isogeny ops ops) :
    (∀ S : Solver ops, ∃ S' : Solver ops, ∀ g h,
        (S'.run g h).1 = (S.run (φ.toFun g) (φ.toFun h)).1 + 2 * φ.cost) ∧
    (∀ T : Solver ops, ∃ T' : Solver ops, ∀ g h,
        (T'.run g h).1 = (T.run (ψ.toFun g) (ψ.toFun h)).1 + 2 * ψ.cost) :=
  within_isogeny_cost φ ψ

/-- Same ceiling with the study's preferred reading: overhead is exactly
`2 · C` when transferring across a single edge of cost `C`. -/
theorem floor_crater_transfer_overhead {α : Type} {ops : MonoidOps α}
    (φ : Isogeny ops ops) (S : Solver ops) (g h : α) :
    ((transfer φ S).run g h).1 =
      (S.run (φ.toFun g) (φ.toFun h)).1 + 2 * φ.cost :=
  transfer_cost φ S g h

/-- If classical Vélu evaluation of a known kernel costs at least `ℓ`
(hypothesis `hVélu`), the abstract transfer overhead is at least `2ℓ`.
This models the bridge cost lower bound used in the protocol; it does not
synthesize Vélu's formulas. -/
theorem floor_crater_bridge_overhead_ge {α : Type} {ops : MonoidOps α}
    {ℓ : ℕ} (φ : Isogeny ops ops) (hVélu : ℓ ≤ φ.cost)
    (S : Solver ops) (g h : α) :
    ((transfer φ S).run g h).1 ≥
      (S.run (φ.toFun g) (φ.toFun h)).1 + 2 * ℓ := by
  have h := floor_crater_transfer_overhead φ S g h
  omega

/-- Linear-Vélu / Hasse comparison specialized to conductor `f₀ = ℓ`:
`3 (2 C ℓ)² ≤ 16 C² q` whenever the Frobenius discriminant carries
`D₀ ≤ −3`. -/
theorem floor_crater_velu_vs_rho {q t D₀ ℓ C : ℤ}
    (hdisc : t ^ 2 - 4 * q = D₀ * ℓ ^ 2) (hD : D₀ ≤ -3) (hℓ : 0 < ℓ)
    (hC : 0 ≤ C) :
    3 * (2 * C * ℓ) ^ 2 ≤ 16 * C ^ 2 * q :=
  linear_velu_gap_is_rho_scale hdisc hD hℓ hC

end FloorCraterEcdlp
