/-!
# Keyed-generic invariance

`ic_framework::residual` (the residual collector) and the pair-table oracles
are not generic in Nechaev's sense: they read a *key* of each element (the
packed point, `CountedGroup::key`) and choose their next step from it. The
theorem of `research/conductor_gap_cm_pairs_20261010/lean/GenericInvariance.lean`
therefore does not cover them; that file's own scope note says so.

This file proves the statement that does cover them. An algorithm sees elements
only through handles. It may ask for `add`, `neg`, an equality test of two
handles, or the key of a handle (a `Nat`), and it chooses its next query from
everything it has been told. Then its whole transcript (every answer and every
key it read, its operation count and its result) is identical on two carriers
joined by an injective map that preserves `add`, `neg` and the key.

**Corollary for the collector.** Let `G` generate a prime-order subgroup of
`E`, and let `φ : Z/r → E`, `k ↦ [k]G`. On `Z/r` take the key `key_E ∘ φ`. Then
`φ` is an injective homomorphism preserving keys by construction, so the
collector run on `E` equals the collector run on `Z/r` with that transported
key. It uses no property of the curve except through its key function.

This says nothing about a *random* key on `Z/r`. Whether `E`'s key function
behaves like a random one is what the empirical curve-free null measures
(`examples/ic_residual_curve_free_null.rs`). The claim that the Rust collector
uses only these four operations is checked by reading its code, not here.
Lean core only; no axioms beyond the standard three.
-/

namespace KeyedInvariance

/-- What the carrier offers: the group law used, and a key per element. -/
structure Carrier (α : Type) where
  add : α → α → α
  neg : α → α
  key : α → Nat

inductive Query where
  | add (i j : Nat)
  | neg (i : Nat)
  | eq (i j : Nat)
  | key (i : Nat)
  | halt (answer : Nat)

/-- An observation returned to the algorithm. -/
inductive Obs where
  | bool (b : Bool)
  | nat (k : Nat)
  deriving DecidableEq

/-- The algorithm chooses its next query from the operation count, every
observation so far and a random tape; it is never given an element. -/
structure Alg where
  next : (cost : Nat) → (seen : List Obs) → (tape : Nat → Bool) → Query

structure State (α : Type) where
  handles : List α
  seen : List Obs
  cost : Nat

def step {α : Type} [DecidableEq α] (C : Carrier α) (A : Alg) (tape : Nat → Bool)
    (s : State α) : State α ⊕ (Nat × Nat) :=
  match A.next s.cost s.seen tape with
  | .add i j =>
    match s.handles[i]?, s.handles[j]? with
    | some a, some b => .inl { s with handles := s.handles ++ [C.add a b], cost := s.cost + 1 }
    | _, _ => .inl { s with cost := s.cost + 1 }
  | .neg i =>
    match s.handles[i]? with
    | some a => .inl { s with handles := s.handles ++ [C.neg a] }
    | none => .inl s
  | .eq i j =>
    match s.handles[i]?, s.handles[j]? with
    | some a, some b => .inl { s with seen := s.seen ++ [.bool (decide (a = b))] }
    | _, _ => .inl { s with seen := s.seen ++ [.bool false] }
  | .key i =>
    match s.handles[i]? with
    | some a => .inl { s with seen := s.seen ++ [.nat (C.key a)] }
    | none => .inl { s with seen := s.seen ++ [.bool false] }
  | .halt ans => .inr (s.cost, ans)

def run {α : Type} [DecidableEq α] (C : Carrier α) (A : Alg) (tape : Nat → Bool) :
    Nat → State α → Option (Nat × Nat)
  | 0, _ => none
  | n + 1, s =>
    match step C A tape s with
    | .inl s' => run C A tape n s'
    | .inr r => some r

/-- An injective map preserving `add`, `neg` and the key. -/
structure Hom {α β : Type} (CG : Carrier α) (CH : Carrier β) where
  toFun : α → β
  inj : ∀ a b, toFun a = toFun b → a = b
  map_add : ∀ a b, toFun (CG.add a b) = CH.add (toFun a) (toFun b)
  map_neg : ∀ a, toFun (CG.neg a) = CH.neg (toFun a)
  map_key : ∀ a, CH.key (toFun a) = CG.key a

variable {α β : Type} [DecidableEq α] [DecidableEq β] {CG : Carrier α} {CH : Carrier β}

def State.map (φ : Hom CG CH) (s : State α) : State β :=
  ⟨s.handles.map φ.toFun, s.seen, s.cost⟩

