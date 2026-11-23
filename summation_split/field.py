"""Independent finite-field E(Fp^2) witness oracle; no polynomial resultants."""
from functools import lru_cache

def nonsquare(p):
    return next(d for d in range(2,p) if pow(d,(p-1)//2,p)==p-1)

class Field:
    def __init__(self,p):self.p=p;self.d=nonsquare(p);self.zero=(0,0);self.one=(1,0)
    def elem(self,u,v=0):return (u%self.p,v%self.p)
    def add(self,a,b):return ((a[0]+b[0])%self.p,(a[1]+b[1])%self.p)
    def neg(self,a):return ((-a[0])%self.p,(-a[1])%self.p)
    def sub(self,a,b):return self.add(a,self.neg(b))
    def mul(self,a,b):
        u,v=a;w,z=b;p=self.p
        return ((u*w+self.d*v*z)%p,(u*z+v*w)%p)
    def inv(self,a):
        u,v=a;p=self.p;norm=(u*u-self.d*v*v)%p
        if not norm:raise ZeroDivisionError
        z=pow(norm,-1,p);return ((u*z)%p,(-v*z)%p)
    def div(self,a,b):return self.mul(a,self.inv(b))
    def sq(self,a):return self.mul(a,a)
    def roots_base(self,c):
        """Return every root of y^2=c in Fp2, c in Fp."""
        p=self.p;c%=p
        if c==0:return [(0,0)]
        if pow(c,(p-1)//2,p)==1:
            u=next(x for x in range(p) if x*x%p==c)
            return list(dict.fromkeys(((u,0),((-u)%p,0))))
        k=c*pow(self.d,-1,p)%p
        u=next(x for x in range(p) if x*x%p==k)
        return list(dict.fromkeys((((0,u),(0,(-u)%p)))))

def add_points(F,a,b,A):
    if a is None:return b
    if b is None:return a
    x,y=a;xx,yy=b
    if x==xx and F.add(y,yy)==F.zero:return None
    if x==xx:
        top=F.add(F.mul((3,0),F.sq(x)),(A,0));den=F.mul((2,0),y)
    else:top=F.sub(yy,y);den=F.sub(xx,x)
    slope=F.div(top,den)
    x3=F.sub(F.sub(F.sq(slope),x),xx)
    y3=F.sub(F.mul(slope,F.sub(x,x3)),y)
    return (x3,y3)

def witness_roots(p,A,B,xs):
    """Fp X for which signed lifts x1..xm,X sum to identity in E(Fp2)."""
    F=Field(p)
    yss={x:F.roots_base((x*x*x+A*x+B)%p) for x in set(xs)|set(range(p))}
    states={None}
    for x in xs:
        states={add_points(F,P,((x,0),y),A) for P in states for y in yss[x]}
    out=set()
    for X in range(p):
        for P in states:
            if any(add_points(F,P,((X,0),y),A) is None for y in yss[X]):
                out.add(X);break
    return out
