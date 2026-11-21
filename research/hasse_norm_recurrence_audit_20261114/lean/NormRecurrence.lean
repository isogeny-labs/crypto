module

public import SparseCoefficient
public import Mathlib.Algebra.CharP.Frobenius

@[expose] public section
namespace HasseNormRecurrenceAudit
noncomputable section
open Polynomial Finset
variable {K : Type*} [Field K]

def coefficientAt (f : K[X]) (Q : ℕ) : K := (f ^ ((Q - 1) / 2)).coeff (Q - 1)

theorem half_product (M p : ℕ) (hM : Odd M) (hp : Odd p) :
    (M * p - 1) / 2 = (M - 1) / 2 + M * ((p - 1) / 2) := by
  obtain ⟨m, hm⟩ := hM
  obtain ⟨n, hn⟩ := hp
  have hM2 : 2 * ((M - 1) / 2) + 1 = M := by omega
  have hp2 : 2 * ((p - 1) / 2) + 1 = p := by omega
  have hmuladd : 2 * ((M - 1) / 2 + M * ((p - 1) / 2)) + 1 = M * p := by
    calc
      _ = M * (2 * ((p - 1) / 2) + 1) := by nlinarith
      _ = M * p := by rw [hp2]
  have hmul : M * p - 1 = 2 * ((M - 1) / 2 + M * ((p - 1) / 2)) := by omega
  rw [hmul, Nat.mul_div_cancel_left _ (by decide)]

theorem half_power_degree (f : K[X]) (hf : f.natDegree ≤ 3)
    (M : ℕ) (hM : Odd M) :
    (f ^ ((M - 1) / 2)).natDegree < 2 * M - 1 := by
  obtain ⟨m, hm⟩ := hM
  have hd := natDegree_pow_le_of_le ((M - 1) / 2) hf
  omega

variable (p : ℕ) [Fact (Nat.Prime p)] [CharP K p]

/-- The source recurrence uses actual coefficients and actual Frobenius powers. -/
theorem coefficient_recurrence (hp : p ≠ 2) (f : K[X]) (hf : f.natDegree ≤ 3)
    (r : ℕ) :
    coefficientAt f (p ^ (r + 1)) =
      coefficientAt f (p ^ r) * coefficientAt f p ^ (p ^ r) := by
  have hpprime := Fact.out (p := Nat.Prime p)
  have hpodd := hpprime.odd_of_ne_two hp
  have hM : Odd (p ^ r) := hpodd.pow
  have hpos : 0 < p ^ r := pow_pos hpprime.pos r
  have hindex : p ^ r * p - 1 = p ^ r * (p - 1) + (p ^ r - 1) := by
    have hp1 : 1 ≤ p := hpprime.one_lt.le
    have hm1 : 1 ≤ p ^ r := hpos
    have hpadd : p - 1 + 1 = p := by omega
    have hprod : p ^ r * (p - 1) + p ^ r = p ^ r * p := by
      calc
        _ = p ^ r * (p - 1 + 1) := by ring
        _ = p ^ r * p := by rw [hpadd]
    omega
  have hpower : f ^ (p ^ r * ((p - 1) / 2)) = (f ^ ((p - 1) / 2)) ^ (p ^ r) := by
    rw [mul_comm (p ^ r) ((p - 1) / 2), pow_mul]
  unfold coefficientAt
  rw [pow_succ, half_product (p ^ r) p hM hpodd, pow_add, hpower,
    ← map_iterateFrobenius_expand p (f ^ ((p - 1) / 2)) r, hindex,
    sparse_product_coefficient (f ^ ((p ^ r - 1) / 2))
      (f ^ ((p - 1) / 2)) (iterateFrobenius K p r) (p ^ r) (p - 1)
      hpos (half_power_degree f hf (p ^ r) hM)]
  rfl

theorem coefficient_norm (hp : p ≠ 2) (f : K[X]) (hf : f.natDegree ≤ 3)
    (r : ℕ) :
    coefficientAt f (p ^ r) = coefficientAt f p ^ (∑ i ∈ range r, p ^ i) := by
  induction r with
  | zero => simp [coefficientAt]
  | succ r ih =>
    rw [coefficient_recurrence p hp f hf r, ih, sum_range_succ, pow_add]

theorem coefficient_zero_iff (hp : p ≠ 2) (f : K[X]) (hf : f.natDegree ≤ 3)
    (r : ℕ) (hr : 0 < r) : coefficientAt f (p ^ r) = 0 ↔ coefficientAt f p = 0 := by
  have hsum : 0 < ∑ i ∈ range r, p ^ i := by
    have hle := Finset.single_le_sum (fun i (_ : i ∈ range r) => Nat.zero_le (p ^ i))
      (show 0 ∈ range r from Finset.mem_range.mpr hr)
    simp only [pow_zero] at hle
    omega
  rw [coefficient_norm p hp f hf r, pow_eq_zero_iff (Nat.ne_of_gt hsum)]

#print axioms coefficient_recurrence
#print axioms coefficient_norm
#print axioms coefficient_zero_iff
end
end HasseNormRecurrenceAudit
