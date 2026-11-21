#!/usr/bin/env python3
"""Bind the four N51 weighted ties and F4 cancellation to Lean."""
import json
import sys
from pathlib import Path

here=Path(__file__).resolve().parent
r=json.loads((here/'result.json').read_text())
keys=('y0_e1','y0_e3','y1_e1','y1_e2')
ties=[0,1,3,4,6,7,8,10,11,13,14]
s='import Std.Tactic\n\nnamespace N51Leading124\n\n'
for key in keys:
    b=r['branches'][key]
    rows=[(x['lambda_exponent'],x['s_order']) for x in b['coefficient_rows']]
    assert all(v is not None for _,v in rows)
    assert all(x['tied_lambda_exponents']==ties and x['leading_F4_sum']==0 for x in b['first_forms'])
    s+=f'def rows_{key} : List (Nat × Int) := [{", ".join(f"({j}, {v})" for j,v in rows)}]\n'
s+='''
def above (rows : List (Nat × Int)) (k m : Int) : Bool :=
 rows.all fun p => decide (p.2 - k * Int.ofNat p.1 >= m)
def tied (rows : List (Nat × Int)) (k m : Int) : List Nat :=
 (rows.filter fun p => decide (p.2 - k * Int.ofNat p.1 = m)).map Prod.fst
def alphaPower (a j : Nat) : Nat :=
 if j % 3 = 0 then 1 else if j % 3 = 1 then a else Nat.xor a 1
def leadingSum (a : Nat) (js : List Nat) : Nat :=
 js.foldl (fun z j => Nat.xor z (alphaPower a j)) 0

'''
for key in keys:
    b=r['branches'][key]
    k=b['lambda_pole_order'];m=b['first_forms'][0]['candidate_s_order']
    s+=f'theorem weighted_{key} : above rows_{key} {k} ({m}) = true ∧ tied rows_{key} {k} ({m}) = {ties} := by native_decide\n'
s+=f'def commonTies : List Nat := {ties}\n'
s+='theorem f4Cancellation : leadingSum 2 commonTies = 0 ∧ leadingSum 3 commonTies = 0 := by native_decide\n'
s+='''
def mul2 (a b : Nat) : Nat :=
 (List.range 15).foldl (fun z j => if Nat.testBit b j then Nat.xor z (Nat.shiftLeft a j) else z) 0
def power2 (a n : Nat) : Nat :=
 (List.range n).foldl (fun z _ => mul2 z a) 1
def tiesPolynomial : Nat :=
 commonTies.foldl (fun z j => Nat.xor z (Nat.shiftLeft 1 j)) 0
theorem seventhPower : tiesPolynomial = power2 7 7 := by native_decide
-- The bit polynomial of the 11 tied powers is (1+T+T^2)^7.
'''
s+='\nend N51Leading124\n'
path=here/'lean/N51Leading124.lean'
if '--check' in sys.argv:
    assert path.read_text()==s
    print('LEAN124_SOURCE_BINDING_PASS')
else:
    path.parent.mkdir(exist_ok=True)
    path.write_text(s)
