import Init

/-! Exact group-law equivalence underlying the H4 pair-index query. -/

namespace H4PairIndex

variable {G : Type}

/-- An ordered three-summand relation is found exactly by looking up `R - c`
    among ordered pair sums. Membership may describe any selected base. -/
theorem pair_lookup_iff
    (add sub : G → G → G) (neg : G → G) (zero : G)
    (add_assoc : ∀ a b c, add (add a b) c = add a (add b c))
    (add_zero : ∀ a, add a zero = a)
    (add_neg : ∀ a, add a (neg a) = zero)
    (neg_add : ∀ a, add (neg a) a = zero)
    (sub_def : ∀ a b, sub a b = add a (neg b))
    (B : G → Prop) (R : G) :
    (∃ a b c : G, B a ∧ B b ∧ B c ∧ add (add a b) c = R) ↔
    (∃ c a b : G, B c ∧ B a ∧ B b ∧ add a b = sub R c) := by
  constructor
  · rintro ⟨a, b, c, ha, hb, hc, h⟩
    refine ⟨c, a, b, hc, ha, hb, ?_⟩
    calc
      add a b = add (add a b) zero := (add_zero (add a b)).symm
      _ = add (add a b) (add c (neg c)) := by rw [add_neg c]
      _ = add (add (add a b) c) (neg c) := (add_assoc (add a b) c (neg c)).symm
      _ = add R (neg c) := by rw [h]
      _ = sub R c := (sub_def R c).symm
  · rintro ⟨c, a, b, hc, ha, hb, h⟩
    refine ⟨a, b, c, ha, hb, hc, ?_⟩
    calc
      add (add a b) c = add (sub R c) c := by rw [h]
      _ = add (add R (neg c)) c := by rw [sub_def]
      _ = add R (add (neg c) c) := add_assoc R (neg c) c
      _ = add R zero := by rw [neg_add c]
      _ = R := add_zero R

#print axioms H4PairIndex.pair_lookup_iff

end H4PairIndex
