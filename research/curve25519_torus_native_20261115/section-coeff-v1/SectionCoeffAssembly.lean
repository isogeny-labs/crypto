module
public import Mathlib.Algebra.Squarefree.Basic
public import Mathlib.Algebra.Polynomial.AlgebraMap
public import Mathlib.Data.ZMod.Basic
public import Mathlib.Tactic.NormNum

public section
namespace SectionCoeffAssembly

-- These endpoints inherit existing polynomial laws. The trace, norm and native
-- coefficient bindings remain separate, unformalized obligations.
/-- DERIVATION.md split-support section: inheritance only; E squarefree is assumed. -/
theorem actual_squarefree_of_divisibility
    (S E : Polynomial (ZMod (2 ^ 255 - 19)))
    (hE : Squarefree E) (hdiv : S ∣ E) : Squarefree S :=
  hE.squarefree_of_dvd hdiv

/-- DERIVATION.md norm section: excludes a squared linear divisor of squarefree S. -/
theorem actual_no_double_linear_factor
    (S : Polynomial (ZMod (2 ^ 255 - 19)))
    (hS : Squarefree S) (x : ZMod (2 ^ 255 - 19)) :
    ¬ (Polynomial.X - Polynomial.C x) ^ 2 ∣ S := by
  let : Fact (1 < (2 ^ 255 - 19 : ℕ)) := ⟨by norm_num⟩
  intro h
  exact Polynomial.not_isUnit_X_sub_C x
    (hS _ (by simpa only [pow_two] using h))

#print axioms actual_squarefree_of_divisibility
#print axioms actual_no_double_linear_factor
end SectionCoeffAssembly
