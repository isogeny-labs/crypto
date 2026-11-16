import Init

/-! Finite arithmetic behind the cyclic-cubic genus-five obstruction.
    The geometric genus, p-rank, and fibre-count formulas are cited externally. -/
namespace TraceMinus14ArtinSchreierPoleArithmetic

def poleTypes : List (Nat × Nat) :=
  [(1,1),(1,2),(1,4),(1,5),(2,1),(2,2),(3,1)]

def cost (t : Nat × Nat) : Nat := t.1 * (t.2 + 1)

def configurations : List (List (Nat × Nat)) :=
  let one := poleTypes.map fun t => [t]
  let two := poleTypes.flatMap fun t => poleTypes.map fun u => [t,u]
  let three := poleTypes.flatMap fun t => poleTypes.flatMap fun u =>
    poleTypes.map fun v => [t,u,v]
  (one ++ two ++ three).filter fun ts => (ts.map cost).sum == 7

def rationalPoles (ts : List (Nat × Nat)) : Nat :=
  (ts.filter fun t => t.1 == 1).length

def quadraticPoles (ts : List (Nat × Nat)) : Nat :=
  (ts.filter fun t => t.1 == 2).length

def countCompatible (ts : List (Nat × Nat)) : Bool :=
  rationalPoles ts % 3 == 1 &&
  (rationalPoles ts + 2 * quadraticPoles ts) % 3 == 1

theorem pole_type_complete_below_eight :
    ∀ d m : Fin 8, d.val ≥ 1 → m.val ≥ 1 → m.val % 3 ≠ 0 →
      d.val * (m.val + 1) ≤ 7 → (d.val,m.val) ∈ poleTypes := by decide

theorem no_compatible_pole_configuration :
    (configurations.filter countCompatible).isEmpty = true := by decide

theorem middle_coefficient_nonzero_mod_three : (14 : Nat) % 3 = 2 := by decide

theorem five_not_even : ∀ r : Nat, 2*r ≠ 5 := by
  intro r h
  have hmod := congrArg (fun n : Nat => n % 2) h
  simp at hmod

end TraceMinus14ArtinSchreierPoleArithmetic
