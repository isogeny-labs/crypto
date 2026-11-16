import random, itertools
from math import comb
POLYS={7:0b10000011,9:0b1000010001,11:0b100000000101,13:0b10000000011011}
def mul(x,y,n):
    P=POLYS[n];r=0
    while y:
        if y&1:r^=x
        y>>=1;x<<=1
        if x>>n&1:x^=P
    return r
def tr(x,n):
    s=0;t=x
    for _ in range(n): s^=t; t=mul(t,t,n)
    return s&1
def rank(rows):
    piv={};r=0
    for v in rows:
        while v:
            h=v.bit_length()-1
            if h in piv: v^=piv[h]
            else: piv[h]=v;r+=1;break
    return r
def indep(vs):
    return rank(list(vs))==len(vs)
def setup(n,k,seed):
    random.seed(seed)
    while True:
        v=[random.randrange(1,2**n) for _ in range(k)]
        if indep(v):return v
def forms(n,k,v,rand=False):
    Q=[]
    for i in range(n):
        d=1<<i
        if rand: Q.append([[random.randrange(2) for l in range(k)] for j in range(k)])
        else: Q.append([[tr(mul(d,mul(v[j],v[l],n),n),n) for l in range(k)] for j in range(k)])
    return Q
def syz(n,k,Q):
    # degree-3 monomials in a (0..k-1), b (k..2k-1): index by sorted tuple
    idx={}
    def m(t):
        t=tuple(sorted(t))
        if t not in idx: idx[t]=len(idx)
        return idx[t]
    rows=[]
    for q in Q:
        for x in range(2*k):
            v=0
            for j in range(k):
                for l in range(k):
                    if q[j][l]: v^=1<<m((j,k+l,x))
            rows.append(v)
    return len(rows)-rank(rows), rank([sum(q[j][l]<<(j*k+l) for j in range(k) for l in range(k)) for q in Q])
for n in (7,9,11,13):
    k=(n+1)//2
    for s in range(2):
        v=setup(n,k,s)
        print(n,k,'moore syz,rankQ',syz(n,k,forms(n,k,v)),'random',syz(n,k,forms(n,k,v,True)))
# zero set: Moore subset ranks over K, n=7: check whether any k rows of Moore matrix singular -> compute via GF(2^n) gaussian elim
def inv(x,n): 
    r=1
    for _ in range(2**n-2): r=mul(r,x,n)
    return r
def krank(M,n):
    M=[r[:] for r in M];r=0;cols=len(M[0])
    for c in range(cols):
        p=next((i for i in range(r,len(M)) if M[i][c]),None)
        if p is None: continue
        M[r],M[p]=M[p],M[r];iv=inv(M[r][c],n)
        M[r]=[mul(x,iv,n) for x in M[r]]
        for i in range(len(M)):
            if i!=r and M[i][c]:
                f=M[i][c];M[i]=[x^mul(f,y,n) for x,y in zip(M[i],M[r])]
        r+=1
    return r
for n in (7,9):
    k=(n+1)//2
    for s in range(3):
        v=setup(n,k,s)
        W=[[v[j] for j in range(k)]]
        for i in range(1,n): W.append([mul(x,x,n) for x in W[-1]])
        sing=sum(1 for S in itertools.combinations(range(n),k) if krank([W[i] for i in S],n)<k)
        print('n',n,'k',k,'seed',s,'singular k-subsets',sing,'of',comb(n,k))
