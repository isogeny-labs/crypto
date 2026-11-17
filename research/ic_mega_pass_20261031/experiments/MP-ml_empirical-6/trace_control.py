"""Counterexample to the harness's positive control (component 2):
On y^2+xy=x^3+a2x^2+a6 / F_{2^n}, Tr(x(P)) = Tr(a2) iff P in 2E (Knudsen halving),
so the 'trace feature' is the indicator of the index-2 subgroup 2E: a function of the
abstract group element, computable generically with O(log r) group ops (test r*P == O).
For a factor base x(P) in V with V inside ker Tr, every m-sum lies in one coset of 2E,
so the trace feature has AUC 0.75 on the real coordinates, while the twin (features of
enc(R), random bijection) has AUC 0.5: the harness certifies a GGM-simulable feature as
'non-generic'.  For a random V the feature has AUC 0.5 (no positive control at all).
"""
import random, sys, json
n = 13; MOD = (1<<13) | (1<<4) | (1<<3) | (1<<1) | 1   # x^13+x^4+x^3+x+1 irreducible
N = 1<<n
def gmul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if a>>n: a^=MOD
    return r
def gpow(a,e):
    r=1
    while e:
        if e&1: r=gmul(r,a)
        a=gmul(a,a); e>>=1
    return r
def ginv(a): return gpow(a, N-2)
def tr(a):
    t=0; x=a
    for _ in range(n): t^=x; x=gmul(x,x)
    return t  # 0 or 1
def htr(a):  # half-trace, n odd: solves z^2+z=a when tr(a)=0
    t=0; x=a
    for _ in range((n+1)//2): t^=x; x=gmul(gmul(x,x),x) if False else gmul(gmul(x,x),gmul(x,x)) if False else x
    # proper half trace: sum_{i=0}^{(n-1)/2} a^{2^{2i}}
    t=0; x=a
    for i in range((n+1)//2):
        t^=x; x=gmul(x,x); x=gmul(x,x)
    return t
O=None
def add(P,Q,a2):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if (y1^y2)==x1 or x1==0: return None   # Q = -P  (-P = (x, x+y)); x=0 is 2-torsion
        lam = x1 ^ gmul(y1, ginv(x1))
        x3 = gmul(lam,lam) ^ lam ^ a2
        y3 = gmul(x1,x1) ^ gmul(lam^1, x3)
        return (x3,y3)
    lam = gmul(y1^y2, ginv(x1^x2))
    x3 = gmul(lam,lam)^lam^x1^x2^a2
    y3 = gmul(lam, x1^x3) ^ x3 ^ y1
    return (x3,y3)
def neg(P,a2):
    if P is None: return None
    return (P[0], P[0]^P[1])
def points(a2,a6):
    pts=[]
    for x in range(N):
        if x==0:
            # y^2 = a6 -> y = sqrt(a6)
            y = gpow(a6, 1<<(n-1)); pts.append((0,y)); continue
        c = x ^ a2 ^ gmul(a6, ginv(gmul(x,x)))
        if tr(c)==0:
            z=htr(c); y=gmul(x,z); pts.append((x,y)); pts.append((x,y^x))
    return pts
def is_prime(k):
    if k<2: return False
    i=2
    while i*i<=k:
        if k%i==0: return False
        i+=1
    return True
rng=random.Random(7)
# find a2 with Tr(a2)=1 and a6 with #E = 2r, r prime
a2 = 1  # Tr(1)=n mod 2 = 1 for n=13
for a6 in range(1,N):
    pts=points(a2,a6); order=len(pts)+1
    if order%2==0 and is_prime(order//2): break
rr=order//2
print("curve a2=%d a6=%d #E=%d = 2*%d"%(a2,a6,order,rr))
# check: P in 2E  <=> Tr(x(P)) = Tr(a2)
twoE=set(add(P,P,a2) for P in pts); twoE.discard(None)
ok = all(((P in twoE) == (tr(P[0])==tr(a2))) for P in pts)
print("Tr(x)=Tr(a2) <=> P in 2E holds for all points:", ok, " |2E|=",len(twoE))
def auc(scores_pos, scores_neg):
    # rank-based with ties
    import bisect
    neg_sorted=sorted(scores_neg); tot=0.0
    for s in scores_pos:
        lo=bisect.bisect_left(neg_sorted,s); hi=bisect.bisect_right(neg_sorted,s)
        tot += lo + 0.5*(hi-lo)
    return tot/(len(scores_pos)*len(scores_neg))
def experiment(V_in_ker_trace, ell, m, seed):
    rng=random.Random(seed)
    # subspace V of dim ell, optionally inside ker Tr
    while True:
        basis=[]
        cand = [v for v in range(1,N) if (tr(v)==0 or not V_in_ker_trace)]
        rng.shuffle(cand)
        span={0}
        for v in cand:
            if v not in span:
                basis.append(v); span = span | {s^v for s in span}
            if len(basis)==ell: break
        V=span; break
    base=[P for P in pts if P[0] in V and P[0]!=0]
    # all signed m-sums
    cur={None}
    for _ in range(m):
        nxt=set()
        for S in cur:
            for P in base:
                nxt.add(add(S,P,a2)); nxt.add(add(S,neg(P,a2),a2))
        cur=nxt
    dec=set(x for x in cur if x is not None)
    labels={P:(P in dec) for P in pts}
    pos=[P for P in pts if labels[P]]; negs=[P for P in pts if not labels[P]]
    # real trace feature
    auc_real = auc([tr(P[0]) for P in pos],[tr(P[0]) for P in negs])
    # twin: random bijection enc, features = 14 bit planes of enc(P); naive-Bayes log-odds, train/test split, 10 encodings
    aucs=[]
    for e in range(10):
        codes=list(range(1<<(n+1))); rng.shuffle(codes)
        enc={P:codes[i] for i,P in enumerate(pts)}
        idx=list(range(len(pts))); rng.shuffle(idx); half=len(idx)//2
        train=[pts[i] for i in idx[:half]]; test=[pts[i] for i in idx[half:]]
        w=[]
        for b in range(n+1):
            p1=sum(1 for P in train if labels[P] and (enc[P]>>b)&1)+1; p0=sum(1 for P in train if labels[P])+2
            q1=sum(1 for P in train if (not labels[P]) and (enc[P]>>b)&1)+1; q0=sum(1 for P in train if not labels[P])+2
            import math
            w.append((math.log(p1/p0)-math.log(q1/q0), math.log(1-p1/p0)-math.log(1-q1/q0)))
        def score(P): return sum(w[b][0] if (enc[P]>>b)&1 else w[b][1] for b in range(n+1))
        aucs.append(auc([score(P) for P in test if labels[P]],[score(P) for P in test if not labels[P]]))
    # also: trace parity of enc as 'twin trace'
    return {"V_in_ker_trace":V_in_ker_trace,"ell":ell,"m":m,"|base|":len(base),"#decomposable":len(dec),
            "frac_decomposable":len(dec)/len(pts),"AUC_real_trace":auc_real,
            "AUC_twin_naiveBayes_mean":sum(aucs)/len(aucs),"AUC_twin_min":min(aucs),"AUC_twin_max":max(aucs),
            "generic_equivalent":"indicator of coset of 2E = [r*R == O], O(log r) group ops"}
res=[]
for V_in in (True,False):
    for m in (2,3):
        res.append(experiment(V_in, 5, m, 11+m)); print(json.dumps(res[-1]))
json.dump({"curve":{"n":n,"a2":a2,"a6":a6,"order":order,"r":rr},"halving_identity_holds":ok,"runs":res}, open(sys.argv[1],'w'), indent=1)
