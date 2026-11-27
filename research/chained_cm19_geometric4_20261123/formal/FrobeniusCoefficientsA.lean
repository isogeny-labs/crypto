import Lean.Elab.Tactic.Omega

/-! A quantified arithmetic dependency for the actual Frobenius-power
argument. This module does not define or assume an abelian variety,
endomorphism ring, Jacobian, curve, Galois closure or monodromy group.
Those actual-object bridges remain separate written proof obligations. -/
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
    omega

theorem coefficientNonzero (n : Nat) : (coeff (n + 1)).2 ≠ 0 := by
  have h := (coeffResidue n).2
  omega

#print axioms coeffResidue
#print axioms coefficientNonzero

end ChainedCM19.Geometric4
