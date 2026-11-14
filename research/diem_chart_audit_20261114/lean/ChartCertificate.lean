import Products

namespace DiemChartAudit

/-- A literal source-helper disproof, exact finite-field qualification, and
explicit rational-point repair. This is not a general Bezout proof. -/
theorem actual_torus_chart_certificate :
    (Nat.card (Line (ZMod 2)) = 3 ∧ Nat.card (ZMod 2)ˣ = 1) ∧
    (∀ f g : (ZMod 2)ˣ → Line (ZMod 2),
      ¬ (∀ p, (∃ u, f u = p) ∨ (∃ u, g u = p))) ∧
    (∀ (n : ℕ), 0 < n →
      ∀ f : (Fin n → Fin 2 × (ZMod 2)ˣ) → (Fin n → Line (ZMod 2)),
        ¬ Function.Surjective f) ∧
    (∀ (K : Type) [Field K],
      (∀ i : Fin 3, Function.Injective (threeChart i : Kˣ → Line K)) ∧
      Function.Surjective (fun z : Fin 3 × Kˣ => threeChart z.1 z.2) ∧
      (∀ (n : ℕ) (p : Fin n → Line K),
        ∃ i : Fin n → Fin 3, ∃ u : Fin n → Kˣ,
          ∀ j, threeChart (i j) (u j) = p j)) ∧
    (∀ (K : Type) [Field K] [Finite K],
      (∃ f : Fin 2 × Kˣ → Line K, Function.Surjective f) ↔ 3 ≤ Nat.card K) ∧
    (∀ (K : Type) [Field K] (a : K) (_ha0 : a ≠ 0) (ha1 : a ≠ 1),
      (∀ i : Fin 2, Function.Injective (twoChart a ha1 i)) ∧
      (∀ (n : ℕ) (p : Fin n → Line K),
        ∃ i : Fin n → Fin 2, ∃ u : Fin n → Kˣ,
          ∀ j, twoChart a ha1 (i j) (u j) = p j)) ∧
    (∀ (n m : ℕ)
      (f : Fin m × (Fin n → (ZMod 2)ˣ) → (Fin n → Line (ZMod 2))),
        Function.Surjective f → 3^n ≤ m) ∧
    (∀ n : ℕ, Nat.card (Fin n → Fin 3) = 3^n ∧ 3^n ≤ 5*n.factorial) ∧
    (∀ n d : ℕ, 0 < d →
      3^n * (n.factorial*d^n) ≤ 5*(n.factorial*d^n)^2) := by
  refine ⟨⟨binary_line_card,binary_torus_card⟩,binary_two_images_not_cover,
    binary_product_obstruction,?_,?_,?_,binary_product_chart_lower_bound,
    ?_,chart_calls_times_bezout⟩
  · intro K _
    exact ⟨three_chart_injective,three_chart_surjective,product_three_chart_cover⟩
  · intro K _ _
    exact exists_two_chart_cover_iff
  · intro K _ a ha0 ha1
    exact ⟨two_chart_injective a ha1,product_two_chart_cover a ha0 ha1⟩
  · intro n
    exact ⟨product_chart_count n,chart_factorial_bound n⟩

#print axioms actual_torus_chart_certificate
end DiemChartAudit
