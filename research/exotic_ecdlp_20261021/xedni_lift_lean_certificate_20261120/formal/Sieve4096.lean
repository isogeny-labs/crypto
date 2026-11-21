import Mathlib.Tactic
import CandidateListProof
import SquareModulo

namespace XedniSieve4096

def curveRight (a : ℤ) (b : ℕ) : ℤ :=
  a^3 + 9 * a * (b : ℤ)^4 + (b : ℤ)^6

abbrev candidateList := XedniCandidateList.candidateList

def witnessPrimes : List ℕ :=
  [3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97]

def nonsquareMod (a : ℤ) (b m : ℕ) : Bool :=
  !(List.range m).any fun z =>
    decide (((z : ℤ)^2 - curveRight a b) % m = 0)

def rejected (a : ℤ) (b : ℕ) : Bool :=
  decide (curveRight a b < 0) || witnessPrimes.any (nonsquareMod a b)

def checkB (b cap : ℕ) : Bool :=
  (candidateList b).all fun a =>
    if decide (-(cap : ℤ) ≤ a ∧ a ≤ cap ∧ a.natAbs.Coprime b) then
      rejected a b
    else true

def fullCheck (cap : ℕ) : Bool :=
  (List.range cap).all fun i => checkB (i+1) cap

set_option maxRecDepth 1000000 in
set_option maxHeartbeats 0 in
theorem check256 : fullCheck 256 = true := by decide

set_option maxRecDepth 1000000 in
set_option maxHeartbeats 0 in
theorem check4096 : fullCheck 4096 = true := by decide

#print axioms check256
#print axioms check4096

end XedniSieve4096
