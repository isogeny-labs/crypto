#!/usr/bin/env python3
"""Generate or check Lean finite residual certificates for all R116 branches."""
import json
import sys
from pathlib import Path
root=Path(__file__).resolve().parent
r=json.loads((root/'result.json').read_text())
W=640
header=f'''import Std.Tactic

namespace FourBaseSeries116

-- Bit i is the coefficient of s^i in F2[s]/(s^{W}).
def mul (a b : Nat) : Nat :=
 (List.range {W}).foldl (fun acc i => if Nat.testBit b i then Nat.xor acc (Nat.shiftLeft a i) else acc) 0 % (2^{W})
def power (a n : Nat) : Nat :=
 (List.range n).foldl (fun acc _ => mul acc a) 1
def eval (terms : List (Nat × Nat)) (z : Nat) : Nat :=
 terms.foldl (fun acc p => Nat.xor acc (Nat.shiftLeft (power z p.2) p.1)) 0 % (2^{W})

'''
body=''
for key in ('y0_e1','y0_e3','y1_e1','y1_e2'):
 b=r['branches'][key]
 terms=', '.join(f'({a}, {c})' for a,c in b['equation_support'])
 z=sum(1<<i for i in b['root_exponents'])
 body+=f'def terms_{key} : List (Nat × Nat) := [{terms}]\n'
 body+=f'def root_{key} : Nat := {z}\n'
 body+=f'theorem residual_{key} : eval terms_{key} root_{key} = 0 := by native_decide\n\n'
body+='theorem denominatorMargin : 483 < 640 := by decide\n'
body+='theorem localDegreeSum : 1 + 3 + 1 + 2 = 7 := by decide\n\nend FourBaseSeries116\n'
source=header+body
p=root/'lean/FourBaseSeries116.lean'
if '--check' in sys.argv:
 assert p.read_text()==source
 print('LEAN116_SOURCE_BINDING_PASS')
else:p.write_text(source)
