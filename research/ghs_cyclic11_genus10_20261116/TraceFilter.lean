-- Finite arithmetic portion of the cyclic-11 cover screen.
-- The orbit enumeration, point counts, geometry, and trace theorem are external.
namespace GHSCyclic11

def fifthTrace (s : Int) : Int := s^5 - 15*s^3 + 45*s
def baseTraces : List Int := [-3,-2,-1,0,1,2,3]
def fifthTraces : List Int := [27,-2,-31,0,31,2,-27]
def splitPairs : List (Int × Int) :=
  [(-31,-9),(-24,-24),(-24,24),(-9,-9),(-9,9),(-9,31),(2,2),(9,24),(24,24)]
def genusFiveTraces : List Int := [-24,-9,24]

theorem descent_recurrence :
    baseTraces.map fifthTrace = fifthTraces := by decide

theorem genus_five_source_exclusion :
    genusFiveTraces.all (fun t => decide (t % 3 = 0 ∧ t ≠ 0 ∧ t ≠ -27 ∧ t ≠ 27)) = true := by decide

theorem split_factors_eligible_only_if_descended :
    splitPairs.all (fun pair =>
      decide ((pair.1 % 3 ≠ 0 ∨ pair.1 = 0 ∨ pair.1 = -27 ∨ pair.1 = 27)
        → pair.1 ∈ fifthTraces) &&
      decide ((pair.2 % 3 ≠ 0 ∨ pair.2 = 0 ∨ pair.2 = -27 ∨ pair.2 = 27)
        → pair.2 ∈ fifthTraces)) = true := by decide

end GHSCyclic11
