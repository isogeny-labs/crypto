#!/usr/bin/env python3
"""Only literal text extraction and assembly. Native Singular does arithmetic."""
from pathlib import Path
import re
h=Path(__file__).resolve().parent
for d in (83,284):
    paired=(h/f'source-data/paired-{d}.sing').read_text().replace('(X),lp;','(X,Z,T),lp;')
    lines=(h/f'runs/reverse{d}-01/native.log').read_text().splitlines()
    assert 'REVERSE31_ALL_CHECKS_PASS' in lines
    out=paired+'\n'+(h/'FROB31.sing').read_text()+'\n'
    count=0
    for i,line in enumerate(lines):
        m=re.fullmatch(r'REVERSE31_POINT root=(\d+) twist=(\d+) split=(\d+)',line)
        if not m:continue
        root,twist,_=m.groups(); values=[]
        for offset,label in ((1,'L'),(3,'b'),(5,'c'),(7,'lambda'),(9,'nu')):
            assert lines[i+offset]=='REVERSE31_'+label
            values.append(lines[i+offset+1])
        L,b,c,l,n=values
        # Evaluate the recorded physical AS coordinates at T=0: descent coordinates.
        out+=f'closedF31({root},{twist},{L},{b},{c},subst(poly({l}),T,0),subst(poly({n}),T,0),{d});\n'
        count+=1
    assert count==10
    out+='print("FROB31_ALL_CHECKS_PASS");quit;\n'
    (h/f'source-data/frob-{d}-input-template.sing').write_text(out)
print('Assembled all ten accepted point records per field, retaining all quadratic factors.')
