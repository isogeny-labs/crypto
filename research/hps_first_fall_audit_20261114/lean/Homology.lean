module
public import FirstFall
public import Mathlib.LinearAlgebra.Quotient.Basic

@[expose] public section
namespace HPSFirstFallAudit
noncomputable section
variable (K : Type*) [Field K]

/-- Actual annihilator quotient, using the source §4's single-polynomial homology. -/
abbrev sourceHomology (q d k : ℕ) :=
  annihilators K q d k ⧸
    (trivialAnnihilators K q d k).comap (annihilators K q d k).subtype

theorem homology_nontrivial_iff (q d k : ℕ) :
    Nontrivial (sourceHomology K q d k) ↔ nontrivialFall K q d k := by
  rw [Submodule.Quotient.nontrivial_iff]
  constructor
  · intro hn
    by_contra h
    apply hn
    apply le_antisymm le_top
    intro a _
    change (a : Truncated K q) ∈ trivialAnnihilators K q d k
    by_contra ha
    exact h ⟨a, a.property, ha⟩
  · rintro ⟨a, ha, hn⟩ he
    apply hn
    have hm : (⟨a, ha⟩ : annihilators K q d k) ∈
        (trivialAnnihilators K q d k).comap (annihilators K q d k).subtype := by
      rw [he]
      trivial
    exact hm

theorem homology_zero_before (q d k : ℕ) (hk : k + d < q) :
    Subsingleton (sourceHomology K q d k) := by
  rw [Submodule.Quotient.subsingleton_iff]
  apply le_antisymm le_top
  intro a _
  have ha : (a : Truncated K q) = 0 := by
    have hh := a.property
    simpa [annihilators_below K q d k hk] using hh
  change (a : Truncated K q) ∈ trivialAnnihilators K q d k
  rw [ha]
  exact (trivialAnnihilators K q d k).zero_mem

theorem homology_boundary_nonzero (q d : ℕ) (hd : 0 < d) (hdq : d < q)
    (ht : q ≤ d * (q - 1)) : Nontrivial (sourceHomology K q d (q - d)) :=
  (homology_nontrivial_iff K q d _).mpr (fall_at_boundary K q d hd hdq ht)

#print axioms homology_nontrivial_iff
#print axioms homology_zero_before
#print axioms homology_boundary_nonzero
end
end HPSFirstFallAudit
