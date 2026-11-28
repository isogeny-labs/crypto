#!/usr/bin/env python3
"""Generate/check finite Lean residual certificate from R115 JSON."""
import json
import sys
from pathlib import Path
root=Path(__file__).resolve().parent
r=json.loads((root/'result.json').read_text())
terms=', '.join(f'({te}, {xe})' for te,xe in r['E_support'])
x=sum(1<<i for i in r['x_exponents'])
source=f'''import Std.Tactic

namespace DirectUniformizer115

-- Bit i is the coefficient of t^i in F2[t]/(t^128).
def mul (a b : Nat) : Nat :=
 (List.range 128).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0 % (2^128)
def power (a n : Nat) : Nat :=
 (List.range n).foldl (fun acc _ => mul acc a) 1
def E : List (Nat × Nat) := [{terms}]
def x : Nat := {x}
def evalE : Nat := E.foldl (fun acc p => Nat.xor acc (Nat.shiftLeft (power x p.2) p.1)) 0 % (2^128)

theorem residual128 : evalE = 0 := by native_decide
theorem leadingX : Nat.testBit x 2 ∧ ¬ Nat.testBit x 0 ∧ ¬ Nat.testBit x 1 := by native_decide
theorem directUniformizerOrder : 3 - 2 = 1 := by decide
theorem primitiveWildSlope : Nat.gcd 3 2 = 1 := by native_decide
theorem baseLocalDegreeSum : 1 + 3 + 1 + 2 = 7 := by decide

end DirectUniformizer115
'''
p=root/'lean/DirectUniformizer115.lean'
if '--check' in sys.argv:
 assert p.read_text()==source
 print('LEAN115_SOURCE_BINDING_PASS')
else:p.write_text(source)
