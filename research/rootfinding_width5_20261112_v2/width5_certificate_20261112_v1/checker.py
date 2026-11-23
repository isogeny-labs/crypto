"""Independent exact replay for width5 pools; no reduction reused."""
from research.curved_blocks_20261109_v6.worker import curve
from research.seven_feature_blocks_20261109_v5.worker import bounds
from itertools import permutations
from math import prod
def determinant(rows):
 assert len(rows)==5 and all(len(row)==5 for row in rows)
 total=0
 for q in permutations(range(5)):
  sign=(-1)**sum(q[i]>q[j] for i in range(5) for j in range(i+1,5))
  total+=sign*prod(rows[i][q[i]] for i in range(5))
 return total

def audit(c,row):
 p,M,H=c['p'],c['M'],c['H'];a,b=c['coefficients'];lo,hi=bounds(M,H);mask=[False]*(H+1);coarse=False;checks=tighter=0;widthsum=coarsesum=0
 assert len(row['candidates'])==25
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

def interval_excludes(L,R,p):
 return -((-L)//p)>R//p

def seed_audit(c,row):
 p,M,H=c['p'],c['M'],c['H'];a,b=c['coefficients'];A=3*M
 assert list(map(int,row['seed_weights']))==[max(1,A*A+2*A*H*H),max(1,A*H),max(1,A),max(1,3*M*M),max(1,M**3)]
 seeds=[list(map(int,r['seed']))for r in row['candidates']];assert len(seeds)==25
 basis=seeds[:5];assert abs(determinant(basis))==p**3
 wanted=list(basis)
 for i in range(5):
  for j in range(i+1,5):
   for sign in [-1,1]:wanted.append([x+sign*y for x,y in zip(basis[i],basis[j])])
 assert wanted==seeds
 def center(x):
  x%=p
  return x-p if 2*x>p else x
 for seed,r in zip(seeds,row['candidates']):
  assert len(seed)==5
  for j,k in enumerate([0,1,3]):assert(seed[j]-seed[3]*a[k]-seed[4]*b[k])%p==0
  g,d=center(seed[3]),center(seed[4]);want=[seed[0],seed[1],center(g*a[2]+d*b[2]),seed[2],center(g*a[4]+d*b[4]),g,d]
  assert list(map(int,r['vector']))==want
  cc=center(g*a[5]+d*b[5]);assert int(r['constant'])==cc
  assert len(r['phases'])==H+1
  for u,phase in enumerate(map(int,r['phases'])):
   fixedphase=center(g*(a[0]*u**4+a[1]*u**3+a[2]*u*u+a[4]*u+a[5])+d*(b[0]*u**4+b[1]*u**3+b[2]*u*u+b[4]*u+b[5]))
   C=want[0]*u**4+want[1]*u**3+want[2]*u*u+want[4]*u+cc
   assert phase==fixedphase==center(C)
   L,R=curve(want,cc,M,u);shift=phase-C;assert shift%p==0
   assert interval_excludes(L,R,p)==interval_excludes(L+shift,R+shift,p)
 return audit(c,row)
