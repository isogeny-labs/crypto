module

public import CenteredCell

@[expose] public section

namespace HildebrandCellAudit

open CycleCouplingAudit

noncomputable def cellProbability (p : ℕ) [Fact p.Prime] : ℝ := by
  classical
  exact expectation (uniformLaw (Choice p)) (fun u => if printedCell u then 1 else 0)

variable {p : ℕ} [Fact p.Prime]

theorem cell_probability_exact (hp : 3 ≤ p) :
    cellProbability p = 1/((p-1 : ℕ) : ℝ) := by
  classical
  unfold cellProbability expectation
  simp_rw [uniform_real,printed_cell_singleton hp]
  simp only [mul_ite,mul_one,mul_zero,Finset.sum_ite_eq',Finset.mem_univ,↓reduceIte]
  rw [choice_card]

theorem cell_bound_violated (hp : 3 ≤ p) : printedBound p < cellProbability p := by
  have hpr : (0 : ℝ)<p := by exact_mod_cast (Fact.out : p.Prime).pos
  have hp1 : (1 : ℝ)<p := by exact_mod_cast (show 1<p by omega)
  rw [(printed_formulas p (Fact.out : p.Prime).pos).2.2,cell_probability_exact hp,
    Nat.cast_sub (by omega : 1 ≤ p),Nat.cast_one]
  apply (div_lt_div_iff₀ (by positivity : (0 : ℝ)<40*p) (by linarith : (0 : ℝ)<p-1)).mpr
  nlinarith

theorem original_cell_bound_false (hp : 3 ≤ p) : ¬ cellProbability p ≤ printedBound p :=
  not_le.mpr (cell_bound_violated hp)

theorem no_eventual_prime_cell_bound :
    ¬ ∃ N : ℕ, ∀ (p : ℕ) [Fact p.Prime], N ≤ p → 3 ≤ p →
      cellProbability p ≤ printedBound p := by
  rintro ⟨N,h⟩
  obtain ⟨p,hp,hprime⟩ := Nat.exists_infinite_primes (max N 3)
  exact @original_cell_bound_false p ⟨hprime⟩ (by omega)
    (@h p ⟨hprime⟩ (by omega) (by omega))

end HildebrandCellAudit
