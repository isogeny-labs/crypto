module
public import TruncatedGrading

@[expose] public section
namespace HPSFirstFallAudit
noncomputable section
open Polynomial
variable (K : Type*) [Field K]

/-- Actual degree-k annihilators of λ=x^d. -/
def annihilators (q d k : ℕ) : Submodule K (Truncated K q) :=
  piece K q k ⊓ LinearMap.ker (LinearMap.mulRight K (x K q ^ d))

/-- Source §4's trivial annihilators λ^(q-1) B_(k-d(q-1)); negative pieces are zero. -/
def trivialAnnihilators (q d k : ℕ) : Submodule K (Truncated K q) :=
  if d * (q - 1) ≤ k then
    (piece K q (k - d * (q - 1))).map
      (LinearMap.mulRight K ((x K q ^ d) ^ (q - 1)))
  else ⊥

def nontrivialFall (q d k : ℕ) : Prop :=
  ∃ a : Truncated K q, a ∈ annihilators K q d k ∧ a ∉ trivialAnnihilators K q d k

/-- The minimum total relation degree, with the source's degree shift d. -/
def firstFallDegree (q d : ℕ) : ℕ :=
  sInf {t : ℕ | d ≤ t ∧ nontrivialFall K q d (t - d)}

theorem trivial_zero (q d k : ℕ) (h : q ≤ d * (q - 1)) :
    trivialAnnihilators K q d k = ⊥ := by
  have hz : (x K q ^ d) ^ (q - 1) = 0 := by
    rw [← pow_mul]
    exact x_pow_eq_zero K q _ h
  simp [trivialAnnihilators, hz]

theorem annihilators_below (q d k : ℕ) (hk : k + d < q) :
    annihilators K q d k = ⊥ := by
  apply le_antisymm ?_ bot_le
  intro a ha
  change a = 0
  obtain ⟨c, rfl⟩ := (mem_piece K q k a).mp ha.1
  have hmul := ha.2
  change (c • x K q ^ k) * x K q ^ d = 0 at hmul
  have hz : c • (x K q ^ (k + d)) = 0 := by
    simpa only [smul_mul_assoc, pow_add] using hmul
  have hc : c = 0 := (smul_eq_zero.mp hz).resolve_right (x_pow_ne_zero K q _ hk)
  simp [hc]

theorem fall_at_boundary (q d : ℕ) (hd : 0 < d) (hdq : d < q)
    (ht : q ≤ d * (q - 1)) : nontrivialFall K q d (q - d) := by
  refine ⟨x K q ^ (q - d), ⟨?_, ?_⟩, ?_⟩
  · exact Submodule.mem_span_singleton_self _
  · change x K q ^ (q - d) * x K q ^ d = 0
    rw [← pow_add, Nat.sub_add_cancel hdq.le, x_pow_q]
  · rw [trivial_zero K q d _ ht, Submodule.mem_bot]
    exact x_pow_ne_zero K q _ (by omega)

theorem no_fall_below (q d k : ℕ) (hk : k + d < q) : ¬nontrivialFall K q d k := by
  rintro ⟨a, ha, hn⟩
  have hz : a = 0 := by simpa [annihilators_below K q d k hk] using ha
  exact hn (hz ▸ (trivialAnnihilators K q d k).zero_mem)

/-- Exact first fall for every allowed monomial, derived from actual multiplication. -/
theorem monomial_first_fall (q d : ℕ) (hd : 0 < d) (hdq : d < q)
    (ht : q ≤ d * (q - 1)) : firstFallDegree K q d = q := by
  let S := {t : ℕ | d ≤ t ∧ nontrivialFall K q d (t - d)}
  have hq : q ∈ S := ⟨hdq.le, fall_at_boundary K q d hd hdq ht⟩
  change sInf S = q
  apply (show IsLeast S q from ⟨hq, ?_⟩).csInf_eq
  intro t ht'
  by_contra h
  have hlt : t < q := by omega
  exact no_fall_below K q d (t - d) (by omega) ht'.2

#print axioms trivial_zero
#print axioms annihilators_below
#print axioms fall_at_boundary
#print axioms no_fall_below
#print axioms monomial_first_fall
end
end HPSFirstFallAudit
