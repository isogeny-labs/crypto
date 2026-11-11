/- Pure logical part of the shifted-coordinate extractor. The elliptic curve
   fiber identity and nonrationality of the second lift are explicit inputs.
   This does not formalize the trace, local coefficient, or field binding. -/
namespace CyclicTrace

theorem unique_rational_fiber {A B : Type} (rational : A → Prop)
    (coordinate : A → B) (other : A → A)
    (fiber : ∀ a b, coordinate a = coordinate b → a = b ∨ a = other b)
    (other_not_rational : ∀ b, rational b → ¬ rational (other b))
    (a b : A) (ha : rational a) (hb : rational b)
    (equal_coordinate : coordinate a = coordinate b) : a = b := by
  cases fiber a b equal_coordinate with
  | inl h => exact h
  | inr h =>
    have hbad : rational (other b) := h ▸ ha
    exact False.elim (other_not_rational b hb hbad)

#print axioms unique_rational_fiber
end CyclicTrace
