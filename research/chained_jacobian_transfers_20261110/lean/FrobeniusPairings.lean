import Init.Grind.Ring.Basic

/-! Finite matching certificate and algebra for the fixed kernel assessment.
This does not construct branch divisors, finite fields or an isogeny. -/
set_option autoImplicit false
namespace FrobeniusPairings
abbrev Matching := List (Nat × Nat)
def candidates : List Matching := [
  [(0,1),(2,3),(4,5)], [(0,1),(2,4),(3,5)], [(0,1),(2,5),(3,4)],
  [(0,2),(1,3),(4,5)], [(0,2),(1,4),(3,5)], [(0,2),(1,5),(3,4)],
  [(0,3),(1,2),(4,5)], [(0,3),(1,4),(2,5)], [(0,3),(1,5),(2,4)],
  [(0,4),(1,2),(3,5)], [(0,4),(1,3),(2,5)], [(0,4),(1,5),(2,3)],
  [(0,5),(1,2),(3,4)], [(0,5),(1,3),(2,4)], [(0,5),(1,4),(2,3)] ]
def valid (m : Matching) : Bool :=
  m.length == 3 && (List.range 6).all (fun i =>
    (m.flatMap (fun p => [p.1,p.2])).count i == 1)
def partner (m : Matching) (i : Nat) : Nat :=
  match m.find? (fun p => p.1 == i || p.2 == i) with
  | some (a,b) => if a = i then b else a
  | none => i
def stable (sigma : Nat → Nat) (m : Matching) : Bool :=
  (List.range 6).all (fun i => partner m (sigma i) == sigma (partner m i))
def sixCycle (i : Nat) : Nat := (i+1)%6
def twoThreeCycles (i : Nat) : Nat := if i < 3 then (i+1)%3 else 3+(i-3+1)%3

theorem pairing_table_valid : candidates.length = 15 ∧
    candidates.eraseDups.length = 15 ∧ candidates.all valid = true := by decide

theorem six_cycle_table : candidates.filter (stable sixCycle) =
    [[(0,3),(1,4),(2,5)]] := by decide

theorem two_three_cycles_table : candidates.filter (stable twoThreeCycles) =
    [[(0,3),(1,4),(2,5)],[(0,4),(1,5),(2,3)],[(0,5),(1,3),(2,4)]] := by decide

open Lean.Grind
theorem zero_linear_column {K : Type} [CommRing K] (a b c : K) :
    a*0*1 + 0*1*c + 1*b*0 - (a*1*0 + 0*b*1 + 1*0*c) = 0 := by grind

theorem odd_norm_exponent (k : Int) :
    (2*k+1)^3-1 = 2*k*((2*k+1)^2+(2*k+1)+1) := by grind

end FrobeniusPairings
#print axioms FrobeniusPairings.pairing_table_valid
#print axioms FrobeniusPairings.six_cycle_table
#print axioms FrobeniusPairings.two_three_cycles_table
#print axioms FrobeniusPairings.zero_linear_column
#print axioms FrobeniusPairings.odd_norm_exponent
