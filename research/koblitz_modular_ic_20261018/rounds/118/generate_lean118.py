#!/usr/bin/env python3
"""Bind all four exact R118 Newton-row arithmetic claims to Lean."""
import json
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
result = json.loads((HERE/'result.json').read_text())
keys = ('y0_e1', 'y0_e3', 'y1_e1', 'y1_e2')
rows = {k: [(r['lambda_exponent'], r['s_order']) for r in result['branches'][k]['coefficient_rows']] for k in keys}
assert all(v is not None for values in rows.values() for _, v in values)
s = 'import Std.Tactic\n\nnamespace GNewton118\n\n'
for key in keys:
    values = ', '.join(f'({j}, {v})' for j, v in rows[key])
    s += f'def rows_{key} : List (Nat × Int) := [{values}]\n'
s += '''
def onOrAbove (rows : List (Nat × Int)) (base slope : Int) : Bool :=
 rows.all fun p => decide (base + slope * Int.ofNat p.1 <= p.2)
def onLine (rows : List (Nat × Int)) (base slope : Int) : List Nat :=
 (rows.filter fun p => p.2 == base + slope * Int.ofNat p.1).map Prod.fst

'''
for key, base, slope in [('y0_e1', -160, 10), ('y0_e3', -112, 7), ('y1_e1', 0, 0), ('y1_e2', 0, 0)]:
    s += f'theorem edge_{key} : onOrAbove rows_{key} ({base}) {slope} = true ∧ onLine rows_{key} ({base}) {slope} = [0, 8, 16] := by native_decide\n'
s += '''
def mul2 (a b : Nat) : Nat :=
 (List.range 17).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0
def power2 (a n : Nat) : Nat :=
 (List.range n).foldl (fun acc _ => mul2 acc a) 1
theorem repeatedEdgePolynomial : power2 7 8 = 65793 := by native_decide
-- (1+T+T^2)^8 = 1+T^8+T^16 over F2.

end GNewton118
'''
p = HERE/'lean/GNewton118.lean'
if '--check' in sys.argv:
    assert p.read_text() == s
    print('LEAN118_SOURCE_BINDING_PASS')
else:
    p.parent.mkdir(exist_ok=True)
    p.write_text(s)
