import math

def mul(a,b):
 v=0
 while b:
  if b&1:v^=a
  a<<=1;b>>=1
 return v

def divrem(a,m):
 q=0;n=m.bit_length()-1
 while a and a.bit_length()-1>=n:
  j=a.bit_length()-1-n;q^=1<<j;a^=m<<j
 return q,a

def polynomial(xs):
 c=[1]
 for x in xs:
  out=[0]*(len(c)+1)
  for i,a in enumerate(c):out[i]^=mul(a,x);out[i+1]^=a
  c=out
 return c

def infinity_ok(c,d):
 # Remove zero roots; upper Newton polygon, leading coefficient1 over F2.
 first=next((i for i,a in enumerate(c)if a),None)
 if first is None:return False
 c=c[first:];pts=[(i,a.bit_length()-1)for i,a in enumerate(c)if a];hull=[]
 for p in pts:
  while len(hull)>=2:
   a,b=hull[-2:];cross=(b[0]-a[0])*(p[1]-b[1])-(b[1]-a[1])*(p[0]-b[0])
   if cross<0:break
   hull.pop()
  hull.append(p)
 for (i,y),(j,z)in zip(hull,hull[1:]):
  length=j-i;drop=y-z
  if drop%length or not 0<=drop//length<=d:return False
  degree=drop//length
  for l in range(length+1):
   at=c[i+l].bit_length()-1==y-degree*l
   if at!=bool(math.comb(length,l)&1):return False
 return True

def code(c,s,M):return sum(divrem(a,M)[1]<<(s*i)for i,a in enumerate(c[:-1]))
def echelon(values):
 p={}
 for v in values:
  while v:
   j=v.bit_length()-1
   if j in p:v^=p[j]
   else:p[j]=v;break
 return p

def solve(rows,n):
 piv={}
 for a,b in rows:
  while a:
   j=a.bit_length()-1
   if j in piv:a^=piv[j][0];b^=piv[j][1]
   else:piv[j]=(a,b);break
  if not a and b:return None
 return n-len(piv)
