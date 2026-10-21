import Mathlib

/-!
# The combinatorial core of the isogeny-augmented generic group model (Thm. 15.1)

Companion to `papers/ic-negative-results/`, statements S2.  Source theorem:
`RESEARCH_ISOGENY_FIBERS_ECDLP.md` section 15.1 (Theorem 15.1): with the scalars
`lambda_psi` given, every oracle answer in the isogeny-augmented generic group
model is the encoding of a known *affine function* `a + b x` of the unknown
`x = log(Q_1)`, and Shoup's argument bounds the success probability by
`(Q + 2)^2 / r + 1 / r`.

What is formalised here is exactly the counting step of that argument, over an
arbitrary field `F` (the proof needs only that two distinct affine functions of
one variable agree at no more than one point):

* `agree_card_le_one`: two distinct affine forms agree on at most one secret;
* `badSecrets_card_le`: the number of secrets at which two different members of
  a family `P` of forms collide is at most `|P|^2 - |P|`;
* `badSecrets_card_le_sq`: with `|P| <= n` (n = Q + 2 transcripts), at most `n^2`
  secrets are bad, which is the `(Q + 2)^2 / r` term for `S = univ` of a field
  of `r` elements;
* `injOn_of_not_bad`: off the bad set the evaluation map is injective on `P`,
  which is what lets the simulator answer every fresh query with a fresh string.

NOT formalised (see `OPEN_TARGETS.md`): the oracle model itself, the random
injections, the simulator and the hybrid step that turns this count into the
success probability, and the final `1 / r` guessing term.  The statement here
is the combinatorial core, not Theorem 15.1.
-/

namespace IcNegative

open Finset

variable {F : Type*} [Field F] [DecidableEq F]

/-- **Two distinct affine forms agree at most once.**  A form `(a, b)` is the
function `x ↦ a + b * x`.  Over a field, if `(a, b) ≠ (c, d)` then
`a + b * x = c + d * x` has at most one solution `x`. -/
theorem agree_card_le_one (S : Finset F) (p q : F × F) (hpq : p ≠ q) :
    (S.filter (fun x => p.1 + p.2 * x = q.1 + q.2 * x)).card ≤ 1 := by
  rw [Finset.card_le_one]
  intro x hx y hy
  rw [Finset.mem_filter] at hx hy
  have hx' : p.1 + p.2 * x = q.1 + q.2 * x := hx.2
  have hy' : p.1 + p.2 * y = q.1 + q.2 * y := hy.2
  by_cases hb : p.2 = q.2
  · exfalso
    apply hpq
    have ha : p.1 = q.1 := by
      rw [hb] at hx'
      exact add_right_cancel hx'
    exact Prod.ext ha hb
  · have h1 : (p.2 - q.2) * (x - y) = 0 := by
      linear_combination hx' - hy'
    rcases mul_eq_zero.mp h1 with h2 | h2
    · exact absurd (sub_eq_zero.mp h2) hb
    · exact sub_eq_zero.mp h2

/-- The secrets in `S` at which the two forms `pq.1` and `pq.2` agree. -/
def agreeSet (S : Finset F) (pq : (F × F) × (F × F)) : Finset F :=
  S.filter (fun x => pq.1.1 + pq.1.2 * x = pq.2.1 + pq.2.2 * x)

/-- The secrets in `S` at which two *different* members of the family `P` of
affine forms take the same value (the transcript would be inconsistent). -/
def badSecrets (S : Finset F) (P : Finset (F × F)) : Finset F :=
  S.filter (fun x => ∃ p ∈ P, ∃ q ∈ P, p ≠ q ∧ p.1 + p.2 * x = q.1 + q.2 * x)

/-- **Union bound.**  At most `|P|^2 - |P|` secrets are bad (ordered pairs of
distinct forms; each pair is bad at no more than one secret). -/
theorem badSecrets_card_le (S : Finset F) (P : Finset (F × F)) :
    (badSecrets S P).card ≤ P.card * P.card - P.card := by
  have hsub : badSecrets S P ⊆ P.offDiag.biUnion (agreeSet S) := by
    intro x hx
    unfold badSecrets at hx
    rw [Finset.mem_filter] at hx
    obtain ⟨hxS, p, hp, q, hq, hpq, hxe⟩ := hx
    refine Finset.mem_biUnion.mpr ⟨(p, q), ?_, ?_⟩
    · exact Finset.mem_offDiag.mpr ⟨hp, hq, hpq⟩
    · unfold agreeSet
      exact Finset.mem_filter.mpr ⟨hxS, hxe⟩
  have hle : ∀ pq ∈ P.offDiag, (agreeSet S pq).card ≤ 1 := by
    intro pq hpq
    rw [Finset.mem_offDiag] at hpq
    exact agree_card_le_one S pq.1 pq.2 hpq.2.2
  calc (badSecrets S P).card
      ≤ (P.offDiag.biUnion (agreeSet S)).card := Finset.card_le_card hsub
    _ ≤ ∑ pq ∈ P.offDiag, (agreeSet S pq).card := Finset.card_biUnion_le
    _ ≤ ∑ pq ∈ P.offDiag, 1 := Finset.sum_le_sum hle
    _ = P.offDiag.card := by
        first
        | exact (Finset.card_eq_sum_ones _).symm
        | simp
    _ = P.card * P.card - P.card := by rw [Finset.offDiag_card]

/-- With at most `n` forms (n = Q + 2: the Q queries plus the two inputs), at most
`n * n` secrets are bad. -/
theorem badSecrets_card_le_sq (S : Finset F) (P : Finset (F × F)) (n : ℕ)
    (hn : P.card ≤ n) : (badSecrets S P).card ≤ n * n :=
  le_trans (badSecrets_card_le S P) (le_trans (Nat.sub_le _ _) (Nat.mul_le_mul hn hn))

/-- Off the bad set, distinct forms take distinct values: the simulator never has
to give two different forms the same answer. -/
theorem injOn_of_not_bad (S : Finset F) (P : Finset (F × F)) (x : F) (hx : x ∈ S)
    (hnb : x ∉ badSecrets S P) :
    Set.InjOn (fun p : F × F => p.1 + p.2 * x) (P : Set (F × F)) := by
  intro p hp q hq h
  by_contra hne
  apply hnb
  unfold badSecrets
  rw [Finset.mem_filter]
  exact ⟨hx, p, hp, q, hq, hne, h⟩

end IcNegative
