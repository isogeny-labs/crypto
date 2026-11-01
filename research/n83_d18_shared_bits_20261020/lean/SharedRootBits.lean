/- Exact factorization of selector implications for common root bits.
   Core Lean only; no external dependencies or native evaluation. -/
namespace N83SharedRootBits

def Original {I B : Type} (selected : I → Prop)
    (root : B → Bool) (representative : I → B → Bool) : Prop :=
  ∀ i, selected i → ∀ b, root b = representative i b

def Factored {I B : Type} (selected : I → Prop)
    (root : B → Bool) (representative : I → B → Bool)
    (shared : B → Option Bool) : Prop :=
  (∀ b v, shared b = some v → root b = v) ∧
  (∀ i, selected i → ∀ b, shared b = none → root b = representative i b)

/-- At least one selector is necessary. No at-most-one premise is needed.
    A reported shared bit must agree with every representative; unreported
    common bits may remain in the original implication form. -/
theorem factorization {I B : Type} (selected : I → Prop)
    (root : B → Bool) (representative : I → B → Bool)
    (shared : B → Option Bool)
    (atLeastOne : ∃ i, selected i)
    (sharedValid : ∀ b v, shared b = some v → ∀ i, representative i b = v) :
    Original selected root representative ↔
      Factored selected root representative shared := by
  constructor
  · intro original
    constructor
    · intro b v common
      obtain ⟨i, chosen⟩ := atLeastOne
      exact (original i chosen b).trans (sharedValid b v common i)
    · intro i chosen b _
      exact original i chosen b
  · intro factored i chosen b
    cases h : shared b with
    | none => exact factored.2 i chosen b h
    | some v =>
      exact (factored.1 b v h).trans (sharedValid b v h i).symm

/-- The remaining CNF/XOR/phase constraints can be arbitrary. Keeping
    their wires and truth predicate unchanged preserves all assignments. -/
theorem withinUnchangedContext {I B : Type} (context : Prop)
    (selected : I → Prop) (root : B → Bool)
    (representative : I → B → Bool) (shared : B → Option Bool)
    (atLeastOne : ∃ i, selected i)
    (sharedValid : ∀ b v, shared b = some v → ∀ i, representative i b = v) :
    (context ∧ Original selected root representative) ↔
      (context ∧ Factored selected root representative shared) := by
  constructor
  · intro h
    exact ⟨h.1, (factorization selected root representative shared
      atLeastOne sharedValid).mp h.2⟩
  · intro h
    exact ⟨h.1, (factorization selected root representative shared
      atLeastOne sharedValid).mpr h.2⟩

/-- With K = k+1 representatives, v varying bits and c common bits,
    the original root clauses equal the factored clauses plus m*c*k.
    Non-root clauses are unchanged and cancel from the difference. -/
theorem clauseCount (m k v c : Nat) :
    m * ((k + 1) * (v + c)) =
      m * ((k + 1) * v + c) + m * c * k := by
  simp [Nat.mul_add, Nat.mul_comm,
    Nat.mul_left_comm, Nat.add_assoc, Nat.add_comm, Nat.add_left_comm]

end N83SharedRootBits

#print axioms N83SharedRootBits.factorization
#print axioms N83SharedRootBits.withinUnchangedContext
#print axioms N83SharedRootBits.clauseCount
