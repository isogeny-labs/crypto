from pathlib import Path
import json
c=json.loads(Path('certificate.json').read_text());q=json.loads(Path('point-certificate.json').read_text());r=c['r'];p=2**192-2**64-1
G=[int('188DA80EB03090F67CBF20EB43A18800F4FF0AFD82FF1012',16),int('07192B95FFC8DA78631011ED6B24CDD573F977A11E794811',16)]
def add(P,Q):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u and (y+v)%p==0:return None
 s=((3*x*x-3)*pow(2*y,-1,p) if P==Q else (v-y)*pow(u-x,-1,p))%p
 z=(s*s-x-u)%p;return [z,(s*(x-z)-y)%p]
def mul(n,P):
 n%=r;out=None
 while n:
  if n&1:out=add(out,P)
  P=add(P,P);n//=2
 return out
checks=controls=0
for f in q['fixtures']:
 row=next(z for z in c['rows'] if z['power']==f['power']);lam=row['lambda'];P=mul(f['scalar'],G)
 orbit=[mul(pow(lam,j,r),P) for j in range(5)]
 assert orbit==f['orbit'];checks+=5
 if P is None:
  assert f['representative']==[None]*3 and all(v==0 for v in f['counts'].values());continue
 for offset in range(5):
  for sign in [1,-1]:
   shifted=orbit[offset:]+orbit[:offset]
   if sign<0:shifted=[[x,-y%p] for x,y in shifted]
   j=min(range(5),key=lambda k:shifted[k][0]);rep=[shifted[(j+i)%5] for i in range(3)]
   if rep[0][1]>p-rep[0][1]:rep=[[x,-y%p] for x,y in rep]
   assert rep==f['representative'];controls+=1
 assert mul(f['sign']*pow(lam,f['rotation'],r)*f['scalar'],G)==f['representative'][0]
 assert f['counts']['D']==row['selected']['D'] and f['counts']['A']==row['selected']['A']
out={'all_checks_pass':True,'independent_point_images':checks,'equivalent_encoding_controls':controls,'fixtures':len(q['fixtures']),'producer_imports':False}
Path('point-verification.json').write_text(json.dumps(out,indent=2)+'\n');print(json.dumps(out,indent=2))
