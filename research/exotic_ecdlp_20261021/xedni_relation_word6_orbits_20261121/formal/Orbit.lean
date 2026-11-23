import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! The coefficient-six relation word stays within a multiplier-minus-six orbit. -/

namespace XedniRelationWord6Orbits

set_option maxRecDepth 20000
set_option maxHeartbeats 0
set_option linter.style.haveILetI false

abbrev K := ZMod 43

def orbit (seed : K) : Finset K :=
  (Finset.range 6).image (fun i => seed * (-6 : K)^i)

def seeds : Finset K := {1, 2, 3, 4, 5, 9, 10}

def fold (x : K) : K :=
  if x.val ≤ (-x).val then x else -x

def foldedOrbit (seed : K) : Finset K :=
  (orbit seed).image fold

theorem multiplier_six : (-6 : K)^6 = 1 := by decide

theorem multiplier_half_turn : (-6 : K)^3 = -1 := by decide

theorem seven_orbits_cover :
    ∀ x : K, x ≠ 0 → ∃ s ∈ seeds, x ∈ orbit s := by decide

theorem seven_orbits_disjoint :
    ∀ s ∈ seeds, ∀ t ∈ seeds, s ≠ t → Disjoint (orbit s) (orbit t) := by decide

theorem orbit_card_six :
    ∀ s ∈ seeds, (orbit s).card = 6 := by decide

theorem orbit_closed :
    ∀ s ∈ seeds, ∀ x ∈ orbit s, (-6 : K)*x ∈ orbit s := by decide

theorem orbit_neg_closed :
    ∀ s ∈ seeds, ∀ x ∈ orbit s, -x ∈ orbit s := by decide

theorem folded_orbit_card_three :
    ∀ s ∈ seeds, (foldedOrbit s).card = 3 := by decide

theorem folded_orbits_disjoint :
    ∀ s ∈ seeds, ∀ t ∈ seeds, s ≠ t →
      Disjoint (foldedOrbit s) (foldedOrbit t) := by decide

theorem folded_orbits_cover :
    ∀ x : K, x ≠ 0 → ∃ s ∈ seeds, fold x ∈ foldedOrbit s := by decide

theorem relation_forces_neg_six {A : Type*} [AddCommGroup A]
    (P Q : A) (h : P + 6 • Q = 0) : P = -(6 • Q) := by
  calc
    P = (P + 6 • Q) - 6 • Q := by abel
    _ = -(6 • Q) := by rw [h]; simp

theorem scalar_relation_same_orbit (a b s : K)
    (hs : s ∈ seeds) (hb : b ∈ orbit s) (h : a + 6*b = 0) :
    a ∈ orbit s := by
  have ha : a = (-6 : K)*b := by linear_combination h
  rw [ha]
  exact orbit_closed s hs b hb

theorem scalar_relation_same_folded_orbit :
    ∀ a b s : K, s ∈ seeds → fold b ∈ foldedOrbit s →
      a + 6*b = 0 → fold a ∈ foldedOrbit s := by decide

def maskedLog (seed x : K) : K :=
  if x ∈ orbit seed then x else 0

theorem masked_log_annihilates_relation :
    ∀ s ∈ seeds, ∀ a b : K, a + 6*b = 0 →
      maskedLog s a + 6*maskedLog s b = 0 := by decide

theorem masked_log_is_odd :
    ∀ s ∈ seeds, ∀ x : K, maskedLog s (-x) = -maskedLog s x := by decide

theorem masked_modes_separate :
    ∀ s ∈ seeds, ∀ t ∈ seeds,
      maskedLog s t = if s = t then t else 0 := by decide

theorem masked_modes_independent (c : K → K)
    (hvanish : ∀ x : K, ∑ s ∈ seeds, c s * maskedLog s x = 0) :
    ∀ s ∈ seeds, c s = 0 := by
  intro s hs
  have hsum : (∑ t ∈ seeds, c t * maskedLog t s) = c s * s := by
    calc
      _ = ∑ t ∈ seeds, if t = s then c t * s else 0 := by
        apply Finset.sum_congr rfl
        intro t ht
        rw [masked_modes_separate t ht s hs]
        split_ifs <;> ring
      _ = c s * s := by simp [hs]
  have hproduct : c s * s = 0 := by rw [← hsum]; exact hvanish s
  have hnonzero : s ≠ 0 := by
    have h : ∀ t ∈ seeds, t ≠ 0 := by decide
    exact h s hs
  letI : Fact (Nat.Prime 43) := ⟨by norm_num⟩
  exact (mul_eq_zero.mp hproduct).resolve_right hnonzero

