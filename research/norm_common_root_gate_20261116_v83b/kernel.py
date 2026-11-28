from research.image_cells_20261115_v67.algebra import evaluate,pmul,trim

def power(a,n,f):
 v=1
 while n:
  if n&1:v=f.mul(v,a)
  n>>=1
  if n:a=f.sq(a)
 return v

def norm(a,b,A,B,f):
 F=[B,0,A,1]if f.p==2 else[B,A,0,1];out=pmul(a,a,f);terms=[pmul([0]+pmul(a,b,f),[1],f),pmul(F,pmul(b,b,f),f)]if f.p==2 else[pmul(F,pmul(b,b,f),f)]
 for term in terms:
  if len(out)<len(term):out +=[0]*(len(term)-len(out))
  for i,v in enumerate(term):out[i]=f.add(out[i],v if f.p==2 else f.neg(v))
 return trim(out)

def gate(a,b,A,B,f,q):
 b=trim(b)
 if not b:return {'status':'unresolved','reason':'identically zero b needs separate factor/target-domain branch','anti_x':None}
 if len(b)>2:raise ValueError('linear b required')
 if len(b)==1:return {'status':'certified','anti_x':[],'common_x':None}
 r=f.mul(f.neg(b[0]),power(b[1],q-2,f))
 if evaluate(a,r,f):return {'status':'certified','anti_x':[],'common_x':None}
 rhs=f.add(f.mul(f.sq(r),r),f.add(f.mul(A,f.sq(r)if f.p==2 else r),B))
 if f.p==2:
  if not r:anti=False
  else:
   u=f.mul(rhs,power(f.sq(r),q-2,f));v=u;t=0
   for _ in range(f.n):t^=v;v=f.sq(v)
   assert t in [0,1];anti=bool(t)
 else:
  v=power(rhs,(q-1)//2,f);assert v in [0,1,f.neg(1)];anti=v==f.neg(1)
 return {'status':'certified','anti_x':[r]if anti else[],'common_x':r,'anti_multiplicity_in_norm':2 if anti else 0}
