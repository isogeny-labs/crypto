#!/usr/bin/env python3
"""Literal accepted-data/source assembly only."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent
for attempt,marker in [('desc33-02','DESC33_ALL_CHECKS_PASS'),('pad33-01','PAD33_ALL_CHECKS_PASS'),('local33-check5-01','LOCAL33_CHECK5_ALL_CHECKS_PASS')]:
 p=h/'runs'/attempt;s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and marker in s and not re.search(r'^\s*\?',s,re.M)
s=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])
s+='\nideal originalLambdaDerivatives33;ideal originalNuDerivatives33;int derIndex33;for(derIndex33=1;derIndex33<=5;derIndex33++){originalLambdaDerivatives33[derIndex33]=diff(original[derIndex33],l);originalNuDerivatives33[derIndex33]=diff(original[derIndex33],n);}\n'
s+=(h/'source-data/basis-functions.sing').read_text()+'\n'+(h/'runs/desc33-02/twist-basis-functions.sing').read_text()+'\n'
s+='ring jetField33=(2,a),(z,T,w),dp;minpoly=a^5+a^2+1;\n'
s+=(h/'runs/pad33-01/padding-representatives.sing').read_text()+'\nintmat sectionPairs2[196][2];\n'
pairs=[x for x in (h/'runs/desc33-02/products2.sing').read_text().splitlines() if x.startswith('sectionPairs2[')];assert len(pairs)==196
s+='\n'.join(pairs)+'\n'+(h/'JETS33.sing').read_text()
(h/'source-data/jets-input-template.sing').write_text(s)
print('Assembled fixed first padding point, nine complete source jets and196 descended section evaluations.')
