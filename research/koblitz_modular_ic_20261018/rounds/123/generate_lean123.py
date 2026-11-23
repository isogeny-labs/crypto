#!/usr/bin/env python3
"""Bind finite x=0 branch and degree arithmetic to Lean."""
import json
import sys
from pathlib import Path

here=Path(__file__).resolve().parent
r=json.loads((here/'result.json').read_text())
keys=('y0_e1','y0_e3','y1_e1','y1_e2')
es=[r['base_places'][k]['x_order_on_base'] for k in keys]
assert es==[1,3,1,2]
assert r['normalized_places_over_f2']==8 and r['x_zero_divisor_degree']==112
s=f'''import Std.Tactic

namespace BranchCensus123

def baseRamification : List Nat := {es}
def localDegree (e : Nat) : Nat := 2 * e + 2 * 7 * e
theorem baseDegree : baseRamification.sum = 7 := by native_decide
theorem totalDegree : (baseRamification.map localDegree).sum = 112 := by native_decide
theorem eightPlaces : baseRamification.length * 2 = 8 := by native_decide
theorem localSplits : 2 * (1 + 7) = 16 := by native_decide

end BranchCensus123
'''
path=here/'lean/BranchCensus123.lean'
if '--check' in sys.argv:
    assert path.read_text()==s
    print('LEAN123_SOURCE_BINDING_PASS')
else:
    path.parent.mkdir(exist_ok=True)
    path.write_text(s)
