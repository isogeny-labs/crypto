# F_11[Z]/(Z^3+Z+4), with coefficients in ascending order.
class F:
 def __init__(self,v=0):self.c=v.c if isinstance(v,F) else tuple(x%11 for x in (v if isinstance(v,(list,tuple)) else [v,0,0]))
 def __add__(self,b):b=F(b);return F([x+y for x,y in zip(self.c,b.c)])
 __radd__=__add__
 def __neg__(self):return F([-x for x in self.c])
 def __sub__(self,b):return self+-F(b)
 def __rsub__(self,b):return F(b)+-self
 def __mul__(self,b):
  b=F(b);v=[0]*5
  for i,x in enumerate(self.c):
   for j,y in enumerate(b.c):v[i+j]+=x*y
  for i in [4,3]:v[i-2]-=v[i];v[i-3]-=4*v[i]
  return F(v[:3])
 __rmul__=__mul__
 def __pow__(self,n,mod=None):
  if n<0:
   if not self:raise ZeroDivisionError('zero field inverse')
   return (self**1329)**(-n)
  A=self;out=F(1)
  while n:
   if n&1:out=out*A
   A=A*A;n//=2
  return out
 def __truediv__(self,b):return self*F(b)**-1
 def __mod__(self,p):assert p==11;return self
 def __bool__(self):return any(self.c)
 def __eq__(self,b):return self.c==F(b).c
 def __hash__(self):return hash(self.c)
 def __repr__(self):return str(list(self.c))
