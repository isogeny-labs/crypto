from research.disjoint_separator_20261116_v92.kernel import Prefix

def parity(x):return x.bit_count()&1

class Cells:
 def __init__(self,n,mod,d,B):
  self.S=Prefix(n,mod,d);self.n=n;self.d=d;self.e=min(2*d-1,n);self.B=B;f=self.S.f
  self.constant=[0]*d+[f.sq(1<<j)for j in range(self.e)]
  operators=[[f.mul(f.sq(1<<i),f.sq(1<<j))for i in range(n)]for j in range(d)]+[[f.mul(1<<i,1<<j)for i in range(n)]for j in range(self.e)]
  self.transpose=[[sum(((col>>p)&1)<<i for i,col in enumerate(v))for p in range(n)]for v in operators]
 def columns(self,w):
  S=self.S;v=S.f.sq(w);out=[]
  for _ in range(self.d):out.append(v);v=S.timesT(S.timesT(v))
  a=1;b=w
  for _ in range(self.e):out.append(a^b);a=S.timesT(S.timesT(a));b=S.timesT(b)
  return out
 def duals(self,w,count):
  rows=self.S.prepare(self.columns(w))[0]
  def reduce(v):
   for p in sorted(rows,reverse=True):
    if v>>p&1:v^=rows[p][0]
   return v
  if not reduce(self.B):return []
  basis=[sum(((reduce(1<<i)>>j)&1)<<i for i in range(self.n))for j in range(self.n)if j not in rows]
  base=next(v for v in basis if parity(v&self.B));directions=[v^(base if parity(v&self.B)else 0)for v in basis if v!=base]
  out=[]
  for mask in range(min(count,1<<len(directions))):
   v=base
   for i,b in enumerate(directions):
    if mask>>i&1:v^=b
   out.append(v)
  return out
 def cell(self,dual):
  assert parity(dual&self.B)==1
  equations=[]
  for values,c in zip(self.transpose,self.constant):
   row=0;x=dual
   while x:
    bit=x&-x;row^=values[bit.bit_length()-1];x^=bit
   equations.append((row,parity(dual&c)))
  piv={}
  for row,rhs in equations:
   while row:
    p=row.bit_length()-1
    if p not in piv:piv[p]=(row,rhs);break
    a,b=piv[p];row^=a;rhs^=b
   if row==0 and rhs:return {'dual':dual,'empty_cell':True,'rank':len(piv),'equations':equations}
  free=[i for i in range(self.n)if i not in piv]
  def solution(free_value,homogeneous):
   v=sum(((free_value>>i)&1)<<j for i,j in enumerate(free))
   for p,(row,rhs)in sorted(piv.items()):
    bit=parity((row^(1<<p))&v)^(0 if homogeneous else rhs);v|=bit<<p
   return v
  return {'dual':dual,'empty_cell':False,'rank':len(piv),'dimension':len(free),'base':solution(0,False),'directions':[solution(1<<j,True)for j in range(len(free))],'equations':equations}
