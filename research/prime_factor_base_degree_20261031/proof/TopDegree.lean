import Lean.Elab.Tactic.Omega

/- The monomial ideal (x^b, y^b, x^2*y^2). This file proves the exact
   exponent coverage theorem, not the bridge from polynomial ideals. -/
namespace DensePairTop

def Covered (b i j : Nat) : Prop := b ≤ i ∨ b ≤ j ∨ (2 ≤ i ∧ 2 ≤ j)

theorem all_above (b i j : Nat) (hd : b + 1 ≤ i + j) :
    Covered b i j := by
  unfold Covered
  omega

theorem survives_at_top (b : Nat) (hb : 2 ≤ b) :
    (b - 1) + 1 = b ∧ ¬ Covered b (b - 1) 1 := by
  unfold Covered
  omega

theorem survives_below (b d : Nat) (hb : 2 ≤ b) (hd : d ≤ b) :
    ∃ i j, i + j = d ∧ ¬ Covered b i j := by
  by_cases h : d < b
  · refine ⟨d, 0, ?_, ?_⟩
    · omega
    · unfold Covered
      omega
  · refine ⟨b - 1, 1, ?_, ?_⟩
    · omega
    · unfold Covered
      omega

theorem first_full_degree (b : Nat) (hb : 2 ≤ b) :
    (∀ i j, i + j = b + 1 → Covered b i j) ∧
    (∀ d, d < b + 1 → ∃ i j, i + j = d ∧ ¬ Covered b i j) := by
  constructor
  · intro i j hd
    apply all_above b i j
    omega
  · intro d hd
    apply survives_below b d hb
    omega

end DensePairTop

#print axioms DensePairTop.all_above
#print axioms DensePairTop.survives_at_top
#print axioms DensePairTop.survives_below
#print axioms DensePairTop.first_full_degree
