module
public import TruncatedGrading

@[expose] public section
namespace HPSFirstFallAudit
noncomputable section
variable (K : Type*) [Field K]

/-- Source Definition4.6: minimum number of linear algebra generators for λ. -/
def linearGeneratorRank (q : ℕ) (a : Truncated K q) : ℕ :=
  sInf {s : ℕ | ∃ v : Fin s → Truncated K q,
    (∀ i, v i ∈ piece K q 1) ∧ a ∈ Algebra.adjoin K (Set.range v)}

theorem monomial_not_constant (q d : ℕ) (hd : 0 < d) (hdq : d < q) :
    x K q ^ d ∉ (⊥ : Subalgebra K (Truncated K q)) := by
  rintro ⟨c, hc⟩
  have h : c • x K q ^ 0 = x K q ^ d := by simpa [Algebra.smul_def] using hc
  exact distinct_powers_not_scalar K q 0 d (by omega) hdq (by omega) c h

theorem monomial_rank_one (q d : ℕ) (hd : 0 < d) (hdq : d < q) :
    linearGeneratorRank K q (x K q ^ d) = 1 := by
  let S := {s : ℕ | ∃ v : Fin s → Truncated K q,
    (∀ i, v i ∈ piece K q 1) ∧ x K q ^ d ∈ Algebra.adjoin K (Set.range v)}
  have h1 : 1 ∈ S := by
    refine ⟨fun _ => x K q, ?_, ?_⟩
    · intro i
      apply (mem_piece K q 1 (x K q)).mpr
      exact ⟨1, by simp⟩
    · exact (Algebra.adjoin K (Set.range (fun _ : Fin 1 => x K q))).pow_mem
        (Algebra.subset_adjoin ⟨0, rfl⟩) d
  change sInf S = 1
  apply (show IsLeast S 1 from ⟨h1, ?_⟩).csInf_eq
  rintro s ⟨v, hv, hm⟩
  by_contra hs
  have he : s = 0 := by omega
  subst s
  have hr : Set.range v = ∅ := Set.eq_empty_iff_forall_notMem.mpr (by
    rintro a ⟨i, _⟩
    exact Fin.elim0 i)
  rw [hr, Algebra.adjoin_empty] at hm
  exact monomial_not_constant K q d hd hdq hm

#print axioms monomial_not_constant
#print axioms monomial_rank_one
end
end HPSFirstFallAudit
