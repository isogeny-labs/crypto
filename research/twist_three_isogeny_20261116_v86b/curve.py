def on(P,p,A,B):return P is None or(P[1]*P[1]-P[0]**3-A*P[0]-B)%p==0

def add(P,Q,p,A):
 if P is None:return Q
 if Q is None:return P
 x,y=P;u,v=Q
 if x==u:
  if(y+v)%p==0:return None
  k=(3*x*x+A)*pow(2*y,-1,p)%p
 else:k=(v-y)*pow(u-x,-1,p)%p
 w=(k*k-x-u)%p;return(w,(k*(x-w)-y)%p)

def scale(k,P,p,A):
 out=None
 while k:
  if k&1:out=add(out,P,p,A)
  k>>=1
  if k:P=add(P,P,p,A)
 return out

def neg(P,p):return None if P is None else(P[0],-P[1]%p)

def sqrt(a,p):
 a%=p
 if not a:return 0
 if pow(a,(p-1)//2,p)!=1:return None
 if p%4==3:return pow(a,(p+1)//4,p)
 q=p-1;s=0
 while q%2==0:q//=2;s+=1
 z=2
 while pow(z,(p-1)//2,p)!=p-1:z+=1
 c=pow(z,q,p);x=pow(a,(q+1)//2,p);t=pow(a,q,p);m=s
 while t!=1:
  i=0;v=t
  while v!=1:v=v*v%p;i+=1
  b=pow(c,1<<(m-i-1),p);x=x*b%p;t=t*b*b%p;c=b*b%p;m=i
 assert x*x%p==a;return x

def target(T,p,A,B):
 r,s=T;t=2*(3*r*r+A);w=2*(5*r**3+3*A*r+2*B)
 return((A-5*t)%p,(B-7*w)%p)

def phi(P,T,p,A):
 if P is None or P==T or P==neg(T,p):return None
 U=add(P,T,p,A);V=add(P,neg(T,p),p,A)
 return((P[0]+U[0]+V[0]-2*T[0])%p,(P[1]+U[1]+V[1])%p)
