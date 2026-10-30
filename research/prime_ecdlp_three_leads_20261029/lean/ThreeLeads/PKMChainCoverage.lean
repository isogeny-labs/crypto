import Mathlib.Algebra.Group.Basic

/-! Structural chain-order control only. Applying this to Rust's curve points
still needs the ECC/group bridge. Distinct abscissae imply the two hypotheses
on actual elliptic-curve points; no polynomial or solving-cost bound is proved. -/
namespace PKMChainCoverage

variable {G : Type*} [AddCommGroup G]

/-- Keep the first pair and exchange the last two points if needed. One order
has both intermediate sums nonzero and the same final sum. The hypotheses are
explicit rather than inferred from unformalized Rust coordinates. -/
theorem finite_chain_order (a b c d : G) (hab : a + b ≠ 0) (hcd : c ≠ d) :
    (a + b ≠ 0 ∧ a + b + c ≠ 0) ∨
    (a + b ≠ 0 ∧ a + b + d ≠ 0 ∧ a + b + d + c = a + b + c + d) := by
  by_cases hc : a + b + c = 0
  · right
    refine ⟨hab, ?_, ?_⟩
    · intro hd
      exact hcd (add_left_cancel (hc.trans hd.symm))
    · ac_rfl
  · exact Or.inl ⟨hab, hc⟩

#print axioms finite_chain_order
end PKMChainCoverage
