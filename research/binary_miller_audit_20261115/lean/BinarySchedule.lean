module
public import NumericalTrace

@[expose] public section
namespace BinaryMillerAudit

/-- Number of recursive binary iterations after the leading-bit base case, with n0 convention0. -/
def binaryRounds (n : ℕ) : ℕ :=
  if h : 2 ≤ n then binaryRounds (n / 2) + 1 else 0
termination_by n
decreasing_by exact Nat.div_lt_self (by omega) (by decide)

/-- Number of conditional additions of P; this counts bit positions, not native runtime. -/
def binaryAdditions (n : ℕ) : ℕ :=
  if h : 2 ≤ n then binaryAdditions (n / 2) + n % 2 else 0
termination_by n
decreasing_by exact Nat.div_lt_self (by omega) (by decide)

theorem binary_rounds_eq_log2 (n : ℕ) : binaryRounds n = Nat.log2 n := by
  induction n using Nat.strong_induction_on with
  | h n ih =>
    rw [binaryRounds, Nat.log2_def]
    split_ifs with h
    all_goals first
      | rfl
      | rw [ih (n / 2) (Nat.div_lt_self (by omega) (by decide))]

theorem binary_additions_le_rounds (n : ℕ) : binaryAdditions n ≤ binaryRounds n := by
  induction n using Nat.strong_induction_on with
  | h n ih =>
    rw [binaryAdditions, binaryRounds]
    split_ifs with h
    all_goals first
      | omega
      | exact Nat.add_le_add (ih (n / 2) (Nat.div_lt_self (by omega) (by decide)))
          (by omega)

theorem three_rounds : binaryRounds 3 = 1 := by decide +kernel

theorem three_additions : binaryAdditions 3 = 1 := by decide +kernel

#print axioms binary_rounds_eq_log2
#print axioms binary_additions_le_rounds
end BinaryMillerAudit
