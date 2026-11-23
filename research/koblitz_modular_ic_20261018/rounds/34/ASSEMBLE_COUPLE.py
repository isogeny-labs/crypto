#!/usr/bin/env python3
"""Literal binary evaluation columns only, not field arithmetic."""
from pathlib import Path
import re,hashlib,json
h=Path(__file__).resolve().parent;old=h.parent/'33';p=h/'runs/jets34-01';log=(p/'native.log').read_text()
assert (p/'exit.txt').read_text().strip()=='0' and 'JETS34_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
q=[line for line in (p/'padding-jets.sing').read_text().splitlines() if line.startswith('paddingJetBits33[')];assert len(q)==196
s='ring coupledField34=2,(w),dp;ideal paddingJetBits33;ideal currentPointBits34;\n'+'\n'.join(q)+'\n'+(h/'COUPLE34.sing').read_text()+'\n'
manifest={n:d for d,n in (line.split(maxsplit=1) for line in (old/'runs.sha256').read_text().splitlines())};pins=[]
for root in range(1,6):
 for factor in (2,3):
  name=f'runs/eval33-01/point-r{root}-f{factor}-2D.sing';data=(old/name).read_bytes();assert hashlib.sha256(data).hexdigest()==manifest[name]
  lines=[line.replace('evaluationBits[','currentPointBits34[') for line in data.decode().splitlines() if line.startswith('evaluationBits[')];assert len(lines)==196
  s+='currentPointBits34=0;\n'+'\n'.join(lines)+f'\ncouple34("r{root}-f{factor}",currentPointBits34);\n';pins.append({'donor':name,'sha256':manifest[name]})
s+='print("COUPLE34_ALL_CHECKS_PASS");quit;\n';(h/'source-data/coupled-input-template.sing').write_text(s);(h/'coupled-donor-verification.json').write_text(json.dumps(pins,indent=2)+'\n')
print('Assembled fixed ten83-point and complete45-jet constraints in the same196section basis.')
