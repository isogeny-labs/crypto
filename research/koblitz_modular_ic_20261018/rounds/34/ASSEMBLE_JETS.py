#!/usr/bin/env python3
"""Literal accepted-data/source assembly only."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent
p=h/'runs/qcolonpoint34-01';log=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and 'QCOLONPOINT34_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
assert len(re.findall(r'^QCOLONPOINT34_POINT q=[12] products=148 presentation=1 all_syzygy_values_zero=0$',log,re.M))==2
old=h.parent/'33'
import hashlib
manifest={n:d for d,n in (line.split(maxsplit=1) for line in (old/'runs.sha256').read_text().splitlines())}
for name in ['source-data/basis-functions.sing','runs/desc33-02/twist-basis-functions.sing','runs/desc33-02/products2.sing','runs/pad33-01/padding-representatives.sing']:
 assert hashlib.sha256((old/name).read_bytes()).hexdigest()==manifest[name]
s=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])
s+='\nideal originalLambdaDerivatives33;ideal originalNuDerivatives33;int derIndex33;for(derIndex33=1;derIndex33<=5;derIndex33++){originalLambdaDerivatives33[derIndex33]=diff(original[derIndex33],l);originalNuDerivatives33[derIndex33]=diff(original[derIndex33],n);}\n'
s+=(old/'source-data/basis-functions.sing').read_text()+'\n'+(old/'runs/desc33-02/twist-basis-functions.sing').read_text()+'\n'
s+='ring jetField33=(2,a),(z,T,w),dp;minpoly=a^5+a^2+1;\n'
s+=(old/'runs/pad33-01/padding-representatives.sing').read_text()+'\nintmat sectionPairs2[196][2];\n'
pairs=[x for x in (old/'runs/desc33-02/products2.sing').read_text().splitlines() if x.startswith('sectionPairs2[')];assert len(pairs)==196
s+='\n'.join(pairs)+'\n'+(h/'JETS34.sing').read_text()
(h/'source-data/jets-input-template.sing').write_text(s)
print('Assembled fixed first padding point, nine complete source jets and196 descended section evaluations.')
