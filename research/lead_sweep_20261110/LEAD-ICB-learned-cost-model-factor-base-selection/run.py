import sys, json, random, math
N=13; POLY=(1<<13)|(1<<4)|(1<<3)|(1<<1)|1
Q=1<<N
EXP=[0]*(2*Q); LOG=[0]*Q
x=1
for i in range(Q-1):
    EXP[i]=x; LOG[x]=i
    x<<=1
    if x&Q: x^=POLY
for i in range(Q-1,2*Q): EXP[i]=EXP[i-(Q-1)]
def mul(a,b): return 0 if a==0 or b==0 else EXP[LOG[a]+LOG[b]]
def inv(a): return EXP[(Q-1)-LOG[a]]
def tr(a):
    t=0; y=a
    for _ in range(N): t^=y; y=mul(y,y)
    return t  # 0 or 1
def ht(c):
    t=0; y=c
    for _ in range((N+1)//2): t^=y; y=mul(mul(y,y),mul(y,y)) if False else mul(mul(y,y),mul(y,y))
    return t
def sq(a): return mul(a,a)
# curve y^2+xy=x^3+x^2+b
def points_x(xv,b):
    if xv==0: return []
    rhs=xv^1^mul(b,inv(sq(xv)))
    if tr(rhs)!=0: return []
    z=ht(rhs)
    if sq(z)^z!=rhs: return []
    y=mul(xv,z); return [(xv,y),(xv,y^xv)]
def add(P,R):
    x1,y1=P; x2,y2=R
    if x1==x2:
        if y1!=y2 or x1==0: return None
        lam=x1^mul(y1,inv(x1)); x3=sq(lam)^lam^1
        return (x3, sq(x1)^mul(lam^1,x3))
    lam=mul(y1^y2,inv(x1^x2)); x3=sq(lam)^lam^x1^x2^1
    return (x3, mul(lam,x1^x3)^x3^y1)
def span(basis):
    S=[0]
    for b in basis: S+= [s^b for s in S]
    return S
def measure(V,b):
    fb=[]
    for v in V: fb+=points_x(v,b)
    seen=set()
    for i in range(len(fb)):
        for j in range(i+1,len(fb)):
            s=add(fb[i],fb[j])
            if s: seen.add(s)
    return len(fb), len(seen)
def feats(V,l,nfb):
    f2=sum(1 for v in V if tr(v)==0)/len(V)
    g=1.0 if 1 in V else 0.0
    return [1.0, math.log(max(nfb,1)), f2, g, g*f2]
def draw(rng,l,gate):
    while True:
        basis=[]
        if gate: basis=[1]
        while len(basis)<l:
            c=rng.randrange(1,Q)
            if len(span(basis+[c]))==2**(len(basis)+1) and len(set(span(basis+[c])))==2**(len(basis)+1): basis.append(c)
        return span(basis)
def dataset(rng,b,l,k):
    out=[]
    for t in range(k):
        V=draw(rng,l,t%2==0)
        nfb,cov=measure(V,b)
        pairs=nfb*(nfb-1)/2
        out.append((feats(V,l,nfb),math.log(max(cov,1)/max(pairs,1)),nfb))
    return out
def ridge(X,y,lam=1e-2):
    d=len(X[0]); A=[[sum(r[i]*r[j] for r in X)+(lam if i==j else 0) for j in range(d)] for i in range(d)]
    B=[sum(r[i]*t for r,t in zip(X,y)) for i in range(d)]
    for i in range(d):
        p=max(range(i,d),key=lambda r:abs(A[r][i])); A[i],A[p]=A[p],A[i]; B[i],B[p]=B[p],B[i]
        for r in range(d):
            if r!=i:
                f=A[r][i]/A[i][i]
                A[r]=[a-f*c for a,c in zip(A[r],A[i])]; B[r]-=f*B[i]
    return [B[i]/A[i][i] for i in range(d)]
def rank(v):
    o=sorted(range(len(v)),key=lambda i:v[i]); r=[0]*len(v); i=0
    while i<len(o):
        j=i
        while j+1<len(o) and v[o[j+1]]==v[o[i]]: j+=1
        for k in range(i,j+1): r[o[k]]=(i+j)/2
        i=j+1
    return r
def spear(a,b):
    ra,rb=rank(a),rank(b); n=len(a); ma=sum(ra)/n; mb=sum(rb)/n
    c=sum((x-ma)*(y-mb) for x,y in zip(ra,rb)); da=math.sqrt(sum((x-ma)**2 for x in ra)); db=math.sqrt(sum((y-mb)**2 for y in rb))
    return c/(da*db) if da*db>0 else 0.0
def pred(w,X): return [sum(a*b for a,b in zip(w,r)) for r in X]
def main():
    smoke='--smoke' in sys.argv
    train_ls=[3,4] if smoke else [3,4,5]
    test_ls=[5] if smoke else [6,7,8]
    k=6 if smoke else 36
    bA,bB=(5,9) if True else (0,0)
    # ensure bA,bB have Tr=1 and not degenerate
    bs=[b for b in range(3,Q) if tr(b)==1][:2]
    bA,bB=bs
    rng=random.Random(20261110)
    train=[]
    for l in train_ls: train+=dataset(rng,bA,l,k)
    X=[r[0] for r in train]; y=[r[1] for r in train]
    w=ridge(X,y)
    nullrng=random.Random(7); nulls=[]
    res={}
    rows=[]
    for l in test_ls:
        for name,b in (("same_curve",bA),("transfer_curve",bB)):
            for rep in ("A1","A2"):
                d=dataset(random.Random(hash((l,name,rep))&0 or (l*1000+len(name)*10+(rep=="A2"))),b,l,k)
                Xt=[r[0] for r in d]; yt=[r[1] for r in d]
                rows.append((l,name,rep,spear(pred(w,Xt),yt),spear([r[2] for r in d],yt),spear(pred(ridge(Xt,yt),Xt),yt)))
    def avg(name,rep,idx):
        v=[r[idx] for r in rows if r[1]==name and r[2]==rep]; return sum(v)/len(v)
    stat={n:avg(n,"A1",3) for n in ("same_curve","transfer_curve")}
    aa={n:abs(avg(n,"A1",3)-avg(n,"A2",3)) for n in stat}
    base={n:avg(n,"A1",4) for n in stat}
    pos=avg("same_curve","A1",5)
    # null: ridge trained on permuted labels
    nv=[]
    for _ in range(10 if smoke else 100):
        yp=y[:]; nullrng.shuffle(yp); wn=ridge(X,yp)
        s=[]
        for l in test_ls:
            d=dataset(random.Random(l*1000+len("transfer_curve")*10),bB,l,k) if False else None
        nv.append(wn)
    # evaluate null weights on cached A1 transfer data
    cache=[]
    for l in test_ls:
        d=dataset(random.Random(l*1000+len("transfer_curve")*10),bB,l,k)
        cache.append(([r[0] for r in d],[r[1] for r in d]))
    nullstats=sorted(sum(spear(pred(wn,Xt),yt) for Xt,yt in cache)/len(cache) for wn in nv)
    n95=nullstats[int(0.95*(len(nullstats)-1))]
    t=stat["transfer_curve"]
    if smoke: verdict="inconclusive"
    elif max(aa.values())>0.2 or pos<0.5: verdict="inconclusive"
    elif t>=0.5 and t>n95: verdict="consistent_with_prediction"
    else: verdict="falsified"
    print(json.dumps({"lead":"LEAD-ICB-learned-cost-model-factor-base-selection","smoke":smoke,"field":"GF(2^%d)"%N,
      "curve_b":[bA,bB],"train_l":train_ls,"test_l":test_ls,"subspaces_per_cell":k,"seed":20261110,
      "weights":w,"spearman_ridge_heldout":stat,"spearman_fbsize_only_baseline":base,
      "aa_abs_diff":aa,"positive_control_insample_spearman":pos,"null_permuted_label_p95":n95,
      "threshold":0.5,"verdict":verdict}))
main()
