"""General-b symmetrised S4 (general_b_s4_derivation.py) on real points: it must vanish at (e(x1,x2,x3), x(P1+P2+P3))
for random points of y^2+xy=x^3+ax^2+b over F_2^9 (random b, a in {0,1}); negative control: random x_R."""
import random
n=9; M=(1<<9)|(1<<4)|1
def mul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if a>>n&1: a^=M
    return r
def inv(a):
    r=1;e=(1<<n)-2
    while e:
        if e&1:r=mul(r,a)
        a=mul(a,a);e>>=1
    return r
def pw(a,k):
    r=1
    for _ in range(k): r=mul(r,a)
    return r
def f(e1,e2,e3,xr,b):
    b2=mul(b,b)
    t=[mul(b2,pw(e1,4)),mul(b2,pw(xr,4)),mul(b,mul(mul(e1,e1),mul(e3,xr))),mul(b,mul(mul(e2,e2),mul(xr,xr))),mul(b,mul(e3,e3)),mul(b,mul(e3,pw(xr,3))),
       mul(mul(e1,e1),mul(mul(e3,e3),mul(xr,xr))),mul(pw(e2,4),pw(xr,4)),mul(mul(e2,e2),mul(e3,pw(xr,3))),pw(e3,4),mul(pw(e3,3),xr),mul(mul(e3,e3),pw(xr,4))]
    r=0
    for v in t: r^=v
    return r
def esym(x1,x2,x3): return (x1^x2^x3, mul(x1,x2)^mul(x1,x3)^mul(x2,x3), mul(mul(x1,x2),x3))
random.seed(5); bad=tot=0
for A in (0,1):
  for trial in range(3):
    B=random.randrange(2,1<<n)
    pts=[(x,y) for x in range(1,1<<n) for y in range(1<<n) if mul(y,y)^mul(x,y)==pw(x,3)^mul(A,mul(x,x))^B]
    def add(P,Q):
        if P is None: return Q
        if Q is None: return P
        (x1,y1),(x2,y2)=P,Q
        if x1==x2:
            if y1^y2==x1: return None
            l=x1^mul(y1,inv(x1)); x3=mul(l,l)^l^A; return (x3, mul(x1,x1)^mul(l^1,x3))
        l=mul(y1^y2,inv(x1^x2)); x3=mul(l,l)^l^x1^x2^A; return (x3, mul(l,x1^x3)^x3^y1)
    for _ in range(300):
        P1,P2,P3=random.sample(pts,3); R=add(add(P1,P2),P3)
        if R is None or R[0]==0: continue
        tot+=1; bad+= f(*esym(P1[0],P2[0],P3[0]),R[0],B)!=0
    neg=sum(f(*esym(*[random.choice(pts)[0] for _ in range(3)]),random.randrange(1,1<<n),B)==0 for _ in range(300))
    print('a',A,'b',B,'points',len(pts),'negative control zeros',neg,'/300')
print('planted sums where f != 0:',bad,'of',tot)
