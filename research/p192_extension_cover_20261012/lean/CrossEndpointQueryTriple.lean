import Mathlib.Tactic.Abel

/-!
A cubic's three projected roots have a common-sum relation, and each complete
line-cut bundle has a six-column sum relation. Taking bundles at all three
roots cancels the unknown endpoints; comparing two query cubics cancels the
section and cubic constants, leaving a known query-point multiple.
-/

namespace P192ExtensionCover

variable {G : Type*} [AddCommGroup G]

/-- Cross-endpoint known-scalar relation for two three-root query cubics.
`X₁`, `X₂` are sums of three complete six-column bundles, `U₁`, `U₂`
are sums of their respective endpoint roots, and `A₁`, `A₂` are the sums
of the three known bundle-query points. Each aggregate bundle equation is
the sum of three verified individual six-column equations. -/
theorem cross_endpoint_query_triple_difference
    (X₁ X₂ U₁ U₂ A₁ A₂ q₁ q₂ S C : G) (k : ℕ)
    (hc₁ : U₁ + k • q₁ = C) (hc₂ : U₂ + k • q₂ = C)
    (hb₁ : X₁ + 3 • U₁ + 3 • C = 9 • S + k • A₁)
    (hb₂ : X₂ + 3 • U₂ + 3 • C = 9 • S + k • A₂) :
    X₁ - X₂ = k • ((3 • q₁ + A₁) - (3 • q₂ + A₂)) := by
  have hu₁ : U₁ = C - k • q₁ := by
    calc
      U₁ = (U₁ + k • q₁) - k • q₁ := by abel
      _ = C - k • q₁ := by rw [hc₁]
  have hu₂ : U₂ = C - k • q₂ := by
    calc
      U₂ = (U₂ + k • q₂) - k • q₂ := by abel
      _ = C - k • q₂ := by rw [hc₂]
  rw [hu₁] at hb₁
  rw [hu₂] at hb₂
  have hx₁ : X₁ = 9 • S + k • A₁ - 3 • (C - k • q₁) - 3 • C := by
    calc
      X₁ = (X₁ + 3 • (C - k • q₁) + 3 • C) -
            3 • (C - k • q₁) - 3 • C := by abel
      _ = 9 • S + k • A₁ - 3 • (C - k • q₁) - 3 • C := by rw [hb₁]
  have hx₂ : X₂ = 9 • S + k • A₂ - 3 • (C - k • q₂) - 3 • C := by
    calc
      X₂ = (X₂ + 3 • (C - k • q₂) + 3 • C) -
            3 • (C - k • q₂) - 3 • C := by abel
      _ = 9 • S + k • A₂ - 3 • (C - k • q₂) - 3 • C := by rw [hb₂]
  rw [hx₁, hx₂]
  simp only [nsmul_sub, nsmul_add]
  have hcomm (q : G) : 3 • (k • q) = k • (3 • q) := by
    calc
      3 • (k • q) = (k * 3) • q := (mul_nsmul q k 3).symm
      _ = (3 * k) • q := by rw [Nat.mul_comm k 3]
      _ = k • (3 • q) := mul_nsmul q 3 k
  rw [hcomm q₁, hcomm q₂]
  abel


/-- Summing three individual six-column bundle equations gives the
aggregate hypothesis used by the cross-endpoint cancellation theorem. -/
theorem aggregate_three_query_bundles
    (x₁ x₂ x₃ u₁ u₂ u₃ a₁ a₂ a₃ S C : G) (k : ℕ)
    (h₁ : x₁ + 3 • u₁ + C = 3 • S + k • a₁)
    (h₂ : x₂ + 3 • u₂ + C = 3 • S + k • a₂)
    (h₃ : x₃ + 3 • u₃ + C = 3 • S + k • a₃) :
    (x₁ + x₂ + x₃) + 3 • (u₁ + u₂ + u₃) + 3 • C =
      9 • S + k • (a₁ + a₂ + a₃) := by
  calc
    _ = (x₁ + 3 • u₁ + C) + (x₂ + 3 • u₂ + C) +
          (x₃ + 3 • u₃ + C) := by simp only [nsmul_add]; abel
    _ = (3 • S + k • a₁) + (3 • S + k • a₂) +
          (3 • S + k • a₃) := by rw [h₁, h₂, h₃]
    _ = 9 • S + k • (a₁ + a₂ + a₃) := by simp only [nsmul_add]; abel

/-- The complete two-cubic relation from six individual bundle equations
and two projected-root sums. -/
theorem cross_endpoint_query_triple_from_six_bundles
    (x₁₁ x₁₂ x₁₃ x₂₁ x₂₂ x₂₃
      u₁₁ u₁₂ u₁₃ u₂₁ u₂₂ u₂₃
      a₁₁ a₁₂ a₁₃ a₂₁ a₂₂ a₂₃ q₁ q₂ S C : G) (k : ℕ)
    (hc₁ : u₁₁ + u₁₂ + u₁₃ + k • q₁ = C)
    (hc₂ : u₂₁ + u₂₂ + u₂₃ + k • q₂ = C)
    (h₁₁ : x₁₁ + 3 • u₁₁ + C = 3 • S + k • a₁₁)
    (h₁₂ : x₁₂ + 3 • u₁₂ + C = 3 • S + k • a₁₂)
    (h₁₃ : x₁₃ + 3 • u₁₃ + C = 3 • S + k • a₁₃)
    (h₂₁ : x₂₁ + 3 • u₂₁ + C = 3 • S + k • a₂₁)
    (h₂₂ : x₂₂ + 3 • u₂₂ + C = 3 • S + k • a₂₂)
    (h₂₃ : x₂₃ + 3 • u₂₃ + C = 3 • S + k • a₂₃) :
    (x₁₁ + x₁₂ + x₁₃) - (x₂₁ + x₂₂ + x₂₃) =
      k • ((3 • q₁ + (a₁₁ + a₁₂ + a₁₃)) -
        (3 • q₂ + (a₂₁ + a₂₂ + a₂₃))) := by
  exact cross_endpoint_query_triple_difference
    (x₁₁ + x₁₂ + x₁₃) (x₂₁ + x₂₂ + x₂₃)
    (u₁₁ + u₁₂ + u₁₃) (u₂₁ + u₂₂ + u₂₃)
    (a₁₁ + a₁₂ + a₁₃) (a₂₁ + a₂₂ + a₂₃)
    q₁ q₂ S C k hc₁ hc₂
    (aggregate_three_query_bundles x₁₁ x₁₂ x₁₃ u₁₁ u₁₂ u₁₃
      a₁₁ a₁₂ a₁₃ S C k h₁₁ h₁₂ h₁₃)
    (aggregate_three_query_bundles x₂₁ x₂₂ x₂₃ u₂₁ u₂₂ u₂₃
      a₂₁ a₂₂ a₂₃ S C k h₂₁ h₂₂ h₂₃)

#print axioms aggregate_three_query_bundles
#print axioms cross_endpoint_query_triple_from_six_bundles

#print axioms cross_endpoint_query_triple_difference

end P192ExtensionCover
