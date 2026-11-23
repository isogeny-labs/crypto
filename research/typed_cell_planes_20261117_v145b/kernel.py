from research.shifted_tangent_20261117_v144b.kernel import at
from research.odd_coprime_factor_20261116_v119c.kernel import inverse_block,initial
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic
from research.precision_filtration_20261117_v132b.kernel import actual_family,filtration,evaluate_family
from research.residue_quotient_20261116_v99b.kernel import basis,apply
from research.delayed_block_20261117_v137b.kernel import coefficients
def parallel_rank(A,full):
 A=[list(r)for r in A];h=len(A);w=len(A[0]);masks=[full]+[0]*h
 for col in range(w):
  previous=list(masks);masks=[0]*(h+1);masks[h]=previous[h]
  for r in range(h):
   active=previous[r];found=0
   for i in range(r,h):
    selected=active&A[i][col]&(full^found);found|=selected
    if i!=r:
     for j in range(w):delta=(A[r][j]^A[i][j])&selected;A[r][j]^=delta;A[i][j]^=delta
   for i in range(h):
    if i!=r:
     condition=found&A[i][col]
     for j in range(w):A[i][j]^=A[r][j]&condition
   masks[r]|=active&(full^found);masks[r+1]|=found
 assert sum(m.bit_count()for m in masks)==full.bit_count()
 return masks

def plane_rows(F,Ds,full,d=18,ell=16,n=23,trace=None):
 h=n-ell;f=Arithmetic(2,[0,1]);S,I=inverse_block(f);A=[[0],[0],[full]];B=[[full if a else 0]for a in initial(f)];m=d-1
 def transform(e):
  out=[]
  for row in I:
   a=0
   for j,b in enumerate(row):
    if b:a^=e[j]
   out.append(a)
  return out
 for k in range(1,h+1):
  e=[F[j][k]for j in range(5)]
  for i,a in enumerate(A):
   for j,b in enumerate(B):
    if i+j<5:
     for t in range(k+1):e[i+j]^=at(a,t)&at(b,k-t)
  c=transform(e)
  if trace is not None:trace.append(("primal",k,-1,list(c)))
  for V,degree,lo in [(A,2,0),(B,3,2)]:
   for j in range(degree):
    if k<=(degree-j)*m:V[j].append(c[lo+j])
 rows=[[0]*len(Ds)for _ in range(6)]
 for lane,D in enumerate(Ds):
  DA=[[]for _ in range(3)];DB=[[]for _ in range(4)]
  for s in range(h+1):
   e=[full if D[j]>>(ell+s)&1 else 0 for j in range(5)]
   for i in range(3):
    for j in range(4):
     if i+j<5:
      for r in range(s):e[i+j]^=at(DA[i],r)&at(B[j],s-r);e[i+j]^=at(A[i],s-r)&at(DB[j],r)
   dc=transform(e)
   if trace is not None:trace.append(("tangent",ell+s,lane,list(dc)))
   if ell+s>=18:rows[ell+s-18][lane]=dc[1]
   for V,degree,lo in [(DA,2,0),(DB,3,2)]:
    for j in range(degree):
     if ell+s<=(degree-j)*m:V[j].append(dc[lo+j])
 return rows

def column_rank(A,full):
 h=len(A);w=len(A[0]);B=[[0]*h for _ in range(h)];present=[0]*h
 for j in range(w):
  x=[A[i][j]for i in range(h)]
  for p in range(h-1,-1,-1):
   active=x[p];reduce=active&present[p];insert=active&(full^present[p])
   for i in range(h):x[i]^=B[p][i]&reduce
   for i in range(h):B[p][i]|=x[i]&insert
   present[p]|=insert
   for i in range(h):x[i]&=full^insert
 masks=[full]+[0]*h
 for flag in present:
  previous=masks;masks=[0]*(h+1)
  for r,m in enumerate(previous):
   masks[r]|=m&(full^flag)
   if r<h:masks[r+1]|=m&flag
 return masks

def context(cfg,chart,known):
 base,cols=actual_family(cfg,chart);K=[known]
 for v in filtration(cols,16)['kernel']:
  if len(basis(K+[v])[0])>len(K):K.append(v)
 assert len(K)==7
 early=filtration(cols,8)['lower_columns'];piv,_=basis(early);entries=[piv[k]for k in sorted(piv)];images=[a for a,b in entries];selectors=[b for a,b in entries]
 assert len(images)==24 and all(apply(early,s)==im for im,s in zip(images,selectors))
 assert all(not(im&31)for im in images),'early coefficient constants must not vary'
 return base,cols,images,selectors,[coefficients(cols,v)for v in K]

def make_planes(base,images):
 r=len(images);N=1<<r;full=(1<<N)-1;patterns=[]
 for j in range(r):
  run=1<<j;a=((1<<run)-1)<<run;length=2*run
  while length<N:a|=a<<length;length*=2
  patterns.append(a)
 F=[]
 for j in range(6):
  a=[]
  for k in range(8):
   v=full if base[j]>>k&1 else 0
   if j<5:
    for i,b in enumerate(images):
     if b>>(5*k+j)&1:v^=patterns[i]
   a.append(v)
  F.append(a)
 return F,full

def prefix_code(F):return sum(((F[j]>>k)&1)<<(5*k+j)for k in range(8)for j in range(5))
