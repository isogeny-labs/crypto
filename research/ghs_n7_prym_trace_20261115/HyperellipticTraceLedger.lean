import Init

/-!
Finite set arithmetic for the F2 genus-seven hyperelliptic census.
The input lists are results of the native exhaustive programs, not proved here
to be the geometric classification. This module proves the stated intersection
and the witness point-count arithmetic, with no axiom added for the inputs.
-/

namespace GHSN7HyperellipticTraceLedger

def pairedCoefficientTraces : List Int := [-17, -15, -9, -7, -1, 1, 7, 9, 15, 17]
def w1Traces : List Int := [-17, -9, -1, 7, 15]
def hyperellipticTraces : List Int := [-15, -13, -9, -1, 1, 9, 13, 15]

theorem w1_hyperelliptic_intersection :
    (w1Traces.filter fun t => hyperellipticTraces.contains t) =
      [-9, -1, 15] := by
  decide

theorem excluded_w1_traces :
    (w1Traces.filter fun t => !(hyperellipticTraces.contains t)) =
      [-17, 7] := by
  decide

theorem witness_seventh_counts :
    (129 - 234 : Int) = 7 * (-15) ∧
    (129 - 220 : Int) = 7 * (-13) ∧
    (129 - 192 : Int) = 7 * (-9) ∧
    (129 - 136 : Int) = 7 * (-1) ∧
    (129 - 122 : Int) = 7 * 1 ∧
    (129 - 66 : Int) = 7 * 9 ∧
    (129 - 38 : Int) = 7 * 13 ∧
    (129 - 24 : Int) = 7 * 15 := by
  decide

def supportSum (q : Nat) : Nat := 1 + q + q ^ 2 + q ^ 3 + q ^ 4 + q ^ 5

theorem sample_support_counts_arithmetic :
    supportSum 2 = 7 * 9 ∧
    supportSum 4 = 7 * 195 ∧
    supportSum 8 + 36 = 7 * 5355 := by
  decide

#print axioms w1_hyperelliptic_intersection
#print axioms excluded_w1_traces
#print axioms witness_seventh_counts
#print axioms sample_support_counts_arithmetic

end GHSN7HyperellipticTraceLedger
