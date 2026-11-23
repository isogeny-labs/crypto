import Lean.Elab.Tactic.Omega

/-!
Disk preflight used before building the fixed-workflow pair table.

`buildPairs` computes

    required = planned * bytes_per_pair * 5 / 4 + 32 MiB

with Python arbitrary-precision integers. This file proves the arithmetic
properties the guard relies on:

* the safety overhead is always included;
* the estimate is at least the raw planned bytes;
* increasing either the planned work or the observed bytes per pair never
  lowers the estimate.

The integer model matches the Python expression exactly (floor division by 4).
-/

namespace DiskBudget

def overhead : Nat := 32 * 1024 * 1024

def required (planned bytesPerPair : Nat) : Nat :=
  planned * bytesPerPair * 5 / 4 + overhead

theorem required_overhead (planned bytesPerPair : Nat) :
    overhead ≤ required planned bytesPerPair := by
  unfold required
  omega

theorem required_zero (bytesPerPair : Nat) :
    required 0 bytesPerPair = overhead := by
  simp [required, overhead]

theorem required_at_least_raw (planned bytesPerPair : Nat) :
    planned * bytesPerPair ≤ required planned bytesPerPair := by
  unfold required
  have h : planned * bytesPerPair ≤ planned * bytesPerPair * 5 / 4 := by omega
  omega

theorem required_mono_planned {planned₁ planned₂ bytesPerPair : Nat}
    (h : planned₁ ≤ planned₂) :
    required planned₁ bytesPerPair ≤ required planned₂ bytesPerPair := by
  unfold required
  have hmul : planned₁ * bytesPerPair * 5 ≤ planned₂ * bytesPerPair * 5 :=
    Nat.mul_le_mul_right _ (Nat.mul_le_mul_right _ h)
  have hdiv : planned₁ * bytesPerPair * 5 / 4 ≤ planned₂ * bytesPerPair * 5 / 4 :=
    Nat.div_le_div_right hmul
  omega

theorem required_mono_bytes {planned bytesPerPair₁ bytesPerPair₂ : Nat}
    (h : bytesPerPair₁ ≤ bytesPerPair₂) :
    required planned bytesPerPair₁ ≤ required planned bytesPerPair₂ := by
  unfold required
  have hmul : planned * bytesPerPair₁ * 5 ≤ planned * bytesPerPair₂ * 5 := by
    exact Nat.mul_le_mul_right _ (Nat.mul_le_mul_left _ h)
  have hdiv : planned * bytesPerPair₁ * 5 / 4 ≤ planned * bytesPerPair₂ * 5 / 4 :=
    Nat.div_le_div_right hmul
  omega

/-- Reserve for the final `pair_lookup` index when a bulk build keeps it off. -/
def indexAllowance (totalPairs : Nat) : Nat := totalPairs * 192

def pairTableRequired (planned bytesPerPair totalPairs : Nat) (indexPresent : Bool) : Nat :=
  required planned bytesPerPair +
    (if indexPresent then 0 else indexAllowance totalPairs)

theorem pair_table_required_ge (planned bytesPerPair totalPairs : Nat) (indexPresent : Bool) :
    required planned bytesPerPair ≤ pairTableRequired planned bytesPerPair totalPairs indexPresent := by
  unfold pairTableRequired
  split <;> omega

theorem pair_table_required_mono_total {total₁ total₂ planned bytesPerPair : Nat}
    (h : total₁ ≤ total₂) :
    pairTableRequired planned bytesPerPair total₁ false ≤
      pairTableRequired planned bytesPerPair total₂ false := by
  unfold pairTableRequired indexAllowance
  simp
  have hmul : total₁ * 192 ≤ total₂ * 192 := Nat.mul_le_mul_right 192 h
  omega

end DiskBudget

#print axioms DiskBudget.required_overhead
#print axioms DiskBudget.required_zero
#print axioms DiskBudget.required_at_least_raw
#print axioms DiskBudget.required_mono_planned
#print axioms DiskBudget.required_mono_bytes
#print axioms DiskBudget.pair_table_required_ge
#print axioms DiskBudget.pair_table_required_mono_total
