# Sanity check for MP-additive_comb-2: on E: y^2+xy = x^3+a2 x^2+a6 over F_{2^n},
# for P,Q with x in an F_2-subspace V and xi in V^perp:  Tr(xi*x(P+Q)) = Tr((xi+sqrt(xi))*lambda) + Tr(xi*a2).
# Also: slope histogram Walsh spectrum at n=13 (scratch, one random V).
import random, sys, math
n=13; MOD=(1<<13)|(1<<4)|(1<<3)|1|(1<<1)  # x^13+x^4+x^3+x+1 irreducible
def mul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if a>>n: a^=MOD
    return r
def sq(a): return mul(a,a)
def inv(a): # a^(2^n-2)
    r=1; e=(1<<n)-2; b=a
    while e:
        if e&1: r=mul(r,b)
        b=mul(b,b); e>>=1
    return r
def tr(a):
    t=0; b=a
    for _ in range(n): t^=b; b=sq(b)
    return t&1  # trace lands in F_2 = {0,1}
def sqrt(a):
    for _ in range(n-1): a=sq(a)
    return a
# half trace to solve z^2+z=c when tr(c)=0 (n odd)
def halftrace(c):
    h=0; b=c
    for i in range((n+1)//2):
        h^=b; b=sq(sq(b))
    return h
a2=0; a6=1  # K0-like: y^2+xy=x^3+1
pts=[]
for x in range(1,1<<n):
    # y^2+xy=x^3+a2x^2+a6 -> (y/x)^2+(y/x) = x + a2 + a6/x^2
    c = x ^ a2 ^ mul(a6, inv(sq(x)))
    if tr(c)==0:
        z=halftrace(c); assert sq(z)^z==c
        y=mul(z,x); pts.append((x,y)); pts.append((x,y^x))
print("affine points with x!=0:",len(pts))
def add(P,Q):
    (x1,y1),(x2,y2)=P,Q
    assert x1!=x2
    lam=mul(y1^y2, inv(x1^x2))
    x3=sq(lam)^lam^x1^x2^a2
    y3=mul(lam,x1^x3)^x3^y1
    return (x3,y3),lam
random.seed(1)
l=7
# random subspace V of dim l
while True:
    basis=[random.randrange(1,1<<n) for _ in range(l)]
    # check independence
    span={0}
    for b in basis: span|={s^b for s in span}
    if len(span)==1<<l: break
V=span
# V^perp
Vperp=[xi for xi in range(1<<n) if all(tr(mul(xi,v))==0 for v in basis)]
assert len(Vperp)==1<<(n-l)
F=[P for P in pts if P[0] in V]
print("base size B=",len(F))
# identity check + slope histogram
hist=[0]*(1<<n); viol=0; checks=0
xis=random.sample([x for x in Vperp if x>1],3)
for P in F:
    for Q in F:
        if P[0]==Q[0]: continue
        R,lam=add(P,Q); hist[lam]+=1
        for xi in xis:
            lhs=tr(mul(xi,R[0])); rhs=tr(mul(xi^sqrt(xi),lam))^tr(mul(xi,a2))
            checks+=1; viol+= (lhs!=rhs)
print("identity checks:",checks,"violations:",viol)
# Walsh spectrum of slope histogram
N=1<<n; W=hist[:]
h=1
while h<N:
    for i in range(0,N,2*h):
        for j in range(i,i+h):
            u,v=W[j],W[j+h]; W[j]=u+v; W[j+h]=u-v
    h*=2
pairs=sum(hist); B=len(F)
mx=max(abs(W[k]) for k in range(1,N))
null=math.sqrt(2*pairs*n*math.log(2))
print("pairs:",pairs,"max|Lambda(xi')| (xi'!=0):",mx,"random-extreme-value scale sqrt(2*pairs*n ln2):",round(null,1),"ratio:",round(mx/null,3))
# nonzero spectrum only at trace-zero xi'? check
tr1=[abs(W[k]) for k in range(1,N) if tr(k)==1]
print("max |Lambda| over Tr(xi')=1 (should be 0 by negation symmetry):",max(tr1))
# top 5
top=sorted(range(1,N),key=lambda k:-abs(W[k]))[:5]
print("top5 (xi',Lambda):",[(k,W[k]) for k in top])
