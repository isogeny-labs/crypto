module
public import BinarySchedule

@[expose] public section
namespace BinaryMillerAudit
noncomputable section
open WeierstrassCurve.Affine
open MillerRecursionAudit MillerBasechangeAudit FunctionDivisorFinitenessAudit
variable {K : Type*} [Field K] [DecidableEq K]

/-- Actual carried point/function state. Point multiples occur in the proof, not in each iteration. -/
def binaryState (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) :
    W.Point × W.FunctionField :=
  if _ : 2 ≤ n then
    let s := binaryState W P (n / 2)
    let R := s.1 + s.1
    let f := s.2 * s.2 * stepFunction W s.1 s.1
    if n % 2 = 0 then (R, f) else (R + P, f * stepFunction W R P)
  else if n = 0 then (0, 1) else (P, 1)
termination_by n
decreasing_by exact Nat.div_lt_self (by omega) (by decide)

theorem binary_state_large (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) (hn : 2 ≤ n) :
    binaryState W P n =
      let s := binaryState W P (n / 2)
      let R := s.1 + s.1
      let f := s.2 * s.2 * stepFunction W s.1 s.1
      if n % 2 = 0 then (R, f) else (R + P, f * stepFunction W R P) := by
  rw [binaryState]
  split_ifs <;> simp_all

theorem binary_state_small (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) (hn : ¬ 2 ≤ n) :
    binaryState W P n = if n = 0 then (0, 1) else (P, 1) := by
  rw [binaryState]
  split_ifs <;> simp_all

theorem binary_state_correct (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point) (n : ℕ) :
    binaryState W P n = (n • P, millerFunction W P n) := by
  induction n using Nat.strong_induction_on with
  | h n ih =>
    by_cases hn : 2 ≤ n
    · have hlt : n / 2 < n := Nat.div_lt_self (by omega) (by decide)
      rw [binary_state_large W P n hn, ih (n / 2) hlt]
      dsimp only
      split_ifs with he
      · have hne : n = n / 2 + n / 2 := by
          have hm := Nat.mod_add_div n 2
          omega
        refine Prod.ext ?_ ?_
        · change (n / 2) • P + (n / 2) • P = n • P
          rw [← add_nsmul, ← hne]
        · change millerFunction W P (n / 2) * millerFunction W P (n / 2) *
            stepFunction W ((n / 2) • P) ((n / 2) • P) = millerFunction W P n
          have hf := chosen_miller_addition_all_fields W hΔ P (n / 2) (n / 2)
          rw [← hf, ← hne]
      · have hne : n = n / 2 + n / 2 + 1 := by
          have hm := Nat.mod_add_div n 2
          have hr := Nat.mod_lt n (by decide : 0 < 2)
          omega
        refine Prod.ext ?_ ?_
        · change ((n / 2) • P + (n / 2) • P) + P = n • P
          calc
            ((n / 2) • P + (n / 2) • P) + P = (n / 2 + n / 2 + 1) • P := by
              simp only [add_nsmul, one_nsmul]
            _ = n • P := congrArg (fun m : ℕ => m • P) hne.symm
        · change (millerFunction W P (n / 2) * millerFunction W P (n / 2) *
            stepFunction W ((n / 2) • P) ((n / 2) • P)) *
            stepFunction W (((n / 2) • P) + ((n / 2) • P)) P = millerFunction W P n
          have hf := chosen_miller_addition_all_fields W hΔ P (n / 2) (n / 2)
          have hg := chosen_miller_addition_all_fields W hΔ P (n / 2 + n / 2) 1
          have h1 : millerFunction W P 1 = 1 := by
            change (1 : W.FunctionField) * 1 = 1
            exact mul_one 1
          rw [h1, one_nsmul, mul_one] at hg
          rw [← hf, ← add_nsmul, ← hg, ← hne]
    · have hs : n = 0 ∨ n = 1 := by omega
      rcases hs with rfl | rfl
      · simp [binary_state_small W P 0 (by omega), millerFunction]
      · simp [binary_state_small W P 1 (by omega), millerFunction, stepFunction]

abbrev binaryFunction (W : WeierstrassCurve.Affine K) (P : W.Point) (n : ℕ) : W.FunctionField :=
  (binaryState W P n).2

theorem binary_function_correct (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point) (n : ℕ) :
    binaryFunction W P n = millerFunction W P n :=
  congrArg Prod.snd (binary_state_correct W hΔ P n)

theorem binary_point_correct (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point) (n : ℕ) :
    (binaryState W P n).1 = n • P := congrArg Prod.fst (binary_state_correct W hΔ P n)

theorem binary_function_nonzero (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point) (n : ℕ) :
    binaryFunction W P n ≠ 0 := by
  rw [binary_function_correct W hΔ P n]
  exact miller_function_nonzero W P n

theorem binary_geometric_miller (W : WeierstrassCurve.Affine K) (hΔ : W.Δ ≠ 0) (P : W.Point) (n : ℕ) :
    GeometricMiller W P n (binaryFunction W P n) := by
  rw [binary_function_correct W hΔ P n]
  exact constructed_geometric_miller W hΔ P n

#print axioms binary_state_correct
#print axioms binary_geometric_miller
end
end BinaryMillerAudit
