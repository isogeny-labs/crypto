import Init

/-! Same quantified Int/Nat statements as A. Existing standard Int modular
laws and explicit three-residue proof replace the unavailable Omega path.
No actual End(J), curve or monodromy bridge is assumed or formalized here. -/
namespace ChainedCM19.Geometric4

def coeff : Nat → Int × Int
  | 0 => (1, 0)
  | n + 1 => let c := coeff n; (-7 * c.2, c.1 - 3 * c.2)

theorem coeffResidue (n : Nat) :
    (coeff (n + 1)).1 % 7 = 0 ∧
    ((coeff (n + 1)).2 % 7 = 1 ∨
      (coeff (n + 1)).2 % 7 = 2 ∨
      (coeff (n + 1)).2 % 7 = 4) := by
  induction n with
  | zero => decide
  | succ n ih =>
    change (-7 * (coeff (n + 1)).2) % 7 = 0 ∧
      (((coeff (n + 1)).1 - 3 * (coeff (n + 1)).2) % 7 = 1 ∨
        ((coeff (n + 1)).1 - 3 * (coeff (n + 1)).2) % 7 = 2 ∨
        ((coeff (n + 1)).1 - 3 * (coeff (n + 1)).2) % 7 = 4)
    constructor
    · rw [Int.mul_emod]
      change (0 * ((coeff (n + 1)).2 % 7)) % 7 = 0
      rw [Int.zero_mul, Int.zero_emod]
    · cases ih.2 with
      | inl h =>
        apply Or.inr
        apply Or.inr
        rw [Int.sub_emod, Int.mul_emod, ih.1, h]
        decide
      | inr h =>
        cases h with
        | inl h =>
          apply Or.inl
          rw [Int.sub_emod, Int.mul_emod, ih.1, h]
          decide
        | inr h =>
          apply Or.inr
          apply Or.inl
          rw [Int.sub_emod, Int.mul_emod, ih.1, h]
          decide

theorem coefficientNonzero (n : Nat) : (coeff (n + 1)).2 ≠ 0 := by
  intro hz
  have h := (coeffResidue n).2
  rw [hz] at h
  exact (by decide : ¬ ((0 : Int) % 7 = 1 ∨
    (0 : Int) % 7 = 2 ∨ (0 : Int) % 7 = 4)) h

#print axioms coeffResidue
#print axioms coefficientNonzero

end ChainedCM19.Geometric4
