from research.local_split_circuit_20261116_v100.kernel import Circuit,cnf
from research.systematic_residue_20261116_v109.kernel import prune
from research.root_verifier_checks_20261117_v123b.kernel import generate
from research.shared_cubic_prefix_20261117_v122c.circuit import xor_poly,multiply
from research.coprime_factor_checks_20261116_v119b.circuit import coefficient

class BudgetCircuit(Circuit):
 def __init__(self,n,cap=250000):super().__init__(n);self.cap=cap
 def op(self,kind,a,b=None):
  if len(self.nodes)>=self.cap:raise RuntimeError('node_cap')
  return super().op(kind,a,b)

class Bit:
 def __init__(self,C,v):self.C=C;self.v=v
 def wire(self,x):return x.v if isinstance(x,Bit)else x
 def __xor__(self,x):return Bit(self.C,self.C.xor(self.v,self.wire(x)))
 def __rxor__(self,x):return self^x
 def __and__(self,x):return Bit(self.C,self.C.op('and',self.v,self.wire(x)))
 def __rand__(self,x):return self&x
 def __or__(self,x):return ((self^1)&(Bit(self.C,self.wire(x))^1))^1
 def __ror__(self,x):return self|x

def decoder_dynamic(A,b,Z,one):
 A=[list(row)for row in A];b=list(b);h=len(A);w=len(A[0]);zero=one^one;caps=len(Z);ranks=[one]+[zero]*h;piv=[[zero]*w for _ in range(h)]
 for col in range(w):
  old=list(ranks);ranks=[zero]*(h+1);ranks[h]=old[h]
  for r in range(h):
   active=old[r];found=zero
   for i in range(r,h):
    selected=active&A[i][col]&(one^found);found=found|selected
    if i!=r:
     for j in range(w):delta=(A[r][j]^A[i][j])&selected;A[r][j]=A[r][j]^delta;A[i][j]=A[i][j]^delta
     delta=(b[r]^b[i])&selected;b[r]=b[r]^delta;b[i]=b[i]^delta
   for i in range(h):
    if i!=r:
     condition=found&A[i][col]
     for j in range(w):A[i][j]=A[i][j]^(A[r][j]&condition)
     b[i]=b[i]^(b[r]&condition)
   piv[r][col]=found;ranks[r]=ranks[r]|(active&(one^found));ranks[r+1]=ranks[r+1]|found
 incompatible=zero
 for i in range(h):
  iszero=one
  for x in A[i]:iszero=iszero&(one^x)
  incompatible=incompatible|(iszero&b[i])
 count=[one]+[zero]*caps;free=[zero]*w;unsupported=zero
 for col in range(w):
  pivot=zero
  for row in piv:pivot=pivot|row[col]
  flag=one^pivot;old=list(count);count=[zero]*(caps+1)
  for k in range(caps+1):
   if k<caps:free[col]=free[col]^(old[k]&flag&Z[k]);count[k+1]=count[k+1]|(old[k]&flag)
   else:unsupported=unsupported|(old[k]&flag)
   count[k]=count[k]|(old[k]&(one^flag))
 unused=zero
 for k,mask in enumerate(count):
  for z in Z[k:]:unused=unused|(mask&z)
 x=list(free)
 for col in range(w):
  for r in range(h):
   value=b[r]
   for j in range(w):value=value^(A[r][j]&free[j])
   x[col]=x[col]^(piv[r][col]&value)
 return x,(one^incompatible)&(one^unused)&(one^unsupported),one^unsupported,ranks

def planes(C,inputs,full):
 values=[0,full]+list(inputs)
 for kind,a,*rest in C.nodes[C.n+2:]:values.append((values[a]^values[rest[0]])if kind=='xor'else(values[a]&values[rest[0]])if kind=='and'else full^values[a])
 return values

def patterns(n):
 N=1<<n;out=[]
 for j in range(n):
  run=1<<j;a=((1<<run)-1)<<run;size=2*run
  while size<N:a|=a<<size;size*=2
  out.append(a)
 return out,(1<<N)-1

def prefix_rhs(C,F,d,n):
 m=d-1;A=[[0],[0],[1]];B=[[1],[1],[1],[1]];out=[]
 for k in range(1,n+1):
  e=[C.xor(F[j][k]if k<len(F[j])else 0,coefficient(A,B,j,k,C))for j in range(5)];a1=C.xor(e[0],e[1]);c=[e[0],a1,C.xor(e[1],e[2]),C.xor(e[1],e[3]),C.xor(a1,e[4])];out.append(c)
  for V,degree,lo in [(A,2,0),(B,3,2)]:
   for j in range(degree):
    if k<=(degree-j)*m:V[j].append(c[lo+j])
 return out

def final_product(C,F,d):
 discarded,roots=generate(C,F,d);poly=[[1]]
 for root in roots:
  new=[[]for _ in range(len(poly)+1)]
  for j,value in enumerate(poly):new[j]=xor_poly(new[j],multiply(value,root,C),C);new[j+1]=xor_poly(new[j+1],value,C)
  poly=new
 checks=[]
 for a,b in zip(poly,[list(reversed(v))for v in F]):checks+=xor_poly(a,b,C)
 return checks,roots

def affine(C,base,columns,variables,d):
 out=[]
 for j in range(6):
  row=[]
  for k in range((5-j)*(d-1)+1):
   v=base[j]>>k&1
   for wire,col in zip(variables,columns):
    if col[j]>>k&1:v=C.xor(v,wire)
   row.append(v)
  out.append(row)
 return out


def decoder(A,b,Z,one):
 # Precondition:A[0][0]is the separately certified unit.
 reduced=[[A[i][j]^(A[i][0]&A[0][j])for j in range(1,len(A[0]))]for i in range(1,len(A))]
 rhs=[b[i]^(A[i][0]&b[0])for i in range(1,len(A))]
 rest,valid,admitted,ranks=decoder_dynamic(reduced,rhs,Z,one)
 first=b[0]
 for j,x in enumerate(rest,1):first=first^(A[0][j]&x)
 return [first]+rest,valid,admitted,[one^one]+ranks
