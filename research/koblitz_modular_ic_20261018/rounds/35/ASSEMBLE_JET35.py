#!/usr/bin/env python3
"""Lexical assembly and donor-byte verification only; no mathematical computation."""
from pathlib import Path
import hashlib,json,re
h=Path(__file__).resolve().parent
old=h.parent/'33';prior=h.parent/'34'
manifest={n:d for d,n in (line.split(maxsplit=1) for line in (old/'runs.sha256').read_text().splitlines())}
names=['source-data/basis-functions.sing','runs/desc33-02/twist-basis-functions.sing','runs/desc33-02/products2.sing','runs/desc33-02/products3.sing','runs/pad33-01/padding-representatives.sing']
verified={}
for n in names:
 d=hashlib.sha256((old/n).read_bytes()).hexdigest();assert d==manifest[n];verified['R33/'+n]=d
manifest34={n:d for d,n in (line.split(maxsplit=1) for line in (prior/'runs.sha256').read_text().splitlines())}
n='runs/jets34-01/padding-jets.sing';d=hashlib.sha256((prior/n).read_bytes()).hexdigest();assert d==manifest34[n];verified['R34/'+n]=d
s=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])+'\n'+(h/'ENCODE35.sing').read_text()+'\n'
s+=(old/names[0]).read_text()+'\nlist oldEncoded35;for(encIndex35=1;encIndex35<=68;encIndex35++){oldEncoded35[encIndex35]=encode35(basisNumerators[encIndex35]);}\n'
s+=(old/names[1]).read_text()+'\nlist newEncoded35;for(encIndex35=1;encIndex35<=68;encIndex35++){newEncoded35[encIndex35]=encode35(twistNumerators33[encIndex35]);}\n'
s+='ring jetField35=(2,a),(z,T,w),dp;minpoly=a^5+a^2+1;\n'+(old/names[4]).read_text()+'\n'
for degree,count in [(2,196),(3,324)]:
 rows=[x for x in (old/f'runs/desc33-02/products{degree}.sing').read_text().splitlines() if x.startswith(f'sectionPairs{degree}[')];assert len(rows)==count
 s+=f'intmat sectionPairs{degree}[{count}][2];\n'+'\n'.join(rows)+'\n'
s+=(prior/n).read_text()+'\n'+(h/'JET35.sing').read_text()
(h/'source-data/jet-input-template.sing').write_text(s)
(h/'jet-donors.json').write_text(json.dumps(verified,indent=2)+'\n')
print('Verified six donors and assembled precision154 source; all field computation remains native.')
