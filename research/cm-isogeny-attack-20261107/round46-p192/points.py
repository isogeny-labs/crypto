from pathlib import Path
import importlib.util,json
root=Path(__file__).resolve().parent;s=importlib.util.spec_from_file_location('ec18',root.parent/'round18-p192/cached_orbit.py');ec=importlib.util.module_from_spec(s);s.loader.exec_module(ec)
c=json.loads((root/'certificate.json').read_text());fixtures=[]
for row in c['rows']:
 sel=row['selected'];lam=row['lambda']
 for k in [0,1,2,17,20261107]:
  R=ec.mul(k,ec.G);Ps=[ec.mul(pow(lam,j,ec.r),R) for j in range(3)];counts={'D':0,'A':0,'N':0}
  if R is None:orbit=[None]*5;rep=[None]*3;rot,sign=0,1
  else:
   tables=[]
   for P,w in zip(Ps,sel['widths']):
    m=2**(w-2);tab=[P]
    if m>1:
     twice=ec.add(P,P);counts['D']+=1
     for _ in range(1,m):tab.append(ec.add(tab[-1],twice));counts['A']+=1
    tables.append(tab)
   X=None
   for bit in range(max(map(len,sel['digits']))-1,-1,-1):
    if X is not None:X=ec.add(X,X);counts['D']+=1
    for tab,ds in zip(tables,sel['digits']):
     d=ds[bit] if bit<len(ds) else 0
     if not d:continue
     P=tab[(abs(d)-1)//2]
     if d<0:P=ec.neg(P);counts['N']+=1
     if X is None:X=P
     else:X=ec.add(X,P);counts['A']+=1
   orbit=Ps+[X];total=orbit[0]
   for P in orbit[1:]:total=ec.add(total,P);counts['A']+=1
   orbit.append(ec.neg(total));counts['N']+=1
   assert orbit==[ec.mul(pow(lam,j,ec.r),R) for j in range(5)]
   rot=min(range(5),key=lambda j:orbit[j][0]);sign=1 if orbit[rot][1]<ec.p-orbit[rot][1] else -1
   rep=[orbit[(rot+j)%5] for j in range(3)]
   if sign==-1:rep=list(map(ec.neg,rep));counts['N']+=3
   assert counts['D']==sel['D'] and counts['A']==sel['A']
  fixtures.append({'power':row['power'],'scalar':k,'orbit':orbit,'representative':rep,'rotation':rot,'sign':sign,'counts':counts})
Path('point-certificate.json').write_text(json.dumps({'all_checks_pass':True,'fixtures':fixtures},indent=2)+'\n');print('TRIPLE_POINT_REPRESENTATIVES_PASS',len(fixtures))
