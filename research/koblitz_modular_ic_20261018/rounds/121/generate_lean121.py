#!/usr/bin/env python3
"""Bind R121 exact order rows to Lean finite hull checks."""
import json
import sys
from pathlib import Path

here = Path(__file__).resolve().parent
r = json.loads((here/'result.json').read_text())
keys = ('y0_e1','y0_e3')
rows = {k:[(x['u_exponent'],x['s_order']) for x in r['branches'][k][0]['translated_rows']] for k in keys}
assert [[v for _,v in rows[k]] for k in keys] == [[6,2,11,0,24],[8,9,15,0,16]]
s = 'import Std.Tactic\n\nnamespace F4Newton121\n\n'
for key in keys:
    s += f'def rows_{key} : List (Nat × Int) := [{", ".join(f"({j}, {v})" for j,v in rows[key])}]\n'
s += '''
def above (rows : List (Nat × Int)) (a b c : Int) : Bool :=
 rows.all fun p => decide (a * p.2 >= b + c * Int.ofNat p.1)
def onLine (rows : List (Nat × Int)) (a b c : Int) : List Nat :=
 (rows.filter fun p => decide (a * p.2 = b + c * Int.ofNat p.1)).map Prod.fst

'''
checks=[('y0_e1',1,6,-4,[0,1]),('y0_e1',7,16,-2,[1,8]),
        ('y0_e1',1,-24,3,[8,16]),
        ('y0_e3',1,8,-1,[0,8]),('y0_e3',1,-16,2,[8,16])]
for i,(key,a,b,c,on) in enumerate(checks):
    s += f'theorem edge_{i} : above rows_{key} {a} ({b}) ({c}) = true ∧ onLine rows_{key} {a} ({b}) ({c}) = {on} := by native_decide\n'
s += '\nend F4Newton121\n'
path=here/'lean/F4Newton121.lean'
if '--check' in sys.argv:
    assert path.read_text()==s
    print('LEAN121_SOURCE_BINDING_PASS')
else:
    path.parent.mkdir(exist_ok=True)
    path.write_text(s)
