import Mathlib

/-! Layer 2c: the adaptive generic-algorithm decision tree.

A generic algorithm is a decision tree: at each node it tests one affine form
`a + b·x` of the secret and branches on whether it vanishes.  Leaves carry an
output.  This file proves the deterministic core of the adaptive bound:

* a positive branch of a nontrivial test (`b ≠ 0`) fixes at most one secret;
* a trivial test (`b = 0`) has only one live branch;
* hence the tree is correct on at most `1 +` the number of nontrivial tests,
  and in particular on at most `2^depth` secrets.

The remaining step to Shoup's `O(t²/p)` bound is probabilistic: over a uniform
secret, a pair of adaptive queries collides with probability at most `1/p`, and
a union bound over the `t(t-1)/2` query pairs gives the `t²/p` shape.  That
requires a probability space and an adversary model for the query set and is
recorded as an explicit premise in the ledger, not as a kernel theorem. -/

namespace ECDLPAdaptive

variable {p : ℕ} [Fact p.Prime]

/-- A depth-first binary decision tree.  A node `node a b t f` tests
`a + b·x = 0`; `t` is the positive branch, `f` the negative one. -/
inductive Tree (p : ℕ) where
  | leaf : ZMod p → Tree p
  | node : ZMod p → ZMod p → Tree p → Tree p → Tree p

namespace Tree

/-- Evaluate the tree on a secret. -/
def eval : Tree p → ZMod p → ZMod p
  | leaf y, _ => y
  | node a b t f, x => if a + b * x = 0 then eval t x else eval f x

/-- Number of internal nodes. -/
def nodeCount : Tree p → ℕ
  | leaf _ => 0
  | node _ _ t f => 1 + nodeCount t + nodeCount f

/-- Number of nodes whose test is nontrivial (`b ≠ 0`). -/
def ntCount : Tree p → ℕ
  | leaf _ => 0
  | node _ b t f => (if b = 0 then 0 else 1) + ntCount t + ntCount f

/-- Depth of the tree. -/
def depth : Tree p → ℕ
  | leaf _ => 0
  | node _ _ t f => 1 + max (depth t) (depth f)

/-- The secrets on which the tree's output is correct. -/
def correct (T : Tree p) : Finset (ZMod p) :=
  Finset.univ.filter (fun x => eval T x = x)

/-- A nontrivial affine test has at most one solution. -/
theorem solution_card_le_one {a b : ZMod p} (hb : b ≠ 0) :
    (Finset.univ.filter (fun x : ZMod p => a + b * x = 0)).card ≤ 1 := by
  rw [Finset.card_le_one]
  intro x hx y hy
  simp only [Finset.mem_filter, Finset.mem_univ, true_and] at hx hy
  have h : b * x = b * y := by linear_combination hx - hy
  exact mul_left_cancel₀ hb h

/-- **Adaptive decision-tree bound.**  A tree with `k` nontrivial tests is
correct on at most `k + 1` secrets: one for the all-negative path, one per
positive branch of a nontrivial test. -/
theorem correct_card_le_nt (T : Tree p) : (correct T).card ≤ ntCount T + 1 := by
  classical
  induction T with
  | leaf y =>
      have hle : (correct (leaf y)).card ≤ 1 := by
        rw [Finset.card_le_one]
        intro x hx z hz
        have hx' : y = x := by
          have := (Finset.mem_filter.mp (show x ∈ Finset.univ.filter
            (fun x : ZMod p => eval (leaf y) x = x) from hx)).2
          simpa [eval] using this
        have hz' : y = z := by
          have := (Finset.mem_filter.mp (show z ∈ Finset.univ.filter
            (fun x : ZMod p => eval (leaf y) x = x) from hz)).2
          simpa [eval] using this
        exact hx'.symm.trans hz'
      simpa [ntCount] using hle
  | node a b t f iht ihf =>
      by_cases hb : b = 0
      · subst hb
        by_cases ha : a = 0
        · subst ha
          have heq : correct (node 0 0 t f) = correct t := by
            ext x
            simp [correct, eval]
          rw [heq]
          have : ntCount (node 0 0 t f) = ntCount t + ntCount f := by
            simp [ntCount]
          rw [this]
          omega
        · have heq : correct (node a 0 t f) = correct f := by
            ext x
            simp [correct, eval, ha]
          rw [heq]
          have : ntCount (node a 0 t f) = ntCount t + ntCount f := by
            simp [ntCount]
          rw [this]
          omega
      · have hsub : correct (node a b t f) ⊆
            Finset.univ.filter (fun x : ZMod p => a + b * x = 0) ∪ correct f := by
          intro x hx
          simp only [correct, Finset.mem_filter, Finset.mem_univ, true_and] at hx ⊢
          by_cases htest : a + b * x = 0
          · exact Finset.mem_union_left _ (by simp [htest])
          · exact Finset.mem_union_right _ (by
              simp only [Finset.mem_filter, Finset.mem_univ, true_and]
              simpa [eval, htest] using hx)
        calc (correct (node a b t f)).card
            ≤ (Finset.univ.filter (fun x : ZMod p => a + b * x = 0) ∪ correct f).card :=
              Finset.card_le_card hsub
          _ ≤ (Finset.univ.filter (fun x : ZMod p => a + b * x = 0)).card + (correct f).card :=
              Finset.card_union_le _ _
          _ ≤ 1 + (ntCount f + 1) := Nat.add_le_add (solution_card_le_one hb) ihf
          _ ≤ ntCount (node a b t f) + 1 := by
              have : ntCount (node a b t f) = 1 + ntCount t + ntCount f := by
                simp [ntCount, hb]
              omega

