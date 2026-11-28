module

public import UniformEvents
public import Mathlib.Data.Fintype.EquivFin

@[expose] public section

namespace InjectiveBoxAudit

open HildebrandCellAudit CycleCouplingAudit

abbrev Sample (X : Type*) (d : ℕ) := Fin d ↪ X

variable {X : Type*} [Fintype X] {d : ℕ}

theorem sample_nonempty (h : d ≤ Fintype.card X) : Nonempty (Sample X d) :=
  Function.Embedding.nonempty_of_card_le (by simpa using h)

theorem sample_card : Fintype.card (Sample X d) = (Fintype.card X).descFactorial d := by
  simp only [Sample,Fintype.card_embedding_eq,Fintype.card_fin]

noncomputable def sampleLaw (X : Type*) [Fintype X] (d : ℕ)
    (h : d ≤ Fintype.card X) : PMF (Sample X d) :=
  @uniformLaw (Sample X d) inferInstance (sample_nonempty h)

theorem sample_law_pointmass (h : d ≤ Fintype.card X) (e : Sample X d) :
    (sampleLaw X d h e).toReal = 1/((Fintype.card X).descFactorial d : ℝ) := by
  simpa only [sampleLaw,sample_card] using
    @uniform_real (Sample X d) inferInstance (sample_nonempty h) e

noncomputable def boxEvent (B : Fin d → Finset X) : Finset (Sample X d) := by
  classical
  exact Finset.univ.filter (fun e => ∀ i, e i ∈ B i)

abbrev BoxMembers (B : Fin d → Finset X) := {e : Sample X d // ∀ i, e i ∈ B i}

open Classical in
theorem box_event_card (B : Fin d → Finset X) :
    (boxEvent B).card = Fintype.card (BoxMembers B) := by
  simp only [boxEvent,BoxMembers,Fintype.card_subtype]

def boxToPi (B : Fin d → Finset X) (e : BoxMembers B) : ∀ i, {x : X // x ∈ B i} :=
  fun i => ⟨e.val i,e.property i⟩

omit [Fintype X] in
theorem box_to_pi_injective (B : Fin d → Finset X) : Function.Injective (boxToPi B) := by
  intro e f h
  apply Subtype.ext
  ext i
  exact congrArg Subtype.val (congrFun h i)

open Classical in
theorem box_card_bound (B : Fin d → Finset X) : (boxEvent B).card ≤ ∏ i, (B i).card := by
  rw [box_event_card]
  calc
    _ ≤ Fintype.card (∀ i, {x : X // x ∈ B i}) :=
      Fintype.card_le_of_injective (boxToPi B) (box_to_pi_injective B)
    _ = _ := by simp

noncomputable def boxProbability (X : Type*) [Fintype X] (d : ℕ)
    (h : d ≤ Fintype.card X) (B : Fin d → Finset X) : ℝ := by
  exact expectation (sampleLaw X d h) (indicator (boxEvent B))

theorem box_probability_exact (h : d ≤ Fintype.card X) (B : Fin d → Finset X) :
    boxProbability X d h B = ((boxEvent B).card : ℝ)/((Fintype.card X).descFactorial d : ℝ) := by
  have h₀ := @uniform_event_formula (Sample X d) inferInstance (sample_nonempty h) (boxEvent B)
  simpa only [uniformEvent,boxProbability,sampleLaw,sample_card] using h₀

theorem box_probability_bound (h : d ≤ Fintype.card X) (B : Fin d → Finset X) :
    boxProbability X d h B ≤ (∏ i, (B i).card : ℕ)/((Fintype.card X).descFactorial d : ℝ) := by
  rw [box_probability_exact]
  apply div_le_div_of_nonneg_right
  · exact_mod_cast box_card_bound B
  · positivity

theorem sample_denominator_positive (h : d ≤ Fintype.card X) :
    (0 : ℝ) < (Fintype.card X).descFactorial d := by
  exact_mod_cast (Nat.descFactorial_pos.mpr h)

theorem no_samples_if_too_many (h : Fintype.card X < d) : IsEmpty (Sample X d) := by
  refine ⟨fun e => (not_le_of_gt h) ?_⟩
  simpa using Fintype.card_le_of_embedding e

theorem no_sampler_if_too_many (h : Fintype.card X < d) : IsEmpty (PMF (Sample X d)) :=
  @no_pmf_on_empty (Sample X d) (no_samples_if_too_many h)

end InjectiveBoxAudit
