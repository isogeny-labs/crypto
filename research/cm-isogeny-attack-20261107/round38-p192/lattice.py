from math import floor

def dot(u,v):return sum(a*b for a,b in zip(u,v))
def det(u,v):return u[0]*v[1]-u[1]*v[0]
def nearest(n,d):return (2*n+d)//(2*d)
def reduce(u,v):
 while True:
  if dot(u,u)>dot(v,v):u,v=v,u
  m=nearest(dot(u,v),dot(u,u))
  if not m:break
  v=tuple(y-m*x for x,y in zip(u,v))
 if det(u,v)<0:v=tuple(-x for x in v)
 return u,v

def ceildiv(n,d):return -((-n)//d)
def box(u,v,w,B):
 d=det(u,v);assert d>0
 # i=det(x-w,v)/d, j=det(u,x-w)/d for x in [-B,B]^2.
 ci=-det(w,v);cj=-det(u,w)
 hi=B*(abs(v[0])+abs(v[1]));hj=B*(abs(u[0])+abs(u[1]))
 ir=(ceildiv(ci-hi,d),(ci+hi)//d);jr=(ceildiv(cj-hj,d),(cj+hj)//d)
 points=[]
 for i in range(ir[0],ir[1]+1):
  for j in range(jr[0],jr[1]+1):
   g=w[0]+i*u[0]+j*v[0];k=w[1]+i*u[1]+j*v[1]
   if max(abs(g),abs(k))<=B:points.append((g,k,i,j))
 return points,{'B':B,'i_interval':ir,'j_interval':jr,'coordinate_boxes_examined':(ir[1]-ir[0]+1)*(jr[1]-jr[0]+1)}

def minimax(u,v,w):
 d=det(u,v);ci=-det(w,v);cj=-det(u,w)
 i0=nearest(ci,d);j0=nearest(cj,d)
 feasible=(w[0]+i0*u[0]+j0*v[0],w[1]+i0*u[1]+j0*v[1]);B=max(map(abs,feasible))
 pts,cert=box(u,v,w,B);best=min(pts,key=lambda z:(max(abs(z[0]),abs(z[1])),z[0],z[1]))
 return best,cert

def naf(n,w):
 original=n;sign=1 if n>=0 else -1;n=abs(n);ds=[]
 while n:
  d=0
  if n%2:
   d=n%2**w
   if d>=2**(w-1):d-=2**w
  ds.append(sign*d);n=(n-d)//2
 assert sum(z*2**i for i,z in enumerate(ds))==original
 return ds

def parts(n,w):
 ds=naf(n,w);m=2**(w-2)
 return len(ds),sum(z!=0 for z in ds),sum(z<0 for z in ds),int(m>1),m-1

def joint(g,k,wg,wk):
 L,h,ng,dg,ag=parts(g,wg);M,q,nk,dk,ak=parts(k,wk)
 return max(L,M)-1+dg+dk,h+q-1+ag+ak,ng+nk
