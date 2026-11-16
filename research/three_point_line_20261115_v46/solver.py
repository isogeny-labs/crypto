"""Three-point interpolation and exact binary norm-line constraints."""
from rootfinder import binaryfield as b
from research.norm_block_reuse_20261115_v36 import backend as linear

def xor(vs):
 out=0
 for v in vs:out^=v
 return out

def interpolate(f,points):
 rows=[]
 for x,y in points:rows.append([f.square(x),x,1,f.mul(x,y),y,f.mul(f.square(x),x)])
 pivot=[];row=0
 for j in range(5):
  pos=next((i for i in range(row,len(rows))if rows[i][j]),None)
  if pos is None:continue
  rows[row],rows[pos]=rows[pos],rows[row];inv=f.inv(rows[row][j]);rows[row]=[f.mul(v,inv)for v in rows[row]]
  for i in range(len(rows)):
   if i!=row and rows[i][j]:
    scale=rows[i][j];rows[i]=[a^f.mul(scale,v)for a,v in zip(rows[i],rows[row])]
  pivot.append(j);row+=1
  if row==len(rows):break
 if any(not any(r[:5])and r[5]for r in rows):return None
 particular=[0]*5
 for i,j in enumerate(pivot):particular[j]=rows[i][5]
 basis=[]
 for j in range(5):
  if j in pivot:continue
  v=[0]*5;v[j]=1
  for i,p in enumerate(pivot):v[p]=rows[i][j]
  basis.append(v)
 return dict(particular=particular,basis=basis,rank=len(pivot))

def vector(f,line,t):
 assert len(line['basis'])==1
 return [a^f.mul(t,v)for a,v in zip(line['particular'],line['basis'][0])]

def norm(f,A,B,z,V):
 r,s,t,h,k=V;F=(t,s,r,1);G=(k,h)
 def sq(P):
  out=[0]*(2*len(P)-1)
  for i,v in enumerate(P):out[2*i]=f.square(v)
  return tuple(out)
 N=b._add(b._add(sq(F),(0,)+b._mul(F,G,f)),b._mul((B,0,A,1),sq(G),f))
 H,rem=b._divmod(N,(z,1),f);assert not rem and len(H)==6 and H[-1]==1
 return H

def pack(f,H,d):
 out=shift=0
 for j in range(1,6):
  width=min(f.degree,j*(d-1)+1);out|=(H[5-j]>>width)<<shift;shift+=f.degree-width
 return out,shift

def compile_line(f,A,B,z,line,d):
 if line is None:return dict(status='inconsistent_interpolation')
 if len(line['basis'])!=1:return dict(status='unresolved_exceptional_interpolation',field_dimension=len(line['basis']))
 base=norm(f,A,B,z,vector(f,line,0));columns=[]
 for i in range(f.degree):columns.append(tuple(a^bb for a,bb in zip(norm(f,A,B,z,vector(f,line,1<<i)),base)))
 rhs,width=pack(f,base,d);values=[pack(f,H,d)[0]for H in columns];sol=linear.solve(linear.rows_from(values,rhs,width),f.degree)
 return dict(status='complete_affine_constraint_fiber',base=base,columns=columns,solution=sol,rows=width)
