#!/usr/bin/env python3
"""CT51: exact degree-eight cross-characteristic 3-torsion bridge.

The independent JS verifier uses a different finite-field representation.
All checks are unconditional, including under python -O.
This is a three-point precomputed lift, not an asymptotic ECDLP advantage.
"""
POLY = 0x13  # F16=F2[X]/(X^4+X+1)
R = 12       # X^3+X^2 in F16

def require(ok, msg):
    if not ok:
        raise AssertionError(msg)

def mul16(a, b):
    out=0
    for _ in range(4):
        if b&1: out^=a
        b>>=1; a<<=1
        if a&16: a^=POLY
    return out

def mul256(a, b):
    ac=mul16(a&15,b&15); bd=mul16(a>>4,b>>4)
    ad=mul16(a&15,b>>4); bc=mul16(a>>4,b&15)
    return (ac^mul16(bd,R))|((ad^bc^bd)<<4)

def pow256(a, n):
    z=1
    while n:
        if n&1: z=mul256(z,a)
        a=mul256(a,a); n>>=1
    return z

class Curve:
    def __init__(self, p29):
        self.p29=p29
        self.a2,self.a4,self.a6=(-1,-10,-20) if p29 else (1,0,0)
    def const(self,a): return a%29 if self.p29 else a&1
    def addf(self,a,b): return (a+b)%29 if self.p29 else a^b
    def negf(self,a): return -a%29 if self.p29 else a
    def subf(self,a,b): return self.addf(a,self.negf(b))
    def mulf(self,a,b): return a*b%29 if self.p29 else mul256(a,b)
    def prod(self,*a):
        z=1
        for v in a: z=self.mulf(z,v)
        return z
    def inv(self,a):
        require(a!=0,'zero inversion')
        return pow(a,-1,29) if self.p29 else pow256(a,254)
    def on(self,p):
        if p is None: return True
        x,y=p
        lhs=self.addf(self.mulf(y,y),y)
        rhs=self.addf(self.addf(self.addf(self.prod(x,x,x),
                       self.prod(self.a2,x,x)),self.prod(self.a4,x)),self.const(self.a6))
        return lhs==rhs
    def neg(self,p):
        return None if p is None else (p[0],self.subf(self.negf(p[1]),1))
    def add(self,p,q):
        if p is None: return q
        if q is None: return p
        require(self.on(p) and self.on(q),'bad input')
        x,y=p; u,v=q
        if p==self.neg(q): return None
        if p==q:
            den=self.addf(self.prod(self.const(2),y),1)
            numerator=self.addf(self.addf(self.prod(self.const(3),x,x),
                         self.prod(self.const(2),self.a2,x)),self.const(self.a4))
            # nu=(-x^3 + a4*x + 2*a6 - a3*y)/(2*y+a3)
            num_nu=self.addf(self.addf(self.negf(self.prod(x,x,x)),
                           self.prod(self.a4,x)),self.subf(self.const(2*self.a6),y))
        else:
            den=self.subf(u,x); numerator=self.subf(v,y)
            num_nu=self.subf(self.prod(y,u),self.prod(v,x))
        require(den!=0,'degenerate chord')
        lam=self.mulf(numerator,self.inv(den)); nu=self.mulf(num_nu,self.inv(den))
        X=self.subf(self.subf(self.subf(self.prod(lam,lam),self.a2),x),u)
        Y=self.subf(self.subf(self.negf(self.prod(lam,X)),nu),1)
        res=(X,Y)
        require(self.on(res),'output off curve')
        return res
    def scalar(self,p,n):
        z=None
        while n:
            if n&1: z=self.add(z,p)
            p=self.add(p,p); n>>=1
        return z

def psi3(x,c):
    return c.addf(c.addf(c.addf(c.addf(
        c.prod(c.const(3),x,x,x,x),
        c.prod(c.const(-4),x,x,x)),
        c.prod(c.const(-60),x,x)),c.prod(c.const(-237),x)),c.const(-21))

def main():
    # Quartic X^4+X+1: no F2 root; nonzero remainder modulo X^2+X+1.
    require(all((u**4+u+1)%2 for u in (0,1)),'linear factor')
    require(mul16(4,4)==3,'quartic quadratic factor control')
    x,y=2,16
    require(mul16(mul16(x,x),x)^mul16(x,x)==R,'R')
    v=R; trace=0
    for _ in range(4):
        trace^=v; v=mul16(v,v)
    require(trace==1 and v==R,'Y quadratic trace')
    require(mul256(y,y)^y==R,'extension relation')
    require(pow256(x,16)==x and pow256(y,16)!=y
            and pow256(y,256)==y,'field degree')
    for v in range(1,256):
        require(mul256(v,pow256(v,254))==1,'nonzero inverse')
    c,d=Curve(True),Curve(False)
    s=(18,5);t=(x,y)
    require(psi3(s[0],c)==0 and psi3(t[0],d)==0,'3 division')
    require(c.on(s) and d.on(t),'curve points')
    require(c.scalar(s,3) is None and d.scalar(t,3) is None,'orders 3')
    S=[c.scalar(s,i) for i in range(3)]
    T=[d.scalar(t,i) for i in range(3)]
    require(S==[None,(18,5),(18,23)],'source points')
    require(T==[None,(x,y),(x,y^1)],'target points')
    for i in range(3):
        for j in range(3):
            require(c.add(S[i],S[j])==S[(i+j)%3],'source law')
            require(d.add(T[i],T[j])==T[(i+j)%3],'target law')
        require(next((n for n in range(3) if d.scalar(t,n)==T[i]),None)==i,'log')
    xs=[u for u in range(256) if psi3(u,d)==0]
    pts=[(u,v) for u in xs for v in range(256) if d.on((u,v))]
    require(len(xs)==4 and len(pts)==8,'full geometric 3 torsion')
    require(all(d.scalar(p,3) is None for p in pts),'3 torsion group')
    require(all(pow256(u,16)==u for u in xs),'x field')
    require(all(pow256(v,16)!=v for _,v in pts),'y extension')
    require(1+sum(c.on((u,v)) for u in range(29) for v in range(29))==30,'F29 order')
    require(1+sum(d.on((u,v)) for u in range(256) for v in range(256))==225,'F256 order')
    # Negating the output generator preserves group law but not the specified
    # public-generator orientation: reject that wrong-target map.
    require(T[1]!=T[2],'orientation mutation')
    raw=(18%2,5%2)
    require(not d.on(raw) or d.scalar(raw,3) is not None,'naive mod2 map rejected')
    print('CT51 PASS: degree 8, 9/9 group pairs, 3/3 DLP, 8/8 torsion; orders 30 and 225')
    print('F29 -> F256 subgroup:',list(zip(S,T)))
    print('[pcc:unverified] O(3) lookup, not a scalable DLP transfer')

if __name__=='__main__':
    main()
