import Init

/- Finite inertia arithmetic only. The Kummer normal form, signature formula,
   and Bouw p-rank bound are external mathematical inputs. -/
namespace GHSCyclic11Orbit

def units : List Nat := (List.range 10).map (fun x => x + 1)
def residueMinus (i e : Nat) : Nat := (11 - (i * e) % 11) % 11
def thirdExponent (a b : Nat) : Nat := (11 - (a + b) % 11) % 11
def signature (a b i : Nat) : Nat :=
  (residueMinus i a + residueMinus i b +
   residueMinus i (thirdExponent a b)) / 11 - 1
def firstOrbit : List Nat := [1, 3, 4, 5, 9]
def secondOrbit : List Nat := [2, 6, 7, 8, 10]
def rows : List (Nat × Nat) :=
  units.flatMap (fun a =>
    units.filterMap (fun b =>
      if (a + b) % 11 != 0 then some (a, b) else none))

theorem all_inertia_rows : rows.length = 90 := by decide

theorem no_full_signature_orbit :
    rows.all (fun ab =>
      firstOrbit.any (fun i => signature ab.1 ab.2 i == 0) &&
      secondOrbit.any (fun i => signature ab.1 ab.2 i == 0)) = true := by decide

theorem all_signatures_have_five_ones :
    rows.all (fun ab =>
      ((units.map (signature ab.1 ab.2)).foldl (· + ·) 0) == 5) = true := by decide

def orbitMin (orbit : List Nat) (a b : Nat) : Nat :=
  (orbit.map (signature a b)).foldl min 1

def orbitBound (a b : Nat) : Nat :=
  5 * orbitMin firstOrbit a b + 5 * orbitMin secondOrbit a b

theorem all_rows_orbit_bound_zero :
    rows.all (fun ab => decide (orbitBound ab.1 ab.2 = 0)) = true := by decide

/- Conditional endpoint: the p-rank upper bound itself is the cited external
   Bouw theorem, not a Lean theorem in this study. -/
theorem zero_rank_if_bouw_bound (ab : Nat × Nat) (f : Nat)
    (hrow : ab ∈ rows) (hbound : f ≤ orbitBound ab.1 ab.2) : f = 0 := by
  have hzero : orbitBound ab.1 ab.2 = 0 :=
    of_decide_eq_true ((List.all_eq_true.mp all_rows_orbit_bound_zero) ab hrow)
  exact Nat.eq_zero_of_le_zero (hzero ▸ hbound)

#print axioms all_inertia_rows
#print axioms no_full_signature_orbit
#print axioms all_signatures_have_five_ones
#print axioms all_rows_orbit_bound_zero
#print axioms zero_rank_if_bouw_bound

end GHSCyclic11Orbit
