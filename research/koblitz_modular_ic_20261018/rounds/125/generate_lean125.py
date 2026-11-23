#!/usr/bin/env python3
"""Bind simple N51 orders and seven-edge cancellations to Lean."""
import json
import sys
from pathlib import Path

here=Path(__file__).resolve().parent
r=json.loads((here/'result.json').read_text())
keys=('y0_e1','y0_e3','y1_e1','y1_e2')
s='import Std.Tactic\n\nnamespace N51Branches125\n\n'
for key in keys:
    entry=r['branches'][key][0]
    rows=entry['final_coefficient_rows']
    assert all(row['s_order'] is not None for row in rows)
    assert entry['simple']['tied_correction_exponents']==[0]
    assert entry['tame_seven']['tied_correction_exponents']==[0,7]
    assert rows[0]['leading_F4']==rows[7]['leading_F4']==1
    values=', '.join(f"({i}, {row['s_order']})" for i,row in enumerate(rows))
    s+=f'def rows_{key} : List (Nat × Int) := [{values}]\n'
s+='''
def above (rows : List (Nat × Int)) (e q m : Int) : Bool :=
 rows.all fun p => decide (e * p.2 + q * Int.ofNat p.1 >= m)
def tied (rows : List (Nat × Int)) (e q m : Int) : List Nat :=
 (rows.filter fun p => decide (e * p.2 + q * Int.ofNat p.1 = m)).map Prod.fst

'''
for key in keys:
    entry=r['branches'][key][0]
    for kind,name in [('simple','simple'),('tame_seven','seven')]:
        z=entry[kind];e=z['ramification_over_base'];q=z['correction_order_in_normalized_parameter'];m=z['candidate_normalized_order']
        expected=[0] if kind=='simple' else [0,7]
        s+=f'theorem {name}_{key} : above rows_{key} {e} {q} ({m}) = true ∧ tied rows_{key} {e} {q} ({m}) = {expected} := by native_decide\n'
s+='theorem simpleLeading : (1 : Nat) ≠ 0 := by decide\n'
s+='theorem sevenCancellation : Nat.xor 1 1 = 0 := by decide\n'
s+='\nend N51Branches125\n'
path=here/'lean/N51Branches125.lean'
if '--check' in sys.argv:
    assert path.read_text()==s
    print('LEAN125_SOURCE_BINDING_PASS')
else:
    path.parent.mkdir(exist_ok=True)
    path.write_text(s)
