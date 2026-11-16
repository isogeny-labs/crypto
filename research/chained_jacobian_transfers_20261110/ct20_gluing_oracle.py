#!/usr/bin/env python3
"""CT20 independent F101 gluing inventory. Development check; [pcc:unverified].
Run: python3 -S research/chained_jacobian_transfers_20261110/ct20_gluing_oracle.py
This is a bounded independent oracle, not a native Jacobian map.
"""
from collections import Counter
P=101
class F:
    def __init__(self,c=0):
        if isinstance(c,F):self.c=c.c
        elif isinstance(c,int):self.c=(c%P,0,0)
        else:self.c=tuple(v%P for v in c)
    def __eq__(self,b):return self.c==F(b).c
    def __hash__(self):return hash(self.c)
    def __add__(self,b):return F([(x+y)%P for x,y in zip(self.c,F(b).c)])
    __radd__=__add__
    def __neg__(self):return F([-x for x in self.c])
    def __sub__(self,b):return self+-F(b)
    def __rsub__(self,b):return F(b)-self
    def __mul__(self,b):
        b=F(b).c;v=[0]*5
        for i in range(3):
            for j in range(3):v[i+j]=(v[i+j]+self.c[i]*b[j])%P
        for i in (4,3):
            for k,c in enumerate((56,13,6)):v[i-3+k]=(v[i-3+k]-c*v[i])%P
        return F(v[:3])
    __rmul__=__mul__
    def __pow__(self,n):
        assert n>=0
        a=self;z=F(1)
        while n:
            if n&1:z=z*a
            a=a*a;n//=2
        return z
    def __truediv__(self,b):
        b=F(b);assert b!=0
        return self*b**(P**3-2)
    def __rtruediv__(self,b):return F(b)/self
    def base(self):
        assert self.c[1:]==(0,0),('failed descent',self.c)
        return self.c[0]
def cubic(x,a,b):return x*x*x+a*x+b
def phi2(x,a,k):
    """Kohel x-map at 2-torsion, with vanishing source cubic term."""
    c0,c1,c2,c3=k;assert c3==1
    h=x*x*x+c2*x*x+c1*x+c0
    assert h!=0
    return 7*x+2*c2-(6*x*x+2*a)*(3*x*x+2*c2*x+c1)/h
def glue(a,b):
    x1,x2,x3=a;y1,y2,y3=b
    d=lambda x,y:x-y
    aa=sum((d(x1,x2)**2/d(y1,y2),d(x2,x3)**2/d(y2,y3),d(x3,x1)**2/d(y3,y1)),F(0))
    bb=sum((d(y1,y2)**2/d(x1,x2),d(y2,y3)**2/d(x2,x3),d(y3,y1)**2/d(x3,x1)),F(0))
    aa2=x1*d(y3,y2)+x2*d(y1,y3)+x3*d(y2,y1)
    bb2=y1*d(x3,x2)+y2*d(x1,x3)+y3*d(x2,x1)
    assert aa!=0 and bb!=0 and aa2!=0 and bb2!=0
    da=d(x1,x2)**2*d(x2,x3)**2*d(x3,x1)**2
    db=d(y1,y2)**2*d(y2,y3)**2*d(y3,y1)**2
    A=db*aa/aa2;B=da*bb/bb2
    q=[]
    for (u,v,w,s,t,z) in ((x1,x2,x3,y1,y2,y3),(x2,x3,x1,y2,y3,y1),(x3,x1,x2,y3,y1,y2)):
        q.append((B*(t-s)*(s-z),A*(v-u)*(u-w)))
    out=[F(1)]
    for c0,c2 in q:
        res=[F(0)]*(len(out)+2)
        for i,v in enumerate(out):res[i]+=v*c0;res[i+2]+=v*c2
        out=res
    return [c.base() for c in out]
def smooth(v):
    def trim(a):
        while a and a[-1]==0:a.pop()
        return a
    def rem(a,b):
        a=trim(a[:]);b=trim(b[:]);assert b
        while len(a)>=len(b):
            c=a[-1]*pow(b[-1],-1,P)%P;shift=len(a)-len(b)
            for i,bi in enumerate(b):a[shift+i]=(a[shift+i]-c*bi)%P
            trim(a)
        return a
    a=trim(v[:]);b=trim([i*v[i]%P for i in range(1,len(v))])
    while b:a,b=b,rem(a,b)
    return len(a)==1
