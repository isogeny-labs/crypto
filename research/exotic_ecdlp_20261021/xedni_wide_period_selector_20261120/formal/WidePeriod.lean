import Mathlib.Tactic

namespace XedniWidePeriod

/-- The two local residues identify at most one coefficient in the exact
`1..41` domain. This is the arithmetic CRT bridge used by the selector. -/
theorem coefficient_unique (k₁ k₂ : ℕ)
    (h₁ : 1 ≤ k₁) (h₁' : k₁ ≤ 41)
    (h₂ : 1 ≤ k₂) (h₂' : k₂ ≤ 41)
    (h₅ : k₁ % 5 = k₂ % 5) (h₁₁ : k₁ % 11 = k₂ % 11) :
    k₁ = k₂ := by
  omega

/-- Every residue modulo 55 has at most one representative in `1..41`. -/
theorem representative_unique (k₁ k₂ : ℕ)
    (h₁ : 1 ≤ k₁) (h₁' : k₁ ≤ 41)
    (h₂ : 1 ≤ k₂) (h₂' : k₂ ≤ 41)
    (h : k₁ % 55 = k₂ % 55) : k₁ = k₂ := by
  omega

#print axioms coefficient_unique
#print axioms representative_unique

end XedniWidePeriod
