module

public import BoxAsymptotic
public import CellProbability

public section

namespace HildebrandBoxAsymptoticAudit

open Filter InjectiveBoxAudit HildebrandCellAudit
open scoped Topology

/-- Corrected author's k>=3 box lemma with actual source distribution, literal
endpoints, derived asymptotics and the actual older k=2 failure kept explicit. -/
theorem corrected_box_asymptotic_certificate :
    (∀ d : ℕ, Tendsto (fun n : ℕ => ((n-1).descFactorial d : ℝ)/(n : ℝ)^d) atTop (𝓝 1)) ∧
    (∀ (d : ℕ), 2 ≤ d → ∀ (ε : ℝ), 0<ε →
      Tendsto (sourceWidth d ε) atTop atTop ∧
      (∀ n : ℕ, (sourceWidth d ε n)^d=sourceCoefficient d ε*(n : ℝ)^(d-1)) ∧
      Tendsto (correctionFactor d ε) atTop (𝓝 1)) ∧
    (∀ (d : ℕ) (ε : ℝ) (p : ℕ) (m : Fin d → ℤ) (i : Fin d),
      cellLower d ε p m i=(m i : ℝ)*(ε/2)^(1/(d : ℝ))*
        (p : ℝ)^(((d : ℝ)-1)/(d : ℝ))/(2*(p : ℝ)) ∧
      cellUpper d ε p m i=((m i : ℝ)+1)*(ε/2)^(1/(d : ℝ))*
        (p : ℝ)^(((d : ℝ)-1)/(d : ℝ))/(2*(p : ℝ))) ∧
    (∀ (p d : ℕ) [Fact p.Prime] (a₀ : ZMod p) (h : d ≤ p-1),
      Fintype.card (FullSupports p d a₀)=(p-1).descFactorial d ∧
      sourceLaw p d a₀ h=@uniformLaw (FullSupports p d a₀) inferInstance (full_support_nonempty a₀ h)) ∧
    (∀ (d : ℕ), 2 ≤ d → ∀ (ε : ℝ), 0<ε →
      ∃ N : ℕ, ∃ hN : d+1 ≤ N, ∀ (p : ℕ) [Fact p.Prime] (hp : N ≤ p)
        (a₀ j : ZMod p), j ≠ 0 → ∀ m : Fin d → ℤ,
        sourceProbability p d a₀ j (dimension_fits hN hp) (cellLower d ε p m) (cellUpper d ε p m) ≤
          1.1*(ε/2)/((2 : ℝ)^d*(p : ℝ))) ∧
    (∀ ε : ℝ, (∀ n : ℕ, sourceWidth 1 ε n=ε/4) ∧ ¬ Tendsto (sourceWidth 1 ε) atTop atTop) ∧
    (∀ (p : ℕ) [Fact p.Prime], 3 ≤ p →
      cellProbability p=1/((p-1 : ℕ) : ℝ) ∧ ¬ cellProbability p ≤ printedBound p) ∧
    (¬ ∃ N : ℕ, ∀ (p : ℕ) [Fact p.Prime], N ≤ p → 3 ≤ p → cellProbability p ≤ printedBound p) := by
  refine ⟨actual_denominator_tendsto,?_,literal_cell_endpoints,?_,?_,?_,?_,no_eventual_prime_cell_bound⟩
  · intro d hd ε hε
    exact ⟨source_width_tendsto hd hε,source_width_power (by omega) hε,correction_factor_tendsto hd hε⟩
  · intro p d _ a₀ h
    exact ⟨full_support_card a₀,actual_uniform_source_law a₀ h⟩
  · intro d hd ε hε
    exact corrected_source_box_bound hd hε
  · intro ε
    exact ⟨source_width_one ε,one_width_not_tendsto_atTop ε⟩
  · intro p _ hp
    exact ⟨cell_probability_exact hp,original_cell_bound_false hp⟩

#print axioms corrected_box_asymptotic_certificate

end HildebrandBoxAsymptoticAudit
