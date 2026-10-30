#!/usr/bin/env python3
"""Serialize literal LMFDB coefficients into native input, no mathematical evaluation."""
from pathlib import Path
import json,hashlib
h=Path(__file__).resolve().parent;p=h/'source-data/lmfdb-77-2-l-b-hecke.json';r=json.loads(p.read_text())['data'][0];assert r['label']=='77.2.l.b' and r['hecke_ring_rank']==16 and r['hecke_ring_cyclotomic_generator']==0 and r['hecke_ring_power_basis'] is False
# Polynomial and rational expressions are emitted literally; Singular evaluates them.
def polynomial(values):
 assert all(isinstance(v,int) for v in values)
 return '('+'+'.join(f'({v})*y^{i}' for i,v in enumerate(values))+')'
s='ring cofactorField37=0,(y,T),dp;\npoly fieldPolynomial37='+polynomial(r['field_poly'])+';\nideal heckeBasis37;matrix inverseBasis37[16][16];\n'
for i,(num,den) in enumerate(zip(r['hecke_ring_numerators'],r['hecke_ring_denominators']),1):s+=f'heckeBasis37[{i}]={polynomial(num)}/({den});\n'
for i,(num,den) in enumerate(zip(r['hecke_ring_inverse_numerators'],r['hecke_ring_inverse_denominators']),1):
 for j,v in enumerate(num,1):s+=f'inverseBasis37[{i},{j}]=({v})/({den});\n'
chars=dict(r['hecke_ring_character_values'])
for name,values in [('aOne',r['an'][0]),('aTwo',r['an'][1]),('aFour',r['an'][3]),('apTwo',r['ap'][0]),('chi45',chars[45]),('chi57',chars[57])]:
 assert len(values)==16;s+=f'matrix {name}Coordinates37[1][16];\n'
 for j,v in enumerate(values,1):s+=f'{name}Coordinates37[1,{j}]=({v});\n'
s+=(h/'COFACTOR37.sing').read_text();(h/'source-data/cofactor-input-template.sing').write_text(s);(h/'cofactor-donors.json').write_text(json.dumps({'url':'https://www.lmfdb.org/api/mf_hecke_nf/?hecke_orbit_code=i4504355575169101&_format=json','sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'schema_checked':'LMFDB describe_table mf_hecke_nf, Hecke-basis and inverse-basis semantics'},indent=2)+'\n');print('Serialized coefficient-field basis, inverse matrix, Hecke coefficients and character values literally.')
