"""Independent exact replay for lifted4D pools; no reduction reused."""
from research.curved_blocks_20261109_v6.worker import curve
from research.seven_feature_blocks_20261109_v5.worker import bounds
from itertools import permutations
from math import prod
def determinant(rows):
 assert len(rows)==4 and all(len(row)==4 for row in rows)
 total=0
 for q in permutations(range(4)):
  sign=(-1)**sum(q[i]>q[j] for i in range(4) for j in range(i+1,4))
  total+=sign*prod(rows[i][q[i]] for i in range(4))
 return total

def audit(c,row):
 p,M,H=c['p'],c['M'],c['H'];a,b=c['coefficients'];lo,hi=bounds(M,H);mask=[False]*(H+1);coarse=False;checks=tighter=0;widthsum=coarsesum=0
 assert len(row['candidates'])==16
 for r in row['candidates']:
  v=list(map(int,r['vector']));assert len(v)==7
  for i in range(5):assert (v[i]-v[5]*a[i]-v[6]*b[i])%p==0
  cc=(v[5]*a[5]+v[6]*b[5])%p
  if 2*cc>p:cc-=p
  assert int(r['constant'])==cc
  L=R=cc
  for i in range(7):
   z=v[i]if i<5 else -v[i];L+=min(z*lo[i],z*hi[i]);R+=max(z*lo[i],z*hi[i])
  assert (L,R)==(int(r['L']),int(r['R']));closed=-((-L)//p)>R//p;assert closed==r['coarse'];coarse|=closed;cmask=[];ws=wm=nt=0
  for u in range(H+1):
   l,h=curve(v,cc,M,u);w=h-l;assert w<=R-L;ws+=w;wm=max(wm,w);nt+=w<R-L;excluded=-((-l)//p)>h//p;cmask.append(excluded);mask[u]|=excluded;checks+=1
  assert r['curved_mask']==''.join('1'if v else'0'for v in cmask)
  assert (ws,wm,nt)==(int(r['curved_width_sum']),int(r['curved_max_width']),r['tighter_u']);widthsum+=ws;coarsesum+=(R-L)*(H+1);tighter+=nt
 assert row['coarse_closed']==coarse and row['curved_closed']==all(mask) and row['curved_mask']==''.join('1'if v else'0'for v in mask)
 assert not coarse or all(mask)
 if c['positive']:assert not mask[c['known_u']] and not all(mask) and not coarse
 return dict(candidate_u_checks=checks,tighter_candidate_u_ranges=tighter,curved_width_sum=str(widthsum),coarse_width_sum=str(coarsesum),width_sum_ratio=str(widthsum)+'/'+str(coarsesum),excluded_u=sum(mask),unknown_u=len(mask)-sum(mask),curved_mask=row['curved_mask'])

def seed_audit(c,row):
 p=c['p'];a,b=c['coefficients'];u=c['H']//2;assert int(row['pivot'])==u
 t1=(-2*a[0]*u*u-a[1]*u+a[3])%p;b1=(-2*b[0]*u*u-b[1]*u+b[3])%p
 seeds=[list(map(int,r['seed'])) for r in row['candidates']];assert len(seeds)==16
 basis=seeds[:4];assert abs(determinant(basis))==p*p
 expected=list(basis)
 for i in range(4):
  for j in range(i+1,4):
   for sign in [-1,1]:expected.append([x+sign*y for x,y in zip(basis[i],basis[j])])
 assert expected==seeds
 def center(x):
  x%=p
  return x-p if 2*x>p else x
 for seed,r in zip(seeds,row['candidates']):
  assert (seed[0]-seed[2]*t1-seed[3]*b1)%p==0
  assert (seed[1]-seed[2]*a[0]-seed[3]*b[0])%p==0
  g,d=center(seed[2]),center(seed[3]);want=[center(g*x+d*y) for x,y in zip(a[:5],b[:5])]+[g,d]
  assert list(map(int,r['vector']))==want
 return audit(c,row)
