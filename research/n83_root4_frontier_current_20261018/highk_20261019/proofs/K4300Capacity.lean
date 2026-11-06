import Std.Tactic

namespace N83K4300Capacity

def states (k : Nat) : Nat := 83 * (k * (k - 1) / 2) + 42 * k
def request (k : Nat) : Nat := (4 * states k + 2) / 3

theorem k4300_states : states 4300 = 767337150 := by decide
theorem k4300_request : request 4300 = 1023116200 := by decide
theorem k4300_in_bucket : request 4300 ≤ 2 ^ 30 := by decide

#print axioms N83K4300Capacity.k4300_states
#print axioms N83K4300Capacity.k4300_request
#print axioms N83K4300Capacity.k4300_in_bucket

end N83K4300Capacity
