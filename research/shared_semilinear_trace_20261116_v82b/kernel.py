"""Share quotient powers across simultaneous semilinear actions."""
from research.semilinear_trace_20261116_v82.kernel import Frobenius,add,trace_sequential
from research.image_cells_20261115_v67.algebra import rem,trim,pmul
from research.exact_poly_square_20261116_v80.square import square_mod

def trace_jump(t,k,H,f,linear=None,stats=None):
 if k<0:raise ValueError('negative length')
 if len(H)<2 or H[-1]!=1:raise ValueError('positive monic modulus')
 if f.p!=2:raise ValueError('binary field required')
 t=rem(t,H,f)
 if not k:return []
 if k==1:return t
 linear=Frobenius(f)if linear is None else linear
 if linear.f is not f:raise ValueError('field-specific map')
 stats={}if stats is None else stats;m=len(H)-1
 def actions(items,j,P):
  stats['actions']=stats.get('actions',0)+len(items)
  if j==1:return[square_mod(a,H,f)for a in items]
  degree=max(map(len,items),default=0)-1
  if degree<0:return[[]for a in items]
  cols=[[1]]
  if degree>=1:cols.append(P)
  for _ in range(2,degree+1):cols.append(rem(pmul(cols[-1],P,f),H,f));stats['power_products']=stats.get('power_products',0)+1
  stats['power_tables']=stats.get('power_tables',0)+1;outs=[]
  for a in items:
   out=[0]*m
   for i,c in enumerate(a):
    if not c:continue
    z=linear.twist(c,j)
    for l,v in enumerate(cols[i]):
     if v:out[l]^=f.mul(z,v)
   outs.append(trim(out))
  return outs
 def block(j,needP=True):
  if j==1:return square_mod([0,1],H,f),t
  half=j//2;P,T=block(half)
  required=needP or j%2==1
  vals=actions([T,P]if required else[T],half,P);newT=add(T,vals[0]);newP=vals[1]if required else None
  if j&1:
   newT=add(newT,actions([t],2*half,newP)[0])
   if needP:newP=square_mod(newP,H,f)
  return newP,newT
 unused,T=block(k,False);stats['map_xors']=linear.xors;stats['twist_calls']=linear.calls;return T
