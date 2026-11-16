#!/usr/bin/env python3
"""Literal native-object filtering and lexical renaming only."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent
lines=(h/'runs/local33-compact-dump01/rational-family.sing').read_text().splitlines()
selected=[s for s in lines if s.startswith(('rationalSource33[','rationalJacobian33[','rationalH33='))]
assert len(selected)==17
header='ideal rationalSource33;matrix rationalJacobian33[4][3];poly rationalH33;\n'
rename={'u':'uD33','n':'nD33','l':'lD33','v':'vD33'}
func='proc smallFamily33(number uD33,poly nD33,poly lD33,poly vD33){\n'+header
func+='\n'.join(re.sub(r'\b(u|n|l|v)\b',lambda m:rename[m[0]],s) for s in selected)
func+='\nreturn(list(rationalSource33,rationalJacobian33,rationalH33));}\n'
s=lines[0]+'\n'+header+'\n'.join(selected)+'\n'+func
s+='list smallRT33=smallFamily33(u,n,l,v);ideal smallJ33=smallRT33[1];matrix smallJac33=smallRT33[2];poly smallH33=smallRT33[3];\n'
s+='if(matrix(smallJ33)!=matrix(rationalSource33) || smallJac33!=rationalJacobian33 || smallH33!=rationalH33){ERROR("LOCAL33_SMALL_ROUNDTRIP");}\n'
s+='ideal smallG33=std(smallJ33);matrix smallAB33=lift(smallJ33,smallG33);matrix smallBA33=lift(smallG33,smallJ33);\n'
s+='if(matrix(smallJ33)*smallAB33!=matrix(smallG33) || matrix(smallG33)*smallBA33!=matrix(smallJ33)){ERROR("LOCAL33_SMALL_GB_LIFTS");}\n'
s+='print("LOCAL33_GENERIC_DIAGNOSTIC vdim="+string(vdim(smallG33))+" basis_count="+string(size(smallG33)));\n'
s+='ring smallField33=(2,a),(T,w),dp;minpoly=a^5+a^2+1;\n'+(h/'runs/pad33-01/padding-representatives.sing').read_text()+'\n'
s+=(h/'LOCAL33-DIAGNOSTIC.sing').read_text()
(h/'source-data/local-diagnostic-input-template.sing').write_text(s)
print('Assembled exact small source/Jacobian diagnostic and generic lift checks.')
