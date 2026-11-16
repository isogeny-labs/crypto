from functools import lru_cache
def digits(v):return [v%11,(v//11)%11,v//121]
def pack(v):return sum((x%11)*11**i for i,x in enumerate(v))
def plus(a,b):return pack([x+y for x,y in zip(digits(a),digits(b))])
def neg(a):return pack([-x for x in digits(a)])
def sub(a,b):return plus(a,neg(b))
def times(a,b):
 v=[0]*5
 for i,x in enumerate(digits(a)):
  for j,y in enumerate(digits(b)):v[i+j]=(v[i+j]+x*y)%11
 for j in [4,3]:
  lead=v[j]
  for k,coeff in enumerate([4,1,0,1]):v[j-3+k]=(v[j-3+k]-lead*coeff)%11
 return pack(v[:3])
def power(a,n):
 out=1
 for bit in bin(n)[2:]:
  out=times(out,out)
  if bit=='1':out=times(out,a)
 return out
@lru_cache(None)
def inverse(a):
 assert a;v=power(a,1329);assert times(a,v)==1;return v
def div(a,b):return times(a,inverse(b))
