from itertools import product,combinations_with_replacement
class F:
 def __init__(self,n,m):self.n=n;self.m=m;self.q=1<<n
 def mul(self,a,b):
  v=0
  while b:
   if b&1:v^=a
   a<<=1;b>>=1
  for j in range(v.bit_length()-1,self.n-1,-1):
   if v>>j&1:v^=self.m<<(j-self.n)
  return v
 def sq(self,a):return self.mul(a,a)
def trim(a):
 a=list(a)
 while a and a[-1]==0:a.pop()
 return a

def pmul(a,b,f):
 c=[0]*(len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):c[i+j]^=f.mul(x,y)
 return trim(c)
def rem(a,c,f):
 a=trim(a)
 while len(a)>=len(c):
  shift=len(a)-len(c);x=a[-1]
  for j,y in enumerate(c):a[shift+j]^=f.mul(x,y)
  a=trim(a)
 return a

def peval(a,x,f):
 out=0
 for y in a[::-1]:out=f.mul(out,x)^y
 return out

def rootpoly(xs,f):
 c=[1]
 for x in xs:c=pmul(c,[x,1],f)
 return tuple(c)
def closed(c,q,f):
 e3,e2,e1,_=c;q0,q1,q2=(list(q)+[0,0,0])[:3]
 return f.sq(q0)^f.mul(f.sq(q1),e2)^f.mul(f.mul(q1,q2),f.mul(e1,e2)^e3)^f.mul(f.sq(q2),f.sq(e2))
def matrix_t2(c,q,f):
 cols=[]
 for j in range(3):cols.append((rem([0]*j+list(q),c,f)+[0,0,0])[:3])
 return f.mul(cols[0][0],cols[1][1])^f.mul(cols[1][0],cols[0][1])^f.mul(cols[0][0],cols[2][2])^f.mul(cols[2][0],cols[0][2])^f.mul(cols[1][1],cols[2][2])^f.mul(cols[2][1],cols[1][2])
def controls():
 out={'profiles':[],'monic_cubics':0,'accepted_supported':0,'counterexample':None}
 for n,m,d,k in [(2,7,1,0),(3,11,2,1),(4,19,2,1),(4,19,3,1),(4,19,3,2),(5,37,3,2)]:
  f=F(n,m);assert all(any(f.mul(a,b)==1 for b in range(1,f.q))for a in range(1,f.q));L=rootpoly(range(1<<k),f);W={peval(L,x,f)for x in range(1<<d)};span={0}
  for a in W:
   for b in W:
    v=f.mul(a,b);span|={t^v for t in list(span)}
  allowed={rootpoly(xs,f)for xs in combinations_with_replacement(range(1<<d),3)};rejected=passed_false=positive=0
  for e3,e2,e1 in product(range(f.q),repeat=3):
   c=(e3,e2,e1,1);q=rem(L,c,f);t=closed(c,q,f);assert t==matrix_t2(c,q,f);ok=t in span
   if c in allowed:assert ok;positive+=1
   elif ok:passed_false+=1
   if not ok:rejected+=1
   out['monic_cubics']+=1
  out['accepted_supported']+=positive;out['profiles'].append({'n':n,'modulus':m,'d':d,'prefix':k,'image_dimension':(len(W)-1).bit_length(),'product_rank':(len(span)-1).bit_length(),'all_cubics':f.q**3,'supported':positive,'rejected':rejected,'false_positives_retained':passed_false})
  if n==3:
   c=(2,0,0,1);t=closed(c,rem(L,c,f),f);assert t in span and c not in allowed and peval(c,7,f)==0 and 7>=1<<d;out['counterexample']={'field_modulus':11,'domain':[0,1,2,3],'cubic':[2,0,0,1],'outside_root':7,'guard_second_vieta':t,'product_span':sorted(span)}
 assert out['counterexample']is not None
 return out
