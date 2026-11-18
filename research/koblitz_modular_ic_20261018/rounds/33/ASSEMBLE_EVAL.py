#!/usr/bin/env python3
"""Literal accepted-data extraction; native code evaluates and computes ranks."""
from pathlib import Path
import hashlib,re,json
h=Path(__file__).resolve().parent;d=h/'runs/desc33-02'
assert (d/'exit.txt').read_text().strip()=='0'
log=(d/'native.log').read_text();assert 'DESC33_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
pairlines=(d/'products2.sing').read_text().splitlines()
pairs=[z for z in pairlines if z.startswith('sectionPairs2[')];assert len(pairs)==196
pairtext='intmat sectionPairs2[196][2];\n'+'\n'.join(pairs)+'\n'
old=Path('/home/ec2-user/crypto-koblitz-r31/research/koblitz_modular_ic_20261018/rounds/31')
logpath=old/'runs/reverse83-01/native.log';data=logpath.read_bytes()
manifest={name:dig for dig,name in (line.split(maxsplit=1) for line in (old/'runs.sha256').read_text().splitlines())}
assert hashlib.sha256(data).hexdigest()==manifest['runs/reverse83-01/native.log']
(h/'source-data/accepted-reverse83.log').write_bytes(data)
lines=data.decode().splitlines();assert 'REVERSE31_ALL_CHECKS_PASS' in lines
out=(h/'source-data/basis-functions.sing').read_text()+'\n'+(d/'twist-basis-functions.sing').read_text()+'\n'
out+=(h/'source-data/paired-83.sing').read_text().replace('(X),lp;','(Z,T,w),lp;')+'\n'
out+=(h/'source-data/sigma-matrix.sing').read_text()+'\n'+pairtext+(h/'EVAL33.sing').read_text()+'\n'
count=0
for i,line in enumerate(lines):
 m=re.fullmatch(r'REVERSE31_POINT root=(\d+) twist=1 split=1',line)
 if not m:continue
 root=m[1];vals=[]
 for off,label in ((1,'L'),(3,'b'),(5,'c'),(7,'lambda'),(9,'nu')):
  assert lines[i+off]=='REVERSE31_'+label;vals.append(lines[i+off+1])
 L,b,c,l,n=vals
 out+=f'poly Lroot{root}={L};poly nroot{root}=subst(poly({n}),T,0);\n'
 out+=f'list facroot{root}=factorize(Lroot{root});int idxroot{root};int acceptedroot{root};\n'
 out+=f'for(idxroot{root}=1;idxroot{root}<=size(facroot{root}[1]);idxroot{root}++){{if(deg(facroot{root}[1][idxroot{root}])==1){{\n'
 out+=f'number lr{root}=number(subst(facroot{root}[1][idxroot{root}],Z,0)/leadcoef(facroot{root}[1][idxroot{root}]));\n'
 out+=f'evalOne33({root},idxroot{root},{b},{c},lr{root},number(subst(nroot{root},Z,lr{root})));acceptedroot{root}++;\n'
 out+=f'}}}}if(acceptedroot{root}!=2){{ERROR("EVAL33_ORIENTATION_FACTOR_COUNT");}}\n'
 count+=1
assert count==5
out+='if(evalCases33!=10){ERROR("EVAL33_POINT_COUNT");}print("EVAL33_ALL_CHECKS_PASS");quit;\n'
(h/'source-data/eval-input-template.sing').write_text(out)
print('Assembled every accepted degree83 twist factor and196 descended section pairs.')
