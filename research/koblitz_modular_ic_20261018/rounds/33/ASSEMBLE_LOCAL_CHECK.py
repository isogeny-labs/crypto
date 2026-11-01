#!/usr/bin/env python3
"""Literal accepted-certificate and point assembly only."""
from pathlib import Path
import re,hashlib
h=Path(__file__).resolve().parent;g=h/'runs/local33-generate01'
log=(g/'native.log').read_text();assert 'LOCAL33_GENERATE_ALL_CHECKS_PASS' in log and not re.search(r'^\s*\?',log,re.M)
assert (g/'exit.txt').read_text().strip()=='0'
cert=(g/'local-certificate.sing').read_text()+'\n'
cert+='matrix localJacobian33[4][3];int prepareRow33;int prepareCol33;\n'
cert+='for(prepareRow33=1;prepareRow33<=4;prepareRow33++){for(prepareCol33=1;prepareCol33<=3;prepareCol33++){localJacobian33[prepareRow33,prepareCol33]=diff(localSource33[prepareRow33],var(prepareCol33));}}\n'
body=(h/'LOCAL33-CHECK.sing').read_text()
out=cert+(h/'source-data/paired-83.sing').read_text().replace('(X),lp;','(Z,T,w),lp;')+'\n'+body+'\n'
lines=(h/'source-data/accepted-reverse83.log').read_text().splitlines();count=0
for i,line in enumerate(lines):
 m=re.fullmatch(r'REVERSE31_POINT root=(\d+) twist=1 split=1',line)
 if not m:continue
 root=m[1];vals=[]
 for off,label in ((1,'L'),(3,'b'),(5,'c'),(7,'lambda'),(9,'nu')):
  assert lines[i+off]=='REVERSE31_'+label;vals.append(lines[i+off+1])
 L,b,c,l,n=vals
 out+=f'poly LP{root}={L};poly NP{root}=subst(poly({n}),T,0);list FP{root}=factorize(LP{root});int IP{root};int CP{root};\n'
 out+=f'for(IP{root}=1;IP{root}<=size(FP{root}[1]);IP{root}++){{if(deg(FP{root}[1][IP{root}])==1){{number RP{root}=number(subst(FP{root}[1][IP{root}],Z,0)/leadcoef(FP{root}[1][IP{root}]));\n'
 out+=f'checkLocal33("r{root}-f"+string(IP{root}),{b},{c},RP{root},number(subst(NP{root},Z,RP{root})));CP{root}++;}}}}if(CP{root}!=2){{ERROR("LOCAL33_ORIENTATION_COUNT");}}\n';count+=1
assert count==5
out+='if(casesLocal33!=10){ERROR("LOCAL33_CHECK_COUNT");}print("LOCAL33_CHECK83_ALL_CHECKS_PASS");quit;\n'
(h/'source-data/local-check83-input-template.sing').write_text(out)
p=h/'runs/pad33-01';plog=(p/'native.log').read_text();assert 'PAD33_ALL_CHECKS_PASS' in plog and not re.search(r'^\s*\?',plog,re.M)
out=cert+'ring localField5=(2,a),(T,w),dp;minpoly=a^5+a^2+1;\n'+(p/'padding-representatives.sing').read_text()+'\n'+body+'\n'
out+='int QI33;list QP33;for(QI33=1;QI33<=size(paddingRepresentatives33);QI33++){QP33=paddingRepresentatives33[QI33];checkLocal33("q"+string(QI33),QP33[1],QP33[2],QP33[3],QP33[4]);}\n'
out+='if(casesLocal33!=2){ERROR("LOCAL33_PADDING_POINT_COUNT");}print("LOCAL33_CHECK5_ALL_CHECKS_PASS");quit;\n'
(h/'source-data/local-check5-input-template.sing').write_text(out)
print('Assembled exact local-presentation checks at all ten degree83 points and both degree5 representatives.')
