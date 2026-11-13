#!/usr/bin/env python3
"""Independent genus-two F2 binary model census; elementary F4/group counts."""

def deg(a): return a.bit_length()-1

def mul_poly(a,b):
 z=0
 while b:
  if b&1:z^=a
  a<<=1;b>>=1
 return z

def mod_poly(a,b):
 while a and deg(a)>=deg(b):a^=b<<(deg(a)-deg(b))
 return a

def gcd(a,b):
 while b:a,b=b,mod_poly(a,b)
 return a

def derivative(a):
 return sum(1<<(i-1) for i in range(1,deg(a)+1,2) if a&(1<<i))

def mul4(a,b): return mod_poly(mul_poly(a,b),0b111)

def evalp(p,x,m):
 z=0
 for i in range(deg(p),-1,-1):
  z=(mul4(z,x) if m==4 else z*x)^((p>>i)&1)
 return z

def points_at_infinity(h,f,m):
 h3=(h>>3)&1; f6=(f>>6)&1
 return sum(((mul4(v,v) if m==4 else v*v) ^ (mul4(h3,v) if m==4 else h3*v))==f6 for v in range(m))

def count_points(h,f,m):
 xs=range(m)
 return points_at_infinity(h,f,m)+sum(
 (mul4(y,y) if m==4 else y*y) ^ (mul4(hval,y) if m==4 else hval*y) == fval
 for x in xs for y in xs for hval in [evalp(h,x,m)] for fval in [evalp(f,x,m)])

def smooth(h,f):
 if not h:return False
 k=mul_poly(mul_poly(derivative(h),derivative(h)),f)^mul_poly(derivative(f),derivative(f))
 if gcd(h,k)!=1:return False
 # At weighted infinity v²+h3*v=f6, a double root when h3=0.
 # The u-derivative at infinity is h2*v + f5.
 if (h>>3)&1:return True
 f6=(f>>6)&1
 v=f6 # unique v²=f6 over char2
 return (((h>>2)&1)*v)^((f>>5)&1)==1

rows=[]
for h in range(1,16):
 for tail in range(1<<7):
  f=tail
  if deg(f) not in (5,6):continue
  if not smooth(h,f):continue
  n1=count_points(h,f,2)
  n2=count_points(h,f,4)
  s1,s2=3-n1,5-n2
  assert (s1*s1-s2)%2==0
  a1=-s1;a2=(s1*s1-s2)//2
  nJ=5+3*a1+a2
  rows.append((nJ,h,f,n1,n2,a1,a2))
import json
assert max(r[0] for r in rows)==19
w=[r for r in rows if r[1]==11 and r[2]==34][0]
assert w==(19,11,34,6,6,3,5)
assert len(rows)==640
print(json.dumps(dict(max_j=19,n1=6,n2=6,smooth_models=640,witness_f=34,witness_h=11),sort_keys=True,separators=(',',':')))
# If infinity is incorrectly counted as one point, N1 becomes 5 and N2 becomes 5,
# and the derived Jacobian order cannot equal the genuine 19.
assert points_at_infinity(11,34,2)==2 and points_at_infinity(11,34,4)==2
