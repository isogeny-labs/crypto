import Mathlib

/-! Layer 2a: the counting core of Shoup's generic-group lower bound.

Shoup's bound says that a generic algorithm making `t` group queries has
success probability `O(t²/p)` for a uniformly random secret in a group of
prime order `p`.  The probabilistic part needs an adversary model and a
probability space.  The combinatorial part is unconditional and is proved
here:

* each pair of queried elements is a linear form `(a, b) ↦ a + b·x`;
* a pair with `bᵢ ≠ bⱼ` is consistent with exactly one secret `x`, namely
  `x = -(aᵢ - aⱼ)/(bᵢ - bⱼ)`;
* hence the set of secrets revealed by *some* colliding pair has at most one
  element per ordered pair, i.e. at most `k(k-1)` when `k` elements are known.

The factor `2` against Shoup's `C(k,2)` comes from counting ordered pairs; it
does not affect the `t²/p` shape.  Turning this counting statement into a
probability bound requires the generic-group transcript model. -/

namespace ECDLPGeneric

open Classical Finset

variable {p : ℕ} [Fact p.Prime]

/-- The unique secret compatible with a collision of the coefficient pairs
`i` and `j`, provided their second coordinates differ. -/
noncomputable def collisionSecret (i j : ZMod p × ZMod p) : ZMod p :=
  -(i.1 - j.1) / (i.2 - j.2)

/-- For `bᵢ ≠ bⱼ` the collision equation has exactly the solution
`-(aᵢ-aⱼ)/(bᵢ-bⱼ)`. -/
theorem collision_secret_unique {i j : ZMod p × ZMod p} (h : i.2 ≠ j.2) :
    {x : ZMod p | (i.1 - j.1) + (i.2 - j.2) * x = 0} = {collisionSecret i j} := by
  ext x
  simp only [Set.mem_ofPred_eq, Set.mem_singleton_iff]
  constructor
  · intro hx
    have h1 : (i.2 - j.2) * x = -(i.1 - j.1) := by linear_combination hx
    rw [collisionSecret, eq_div_iff (sub_ne_zero.mpr h), mul_comm]
    exact h1
  · intro hx
    rw [hx, collisionSecret]
    have h1 : (i.2 - j.2) * (-(i.1 - j.1) / (i.2 - j.2)) = -(i.1 - j.1) := by
      rw [mul_comm]
      exact div_mul_cancel₀ _ (sub_ne_zero.mpr h)
    linear_combination h1

/-- A secret is *revealed* by a set `S` of known coefficient pairs when the
collision of two pairs with distinct second coordinates determines it. -/
def Revealed (S : Finset (ZMod p × ZMod p)) (x : ZMod p) : Prop :=
  ∃ i ∈ S, ∃ j ∈ S, i.2 ≠ j.2 ∧ (i.1 - j.1) + (i.2 - j.2) * x = 0

/-- **Shoup's union-bound core.**  With `k` known group elements, at most
`k(k-1)` secrets are revealed by a collision.  Equivalently, a uniform secret
is revealed with probability at most `k(k-1)/p`; the probability step and the
transcript model are the remaining Layer 2 premises. -/
theorem revealed_card_le (S : Finset (ZMod p × ZMod p)) :
    (univ.filter (fun x => Revealed S x)).card ≤ S.card * (S.card - 1) := by
  classical
  -- ordered pairs of distinct elements with distinct second coordinates
  let T : Finset ((ZMod p × ZMod p) × (ZMod p × ZMod p)) :=
    S.offDiag.filter (fun q => q.1.2 ≠ q.2.2)
  have hcover : (univ.filter (fun x => Revealed S x)) ⊆
      T.biUnion (fun q => ({collisionSecret q.1 q.2} : Finset (ZMod p))) := by
    intro x hx
    simp only [mem_filter, mem_univ, true_and] at hx
    unfold Revealed at hx
    obtain ⟨i, hiS, j, hjS, hb, heq⟩ := hx
    have hne : i ≠ j := by
      rintro rfl
      exact hb rfl
    have hmem : (i, j) ∈ T := by
      simp only [T, mem_filter, mem_offDiag]
      exact ⟨⟨hiS, hjS, hne⟩, hb⟩
    rw [mem_biUnion]
    refine ⟨(i, j), hmem, ?_⟩
    have hx' : x ∈ {x : ZMod p | (i.1 - j.1) + (i.2 - j.2) * x = 0} := heq
    rw [collision_secret_unique hb] at hx'
    simpa using hx'
  calc (univ.filter (fun x => Revealed S x)).card
      ≤ (T.biUnion (fun q => ({collisionSecret q.1 q.2} : Finset (ZMod p)))).card :=
        card_le_card hcover
    _ ≤ T.card * 1 :=
        card_biUnion_le_card_mul _ _ _ (by intro q _; simp)
    _ = T.card := Nat.mul_one _
    _ ≤ S.card * (S.card - 1) := by
        have hsub : T ⊆ S.offDiag := by
          intro q hq
          simp only [T, mem_filter] at hq
          exact hq.1
        calc T.card ≤ S.offDiag.card := card_le_card hsub
          _ = S.card * S.card - S.card := offDiag_card S
          _ = S.card * (S.card - 1) := (Nat.mul_sub_one _ _).symm

end ECDLPGeneric
