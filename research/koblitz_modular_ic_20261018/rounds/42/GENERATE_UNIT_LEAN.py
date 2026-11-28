#!/usr/bin/env python3
"""Literal certificate-to-Lean assembly; no mathematical computation."""
from pathlib import Path
import json
h=Path(__file__).resolve().parent
certs=json.loads((h/'runs/residual42-01/output/unit-certificates.json').read_text())
p=h/'lean/CMRecovery42.lean';base=p.read_text();assert 'namespace CMRecovery42Data' not in base
rows=[];mapping=[]
for i,c in enumerate(certs,1):
 name=f'unit_{i:03d}'
 literal=lambda k:'('+c[k]+' : ℤ)'
 rows.append(f'theorem {name} : {literal("gap")}*{literal("inverse")}+{literal("modulus")}*{literal("bezout_k")}=1 := by decide')
 mapping.append({'declaration':'CMRecovery42Data.'+name,'native_label':c['label'],'source':'runs/residual42-01/output/unit-certificates.json','scope':'exact integer identity; not field/geometry interpretation'})
p.write_text(base+'\nnamespace CMRecovery42Data\n'+'\n\n'.join(rows)+'\nend CMRecovery42Data\n')
(h/'unit-proof-map.json').write_text(json.dumps(mapping,indent=2)+'\n')
print('Assembled',len(certs),'literal integer certificate theorems.')
