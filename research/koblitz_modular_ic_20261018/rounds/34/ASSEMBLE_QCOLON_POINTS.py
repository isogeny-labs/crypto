#!/usr/bin/env python3
"""Literal source-bound colon candidates in the fixed q-fiber driver."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;p=h/'runs/qcolon34-01';s=(p/'native.log').read_text();assert (p/'exit.txt').read_text().strip()=='0' and 'QCOLON34_ALL_CHECKS_PASS' in s and not re.search(r'^\s*\?',s,re.M)
lines=(p/'qcolon-family.sing').read_text().splitlines();match=re.search(r'colonTotalColumns34=([0-9]+)',lines[0]);assert match;count=match[1]
cols=[line.replace('saturatedS34[','primitiveS34[') for line in lines if line.startswith('saturatedS34[')]
old=(h/'source-data/qpoly-input-template.sing').read_text();start=old.index('ring paddingPolynomial34=');prefix=old[:start];tail=old[start:]
tail='\n'.join(line for line in tail.splitlines() if not line.startswith('primitiveS34['))+'\n'
header='ring paddingPolynomial34=2,(n,l,v,u),dp;matrix primitiveS34[4][17];ideal localSource33;poly localDenominator33;poly localTateFactor33;'
assert header in tail;tail=tail.replace(header,header+'\n'+'\n'.join(cols))
tail=tail.replace('[4][17]',f'[4][{count}]').replace('[17][3]',f'[{count}][3]').replace('colQP34<=17',f'colQP34<={count}').replace('jQP34<=17',f'jQP34<={count}')
tail=tail.replace('countQP34!=68',f'countQP34!=4*{count}').replace('rowQP34<=68',f'rowQP34<=4*{count}').replace(' products=68',f' products="+string(4*{count})+"')
tail=tail.replace('if(recastS34!=rationalSyzygies33){ERROR("QPOLY34_POLYNOMIAL_RECAST");}', f'matrix reloadedColonZero34[1][{count}];if(matrix(rationalSource33)*recastS34!=reloadedColonZero34){{ERROR("QCOLONPOINT34_SOURCE_IDENTITY");}}')
tail=tail.replace('if(reduce(transpose(sQP34)*jacQP34', '''matrix oldControl34[4][17];matrix oldZero34[4][17];int oldRow34;int oldCol34;
 for(oldRow34=1;oldRow34<=4;oldRow34++){for(oldCol34=1;oldCol34<=17;oldCol34++){oldControl34[oldRow34,oldCol34]=sQP34[oldRow34,oldCol34];}}
 if(oldControl34!=oldZero34){ERROR("QCOLONPOINT34_NEGATIVE_CONTROL_MOVED");}
 if(reduce(transpose(sQP34)*jacQP34''')
tail=tail.replace('QPOLY34_POINT','QCOLONPOINT34_POINT').replace('QPOLY34_ALL_CHECKS_PASS','QCOLONPOINT34_ALL_CHECKS_PASS')
(h/'source-data/qcolon-points-input-template.sing').write_text(prefix+tail)
print('Assembled all native colon columns, source-identity readback and both fixed padding points.')
