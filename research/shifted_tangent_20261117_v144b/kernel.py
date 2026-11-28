from research.odd_coprime_factor_20261116_v119c.kernel import inverse_block,initial
from research.odd_cached_quadratic_20261116_v94b.kernel import Arithmetic

def at(a,k):return a[k]if 0<=k<len(a)else 0

def primal_prefix(F,d,h):
 f=Arithmetic(2,[0,1]);S,I=inverse_block(f);A=[[0],[0],[1]];B=[[v]for v in initial(f)];m=d-1;terms=0
 for k in range(1,h+1):
  old=[0]*5
  for i,a in enumerate(A):
   for j,b in enumerate(B):
    if i+j>=5:continue
    for t in range(k+1):old[i+j]^=at(a,t)&at(b,k-t);terms+=1
  e=[((F[j]>>k)&1)^old[j]for j in range(5)];c=[sum(I[j][i]*e[i]for i in range(5))%2 for j in range(5)]
  for V,degree,lo in [(A,2,0),(B,3,2)]:
   for j in range(degree):
    if k<=(degree-j)*m:V[j].append(c[lo+j])
 return A,B,I,terms

def packed(F,Ds,d,ell,n):
 assert d>=2 and n>=ell>=1 and Ds and all(not any(v&((1<<ell)-1)for v in D)for D in Ds)
 h=n-ell;A,B,I,prefix_terms=primal_prefix(F,d,h);DA=[[]for _ in range(3)];DB=[[]for _ in range(4)];out=[];terms=0;m=d-1
 for s in range(h+1):
  err=[sum(((D[j]>>(ell+s))&1)<<lane for lane,D in enumerate(Ds))for j in range(5)]
  for i in range(3):
   for j in range(4):
    if i+j>=5:continue
    for r in range(s):
     if at(B[j],s-r):err[i+j]^=at(DA[i],r)
     if at(A[i],s-r):err[i+j]^=at(DB[j],r)
     terms+=2
  dc=[]
  for row in I:
   value=0
   for i,a in enumerate(row):
    if a:value^=err[i]
   dc.append(value)
  for V,degree,lo in [(DA,2,0),(DB,3,2)]:
   for j in range(degree):
    if ell+s<=(degree-j)*m:V[j].append(dc[lo+j])
  out.append(dc)
 return {'correction_masks':out,'prefix_precision':h,'lanes':len(Ds),'prefix_scalar_terms':prefix_terms,'packed_tangent_terms':terms}
