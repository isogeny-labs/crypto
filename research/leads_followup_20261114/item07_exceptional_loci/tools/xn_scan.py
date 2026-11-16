import sys,json,collections,itertools
sys.path.insert(0,'.')
from ec import mk
from planeclass import load
from fractions import Fraction
CL=(599,600,602,607)
data={}
for p in (19,23,29,37,41,47):
    F,sqrt,add=mk(p); R=load(p)
    pts=[(r['alpha'],r['beta']) for r in R if r.get('dim6') in CL and (3+r['alpha'])%p]
    X=[]
    for a,b in pts:
        T=(F(1),sqrt((1+a+b)%p)); P=T; xs={1:1}
        for n in range(2,17):
            P=add(P,T,a)
            if P is None: break
            xs[n]=P[0].u if P[0].v==0 else None
        X.append(xs)
    data[p]=X
cs=sorted({Fraction(n,d) for n in range(-12,13) for d in range(1,9) if n!=0})
best=[]
for n in range(2,17):
  for m in range(1,17):
    if n==m: continue
    for c in cs:
        score=0;tot=0;ok=True
        for p,X in data.items():
            cnt=0
            for xs in X:
                if n in xs and m in xs and xs[n] is not None and xs[m] is not None:
                    cp=c.numerator*pow(c.denominator,-1,p)%p
                    cnt+= (xs[n]-cp*xs[m])%p==0
            score+=cnt;tot+=len(X)
        best.append((score/tot,n,m,c))
best.sort(reverse=True); print(best[:10])
