#!/usr/bin/env python3
"""Text assembly only: retain integer modular coefficients for native CAS reduction."""
from pathlib import Path
import re,hashlib
here=Path(__file__).resolve().parent
text=(here/'source-data/phi_j_11.txt').read_text();terms=[];lines=text.splitlines();assert len(lines)==79
for line in lines:
 m=re.fullmatch(r'\[(\d+),(\d+)\]\s+(-?\d+)',line.strip());assert m
 i,j,c=m.groups();terms.append(f'({c})*jj30^{i}*X^{j}')
 if i!=j:terms.append(f'({c})*jj30^{j}*X^{i}')
proc='proc phi30(number jj30){return(poly('+'+'.join(terms)+'));}\n'
for degree,path in [(284,here/'source-data/paired-284.sing'),(83,here/'source-data/paired-83.sing')]:
 data=path.read_text().replace('(X),lp;','(X,Z),lp;')
 output=data+f'\nint sourceDegree30={degree};\n'+proc+(here/'COLOR31.sing').read_text()
 (here/f'source-data/color-{degree}-input-template.sing').write_text(output)
print('Assembled exact selected-pair inputs for degree284 and83; integer coefficients reduced only by Singular.')
