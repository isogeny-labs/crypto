import Init
namespace FoldFiber

def allAt {A : Type} (x : A) : List (A → Prop) → Prop
  | [] => True
  | p :: ps => p x ∧ allAt x ps

def replay {A : Type} (s : A → Prop) : List (A → Prop) → A → Prop
  | [] => s
  | p :: ps => replay (fun x => s x ∧ p x) ps

-- Exactly the invariant required when changing candidate representations.
-- Arithmetic predicates and polynomial-root correctness are separate obligations.
theorem replay_exact {A : Type} (s : A → Prop) (ps : List (A → Prop)) (x : A) :
    replay s ps x ↔ s x ∧ allAt x ps := by
  induction ps generalizing s with
  | nil => simp [replay, allAt]
  | cons p ps ih =>
    simp only [replay, allAt, ih, and_assoc]

#print axioms replay_exact
end FoldFiber
