module

public import CellProbability
public import SourceAndControl

public section

namespace HildebrandCellAudit

/-- Acknowledged k=2 failure of the original printed Lemma4, with faithful
uniform source choices and canonical representatives; valid mixing is retained. -/
theorem hildebrand_cell_certificate :
    (∀ p : ℕ, 0 < p → printedLower p=1/(p : ℝ) ∧
      printedUpper p=5/(4*(p : ℝ)) ∧ printedBound p=11/(40*(p : ℝ))) ∧
    (∀ (p : ℕ) [Fact p.Prime], Fintype.card (Choice p)=p-1 ∧
      (uniformLaw (Choice p)).map supportEquiv=uniformLaw (ConditionalSupports p) ∧
      ∀ u : Choice p, pairSupport u 0=0 ∧ Function.Injective (pairSupport u) ∧
        (pairSupport u 0-pairSupport u 1)*(1 : ZMod p) = -u.val) ∧
    (∀ (p : ℕ) [Fact p.Prime] (z : ZMod p),
      (centered z : ZMod p)=z ∧ -(p : ℤ)<2*centered z ∧ 2*centered z ≤ p ∧
      ∀ v : ℤ, (v : ZMod p)=z → -(p : ℤ)<2*v → 2*v ≤ p → v=centered z) ∧
    (∀ (p : ℕ) [Fact p.Prime], 3 ≤ p → ∀ u : Choice p, printedCell u ↔ u=minusOneChoice) ∧
    (∀ (p : ℕ) [Fact p.Prime], 3 ≤ p →
      cellProbability p=1/((p-1 : ℕ) : ℝ) ∧ printedBound p<cellProbability p) ∧
    (¬ ∃ N : ℕ, ∀ (p : ℕ) [Fact p.Prime], N ≤ p → 3 ≤ p →
      cellProbability p ≤ printedBound p) ∧
    (∀ (w : Bool → NNReal) (hw : ∑ i, w i=1), (∀ i, 0<w i) →
      ∀ ε : ℝ, 0<ε → ∃ γ : ℝ, 0<γ ∧
      ∀ (p : ℕ) [Fact p.Prime] (g : Bool → ZMod p), Function.Injective g →
      ∀ t : ℕ, γ*(p : ℝ)^2 ≤ t → ∀ (x : ZMod p) (A : Finset (ZMod p)),
      abs (WeightedCycleCouplingAudit.probability (WeightedCycleCouplingAudit.labelLaw w hw)
        g t x A-(A.card : ℝ)/p)<ε) := by
  refine ⟨printed_formulas,?_,?_,?_,?_,no_eventual_prime_cell_bound,?_⟩
  · intro p _
    exact ⟨choice_card,actual_uniform_support_law,actual_source_argument⟩
  · intro p _ z
    exact ⟨centered_cast z,(centered_range z).1,(centered_range z).2,centered_unique z⟩
  · intro p _ hp u
    exact printed_cell_singleton hp u
  · intro p _ hp
    exact ⟨cell_probability_exact hp,cell_bound_violated hp⟩
  · intro w hw hpos ε hε
    exact WeightedCycleCouplingAudit.hildebrand_theorem_two w hw hpos ε hε

#print axioms hildebrand_cell_certificate

end HildebrandCellAudit
