import SeaProofs

/- The audit's finite certificates establish a divisor d of the claimed
   order. Lagrange gives divisibility of the true group order, and Hasse
   supplies the two interval hypotheses. This theorem closes the arithmetic
   uniqueness step; it does not verify the Python point calculations. -/
namespace RegistryOrderUniqueness

theorem point_order_forces_group_order
    (p radius claimed actual divisor : Nat)
    (hclaimed : p + 1 - radius ≤ claimed ∧ claimed ≤ p + 1 + radius)
    (hactual : p + 1 - radius ≤ actual ∧ actual ≤ p + 1 + radius)
    (hradius : radius * radius ≤ 4 * p)
    (hlarge : 16 * p < divisor * divisor)
    (hclaimed_div : divisor ∣ claimed)
    (hactual_div : divisor ∣ actual) : claimed = actual := by
  have hgap : 2 * radius < divisor := by
    by_cases h : 2 * radius < divisor
    · exact h
    · have hle : divisor ≤ 2 * radius := Nat.le_of_not_gt h
      have hsquare : divisor * divisor ≤ (2 * radius) * (2 * radius) :=
        Nat.mul_le_mul hle hle
      have hscaled : 4 * (radius * radius) ≤ 4 * (4 * p) :=
        Nat.mul_le_mul_left 4 hradius
      have hcontradiction : divisor * divisor ≤ 16 * p := by
        calc
          divisor * divisor ≤ (2 * radius) * (2 * radius) := hsquare
          _ = 4 * (radius * radius) := by
            simp [Nat.mul_assoc, Nat.mul_left_comm, Nat.mul_comm]
          _ ≤ 4 * (4 * p) := hscaled
          _ = 16 * p := by omega
      omega
  have hwidth : (p + 1 + radius) - (p + 1 - radius) < divisor := by
    omega
  exact SeaProofs.interval_unique
    (p + 1 - radius) (p + 1 + radius) claimed actual divisor
    hclaimed hactual hwidth hclaimed_div hactual_div

end RegistryOrderUniqueness

#print axioms RegistryOrderUniqueness.point_order_forces_group_order
