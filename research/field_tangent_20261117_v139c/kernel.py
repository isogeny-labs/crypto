from research.odd_coprime_factor_20261116_v119c.kernel import initial,inverse_block
from research.delayed_tangent_20261117_v133b.kernel import jet
from research.field_correction_basis_20261117_v128c.kernel import audited_lift

def dot(a,b,f):
 value=0
 for x,y in zip(a,b):value=f.add(value,f.mul(x,y))
 return value

def add_scaled(A,B,t,f):return [[f.add(a,f.mul(t,b))for a,b in zip(x,y)]for x,y in zip(A,B)]
def evaluate(base,columns,values,f):
 F=[list(v)for v in base]
 for col,t in zip(columns,values):F=add_scaled(F,col,t,f)
 return F

def kernel(rows,width,f):
 A=[list(row)for row in rows];piv=[];r=0
 for col in range(width):
  k=next((i for i in range(r,len(A))if A[i][col]),None)
  if k is None:continue
  A[r],A[k]=A[k],A[r];iv=f.inv(A[r][col]);A[r]=[f.mul(v,iv)for v in A[r]]
  for i in range(len(A)):
   if i!=r:
    t=A[i][col];A[i]=[f.sub(a,f.mul(t,b))for a,b in zip(A[i],A[r])]
  piv.append(col);r+=1
 free=[j for j in range(width)if j not in piv];out=[]
 for j in free:
  v=[0]*width;v[j]=1
  for i,p in enumerate(piv):v[p]=f.neg(A[i][j])
  out.append(v)
 return out

def certificate(R0,variations,f):
 for v in kernel(variations,len(R0),f):
  slope=dot(R0,v,f)
  if slope:return v,slope
 return None

def family(f,rng):
 base=[[v]+[rng.randrange(f.q)for _ in range(2*(5-j))]for j,v in enumerate([0,0]+initial(f))]
 def zero():return [[0]*len(v)for v in base]
 E=[zero(),zero()];E[0][0][1]=1;E[1][1][1]=1;D1=zero();D2=zero();D1[0][2]=1;D2[1][2]=1;D0=add_scaled(D1,D2,1,f);_,I=inverse_block(f);j=next(j for j,v in enumerate(I[1])if v);D0[j][3]=f.add(D0[j][3],1)
 return base,E,[D0,D1,D2]

def tangent_row(F,D,f):return [jet(F,col,3,3,f)[-1]['tangent'][1]for col in D]
def correction(F,f):return audited_lift(F,3,f)['audit'][2]['correction'][1]

def reconstruct(x,v,c,slope,f):
 assert slope
 return [f.sub(a,f.mul(b,f.mul(c,f.inv(slope))))for a,b in zip(x,v)]
