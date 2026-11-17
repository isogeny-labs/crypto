import Std.Tactic

namespace BaseSeries128

-- Bit i is the coefficient of s^i in F2[s]/(s^128).
def mul (a b : Nat) : Nat :=
  (List.range 128).foldl
    (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0 % (2^128)

def power (a n : Nat) : Nat :=
  (List.range n).foldl (fun acc _ => mul acc a) 1

def eval (terms : List (Nat × Nat)) (s z : Nat) : Nat :=
  terms.foldl (fun acc p => Nat.xor acc (mul (power s p.1) (power z p.2))) 0

def f0 : List (Nat × Nat) := [(0, 3), (0, 7), (1, 2), (1, 3), (1, 4), (1, 5), (2, 0), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (3, 2), (3, 3), (3, 6), (4, 0), (4, 1), (4, 5), (5, 1), (5, 4), (5, 6), (6, 2), (6, 3), (6, 5), (7, 3), (7, 5)]
def f1 : List (Nat × Nat) := [(0, 4), (0, 5), (0, 6), (0, 7), (1, 3), (1, 5), (2, 2), (2, 3), (2, 4), (2, 5), (2, 6), (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), (3, 6), (4, 0), (4, 4), (4, 5), (5, 0), (5, 1), (5, 2), (5, 6), (6, 0), (6, 3), (6, 4), (6, 5), (7, 2), (7, 3), (7, 4), (7, 5)]
def z0 : Nat := 236450324035628573757255404090796993500
def z1 : Nat := 231101246951910669829123711840382869464

theorem y0SourceResidual128 : eval f0 (mul 2 z0) z0 = 0 := by native_decide
theorem y1SourceResidual128 : eval f1 (mul 2 z1) z1 = 0 := by native_decide
theorem y0Leading : Nat.testBit z0 2 ∧ ¬ Nat.testBit z0 0 ∧ ¬ Nat.testBit z0 1 := by native_decide
theorem y1Leading : Nat.testBit z1 3 ∧ ¬ Nat.testBit z1 0 ∧ ¬ Nat.testBit z1 1 ∧ ¬ Nat.testBit z1 2 := by native_decide

end BaseSeries128