theorem decide_eq_map (φ : Hom CG CH) (a b : α) :
    decide (φ.toFun a = φ.toFun b) = decide (a = b) := by
  by_cases h : a = b
  · subst h; simp
  · have h' : φ.toFun a ≠ φ.toFun b := fun e => h (φ.inj a b e)
    simp [h, h']

/-- One step commutes with transport. -/
theorem step_map (φ : Hom CG CH) (A : Alg) (tape : Nat → Bool) (s : State α) :
    step CH A tape (s.map φ) = Sum.map (State.map φ) id (step CG A tape s) := by
  unfold step
  simp only [State.map]
  cases A.next s.cost s.seen tape with
  | add i j =>
    simp only [List.getElem?_map]
    cases s.handles[i]? <;> cases s.handles[j]? <;> simp [Sum.map, State.map, φ.map_add]
  | neg i =>
    simp only [List.getElem?_map]
    cases s.handles[i]? <;> simp [Sum.map, State.map, φ.map_neg]
  | eq i j =>
    simp only [List.getElem?_map]
    cases s.handles[i]? <;> cases s.handles[j]? <;> simp [Sum.map, State.map, decide_eq_map φ]
  | key i =>
    simp only [List.getElem?_map]
    cases s.handles[i]? <;> simp [Sum.map, State.map, φ.map_key]
  | halt ans => simp [Sum.map]

/-- **Keyed invariance.** For every algorithm, tape, step bound and start
state, the transported run returns the same `(cost, answer)`, or fails to
halt in the same way. -/
theorem run_map (φ : Hom CG CH) (A : Alg) (tape : Nat → Bool) :
    ∀ (n : Nat) (s : State α), run CH A tape n (s.map φ) = run CG A tape n s := by
  intro n
  induction n with
  | zero => intro s; rfl
  | succ n ih =>
    intro s
    simp only [run, step_map φ]
    cases step CG A tape s with
    | inl s' => simp [Sum.map, ih]
    | inr r => simp [Sum.map]

/-- `n` steps, returning the state reached or the halt result. -/
def exec {α : Type} [DecidableEq α] (C : Carrier α) (A : Alg) (tape : Nat → Bool) :
    Nat → State α → State α ⊕ (Nat × Nat)
  | 0, s => .inl s
  | n + 1, s =>
    match step C A tape s with
    | .inl s' => exec C A tape n s'
    | .inr r => .inr r

/-- **Keyed invariance, whole transcript.** After any number of steps the
transported execution is the transport of the original: the observation list
`seen` (every equality answer and every key read) and the operation count are
*equal*, and the handles correspond under `φ`; a halt returns the same
`(cost, answer)`. -/
theorem exec_map (φ : Hom CG CH) (A : Alg) (tape : Nat → Bool) :
    ∀ (n : Nat) (s : State α),
      exec CH A tape n (s.map φ) = Sum.map (State.map φ) id (exec CG A tape n s) := by
  intro n
  induction n with
  | zero => intro s; rfl
  | succ n ih =>
    intro s
    simp only [exec, step_map φ]
    cases step CG A tape s with
    | inl s' => simp [Sum.map, ih]
    | inr r => simp [Sum.map]

theorem exec_map_seen_cost (φ : Hom CG CH) (A : Alg) (tape : Nat → Bool) (n : Nat)
    (s t : State α) (h : exec CG A tape n s = .inl t) :
    ∃ t', exec CH A tape n (s.map φ) = .inl t' ∧ t'.seen = t.seen ∧ t'.cost = t.cost ∧
      t'.handles = t.handles.map φ.toFun := by
  refine ⟨t.map φ, ?_, rfl, rfl, rfl⟩
  rw [exec_map φ A tape n s, h]; rfl

/-- A carrier on the domain of an injective, operation-preserving map, with
the key transported back along it. -/
def pullback (CH : Carrier β) (add : α → α → α) (neg : α → α) : (α → β) → Carrier α :=
  fun f => ⟨add, neg, fun a => CH.key (f a)⟩

/-- **The collector corollary.** If `f : α → β` is injective and preserves
`add` and `neg` (for `α = Z/r`, `β = E`, `f k = [k]G`), then every keyed
algorithm run on `β` from transported handles is the run on `α` with the key
`key_β ∘ f`. -/
theorem run_transported_key (CH : Carrier β) (add : α → α → α) (neg : α → α) (f : α → β)
    (inj : ∀ a b, f a = f b → a = b)
    (hadd : ∀ a b, f (add a b) = CH.add (f a) (f b))
    (hneg : ∀ a, f (neg a) = CH.neg (f a))
    (A : Alg) (tape : Nat → Bool) (n : Nat) (s : State α) :
    run CH A tape n (s.map (CG := pullback CH add neg f)
      ⟨f, inj, hadd, hneg, fun _ => rfl⟩) = run (pullback CH add neg f) A tape n s :=
  run_map _ A tape n s

/-! ## Non-vacuity: a concrete keyed run on `Z/7`

The hypotheses are satisfiable and the model computes: on `Z/7` with the key
`k ↦ 3k mod 7`, an algorithm that adds handle 0 to itself twice and reads the
key of the result halts with that key as its answer, at cost 2. -/

def z7 : Carrier (Fin 7) := ⟨(· + ·), (- ·), fun a => (3 * a.val) % 7⟩

def doubleTwiceThenKey : Alg where
  next cost seen _ :=
    match cost, seen with
    | 0, _ => .add 0 0
    | 1, _ => .add 1 1
    | _, [] => .key 2
    | _, .nat k :: _ => .halt k
    | _, _ => .halt 0

example : run z7 doubleTwiceThenKey (fun _ => false) 10 ⟨[(1 : Fin 7)], [], 0⟩ = some (2, 5) := by
  decide

end KeyedInvariance
