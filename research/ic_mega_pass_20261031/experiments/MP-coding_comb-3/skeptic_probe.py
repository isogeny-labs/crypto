import sys,random,math
sys.path.insert(0,"/Volumes/SSD990-2/ml-cryptanalysis")
from csd import semaev as sm
from sympy import sqrt_mod, nextprime
from collections import Counter
random.seed(1)
S4=sm.S4(); terms=list(S4.items())
p=nextprime(2**24+12345); a=random.randrange(p); b=random.randrange(p)
def lift(x):
    r=(x**3+a*x+b)%p
    y=sqrt_mod(r,p) if r else 0
    return None if y is None else y
def add(P,Q):
    if P is None:return Q
    if Q is None:return P
    (x1,y1),(x2,y2)=P,Q
    if x1==x2:
        if (y1+y2)%p==0:return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else:l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p
    return (x3,(l*(x1-x3)-y1)%p)
def S(x1,x2,x3,x4):
    xs=[x1,x2,x3,x4,a,b]; pw=[[1,v,v*v,v**3,v**4,v**5,v**6] for v in xs]
    tot=0
    for e,c in terms:
        t=c
        for i in range(6):
            if e[i]:t*=pw[i][e[i]]
        tot+=t
    return tot
def smallpt():
    while True:
        x=random.randrange(1,round(p**(1/3)))
        y=lift(x)
        if y is not None:return (x,y)
N=int(sys.argv[1]);rows=[]
for _ in range(N):
    P=[smallpt() for _ in range(3)]
    R=add(add(P[0],P[1]),P[2])
    if R is None:continue
    xs=[q[0] for q in P]
    st=S(xs[0],xs[1],xs[2],R[0]); 
    # random arm: same small xs, random xR
    xr=random.randrange(p); sr=S(xs[0],xs[1],xs[2],xr)
    rows.append((xs,st,sr,p))
def mi(pairs):
    n=len(pairs);c=Counter(pairs);cx=Counter(u for u,_ in pairs);cy=Counter(v for _,v in pairs)
    return sum(m/n*math.log2(m*n/(cx[u]*cy[v])) for (u,v),m in c.items())
def mi_perm(pairs,reps=20):
    ys=[v for _,v in pairs];xs=[u for u,_ in pairs];m0=mi(pairs);ps=[]
    for _ in range(reps):
        random.shuffle(ys);ps.append(mi(list(zip(xs,ys))))
    return round(m0,4),round(sum(ps)/reps,4),round(max(ps),4)
print("p",p,"n",len(rows),"S%p==0 true",sum(r[1]%p==0 for r in rows),"rand",sum(r[2]%p==0 for r in rows))
for q in (3,5,7):
    for name,idx in (("true",1),("rand",2)):
        kq=[(r[idx]//p)%q for r in rows]
        hi=[(r[0][0]>>4,r[0][1]>>4) for r in rows]
        lo=[(r[0][0]%q,r[0][1]%q) for r in rows]
        print(q,name,"MI(k|hi halves)",mi_perm(list(zip(hi,kq))),"MI(k|x mod q)",mi_perm(list(zip(lo,kq))))
    # exact-test: k*p == S mod q
    print(q,"pred k*p==S mod q true",sum((r[1]//p*p-r[1])%q==0 for r in rows)/len(rows),"rand",sum((r[2]//p*p-r[2])%q==0 for r in rows)/len(rows))
