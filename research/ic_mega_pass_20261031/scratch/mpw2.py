import random, itertools, numpy as np, json
def run(n,mod,a=1):
    M=(1<<n)|mod
    def mul(x,y):
        r=0
        while y:
            if y&1:r^=x
            y>>=1;x<<=1
            if x>>n&1:x^=M
        return r
    def inv(x):
        r=1;e=(1<<n)-2
        while e:
            if e&1:r=mul(r,x)
            x=mul(x,x);e>>=1
        return r
    O=None
    def add(P,Q):
        if P is None:return Q
        if Q is None:return P
        x1,y1=P;x2,y2=Q
        if x1==x2:
            if y1^y2==x2 or x1==0 and y1!=y2: return None
            if y1!=y2:return None
            l=x1^mul(y1,inv(x1));x3=mul(l,l)^l^a
            return (x3,mul(x1,x1)^mul(l^1,x3))
        l=mul(y1^y2,inv(x1^x2));x3=mul(l,l)^l^x1^x2^a
        return (x3,mul(l,x1^x3)^x3^y1)
    neg=lambda P:None if P is None else (P[0],P[0]^P[1])
    frob=lambda P:None if P is None else (mul(P[0],P[0]),mul(P[1],P[1]))
    # find point: y^2+xy=x^3+a x^2+1 ; solve by random search with trace via brute half-trace (n odd)
    def ht(c):
        r=c;t=c
        for _ in range((n-1)//2):
            t=mul(mul(t,t),mul(t,t));r^=t
        return r
    while True:
        x=random.randrange(1,1<<n)
        rhs=mul(mul(x,x),x)^mul(a,mul(x,x))^1
        c=mul(rhs,inv(mul(x,x)))
        z=ht(c)
        if mul(z,z)^z==c: G=(x,mul(z,x));break
    T=[G]
    for i in range(11):T.append(frob(T[-1]))
    X=[]
    def naf(i,prev,P):
        if i==12:
            if P is not None:X.append(P[0]);return
            return
        naf(i+1,0,P)
        if not prev:
            naf(i+1,1,add(P,T[i]));naf(i+1,1,add(P,neg(T[i])))
    naf(0,0,None)
    X=sorted(set(X))
    def stats(xs):
        B=np.array([[(x>>j)&1 for j in range(n)] for x in xs],dtype=np.uint8)
        cols=[np.ones(len(xs),np.uint8)]+[B[:,j] for j in range(n)]+[B[:,i]&B[:,j] for i in range(n) for j in range(i+1,n)]
        A=np.array(cols).T.copy()
        # GF2 rank
        A=A.astype(bool);r=0;rows,cs=A.shape
        for c in range(cs):
            p=np.nonzero(A[r:,c])[0]
            if len(p)==0:continue
            p=p[0]+r;A[[r,p]]=A[[p,r]];m=A[:,c].copy();m[r]=False;A[m]^=A[r];r+=1
            if r==rows:break
        lin=np.linalg.matrix_rank(B.astype(float))  # proxy
        return dict(nmon=cs,rank2=int(r),annihilators=int(cs-r))
    k=len(X);R=random.sample(range(1,1<<n),k)
    return dict(n=n,size=k,S_L=stats(X),random=stats(R),sub512_S=stats(X[:300:]),sub512_R=stats(R[:300]))
out=[run(23,(1<<5)|1),run(29,(1<<2)|1)]
print(json.dumps(out))
