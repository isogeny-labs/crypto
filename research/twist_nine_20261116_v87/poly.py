def trim(a):
 a=list(a)
 while a and a[-1]==0:a.pop()
 return a

def mul(a,b,p):
 c=[0]*max(0,len(a)+len(b)-1)
 for i,x in enumerate(a):
  for j,y in enumerate(b):c[i+j]=(c[i+j]+x*y)%p
 return trim(c)

def rem(a,b,p):
 a=trim(a);inv=pow(b[-1],-1,p)
 while len(a)>=len(b):
  j=len(a)-len(b);v=a[-1]*inv%p
  for i,x in enumerate(b):a[i+j]=(a[i+j]-v*x)%p
  a=trim(a)
 return a

def gcd(a,b,p):
 while b:a,b=b,rem(a,b,p)
 v=pow(a[-1],-1,p);return[x*v%p for x in a]

def powmod(a,n,H,p):
 v=[1]
 while n:
  if n&1:v=rem(mul(v,a,p),H,p)
  n>>=1
  if n:a=rem(mul(a,a,p),H,p)
 return v

def three_gcd(p,A,B):
 F=[-A*A%p,12*B%p,6*A%p,0,3%p];r=powmod([0,1],p,F,p);r +=[0]*max(0,2-len(r));r[1]=(r[1]-1)%p
 return F,gcd(F,trim(r),p)

def evaluate(a,x,p):
 v=0
 for c in a[::-1]:v=(v*x+c)%p
 return v
