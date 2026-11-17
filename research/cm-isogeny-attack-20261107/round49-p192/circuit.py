# Division-free arithmetic circuit interpreted in two first-order directions.
class Jet:
 def __init__(self,v,a,b,p,cost):self.t=(v%p,a%p,b%p);self.p=p;self.cost=cost
 def __add__(self,q):
  self.cost['A']+=3;return Jet(*(x+y for x,y in zip(self.t,q.t)),self.p,self.cost)
 def __mul__(self,q):
  v,a,b=self.t;w,c,d=q.t;self.cost['M']+=5;self.cost['A']+=2
  return Jet(v*w,a*w+v*c,b*w+v*d,self.p,self.cost)
def power(z,n):
 if n==1:return z
 if n%2:return power(z,n-1)*z
 t=power(z,n//2);return t*t

def extract(p,x,a,b,exponent=5,mode='descending'):
 cost={'M':0,'A':0,'I':0};J=lambda v,da=0,db=0:Jet(v,da,db,p,cost)
 A=J(a,1,0);B=J(b,0,1)
 if mode=='descending':T=power(A+J(x)*B,exponent)
 elif mode=='nondescending':T=power(A,5)+power(B,5)
 else:raise ValueError(mode)
 N=T+J(2);D=T+J(1);n,na,nb=N.t;d,da,db=D.t
 Wa=(na*d-n*da)%p;Wb=(nb*d-n*db)%p;cost['M']+=4;cost['A']+=2
 recovered=None
 if Wa:cost['I']+=1;cost['M']+=1;recovered=Wb*pow(Wa,-1,p)%p
 return {'a':a,'b':b,'N':N.t,'D':D.t,'Wa':Wa,'Wb':Wb,'recovered':recovered,'cost':cost,'mode':mode,'exponent':exponent}
