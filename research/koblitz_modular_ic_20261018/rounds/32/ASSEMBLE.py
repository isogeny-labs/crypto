#!/usr/bin/env python3
"""Literal text assembly only; differentiation and all arithmetic in Singular."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent;old=h.parent/'31';(h/'source-data').mkdir(exist_ok=True)
source=''.join((h.parent/'27/LAMBDA14.sing').read_text().splitlines(True)[:254])
source+='\nmatrix jacobian32[5][4];int ri32;int ci32;\n'
source+='for(ri32=1;ri32<=5;ri32++){for(ci32=1;ci32<=4;ci32++){jacobian32[ri32,ci32]=diff(original[ri32],var(ci32));}}\n'
for d in (83,284):
 out=source+'\n'+(old/f'source-data/paired-{d}.sing').read_text().replace('(X),lp;','(X,Z,T),lp;')+'\n'+(h/'JAC32.sing').read_text()+'\n'
 lines=(old/f'runs/reverse{d}-01/native.log').read_text().splitlines();assert 'REVERSE31_ALL_CHECKS_PASS' in lines
 count=0
 for i,line in enumerate(lines):
  m=re.fullmatch(r'REVERSE31_POINT root=(\d+) twist=(\d+) split=(\d+)',line)
  if not m:continue
  root,twist,_=m.groups();values=[]
  for off,label in ((1,'L'),(3,'b'),(5,'c'),(7,'lambda'),(9,'nu')):
   assert lines[i+off]=='REVERSE31_'+label;values.append(lines[i+off+1])
  L,b,c,l,n=values;out+=f'checkJac32({root},{twist},{L},{b},{c},{l},{n});\n';count+=1
 assert count==10
 out+='print("JAC32_ALL_CHECKS_PASS");quit;\n';(h/f'source-data/jac-{d}.sing').write_text(out)
print('Assembled all20 literal physical point algebras for native derivative/minor certificates.')
