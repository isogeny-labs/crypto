class Prime:
 def __init__(self,p):
  if p<3 or p%2!=1:raise ValueError('odd prime required')
  self.p=self.q=p
 def add(self,a,b):return(a+b)%self.p
 def sub(self,a,b):return(a-b)%self.p
 def neg(self,a):return(-a)%self.p
 def scale(self,a,k):return a*k%self.p
 def mul(self,a,b):return a*b%self.p
 def sq(self,a):return a*a%self.p
 def inv(self,a):return pow(a,-1,self.p)
 def sqrt(self,a):
  p=self.p;a%=p
  if not a:return 0
  if pow(a,(p-1)//2,p)!=1:return None
  if p%4==3:x=pow(a,(p+1)//4,p)
  else:
   q=p-1;s=0
   while q%2==0:q//=2;s+=1
   z=2
   while pow(z,(p-1)//2,p)!=p-1:z+=1
   c=pow(z,q,p);x=pow(a,(q+1)//2,p);t=pow(a,q,p);m=s
   while t!=1:
    value=t;i=0
    while value!=1 and i<m:value=value*value%p;i+=1
    if i==m:raise ValueError('invalid prime or sqrt state')
    b=pow(c,1<<(m-i-1),p);x=x*b%p;t=t*b*b%p;c=b*b%p;m=i
  assert x*x%p==a;return min(x,p-x)
