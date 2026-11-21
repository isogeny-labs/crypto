import Mathlib.Data.ZMod.Basic
import Mathlib.Tactic

/-! The P+2Q word preserves a -2 multiplier orbit. -/

namespace XedniRelationWordOrbits

set_option maxRecDepth 20000
set_option maxHeartbeats 0

abbrev K := ZMod 43

def orbit (seed : K) : Finset K :=
  (Finset.range 7).image (fun i => seed * (-2 : K)^i)

def seeds : Finset K := {1, 2, 3, 6, 7, 9}

theorem multiplier_seven : (-2 : K)^7 = 1 := by decide

theorem six_orbits_cover :
    ∀ x : K, x ≠ 0 → ∃ s ∈ seeds, x ∈ orbit s := by decide

theorem six_orbits_disjoint :
    ∀ s ∈ seeds, ∀ t ∈ seeds, s ≠ t → Disjoint (orbit s) (orbit t) := by decide

theorem orbit_closed :
    ∀ s ∈ seeds, ∀ x ∈ orbit s, (-2 : K)*x ∈ orbit s := by decide

theorem orbit_card_seven :
    ∀ s ∈ seeds, (orbit s).card = 7 := by decide

theorem sign_pairs :
    (orbit 1).image (fun x => -x) = orbit 2 ∧
    (orbit 3).image (fun x => -x) = orbit 6 ∧
    (orbit 7).image (fun x => -x) = orbit 9 := by decide

theorem relation_forces_neg_double {A : Type*} [AddCommGroup A]
    (P Q : A) (h : P + Q + Q = 0) : P = -(Q+Q) := by
  calc
    P = (P+Q+Q)-(Q+Q) := by abel
    _ = -(Q+Q) := by rw [h]; simp

theorem scalar_relation_same_orbit (a b s : K)
    (hs : s ∈ seeds) (hb : b ∈ orbit s) (h : a+b+b=0) :
    a ∈ orbit s := by
  have ha : a = (-2 : K)*b := by linear_combination h
  rw [ha]
  exact orbit_closed s hs b hb

#print axioms multiplier_seven
#print axioms six_orbits_cover
#print axioms six_orbits_disjoint
#print axioms orbit_closed
#print axioms orbit_card_seven
#print axioms sign_pairs
#print axioms relation_forces_neg_double
#print axioms scalar_relation_same_orbit

end XedniRelationWordOrbits
