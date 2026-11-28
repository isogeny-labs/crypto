import Mathlib.Tactic

/-!
# Soundness and completeness of a finite point-decomposition SAT schema

This file formalizes the semantic clause families of a one-hot SAT encoding.
For every slot there is a positive at-least-one clause and the usual pairwise
at-most-one clauses. For every invalid tuple there is a blocking clause
`∨ i, ¬X(i, tuple i)`.

The main theorem `satisfiable_iff_valid` proves that this schema is satisfiable
exactly when the underlying finite-domain problem has a valid tuple.
`decomposition_satisfiable_iff` instantiates the checker with equality between
a finite sum of abstract points and a target. Signed factor-base points can be
represented directly by the finite candidate type.
`decomposition_satisfiable_iff_direct_subtraction` then splits one head
candidate from that sum and expresses the remaining tuple against the
subtracted target.

This is a semantic encoding theorem. It does not claim correctness of a
particular CNF/Tseitin/XOR compiler or SAT solver.
-/

open scoped BigOperators

namespace PointDecompositionSAT

universe u v

variable {Choice : Type u}

/-- A fixed-arity finite-domain decision problem with a Boolean checker. -/
structure Problem (Choice : Type u) where
  arity : Nat
  check : (Fin arity → Choice) → Bool

/-- A tuple is valid exactly when the problem's checker accepts it. -/
def Valid (problem : Problem Choice) (xs : Fin problem.arity → Choice) : Prop :=
  problem.check xs = true

/-- One Boolean variable for each slot/candidate pair. -/
abbrev Assignment (problem : Problem Choice) :=
  Fin problem.arity → Choice → Bool

/-- The positive clause for each slot: some candidate is selected. -/
def AtLeastOne (problem : Problem Choice) (σ : Assignment problem) : Prop :=
  ∀ i, ∃ p, σ i p = true

/-- The pairwise negative clauses: no slot selects two different candidates. -/
def AtMostOne (problem : Problem Choice) (σ : Assignment problem) : Prop :=
  ∀ i p q, σ i p = true → σ i q = true → p = q

/-- The blocking clause for every invalid tuple has at least one false literal. -/
def RejectsInvalid (problem : Problem Choice) (σ : Assignment problem) : Prop :=
  ∀ ys, ¬Valid problem ys → ∃ i, σ i (ys i) = false

/-- Semantic satisfaction of all three clause families. -/
def Satisfies (problem : Problem Choice) (σ : Assignment problem) : Prop :=
  AtLeastOne problem σ ∧ AtMostOne problem σ ∧ RejectsInvalid problem σ

/-- The semantic SAT schema has a model. -/
def Satisfiable (problem : Problem Choice) : Prop :=
  ∃ σ, Satisfies problem σ

/-- The canonical one-hot assignment for a tuple. -/
def encode [DecidableEq Choice] (problem : Problem Choice)
    (xs : Fin problem.arity → Choice) : Assignment problem :=
  fun i p => decide (p = xs i)

/-- **Soundness.** Every satisfying assignment decodes to a valid tuple, and
the selected variable in each slot is exactly that tuple's candidate. -/
theorem sound [Fintype Choice] [DecidableEq Choice]
    (problem : Problem Choice) {σ : Assignment problem}
    (h : Satisfies problem σ) :
    ∃ xs, Valid problem xs ∧ ∀ i p, σ i p = true ↔ p = xs i := by
  classical
  let xs : Fin problem.arity → Choice := fun i => Classical.choose (h.1 i)
  have hxs (i : Fin problem.arity) : σ i (xs i) = true :=
    Classical.choose_spec (h.1 i)
  have hValid : Valid problem xs := by
    by_contra hInvalid
    obtain ⟨i, hi⟩ := h.2.2 xs hInvalid
    have hFalse : true = false := (hxs i).symm.trans hi
    exact Bool.noConfusion hFalse
  refine ⟨xs, hValid, ?_⟩
  intro i p
  constructor
  · intro hp
    exact h.2.1 i p (xs i) hp (hxs i)
  · intro hp
    subst p
    exact hxs i

