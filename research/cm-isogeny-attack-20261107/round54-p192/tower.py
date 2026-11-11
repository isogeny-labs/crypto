from pathlib import Path
import importlib.util
s=importlib.util.spec_from_file_location('base53',Path(__file__).resolve().parent.parent/'round53-p192/field.py');m=importlib.util.module_from_spec(s);s.loader.exec_module(m);F=m.F
class K:
 def __init__(self,a=0,b=0):
  if isinstance(a,K):self.a,self.b=a.a,a.b
  elif isinstance(a,(list,tuple)) and len(a)==6:self.a,self.b=F(a[:3]),F(a[3:])
  else:self.a,self.b=F(a),F(b)
 def __add__(self,c):c=K(c);return K(self.a+c.a,self.b+c.b)
 __radd__=__add__
 def __neg__(self):return K(-self.a,-self.b)
 def __sub__(self,c):return self+-K(c)
 def __rsub__(self,c):return K(c)+-self
 def __mul__(self,c):c=K(c);return K(self.a*c.a+2*self.b*c.b,self.a*c.b+self.b*c.a)
 __rmul__=__mul__
 def inverse(self):
  d=self.a*self.a-2*self.b*self.b;z=d**-1;return K(self.a*z,-self.b*z)
 def __truediv__(self,c):return self*K(c).inverse()
 def __pow__(self,n,mod=None):
  if n<0:return self.inverse()**(-n)
  x=self;y=K(1)
  while n:
   if n&1:y=y*x
   x=x*x;n//=2
  return y
 def __mod__(self,p):assert p==11;return self
 def __bool__(self):return bool(self.a) or bool(self.b)
 def __eq__(self,c):c=K(c);return self.a==c.a and self.b==c.b
 def __hash__(self):return hash((self.a,self.b))
 def code(self):return list(self.a.c)+list(self.b.c)
 def __repr__(self):return str(self.code())
