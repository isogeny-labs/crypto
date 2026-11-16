#!/usr/bin/env python3
"""Filter native cleared polynomials, no math in Python."""
from pathlib import Path
h=Path(__file__).resolve().parent
lines=(h/'runs/local33-generate01/local-certificate.sing').read_text().splitlines()
selected=[s for s in lines if s.startswith(('localSource33[','localDenominator33=','localTateFactor33='))]
assert len(selected)==6
s='ring paddingPolynomial33=2,(n,l,v,u),dp;ideal localSource33;poly localDenominator33;poly localTateFactor33;\n'+'\n'.join(selected)+'\n'
s+='matrix fullJacPT33[4][4];int rowPreparePT33;int colPreparePT33;for(rowPreparePT33=1;rowPreparePT33<=4;rowPreparePT33++){for(colPreparePT33=1;colPreparePT33<=4;colPreparePT33++){fullJacPT33[rowPreparePT33,colPreparePT33]=diff(localSource33[rowPreparePT33],var(colPreparePT33));}}\n'
s+='ring paddingTotalField33=(2,a),(T,w),dp;minpoly=a^5+a^2+1;\n'+(h/'runs/pad33-01/padding-representatives.sing').read_text()+'\n'+(h/'PADDING33-TANGENT.sing').read_text()
(h/'source-data/padding-tangent-input-template.sing').write_text(s)
print('Assembled actual full four-variable padding Jacobian diagnostic.')
