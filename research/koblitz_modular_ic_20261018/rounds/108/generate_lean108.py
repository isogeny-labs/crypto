#!/usr/bin/env python3
"""Bind R105 support and R108 128-bit series into a Lean residual check."""
import json
from pathlib import Path

root = Path(__file__).resolve().parent
base = json.loads((root.parent / "105" / "result.json").read_text())
result = json.loads((root / "result.json").read_text())


def pairs(key):
    return ", ".join(f"({a}, {b})" for a, b in base[f"f_{key}_support"])


def bits(key):
    return sum(1 << i for i in result[key]["z_exponents"])


source = '''import Std.Tactic

namespace BaseSeries128

-- Bit i is the coefficient of s^i in F2[s]/(s^128).
def mul (a b : Nat) : Nat :=
  (List.range 128).foldl
    (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0 % (2^128)

def power (a n : Nat) : Nat :=
  (List.range n).foldl (fun acc _ => mul acc a) 1

def eval (terms : List (Nat × Nat)) (s z : Nat) : Nat :=
  terms.foldl (fun acc p => Nat.xor acc (mul (power s p.1) (power z p.2))) 0

'''
source += f'def f0 : List (Nat × Nat) := [{pairs("y0")}]\n'
source += f'def f1 : List (Nat × Nat) := [{pairs("y1")}]\n'
source += f'def z0 : Nat := {bits("y0")}\n'
source += f'def z1 : Nat := {bits("y1")}\n'
source += '''
theorem y0SourceResidual128 : eval f0 (mul 2 z0) z0 = 0 := by native_decide
theorem y1SourceResidual128 : eval f1 (mul 2 z1) z1 = 0 := by native_decide
theorem y0Leading : Nat.testBit z0 2 ∧ ¬ Nat.testBit z0 0 ∧ ¬ Nat.testBit z0 1 := by native_decide
theorem y1Leading : Nat.testBit z1 3 ∧ ¬ Nat.testBit z1 0 ∧ ¬ Nat.testBit z1 1 ∧ ¬ Nat.testBit z1 2 := by native_decide

end BaseSeries128
'''
path = root / "lean" / "BaseSeries128.lean"
path.parent.mkdir(parents=True, exist_ok=True)
path.write_text(source)
print(path)
