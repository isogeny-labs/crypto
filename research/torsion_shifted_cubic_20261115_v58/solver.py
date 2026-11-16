"""Exact bounded cubic norm chart and missing binary torsion branch."""
from types import SimpleNamespace
from research.norm_block_reuse_20261115_v36 import backend as linear
from research.original_chain_lift_20261115_v55.solver import Lift
from research.compiled_subspace_20261115_v31 import factor

def coefficients(f,A,B,z,y,h,r):
 h2=f.square(h);s=f.square(z)^f.mul(r,z)^f.mul(h,y);c2=h2^h^z;c1=f.square(r)^f.mul(h,r)^f.mul(A,h2)^f.mul(z,c2);c0=f.mul(h,s)^f.mul(z,c1)
 assert f.square(s)^f.mul(B,h2)^f.mul(z,c0)==0
 return(c0,c1,c2,1)

def chart(f,A,B,z,y,d,max_candidates=100000,max_fiber_bits=20):
 if not 1<=d<=f.degree:raise ValueError('domain width')
 if z<1<<d:raise ValueError('target outside source domain required')
 assert f.square(y)^f.mul(z,y)==f.mul(f.square(z),z)^f.mul(A,f.square(z))^B
 AS=Lift(f,B);hs=set()
 if d>20:raise ValueError('h-space materialization cap; unresolved')
 for c2 in range(1<<d):
  h=AS.AS(c2^z)
  if h is not None:hs.update([h,h^1])
 w1=min(f.degree,2*d-1);w0=min(f.degree,3*d-2);shift=f.degree-w1;width=2*f.degree-w1-w0
 out=[];checked=0;fibers=[]
 def pack(C):return(C[1]>>w1)|((C[0]>>w0)<<shift)
 for h in sorted(hs):
  base=coefficients(f,A,B,z,y,h,0);cols=[]
  for i in range(f.degree):
   v=1<<i;q=f.square(v);cols.append(((q^f.mul(h,v))>>w1)|((f.mul(z,q)>>w0)<<shift))
  sol=linear.solve(linear.rows_from(cols,pack(base),width),f.degree)
  if sol is None:continue
  dim=len(sol['basis']);fibers.append({'h':h,'dimension':dim})
  if dim>max_fiber_bits:raise RuntimeError('fiber cap; unresolved')
  for tag in range(1<<dim):
   if checked>=max_candidates:raise RuntimeError('candidate cap; unresolved')
   r=sol['particular']^linear.apply(sol['basis'],tag);C=coefficients(f,A,B,z,y,h,r);assert not pack(C);checked+=1
   supported=factor.supported_roots(SimpleNamespace(p=2,field=f),C,[1<<i for i in range(d)])
   if supported['supported']:
    assert sum(c for x,c in supported['roots'])==3;out.append({'h':h,'r':r,'C':C,'roots':supported['roots']})
 return {'complete':True,'solutions':out,'h_values':len(hs),'candidates':checked,'fibers':fibers}

def shifted(f,A,B,z,y,d,**budget):
 if B!=1:raise ValueError('double-x1 torsion chart requires B1')
 T=A;tr=0
 for _ in range(f.degree):tr^=T;T=f.square(T)
 if tr!=1:raise ValueError('double-x1 ghost chart requires traceA1')
 if not z:raise ValueError('target pole; unresolved')
 inv=f.inv(z);zp=inv;yp=f.mul(y^1,f.square(inv))^inv^1
 # Q+P0 by the exact binary chord formula.
 assert f.square(yp)^f.mul(zp,yp)==f.mul(f.square(zp),zp)^f.mul(A,f.square(zp))^1
 result=chart(f,A,B,zp,yp,d,**budget);L=Lift(f,B);values=set()
 for row in result['solutions']:
  counts={x:c for x,c in row['roots']};counts[1]=counts.get(1,0)+2;values.update(L.bag(sorted(counts.items()),z))
 result.update(original_tuples=sorted(values),translated_target=(zp,yp));return result
