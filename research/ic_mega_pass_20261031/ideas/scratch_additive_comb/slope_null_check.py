# Corrected null for the slope Walsh spectrum: units = unordered pairs {P,Q} with their negation orbit {-P,-Q} (lambda -> lambda+1),
# each unit adds 2 at lambda and 2 at lambda+1 (ordered pairs). WHT index k pairs with lambda via bit dot product, so W[k]=0 for odd k.
import random, math, sys
exec(open('slope_identity_check.py').read().split("# identity check + slope histogram")[0])
def wht(W):
    N=len(W); h=1
    while h<N:
        for i in range(0,N,2*h):
            for j in range(i,i+h):
                u,v=W[j],W[j+h]; W[j]=u+v; W[j+h]=u-v
        h*=2
    return W
N=1<<n
hist=[0]*N
units=[]
for i,P in enumerate(F):
    for Q in F[i+1:]:
        if P[0]==Q[0]: continue
        R,lam=add(P,Q); hist[lam]+=2; units.append(lam)
W=wht(hist[:])
odd=max(abs(W[k]) for k in range(1,N,2)); evenmax=max(abs(W[k]) for k in range(2,N,2))
print("units(unordered pairs):",len(units),"max|W| odd k (expect 0):",odd,"max|W| even k:",evenmax)
# Monte Carlo null: same number of unordered pairs, slopes uniform, keeping the negation pairing (each unit: 2 at lam, and its mirror unit 2 at lam+1 is also present in the real data as a separate unordered pair)
# In the real data, units come in mirror pairs (lam, lam+1). Null: draw len(units)/2 lambdas uniformly, put 2 at lam and 2 at lam^1.
random.seed(7); mx=[]
for t in range(30):
    H=[0]*N
    for _ in range(len(units)//2):
        lam=random.randrange(N); H[lam]+=2; H[lam^1]+=2
    Wn=wht(H); mx.append(max(abs(Wn[k]) for k in range(2,N,2)))
mx.sort(); print("null max|W| even k over 30 draws: min/median/max =",mx[0],mx[15],mx[-1])
# single-bucket gain for 1 and 2 F_2-linear slope functionals (= x-functionals with xi in V^perp), real vs null
pairs=sum(hist)
g1=1+evenmax/pairs
print("best 1-functional single-bucket gain G(2) real:",round(g1,4),"null median:",round(1+mx[15]/pairs,4))
# best 2-functional: for pairs of even k1,k2 maximize over 4 cosets: count = (pairs + s1 W[k1] + s2 W[k2] + s1 s2 W[k1^k2])/4
best=0
ev=[k for k in range(2,N,2)]
top=sorted(ev,key=lambda k:-abs(W[k]))[:60]
for a in top:
    for b in top:
        if a<b:
            c=a^b
            for s1 in (1,-1):
                for s2 in (1,-1):
                    val=(pairs+s1*W[a]+s2*W[b]+s1*s2*W[c])/4
                    best=max(best,val)
print("best 2-functional single-bucket gain G(4) real (top-60 search):",round(4*best/pairs,4))
