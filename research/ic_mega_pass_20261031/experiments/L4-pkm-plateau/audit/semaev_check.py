# independent check: gen.S3coeffs encodes Semaev S3 correctly on a real curve
import random, sys
sys.path.insert(0,'.')
from gen import S3coeffs
p=10007
rng=random.Random(5)
def sqrt_mod(n):
    n%=p
    if n==0: return 0
    if pow(n,(p-1)//2,p)!=1: return None
    # p%4==3 ?
    assert p%4==3
    return pow(n,(p+1)//4,p)
bad=0;tot=0
for trial in range(20):
    a,b=rng.randrange(1,p),rng.randrange(1,p)
    if (4*a**3+27*b*b)%p==0: continue
    pts=[]
    for x in range(p):
        y=sqrt_mod(x**3+a*x+b)
        if y is not None: pts.append((x,y))
    def add(P,Q):
        if P is None: return Q
        if Q is None: return P
        if P[0]==Q[0] and (P[1]+Q[1])%p==0: return None
        if P==Q: l=(3*P[0]**2+a)*pow(2*P[1],-1,p)%p
        else: l=(Q[1]-P[1])*pow(Q[0]-P[0],-1,p)%p
        x3=(l*l-P[0]-Q[0])%p; return (x3,(l*(P[0]-x3)-P[1])%p)
    g=rng.randrange(1,p)
    for _ in range(50):
        P=rng.choice(pts);Q=rng.choice(pts)
        R=add(P,Q)
        if R is None or P[0]==Q[0]: continue
        xR=R[0]
        c=S3coeffs(a,b,xR,g,p)
        z1=P[0]*pow(g,-1,p)%p; z2=Q[0]*pow(g,-1,p)%p
        val=sum(c[i][j]*pow(z1,i,p)*pow(z2,j,p) for i in range(3) for j in range(3))%p
        tot+=1; bad+= (val!=0)
        # negative control: random xR should not vanish
        c2=S3coeffs(a,b,(xR+1+rng.randrange(p-2))%p,g,p)
        val2=sum(c2[i][j]*pow(z1,i,p)*pow(z2,j,p) for i in range(3) for j in range(3))%p
        if val2==0: print("neg-control zero (possible coincidence)")
print("S3 check: tested",tot,"bad",bad)
