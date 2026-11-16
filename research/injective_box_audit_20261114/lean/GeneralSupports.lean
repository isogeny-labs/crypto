module

public import InjectiveSampling

@[expose] public section

namespace InjectiveBoxAudit

open HildebrandCellAudit

abbrev FullSupports (p d : ℕ) (a₀ : ZMod p) :=
  {f : Fin (d+1) → ZMod p // f 0=a₀ ∧ Function.Injective f}

def supportsFromSample {p d : ℕ} (a₀ : ZMod p) (e : Sample (Choice p) d) : Fin (d+1) → ZMod p :=
  Fin.cases a₀ (fun i => a₀+(e i).val)

theorem supports_from_sample_injective {p d : ℕ} (a₀ : ZMod p) (e : Sample (Choice p) d) :
    Function.Injective (supportsFromSample a₀ e) := by
  intro i j hij
  cases i using Fin.cases with
  | zero =>
    cases j using Fin.cases with
    | zero => rfl
    | succ j =>
      change a₀=a₀+(e j).val at hij
      have hu : (e j).val=0 := add_left_cancel (show a₀+(e j).val=a₀+0 by simpa using hij.symm)
      exact False.elim ((e j).property hu)
  | succ i =>
    cases j using Fin.cases with
    | zero =>
      change a₀+(e i).val=a₀ at hij
      have hu : (e i).val=0 := add_left_cancel (show a₀+(e i).val=a₀+0 by simpa using hij)
      exact False.elim ((e i).property hu)
    | succ j =>
      change a₀+(e i).val=a₀+(e j).val at hij
      have he : e i=e j := Subtype.ext (add_left_cancel hij)
      exact congrArg Fin.succ (e.injective he)

def sampleFromSupports {p d : ℕ} {a₀ : ZMod p} (f : FullSupports p d a₀) : Sample (Choice p) d where
  toFun i := ⟨f.val i.succ-a₀,by
    intro hz
    have he := f.property.2 ((sub_eq_zero.mp hz).trans f.property.1.symm)
    exact Fin.succ_ne_zero i he⟩
  inj' := by
    intro i j hij
    have hval := congrArg Subtype.val hij
    have he : f.val i.succ=f.val j.succ := by
      simpa only [sub_add_cancel] using congrArg (fun z : ZMod p => z+a₀) hval
    exact Fin.succ_injective d (f.property.2 he)

def sourceEquiv {p d : ℕ} (a₀ : ZMod p) : Sample (Choice p) d ≃ FullSupports p d a₀ where
  toFun e := ⟨supportsFromSample a₀ e,rfl,supports_from_sample_injective a₀ e⟩
  invFun := sampleFromSupports
  left_inv e := by
    apply Function.Embedding.ext
    intro i
    apply Subtype.ext
    change (a₀+(e i).val)-a₀=(e i).val
    ring
  right_inv f := by
    apply Subtype.ext
    funext i
    cases i using Fin.cases with
    | zero => exact f.property.1.symm
    | succ i =>
      change a₀+(f.val i.succ-a₀)=f.val i.succ
      ring

variable {p d : ℕ} [Fact p.Prime]

noncomputable instance (a₀ : ZMod p) : Fintype (FullSupports p d a₀) :=
  Fintype.ofFinite {f : Fin (d+1) → ZMod p // f 0=a₀ ∧ Function.Injective f}

theorem full_support_card (a₀ : ZMod p) :
    Fintype.card (FullSupports p d a₀) = (p-1).descFactorial d := by
  have h := Fintype.card_congr (sourceEquiv (d := d) a₀)
  rw [sample_card,choice_card] at h
  exact h.symm

theorem full_support_nonempty (a₀ : ZMod p) (h : d ≤ p-1) : Nonempty (FullSupports p d a₀) := by
  have hn : d ≤ Fintype.card (Choice p) := by rw [choice_card]; exact h
  obtain ⟨e⟩ := sample_nonempty hn
  exact ⟨sourceEquiv a₀ e⟩

noncomputable def sourceLaw (p d : ℕ) [Fact p.Prime] (a₀ : ZMod p) (h : d ≤ p-1) :
    PMF (FullSupports p d a₀) :=
  (sampleLaw (Choice p) d (by rw [choice_card]; exact h)).map (sourceEquiv a₀)

theorem actual_uniform_source_law (a₀ : ZMod p) (h : d ≤ p-1) :
    sourceLaw p d a₀ h = @uniformLaw (FullSupports p d a₀) inferInstance (full_support_nonempty a₀ h) := by
  have hn : d ≤ Fintype.card (Choice p) := by rw [choice_card]; exact h
  have he := @uniform_map_equiv (Sample (Choice p) d) (FullSupports p d a₀)
    inferInstance (sample_nonempty hn) inferInstance (full_support_nonempty a₀ h) (sourceEquiv a₀)
  simpa only [sourceLaw,sampleLaw] using he

theorem full_source_argument (a₀ j : ZMod p) (e : Sample (Choice p) d) (i : Fin d) :
    supportsFromSample a₀ e 0=a₀ ∧
      (a₀-supportsFromSample a₀ e i.succ)*j = -(e i).val*j := by
  refine ⟨rfl,?_⟩
  change (a₀-(a₀+(e i).val))*j = -(e i).val*j
  ring

end InjectiveBoxAudit