S1=Counter((y*y)%P for y in range(P))
S2=Counter((((y*y+2*z*z)%P,(2*y*z)%P)) for y in range(P) for z in range(P))
def count1(v):
    n=2 if S1[v[-1]] else 0
    for x in range(P):
        y=0
        for c in reversed(v):y=(y*x+c)%P
        n+=S1[y]
    return n
def count2(v):
    n=2 if S2[(v[-1],0)] else 0
    def mul(a,b):return ((a[0]*b[0]+2*a[1]*b[1])%P,(a[0]*b[1]+a[1]*b[0])%P)
    for x in range(P):
        for z in range(P):
            r=(0,0)
            for c in reversed(v):
                r=mul(r,(x,z));r=((r[0]+c)%P,r[1])
            n+=S2[r]
    return n
def short(long):
    d,c,b,a=long;assert a==1
    s=b*pow(3,-1,P)%P
    return ((c-b*s)%P,(2*s*s*s-s*c+d)%P,s)
def order(a,b):return 1+sum(S1[(x*x*x+a*x+b)%P] for x in range(P))
FIX1=[('E1A',(9,76,20,1),(5,39)),('E1B',(72,100,35,1),(26,58))]
FIX2=[('E2A',(77,86,71,1),(13,42)),('E2B',(59,46,8,1),(69,50))]
EXPECTED={
 ('E1A','E2A'):([32,0,26,0,49,0,41],False),
 ('E1A','E2B'):([38,0,48,0,9,0,46],False),
 ('E1B','E2A'):([87,0,15,0,13,0,21],True),
 ('E1B','E2B'):([19,0,86,0,34,0,79],True)}
def run():
    a=F((0,1,0));roots=[a**(P**i) for i in range(3)]
    assert len(set(roots))==3
    assert all(t**3+6*t*t+13*t+56==0 for t in roots)
    left=[t+2 for t in roots];right=[56/t+38 for t in roots]
    assert all(cubic(t,1,46)==0 for t in left)
    assert all(cubic(t,44,21)==0 for t in right)
    for name1,k1,tar1 in FIX1:
        for name2,k2,tar2 in FIX2:
            q=[phi2(x,1,k1) for x in left]
            r=[phi2(x,44,k2) for x in right]
            assert len(set(q))==len(set(r))==3
            assert all(cubic(t,*tar1)==0 for t in q)
            assert all(cubic(t,*tar2)==0 for t in r)
            assert all(q[i]**P==q[(i+1)%3] and r[i]**P==r[(i+1)%3] for i in range(3))
            f=glue(q,r);square=bool(S1[f[-1]])
            assert (f,square)==EXPECTED[name1,name2]
            assert smooth(f) and f[1]==f[3]==f[5]==0
            n1,n2=count1(f),count2(f)
            assert (n1,n2)==(120,10436)
            s1=P+1-n1;s2=P*P+1-n2
            c1=-s1;c2=(s1*s1-s2)//2
            assert (c1,c2)==(18,279) and 1+c1+c2+P*c1+P*P==12317
            print(name1,name2,'sextic=',f,'square_lc=',square,'points=',n1,n2,'jac=',12317)
            if square:
                mon=[v*pow(f[-1],-1,P)%P for v in f]
                b,A4,A2=mon[0],mon[2],mon[4]
                longs=[(b,A4,A2,1),(b*b%P,b*A2%P,A4,1)]
                for i,(long,tag,target) in enumerate(zip(longs,[name2,name1],[tar2,tar1])):
                    aa,bb,shift=short(long)
                    us=[u for u in range(1,P) if aa*u**4%P==target[0] and bb*u**6%P==target[1]]
                    assert len(us)==2 and order(aa,bb)==(113 if i==0 else 109)
                    assert us==([7,94] if name2=='E2A' else [21,80])
                    print('  quotient=',i,'long=',long,'short=',(aa,bb),'shift=',shift,'target=',tag,'u=',us)
                print('  monic=',mon)
if __name__=='__main__':run()
