#!/usr/bin/env python3
"""Bind exact R117 Newton-row arithmetic to Lean."""
import json
import sys
from pathlib import Path
root=Path(__file__).resolve().parent
r=json.loads((root/'result.json').read_text())
rows={k:[(x['lambda_exponent'],x['s_order']) for x in r['branches'][k]['coefficient_rows']]
      for k in ('y0_e1','y1_e1')}
assert all(v is not None for vals in rows.values() for j,v in vals)
assert rows['y0_e1']==[(0,-160),(1,-127),(4,-97),(8,-80),(16,0)]
assert rows['y1_e1']==[(0,0),(1,23),(4,23),(8,0),(16,0)]
s='''import Std.Tactic

namespace GNewton117

'''
for key in ('y0_e1','y1_e1'):
 vals=', '.join(f'({j}, {v})' for j,v in rows[key])
 s+=f'def rows_{key} : List (Nat × Int) := [{vals}]\n'
base=-160 if key=='y0_e1' else 0
s+='''def onOrAbove (rows : List (Nat × Int)) (base slope : Int) : Bool :=
 rows.all fun p => decide (base + slope * Int.ofNat p.1 <= p.2)
def onLine (rows : List (Nat × Int)) (base slope : Int) : List Nat :=
 (rows.filter fun p => p.2 == base + slope * Int.ofNat p.1).map Prod.fst

'''
s+='theorem y0ExactEdge : onOrAbove rows_y0_e1 (-160) 10 = true ∧ onLine rows_y0_e1 (-160) 10 = [0, 8, 16] := by native_decide\n'
s+='theorem y1ExactEdge : onOrAbove rows_y1_e1 0 0 = true ∧ onLine rows_y1_e1 0 0 = [0, 8, 16] := by native_decide\n'
s+='''def mul2 (a b : Nat) : Nat :=
 (List.range 17).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0
def power2 (a n : Nat) : Nat :=
 (List.range n).foldl (fun acc _ => mul2 acc a) 1
theorem repeatedEdgePolynomial : power2 7 8 = 65793 := by native_decide
-- (1+T+T^2)^8 = 1+T^8+T^16 over F2.

end GNewton117
'''
p=root/'lean/GNewton117.lean'
if '--check' in sys.argv:
 assert p.read_text()==s
 print('LEAN117_SOURCE_BINDING_PASS')
else:p.write_text(s)
