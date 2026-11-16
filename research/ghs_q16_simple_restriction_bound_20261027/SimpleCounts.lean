-- Finite census accounting. Frobenius irreducibility and the geometric
-- dimension implication are checked externally and stated in REPORT.md.
theorem all_models : 7584 + 606 = 8190 := by omega
theorem all_orders : 120 + 8 = 128 := by omega
theorem residual_models : 2691 + 184 = 2875 := by omega
theorem residual_orders : 46 + 3 = 49 := by omega
theorem simple_residual_catalog : 2643 + 48 = 2691 := by omega
theorem simple_residual_catalog_orders : 44 + 2 = 46 := by omega
theorem complement_4006 : 4006 * 22 = 88132 := by omega
theorem complement_4054 : 4054 * 14 = 56756 := by omega
theorem complement_4078 : 4078 * 14 = 57092 := by omega
theorem complement_4126 : 4126 * 14 = 57764 := by omega

-- The Hasse-Weil input r ≤ 25^g is supplied by the mathematical argument.
theorem genus_at_least_three_of_large_subgroup
    (r g : Nat) (hr : 625 < r) (hbound : r ≤ 25 ^ g) : 3 ≤ g := by
  by_cases h : 3 ≤ g
  · exact h
  · have hg : g = 0 ∨ g = 1 ∨ g = 2 := by omega
    cases hg with
    | inl h0 =>
        subst g
        simp at hbound
        omega
    | inr h12 =>
        cases h12 with
        | inl h1 =>
            subst g
            simp at hbound
            omega
        | inr h2 =>
            subst g
            simp at hbound
            omega
