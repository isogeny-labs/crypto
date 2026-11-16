#!/usr/bin/env python3
"""Bind translated coefficient orders to finite Lean polygon checks."""
import json
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
r = json.loads((here/'result.json').read_text())
keys = ('y0_e1', 'y0_e3', 'y1_e1', 'y1_e2')
rows = {k: [(x['z_exponent'], x['s_order']) for x in r['branches'][k][0]['translated_rows']] for k in keys}
expected = {'y0_e1': [8,23,23,0,0], 'y0_e3': [8,23,23,0,0],
            'y1_e1': [28,23,23,0,0], 'y1_e2': [16,23,23,0,0]}
for key in keys:
    assert [v for _, v in rows[key]] == expected[key]
s = 'import Std.Tactic\n\nnamespace F4Newton119\n\n'
for key in keys:
    vals = ', '.join(f'({j}, {v})' for j,v in rows[key])
    s += f'def rows_{key} : List (Nat × Int) := [{vals}]\n'
s += '''
def above (rows : List (Nat × Int)) (a b c : Int) : Bool :=
 rows.all fun p => decide (a * p.2 >= b + c * Int.ofNat p.1)
def onLine (rows : List (Nat × Int)) (a b c : Int) : List Nat :=
 (rows.filter fun p => decide (a * p.2 = b + c * Int.ofNat p.1)).map Prod.fst

'''
checks = [
 ('y0_e1', 1, 8, -1, [0,8]), ('y0_e3', 1, 8, -1, [0,8]),
 ('y1_e1', 1, 28, -5, [0,1]), ('y1_e1', 7, 184, -23, [1,8]),
 ('y1_e2', 1, 16, -2, [0,8])]
for i,(key,a,b,c,on) in enumerate(checks):
    s += f'theorem edge_{i} : above rows_{key} {a} ({b}) ({c}) = true ∧ onLine rows_{key} {a} ({b}) ({c}) = {on} := by native_decide\n'
for key in keys:
    s += f'theorem horizontal_{key} : above rows_{key} 1 0 0 = true ∧ onLine rows_{key} 1 0 0 = [8, 16] := by native_decide\n'
s += '\nend F4Newton119\n'
path = here/'lean/F4Newton119.lean'
if '--check' in sys.argv:
    assert path.read_text() == s
    print('LEAN119_SOURCE_BINDING_PASS')
else:
    path.parent.mkdir(exist_ok=True)
    path.write_text(s)
