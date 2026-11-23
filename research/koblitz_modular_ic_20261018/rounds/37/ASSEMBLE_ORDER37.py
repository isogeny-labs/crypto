#!/usr/bin/env python3
"""Literal source assembly and donor hashes only. All field arithmetic is native."""
from pathlib import Path
import hashlib,re,json
h=Path(__file__).resolve().parent;old=h.parent/'33'
manifest={n:d for d,n in (line.split(maxsplit=1) for line in(old/'runs.sha256').read_text().splitlines())};pins={}
def donor(name):
 p=old/name;data=p.read_bytes();dig=hashlib.sha256(data).hexdigest();assert dig==manifest[name];pins['R33/'+name]=dig;return data.decode()
s=donor('runs/desc33-02/twist-basis-functions.sing')+'\n'+donor('source-data/paired-83.sing').replace('(X),lp;','(Z,T,w),lp;')+'\nlist savedEvaluations37;\n'
for root in range(1,6):
 for factor in (2,3):
  text=donor(f'runs/eval33-01/point-r{root}-f{factor}-D.sing');lines=[x for x in text.splitlines() if x.startswith('evaluationValues[')];assert len(lines)==68
  # Retain literal 68 named slots, including zeros.
  idx=(root-1)*2+(factor-1);s+=f'ideal savedSlot37_{idx};\n'
  for line in lines:s+=line.replace('evaluationValues',f'savedSlot37_{idx}')+'\n'
  s+=f'savedEvaluations37[{idx}]=savedSlot37_{idx};\n'
s+=(h/'ORDER37.sing').read_text()+'\n'
text=donor('source-data/accepted-reverse83.log');lines=text.splitlines();count=0
for i,line in enumerate(lines):
 m=re.fullmatch(r'REVERSE31_POINT root=(\d+) twist=1 split=1',line)
 if not m:continue
 root=m[1];values=[]
 for offset,label in ((1,'L'),(3,'b'),(5,'c'),(7,'lambda'),(9,'nu')):
  assert lines[i+offset]=='REVERSE31_'+label;values.append(lines[i+offset+1])
 L,b,c,l,n=values
 s+=f'poly polynomial37_{root}={L};poly nPoly37_{root}=subst(poly({n}),T,0);list factors37_{root}=factorize(polynomial37_{root});\n'
 s+=f'for(int factor37_{root}=2;factor37_{root}<=3;factor37_{root}++){{if(deg(factors37_{root}[1][factor37_{root}])!=1){{ERROR("ORDER37_FACTOR_DEGREE");}}number l37_{root}=number(subst(factors37_{root}[1][factor37_{root}],Z,0)/leadcoef(factors37_{root}[1][factor37_{root}]));store37({root},factor37_{root},{b},{c},l37_{root},number(subst(nPoly37_{root},Z,l37_{root})));}}\n'
 count+=1
assert count==5
s+=(h/'ORDER-END37.sing').read_text();(h/'source-data/order-input-template.sing').write_text(s);(h/'order-donors.json').write_text(json.dumps(pins,indent=2)+'\n');print('Assembled ten actual point labels with680 saved literal section values and donor hashes.')
