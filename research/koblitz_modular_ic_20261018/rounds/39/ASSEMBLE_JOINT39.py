#!/usr/bin/env python3
"""Literal retained CM rows/polynomials and donor-byte binding only."""
from pathlib import Path
import json,hashlib,re
h=Path(__file__).resolve().parent
p=Path('/var/tmp/koblitz-r18-cm20-20261030/runs/cm20-shared-residual-Hecke-systems.json')
data=p.read_bytes();manifest={n:d for d,n in (x.split(maxsplit=1) for x in(h.parent/'18/runs.sha256').read_text().splitlines())};assert hashlib.sha256(data).hexdigest()==manifest['runs/cm20-shared-residual-Hecke-systems.json']
r=json.loads(data);s='ring jointCM39=0,(a,t,X),dp;\n'
for label in ('E0','E1'):
 co=r['actual_source_Frobenius_polynomials'][label]
 assert len(co)==9
 s+=f'poly source{label}Polynomial39='+'+'.join(f'({x})*X^{i}' for i,x in enumerate(co))+';\n'
cofactor=h.parent/'37/runs/cofactor37-01/complementary-euler.sing'
oldmanifest={n:d for d,n in (x.split(maxsplit=1) for x in(h.parent/'37/runs.sha256').read_text().splitlines())};assert hashlib.sha256(cofactor.read_bytes()).hexdigest()==oldmanifest['runs/cofactor37-01/complementary-euler.sing']
s+=cofactor.read_text().replace('T','X')+'\nintmat systems39[8][5];\n'
assert len(r['systems'])==8
for i,row in enumerate(r['systems'],1):
 for j,k in enumerate(('diamond2','alpha_residue','eta_exponent','E1_Frobenius','E0_Frobenius'),1):s+=f'systems39[{i},{j}]={row[k]};\n'
s+=(h/'JOINT39.sing').read_text();(h/'source-data/joint-input-template.sing').write_text(s)
(h/'source-data/retained-cm20.json').write_bytes(data)
(h/'joint-donors.json').write_text(json.dumps({'R18/cm20':hashlib.sha256(data).hexdigest(),'R37/complementary-euler':hashlib.sha256(cofactor.read_bytes()).hexdigest(),'actual_diamond39_character':7},indent=2)+'\n')
print('Bound retained eight CM rows and full Frobenius polynomials; assembled exact twist/operator checks.')
