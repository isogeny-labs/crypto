from itertools import product
import hashlib

def oracle():
 q=8
 def bm(a,b):
  r=0
  while b:
   if b&1:r^=a
   b>>=1;a<<=1
   if a&8:a^=11
  return r
 def em(x,y):
  a,b=x&7,x>>3;c,d=y&7,y>>3;t=bm(b,d)
  return (bm(a,c)^t)|((bm(a,d)^bm(b,c)^t)<<3)
 M=[[em(x,y)for y in range(64)]for x in range(64)];S=[M[x][x]for x in range(64)];I=[0]+[M[x].index(1)for x in range(1,64)]
 pts=[None]+[(x,y)for x in range(64)for y in range(64)if S[y]^M[x][y]==M[S[x]][x]^S[x]^1];index={p:i for i,p in enumerate(pts)}
 def add(P,Q):
  if P is None:return Q
  if Q is None:return P
  x,y=P;u,v=Q
  if x==u:
   if y^v==x:return None
   assert y==v and x
   slope=x^M[y][I[x]]
  else:slope=M[y^v][I[x^u]]
  w=S[slope]^slope^x^u^1;return(w,M[slope][x^w]^w^y)
 addtab=[[index[add(P,Q)]for Q in pts]for P in pts];gx=[P[0]if P is not None and P[0]<8 else -1 for P in pts];ground=[i for i,x in enumerate(gx)if x>=0];anti={i for i in ground if pts[i][1]>=8};T=index[(0,1)];targets={pts[i][0]for i in ground if i not in anti and pts[i][0]!=0};geo={z:set()for z in targets};mixed={};witness=None;assignments=0
 for a,b,c,d,e in product(ground,repeat=5):
  u=addtab[a][b]
  if gx[u]<0:continue
  v=addtab[u][c]
  if gx[v]<0:continue
  w=addtab[v][d]
  if gx[w]<0:continue
  end=addtab[w][e]
  if gx[end]not in targets or end in anti:continue
  key=(gx[a],gx[b],gx[c],gx[d],gx[e],gx[u],gx[v],gx[w]);geo[gx[end]].add(key);assignments+=1
  k=0;bsum=0
  for i in [a,b,c,d,e]:
   if i in anti:k+=1;bsum=addtab[bsum][i]
  assert bsum in [0,T]
  mixed[k]=mixed.get(k,0)+1
  if k and witness is None:witness={'target_x':gx[end],'tuple':key,'anti_sources':k,'twist_sum':'O'if bsum==0 else'T'}
 def s3(x,y,z):return bm(bm(x,y),bm(x,y))^bm(bm(x,z),bm(x,z))^bm(bm(y,z),bm(y,z))^bm(bm(x,y),z)^1
 neighbors={(x,y):[z for z in range(8)if s3(x,y,z)==0]for x in range(8)for y in range(8)};poly={z:set()for z in targets}
 for a,b,c,d,e in product(range(8),repeat=5):
  for u in neighbors[a,b]:
   for v in neighbors[u,c]:
    for w in neighbors[v,d]:
     for z in neighbors[w,e]:
      if z in targets:poly[z].add((a,b,c,d,e,u,v,w))
 assert geo==poly and witness is not None
 hashes={str(z):hashlib.sha256(repr(sorted(rows)).encode()).hexdigest()for z,rows in poly.items()}
 return {'field':'GF8','A':1,'B':1,'arity':5,'source_domain_size':8,'target_count':len(targets),'original_tuple_count':sum(map(len,poly.values())),'geometric_assignments':assignments,'anti_count_histogram':mixed,'nonrational_counterexample':witness,'rootset_sha256':hashes,'complete_original_set_equality':True}
