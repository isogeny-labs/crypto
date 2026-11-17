module

public import UniformChoices
public import Mathlib.Data.Fintype.CardEmbedding

@[expose] public section

namespace InjectiveBoxAudit

open HildebrandCellAudit CycleCouplingAudit

noncomputable def indicator {T : Type*} (A : Finset T) (t : T) : ℝ := by
  classical
  exact if t ∈ A then 1 else 0

noncomputable def uniformEvent {T : Type*} [Fintype T] [Nonempty T] (A : Finset T) : ℝ := by
  exact expectation (uniformLaw T) (indicator A)

theorem uniform_event_formula {T : Type*} [Fintype T] [Nonempty T] (A : Finset T) :
    uniformEvent A = (A.card : ℝ)/(Fintype.card T : ℝ) := by
  classical
  unfold uniformEvent expectation indicator
  simp_rw [uniform_real]
  calc
    _ = (∑ t : T, if t ∈ A then (1 : ℝ) else 0)/(Fintype.card T : ℝ) := by
      rw [Finset.sum_div]
      apply Finset.sum_congr rfl
      intro t _
      ring
    _ = _ := by simp

theorem uniform_event_nonneg {T : Type*} [Fintype T] [Nonempty T] (A : Finset T) :
    0 ≤ uniformEvent A := by rw [uniform_event_formula]; positivity

theorem uniform_event_le_one {T : Type*} [Fintype T] [Nonempty T] (A : Finset T) :
    uniformEvent A ≤ 1 := by
  have hc : (A.card : ℝ) ≤ (Fintype.card T : ℝ) := by exact_mod_cast A.card_le_univ
  have hp : (0 : ℝ) < Fintype.card T := by exact_mod_cast (Fintype.card_pos : 0 < Fintype.card T)
  rw [uniform_event_formula]
  exact (div_le_one hp).mpr hc

theorem no_pmf_on_empty (T : Type*) [IsEmpty T] : IsEmpty (PMF T) := by
  refine ⟨fun q => ?_⟩
  have h := q.tsum_coe
  simp at h

theorem expectation_map {A B : Type*} [Fintype A] [Fintype B]
    (q : PMF A) (f : A → B) (g : B → ℝ) :
    expectation (q.map f) g = expectation q (fun a => g (f a)) := by
  rw [PMF.map,WeightedCycleCouplingAudit.expectation_bind]
  simp only [Function.comp_apply,expectation_pure]
  rfl

theorem expectation_one {T : Type*} [Fintype T] (q : PMF T) :
    expectation q (fun _ => 1) = 1 := by
  simpa only [expectation,mul_one,WeightedCycleCouplingAudit.weight] using
    WeightedCycleCouplingAudit.weight_sum q

end InjectiveBoxAudit