/-- The number of nontrivial tests is at most the number of nodes. -/
theorem ntCount_le_nodeCount (T : Tree p) : ntCount T ≤ nodeCount T := by
  induction T with
  | leaf _ => simp [ntCount, nodeCount]
  | node a b t f iht ihf =>
      have h : ntCount (node a b t f) ≤ 1 + ntCount t + ntCount f := by
        by_cases hb : b = 0 <;> simp [ntCount, hb]
      have : 1 + ntCount t + ntCount f ≤ 1 + nodeCount t + nodeCount f := by omega
      simpa [nodeCount] using le_trans h this

omit [Fact p.Prime] in
/-- A depth-`d` tree has `nodeCount + 1 ≤ 2^d` nodes including the sentinel. -/
theorem nodeCount_succ_le_pow (T : Tree p) : nodeCount T + 1 ≤ 2 ^ depth T := by
  induction T with
  | leaf _ => simp [nodeCount, depth]
  | node a b t f iht ihf =>
      have hmax : 2 ^ depth t ≤ 2 ^ max (depth t) (depth f) :=
        Nat.pow_le_pow_right (by norm_num) (Nat.le_max_left _ _)
      have hmax' : 2 ^ depth f ≤ 2 ^ max (depth t) (depth f) :=
        Nat.pow_le_pow_right (by norm_num) (Nat.le_max_right _ _)
      calc nodeCount (node a b t f) + 1
          = (nodeCount t + 1) + (nodeCount f + 1) := by
              simp [nodeCount]
              ring
        _ ≤ 2 ^ depth t + 2 ^ depth f := Nat.add_le_add iht ihf
        _ ≤ 2 ^ max (depth t) (depth f) + 2 ^ max (depth t) (depth f) :=
              Nat.add_le_add hmax hmax'
        _ = 2 * 2 ^ max (depth t) (depth f) := by ring
        _ = 2 ^ (1 + max (depth t) (depth f)) := by
              rw [Nat.add_comm 1 (max (depth t) (depth f)), pow_succ]
              ring
        _ = 2 ^ depth (node a b t f) := by simp [depth]

omit [Fact p.Prime] in
/-- A depth-`d` tree has at most `2^d - 1` internal nodes. -/
theorem nodeCount_le_depth (T : Tree p) : nodeCount T ≤ 2 ^ depth T - 1 := by
  have h := nodeCount_succ_le_pow T
  have hpos : 0 < 2 ^ depth T := pow_pos (by norm_num) _
  omega

/-- **Adaptive depth bound.**  A depth-`d` generic decision tree is correct on
at most `2^d` secrets. -/
theorem correct_card_le_depth (T : Tree p) : (correct T).card ≤ 2 ^ depth T := by
  have h1 : (correct T).card ≤ ntCount T + 1 := correct_card_le_nt T
  have h2 : ntCount T ≤ nodeCount T := ntCount_le_nodeCount T
  have h3 : nodeCount T ≤ 2 ^ depth T - 1 := nodeCount_le_depth T
  have hpos : 0 < 2 ^ depth T := pow_pos (by norm_num) _
  omega

end Tree

end ECDLPAdaptive
