from pathlib import Path
import importlib.util,json
root=Path(__file__).resolve().parent
s=importlib.util.spec_from_file_location('ec18',root.parent/'round18-p192/cached_orbit.py');ec=importlib.util.module_from_spec(s);s.loader.exec_module(ec)
c=json.loads((root/'certificate.json').read_text());r=ec.r
def evaluate(T,selected,cache=None):
 count={'D':0,'A':0,'N':0}
 if T==[None,None]:return T,count
 w=selected['width'];m=2**(w-2);tables=[];reused=0
 if cache is None:cache={}
 for P in T:
  if P in cache:tables.append(cache[P]);reused+=1;continue
  table=[P]
  if m>1:
   twice=ec.add(P,P);count['D']+=1
   for _ in range(1,m):table.append(ec.add(table[-1],twice));count['A']+=1
  tables.append(table);cache[P]=table
 out=[]
 for row in selected['digits']:
  L=max(map(len,row));R=None
  for i in range(L-1,-1,-1):
   if R is not None:R=ec.add(R,R);count['D']+=1
   for j,digits in enumerate(row):
    d=digits[i] if i<len(digits) else 0
    if not d:continue
    P=tables[j][(abs(d)-1)//2]
    if d<0:P=ec.neg(P);count['N']+=1
    if R is None:R=P
    else:R=ec.add(R,P);count['A']+=1
  out.append(R)
 assert count=={'D':selected['D']-reused*int(m>1),'A':selected['A']-reused*(m-1),'N':selected['negative_lookups']}
 return out,count
def total(Ps):
 R=Ps[0]
 for P in Ps[1:]:R=ec.add(R,P)
 return ec.neg(R)
fixtures=[]
for row in c['rows']:
 lam=row['eigenvalue']
 for m in row['matrices']:
  for k in [0,1,2,17,20261107]:
   P=ec.mul(k,ec.G);T=[ec.mul(v,P) for v in m['eigenvector']];states=[T];count={'D':0,'A':0,'N':0}
   times=2 if m['kind']=='companion' else 3;cache={}
   for _ in range(times):
    T,cc=evaluate(T,m['selected'],cache);states.append(T)
    for key in count:count[key]+=cc[key]
   if m['kind']=='companion':
    Rs=[states[0][0],states[0][1],states[1][1],states[2][1]];Rs.append(total(Rs))
    orbit=[[Rs[j],Rs[(j+1)%5]] for j in range(5)]
    if k:count['A']+=3;count['N']+=1
   else:
    orbit=states+[[total([S[j] for S in states]) for j in range(2)]]
    if k:count['A']+=6;count['N']+=2
   assert orbit==[[ec.mul(pow(lam,j,r),V) for V in orbit[0]] for j in range(5)]
   assert evaluate(orbit[-1],m['selected'])[0]==orbit[0]
   if k:
    j=min(range(5),key=lambda z:orbit[z][0][0]);sign=1 if orbit[j][0][1]<ec.p-orbit[j][0][1] else -1
    rep=orbit[j] if sign==1 else list(map(ec.neg,orbit[j]));count['N']+=2*int(sign==-1)
    assert count['D']==m['canonicalization_D'] and count['A']==m['canonicalization_A']
   else:j,sign,rep=0,1,orbit[0]
   fixtures.append({'power':row['power'],'kind':m['kind'],'scalar':k,'orbit':orbit,'rotation':j,'sign':sign,'representative':rep,'canonicalization_count':count})
out={'all_checks_pass':True,'fixtures':fixtures,'point_fixture_count':len(fixtures),'scope':'Actual wNAF matrix actions and five-vector reconstruction; setup of eigenvector images excluded and explicit'}
(root/'point-certificate.json').write_text(json.dumps(out,indent=2)+'\n');print('MARKED_MATRIX_POINT_AND_CANONICALIZATION_PASS',len(fixtures))
