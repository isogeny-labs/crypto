#!/usr/bin/env sage -python
"""Exact point-count filter on one five-rational-node plane-sextic slice."""
from sage.all import GF,PolynomialRing,matrix
from pathlib import Path
import hashlib,json,numpy as np,sys,time
wall0=time.perf_counter();cpu0=time.process_time()
limit=int(sys.argv[1]) if len(sys.argv)>1 else None
F3=GF(3);R=PolynomialRing(F3,names=('x','y','z'));x,y,z=R.gens();variables=(x,y,z)
nodes=tuple(tuple(F3(v) for v in p) for p in ((0,0,1),(0,1,0),(1,0,0),(1,1,0),(0,1,1)))
mons=[x**i*y**j*z**(6-i-j) for i in range(7) for j in range(7-i)]
rows=[]
for p in nodes:
 k=next(i for i,v in enumerate(p) if v)
 free=[i for i in range(3) if i!=k]
 rows.append([m(*p) for m in mons])
 rows.extend([[m.derivative(variables[j])(*p) for m in mons] for j in free])
constraint=matrix(F3,rows)
basis=[sum((v[i]*mons[i] for i in range(28)),R(0)) for v in constraint.right_kernel().basis()]
d=len(basis);assert d==13 and constraint.rank()==15
for p in nodes:
 assert all(P(*p)==0 and all(P.derivative(v)(*p)==0 for v in variables) for P in basis)
tangent=[]
for P in basis:
 coeff=[]
 for p in nodes:
  k=next(i for i,v in enumerate(p) if v)
  u,v=[variables[i] for i in range(3) if i!=k]
  coeff.extend([int(F3(2)*P.derivative(u).derivative(u)(*p)),
                int(P.derivative(u).derivative(v)(*p)),
                int(F3(2)*P.derivative(v).derivative(v)(*p))])
 tangent.append(coeff)
tangent=np.array(tangent,dtype=np.int16)
def evaluation(r):
 K=GF(3**r,name='u');els=list(K);one=K(1);zero=K(0)
 points=[(one,u,v) for u in els for v in els]
 points.extend((zero,one,u) for u in els)
 points.append((zero,zero,one))
 assert len(points)==(3**r)**2+3**r+1
 M=np.empty((d,len(points),r),dtype=np.int16)
 for j,P in enumerate(basis):
  Q=P.change_ring(K)
  for k,p in enumerate(points):
   value=Q(*p)
   w=[int(value)] if r==1 else list(value.polynomial())
   w=w+[F3(0)]*(r-len(w))
   assert len(w)==r
   M[j,k,:]=[int(a) for a in w]
 return M.reshape(d,-1),len(points),r
def count_plane(C,ev):
 M,n,r=ev
 if len(C)==0:return np.empty(0,dtype=np.int32)
 V=(C@M)%3
 return np.all(V.reshape(len(C),n,r)==0,axis=2).sum(axis=1)
evals={1:evaluation(1),2:evaluation(2)}
counts={k:0 for k in ('raw_vectors','projective_vectors','ordinary_nodes','N1','N2','N3','N4','N5')}
hits=[];near=[];controls=0;total=3**d-1
if limit is not None:total=min(total,limit)
powers=np.array([3**i for i in range(d)],dtype=np.int64)
for start in range(1,total+1,4096):
 ids=np.arange(start,min(total+1,start+4096),dtype=np.int64)
 C=((ids[:,None]//powers[None,:])%3).astype(np.int16)
 counts['raw_vectors']+=len(C)
 first=np.argmax(C!=0,axis=1)
 C=C[C[np.arange(len(C)),first]==1]
 counts['projective_vectors']+=len(C)
 if len(C)==0:continue
 T=((C@tangent)%3).reshape(len(C),5,3)
 disc=(T[:,:,1]**2-T[:,:,0]*T[:,:,2])%3
 keep=np.all(disc!=0,axis=1)
 C=C[keep];disc=disc[keep]
 counts['ordinary_nodes']+=len(C)
 if len(C)==0:continue
 if controls==0:
  row=C[0]
  F=sum((F3(int(row[i]))*basis[i] for i in range(d)),R(0))
  for r in (1,2):
   K=GF(3**r,name='w');els=list(K)
   Q=F.change_ring(K);one=K(1);zero=K(0)
   pts=[(one,u,v) for u in els for v in els]
   pts.extend((zero,one,u) for u in els)
   pts.append((zero,zero,one))
   direct=sum(Q(*p)==0 for p in pts)
   assert direct==int(count_plane(C[:1],evals[r])[0])
  controls+=1
 split=np.sum(disc==1,axis=1)
 keep=(count_plane(C,evals[1])+2*split-5)==4
 C=C[keep];split=split[keep]
 counts['N1']+=len(C)
 if len(C)==0:continue
 keep=(count_plane(C,evals[2])+5)==10
 C=C[keep];split=split[keep]
 counts['N2']+=len(C)
 if len(C)==0:continue
 for r,target in ((3,28),(4,82),(5,314)):
  if r not in evals:evals[r]=evaluation(r)
  correction=5 if r%2==0 else 2*split-5
  keep=(count_plane(C,evals[r])+correction)==target
  C=C[keep];split=split[keep]
  counts['N%d'%r]+=len(C)
  if r==3:near.extend([[int(v) for v in row] for row in C[:20]])
  if len(C)==0:break
 else:
  for row in C:
   coef=[int(v) for v in row]
   F=sum((F3(coef[i])*basis[i] for i in range(d)),R(0))
   hits.append({'coefficients':coef,'sextic':str(F)})
 if start==1 or start%65536==1:
  sys.stderr.write('id=%d counts=%s cpu=%.1f\n'%(start,str(counts),time.process_time()-cpu0));sys.stderr.flush()
source=Path(__file__)
print(json.dumps({'schema':'ghs.trace-minus14.five-node-sextic/v1',
                  'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
                  'nodes':[[int(v) for v in p] for p in nodes],
                  'basis_dimension':d,'full_nonzero_vectors':3**d-1,
                  'max_id':total,'counts':counts,'direct_point_controls':controls,'near':near,'hits':hits,
                  'cpu_seconds':time.process_time()-cpu0,
                  'wall_seconds':time.perf_counter()-wall0},sort_keys=True))