/-- **Completeness.** A valid tuple's canonical one-hot assignment satisfies
every positive, pairwise-negative and invalid-tuple blocking clause. -/
theorem complete [Fintype Choice] [DecidableEq Choice]
    (problem : Problem Choice) (xs : Fin problem.arity → Choice)
    (hValid : Valid problem xs) : Satisfies problem (encode problem xs) := by
  refine ⟨?_, ?_, ?_⟩
  · intro i
    exact ⟨xs i, by simp [encode]⟩
  · intro i p q hp hq
    simp only [encode, decide_eq_true_eq] at hp hq
    exact hp.trans hq.symm
  · intro ys hInvalid
    have hne : ys ≠ xs := by
      intro hEq
      apply hInvalid
      simpa [hEq] using hValid
    have hpoint : ∃ i, ys i ≠ xs i := by
      by_contra hNo
      apply hne
      funext i
      by_contra hi
      exact hNo ⟨i, hi⟩
    obtain ⟨i, hi⟩ := hpoint
    exact ⟨i, by simp [encode, hi]⟩

/-- **Exactness.** The one-hot blocking-clause schema is satisfiable if and
only if the underlying finite-domain problem has a valid tuple. -/
theorem satisfiable_iff_valid [Fintype Choice] [DecidableEq Choice]
    (problem : Problem Choice) :
    Satisfiable problem ↔ ∃ xs, Valid problem xs := by
  constructor
  · rintro ⟨σ, hσ⟩
    obtain ⟨xs, hValid, _⟩ := sound problem hσ
    exact ⟨xs, hValid⟩
  · rintro ⟨xs, hValid⟩
    exact ⟨encode problem xs, complete problem xs hValid⟩

section Additive

variable {G : Type v} [AddCommMonoid G] [DecidableEq G]

/-- The abstract point-decomposition instance. Each candidate maps to an
additive point, and a tuple is accepted when its sum is the target. -/
def decompositionProblem (arity : Nat) (base : Choice → G) (target : G) :
    Problem Choice where
  arity := arity
  check := fun xs => decide ((∑ i, base (xs i)) = target)

/-- A finite point-decomposition SAT instance is satisfiable exactly when a
tuple of candidate points sums to the target. -/
theorem decomposition_satisfiable_iff [Fintype Choice] [DecidableEq Choice]
    (arity : Nat) (base : Choice → G) (target : G) :
    Satisfiable (decompositionProblem arity base target) ↔
      ∃ xs : Fin arity → Choice, (∑ i, base (xs i)) = target := by
  rw [satisfiable_iff_valid]
  simp [Valid, decompositionProblem]

end Additive

section DirectSubtraction

variable {G : Type v} [AddCommGroup G] [DecidableEq G]

/-- The semantic direct-subtraction contract: choose one head candidate, then
decompose the target minus that head into `remaining` candidates. -/
def DirectSubtractionSpec (remaining : Nat) (base : Choice → G) (target : G) : Prop :=
  ∃ head : Choice, ∃ tail : Fin remaining → Choice,
    (∑ i, base (tail i)) = target - base head

/-- Splitting off one candidate preserves the exact SAT/decomposition
semantics. This is the abstract refinement used by a direct-subtraction
oracle; it does not describe or verify any particular implementation. -/
theorem decomposition_satisfiable_iff_direct_subtraction
    [Fintype Choice] [DecidableEq Choice]
    (remaining : Nat) (base : Choice → G) (target : G) :
    Satisfiable (decompositionProblem (remaining + 1) base target) ↔
      DirectSubtractionSpec remaining base target := by
  rw [decomposition_satisfiable_iff]
  constructor
  · rintro ⟨xs, hxs⟩
    refine ⟨xs 0, fun i => xs i.succ, ?_⟩
    rw [eq_sub_iff_add_eq]
    simpa [Fin.sum_univ_succ, add_comm] using hxs
  · rintro ⟨head, tail, htail⟩
    refine ⟨Fin.cases head tail, ?_⟩
    rw [Fin.sum_univ_succ]
    simpa [add_comm] using (eq_sub_iff_add_eq.mp htail)

/-- For two summands, direct subtraction reduces the remaining decomposition
to one exact factor-base lookup. -/
theorem decomposition_two_satisfiable_iff_direct_lookup
    [Fintype Choice] [DecidableEq Choice]
    (base : Choice → G) (target : G) :
    Satisfiable (decompositionProblem 2 base target) ↔
      ∃ head tail : Choice, base tail = target - base head := by
  rw [decomposition_satisfiable_iff_direct_subtraction (remaining := 1)]
  constructor
  · rintro ⟨head, tail, htail⟩
    exact ⟨head, tail 0, by simpa [Fin.sum_univ_succ] using htail⟩
  · rintro ⟨head, tail, htail⟩
    refine ⟨head, fun _ => tail, ?_⟩
    simpa [Fin.sum_univ_succ] using htail

end DirectSubtraction

end PointDecompositionSAT
