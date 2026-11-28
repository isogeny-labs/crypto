import Mathlib.Tactic

namespace XedniCandidateList

def residue (b : ℕ) : ℤ := (744 * (b : ℤ)^2) % 1009

def candidateList (b : ℕ) : List ℤ :=
  [residue b - 5045, residue b - 4036, residue b - 3027,
   residue b - 2018, residue b - 1009, residue b,
   residue b + 1009, residue b + 2018, residue b + 3027,
   residue b + 4036]

theorem x_candidate_mem (a : ℤ) (b : ℕ)
    (ha₁ : -4096 ≤ a) (ha₂ : a ≤ 4096)
    (hx : (a - 744 * (b : ℤ)^2) % 1009 = 0) :
    a ∈ candidateList b := by
  let r := residue b
  have hr0 : 0 ≤ r := Int.emod_nonneg _ (by omega)
  have hr1 : r < 1009 := Int.emod_lt_of_pos _ (by omega)
  have hmod : (a - r) % 1009 = 0 := by dsimp [r, residue]; omega
  have hcases : a = r - 5045 ∨ a = r - 4036 ∨ a = r - 3027 ∨
      a = r - 2018 ∨ a = r - 1009 ∨ a = r ∨ a = r + 1009 ∨
      a = r + 2018 ∨ a = r + 3027 ∨ a = r + 4036 := by omega
  simpa [candidateList, r] using hcases

#print axioms x_candidate_mem

end XedniCandidateList