theorem seven_independent_relation_annihilators :
    (∀ s ∈ seeds, ∀ a b : K, a + 6*b = 0 →
      maskedLog s a + 6*maskedLog s b = 0) ∧
    (∀ s ∈ seeds, ∀ x : K, maskedLog s (-x) = -maskedLog s x) ∧
    (∀ c : K → K,
      (∀ x : K, ∑ s ∈ seeds, c s * maskedLog s x = 0) →
      ∀ s ∈ seeds, c s = 0) := by
  exact ⟨masked_log_annihilates_relation, masked_log_is_odd,
    masked_modes_independent⟩

def wordConsistent (f : K → K) : Prop :=
  ∀ a b : K, a + 6*b = 0 → f a + 6*f b = 0

def signConsistent (f : K → K) : Prop :=
  ∀ x : K, f (-x) = -f x

def actualLog (x : K) : K := x

def alternateLog (x : K) : K := x + maskedLog 2 x

theorem actual_word_consistent : wordConsistent actualLog := by
  intro a b h
  simpa [actualLog] using h

theorem alternate_word_consistent : wordConsistent alternateLog := by
  intro a b h
  have hm := masked_log_annihilates_relation (2 : K) (by decide) a b h
  dsimp [alternateLog]
  linear_combination h + hm

theorem actual_sign_consistent : signConsistent actualLog := by
  intro x
  simp [actualLog]

theorem alternate_sign_consistent : signConsistent alternateLog := by
  intro x
  have hm := masked_log_is_odd (2 : K) (by decide) x
  simp only [alternateLog, hm]
  ring

theorem same_anchor : actualLog 1 = 1 ∧ alternateLog 1 = 1 := by decide

theorem different_at_two : actualLog 2 ≠ alternateLog 2 := by decide

theorem six_other_seeds : (seeds.erase 1).card = 6 := by decide

theorem six_modes_preserve_anchor :
    ∀ s ∈ seeds, s ≠ 1 → maskedLog s 1 = 0 := by decide

theorem coefficient_six_word_does_not_determine_all_logs :
    ∃ f g : K → K,
      wordConsistent f ∧ wordConsistent g ∧
      signConsistent f ∧ signConsistent g ∧
      f 1 = 1 ∧ g 1 = 1 ∧ f 2 ≠ g 2 := by
  refine ⟨actualLog, alternateLog, actual_word_consistent,
    alternate_word_consistent, actual_sign_consistent,
    alternate_sign_consistent, same_anchor.1, same_anchor.2,
    different_at_two⟩

#print axioms multiplier_six
#print axioms multiplier_half_turn
#print axioms seven_orbits_cover
#print axioms seven_orbits_disjoint
#print axioms orbit_card_six
#print axioms orbit_closed
#print axioms orbit_neg_closed
#print axioms folded_orbit_card_three
#print axioms folded_orbits_disjoint
#print axioms folded_orbits_cover
#print axioms relation_forces_neg_six
#print axioms scalar_relation_same_orbit
#print axioms scalar_relation_same_folded_orbit
#print axioms masked_log_annihilates_relation
#print axioms masked_log_is_odd
#print axioms masked_modes_separate
#print axioms masked_modes_independent
#print axioms seven_independent_relation_annihilators
#print axioms actual_word_consistent
#print axioms alternate_word_consistent
#print axioms actual_sign_consistent
#print axioms alternate_sign_consistent
#print axioms same_anchor
#print axioms different_at_two
#print axioms six_other_seeds
#print axioms six_modes_preserve_anchor
#print axioms coefficient_six_word_does_not_determine_all_logs

end XedniRelationWord6Orbits
