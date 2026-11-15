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

namespace ECDLPGeneric

open Classical Finset

variable {p : ℕ} [Fact p.Prime]

/-! ## Layer 2b: the non-adaptive transcript bound

A deterministic algorithm whose queries are fixed in advance sees only the
collision pattern of the known linear forms.  If two secrets have the same
pattern, the algorithm must output the same value for both, so it can be
correct on at most one secret in each pattern class.  All secrets outside the
revealed set share one class; every other correct secret lies in the revealed
set.  Hence the number of correct secrets is at most `revealed + 1`. -/

/-- Two secrets have the same transcript when every pair of known linear forms
agrees at one iff it agrees at the other. -/
def SamePattern (S : Finset (ZMod p × ZMod p)) (x y : ZMod p) : Prop :=
  ∀ i ∈ S, ∀ j ∈ S,
    ((i.1 - j.1) + (i.2 - j.2) * x = 0) ↔
      ((i.1 - j.1) + (i.2 - j.2) * y = 0)

/-- Secrets outside the revealed set all have the same transcript. -/
theorem samePattern_of_not_revealed {S : Finset (ZMod p × ZMod p)} {x y : ZMod p}
    (hx : ¬ Revealed S x) (hy : ¬ Revealed S y) : SamePattern S x y := by
  intro i hi j hj
  by_cases hb : i.2 = j.2
  · have hsub : i.2 - j.2 = 0 := sub_eq_zero.mpr hb
    rw [hsub]
    simp
  · have hxn : (i.1 - j.1) + (i.2 - j.2) * x ≠ 0 := by
      intro h
      exact hx ⟨i, hi, j, hj, hb, h⟩
    have hyn : (i.1 - j.1) + (i.2 - j.2) * y ≠ 0 := by
      intro h
      exact hy ⟨i, hi, j, hj, hb, h⟩
    exact ⟨fun h => absurd h hxn, fun h => absurd h hyn⟩

/-- **Non-adaptive Shoup bound.**  If an algorithm's output depends only on
the collision pattern of `S`, it is correct on at most `k(k-1) + 1` of the
`p` possible secrets, where `k = |S|`.  The `+1` is the single secret in the
"no revealing collision" class; the rest lie in the revealed set. -/
theorem correct_card_le {S : Finset (ZMod p × ZMod p)} {f : ZMod p → ZMod p}
    (hf : ∀ x y, SamePattern S x y → f x = f y) :
    (univ.filter (fun x => f x = x)).card ≤ S.card * (S.card - 1) + 1 := by
  classical
  let U : Finset (ZMod p) := univ.filter (fun x => Revealed S x)
  let V : Finset (ZMod p) := univ.filter (fun x => f x = x)
  let W : Finset (ZMod p) := univ.filter (fun x => f x = x ∧ x ∉ U)
  have hU : U.card ≤ S.card * (S.card - 1) := revealed_card_le S
  have hW : W.card ≤ 1 := by
    rw [card_le_one]
    intro x hx y hy
    simp only [W, U, mem_filter, mem_univ, true_and] at hx hy
    have h := hf x y (samePattern_of_not_revealed hx.2 hy.2)
    rw [hx.1, hy.1] at h
    exact h
  have hsub : V ⊆ U ∪ W := by
    intro x hx
    have hx' : f x = x := by simpa [V] using hx
    by_cases hxU : x ∈ U
    · exact mem_union_left _ hxU
    · exact mem_union_right _ (by simp [W, hx', hxU])
  calc V.card ≤ (U ∪ W).card := card_le_card hsub
    _ ≤ U.card + W.card := card_union_le U W
    _ ≤ U.card + 1 := Nat.add_le_add_left hW U.card
    _ ≤ S.card * (S.card - 1) + 1 := Nat.add_le_add_right hU 1

end ECDLPGeneric
