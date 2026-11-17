#!/usr/bin/env python3
"""Lexically render native primitive and unit witnesses; no arithmetic."""
from pathlib import Path
import json,hashlib
h=Path(__file__).resolve().parent
p=h/'runs/integer42-01/output/integer-recovery.json';q=h/'runs/integer42-01/output/least-scalar-unit-certificates.json'
out=['import CMRecovery42','set_option maxRecDepth 100000','set_option maxHeartbeats 0','namespace CMIntegerWitness42']
rows=[]
for c in json.loads(p.read_text())['lattices']:
 w=c['primitive_weights'];a=c['primitive_inverse_polynomial'];assert len(w)==len(a)+1
 terms=[f'({w[0]} : ℤ)*({c["least_scalar"]} : ℤ)']
 terms += [f'({x} : ℤ)*({y} : ℤ)' for x,y in zip(w[1:],a) if x!='0']
 name=f'n{c["n"]}_primitive_witness'
 out.append('theorem '+name+' :\n    '+' + '.join(terms)+' = 1 := by decide')
 rows.append({'declaration':name,'scope':'literal native primitive weighted identity only'})
for i,c in enumerate(json.loads(q.read_text())):
 name=f'target_unit_{i+1}'
 out.append(f'theorem {name} : ({c["gap"]} : ℤ)*({c["inverse"]} : ℤ)+({c["modulus"]} : ℤ)*({c["bezout_k"]} : ℤ)=1 := by decide')
 rows.append({'declaration':name,'label':c['label'],'scope':'literal native target unit identity'})
out.append('end CMIntegerWitness42')
(h/'lean/CMIntegerWitness42.lean').write_text('\n\n'.join(out)+'\n')
(h/'integer-proof-map.json').write_text(json.dumps({'native_sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'units_sha256':hashlib.sha256(q.read_bytes()).hexdigest(),'declarations':rows},indent=2)+'\n')
