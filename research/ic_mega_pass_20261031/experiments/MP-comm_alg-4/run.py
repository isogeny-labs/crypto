import random, subprocess, re, json, time, sys, os, itertools
N_FIELD=13; MOD=(1<<13)|0b11011
def gmul(a,b):
    r=0
    while b:
        if b&1: r^=a
        b>>=1; a<<=1
        if (a>>N_FIELD)&1: a^=MOD
    return r
def gpow(a,e):
    r=1
    while e:
        if e&1: r=gmul(r,a)
        a=gmul(a,a); e>>=1
    return r
def ginv(a): return gpow(a,(1<<N_FIELD)-2)
def tr(a):
    t=a;s=a
    for _ in range(N_FIELD-1): t=gmul(t,t); s^=t
    return s
assert all(gpow(x,(1<<13))==x for x in [2,3,77]) and gpow(2,(1<<13)-1)==1  # 2^13-1 prime => irreducible iff x^(2^13)=x
def pmul(A,B):
    C={}
    for ma,ca in A.items():
        for mb,cb in B.items():
            v=gmul(ca,cb)
            if v: m=ma|mb; C[m]=C.get(m,0)^v
    return {m:c for m,c in C.items() if c}
def padd(*Ps):
    C={}
    for P in Ps:
        for m,c in P.items(): C[m]=C.get(m,0)^c
    return {m:c for m,c in C.items() if c}
def S3(X1,X2,X3,a6):
    e2=padd(pmul(X1,X2),pmul(X1,X3),pmul(X2,X3))
    return padd(pmul(e2,e2),pmul(pmul(X1,X2),X3),{0:a6})
def rand_subspace(d,rng):
    while True:
        B=[rng.randrange(1,1<<N_FIELD) for _ in range(d)]
        # rank check
        piv=[];ok=True
        for b in B:
            for p in piv:
                if b^p<b: b^=p
            if b==0: ok=False;break
            piv.append(b); piv.sort(reverse=True)
        if ok: return B
def chain_system(dims,seed):
    rng=random.Random(seed)
    a6=rng.randrange(1,1<<N_FIELD)
    while True:
        xR=rng.randrange(1,1<<N_FIELD)
        if tr(1^xR^gmul(a6,ginv(gmul(xR,xR))))==0: break
    off=0; blocks=[]
    for d in dims:
        B=rand_subspace(d,rng); blocks.append({1<<(off+j):B[j] for j in range(d)}); off+=d
    x1,x2,x3,y=blocks
    F=[S3(x1,x2,y,a6),S3(y,x3,{0:xR},a6)]
    eqs=[]
    for P in F:
        for j in range(N_FIELD):
            f=frozenset(m for m,c in P.items() if (c>>j)&1)
            if f: eqs.append(f)
    return eqs,off,rng
pc=lambda m: bin(m).count('1')
def random_matched(eqs,N,rng):
    out=[]
    for f in eqs:
        ch=set()
        for d in [pc(m) for m in f]:
            for _ in range(500):
                m=0
                for p in rng.sample(range(N),d): m|=1<<p
                if m not in ch: ch.add(m);break
        out.append(frozenset(ch))
    return out
def to_ms(eqs,N,path):
    V=[f"v{i}" for i in range(N)]
    def mono(m): return "1" if m==0 else "*".join(V[i] for i in range(N) if m>>i&1)
    polys=["+".join(mono(m) for m in sorted(f)) for f in eqs]+[f"{v}^2+{v}" for v in V]
    open(path,"w").write(",".join(V)+"\n2\n"+",\n".join(polys)+"\n")
def msolve(eqs,N,tag):
    d=os.environ.get("TMPDIR","/tmp"); p=f"{d}/mp4_{tag}.ms"; o=p+".out"
    to_ms(eqs,N,p); t=time.time()
    try:
        r=subprocess.run(["msolve","-g","2","-v","2","-f",p,"-o",o],capture_output=True,text=True,timeout=60)
    except subprocess.TimeoutExpired: return dict(D=None,timeout=True,sec=60)
    degs=[int(l.split()[0]) for l in r.stdout.splitlines() if re.match(r"^\s*\d+\s+\d+\s+\d+\s+\d+ x \d+",l)]
    out=open(o).read(); unsat=bool(re.search(r"\[1\]:?\s*$",out.strip())) or "[1]:" in out and "length of basis:      1 element" in out
    return dict(D=max(degs) if degs else None,rounds=len(degs),unsat=unsat,sec=round(time.time()-t,3))
def xl_refute(eqs,N,Dmax=7):
    # exact Boolean XL refutation degree: min D with 1 in span{m*f: deg<=D}
    for D in range(2,Dmax+1):
        monos=[m for k in range(D+1) for m in (sum(1<<i for i in c) for c in itertools.combinations(range(N),k))]
        piv={}
        for f in eqs:
            df=max(pc(m) for m in f)
            for m in monos:
                if pc(m)>D-df: continue
                r=0
                for t in f: r^=1<<(m|t)  # sparse bitmask keyed by monomial mask
                # row as int over monomial masks (column = monomial mask)
                while r:
                    h=r.bit_length()-1
                    if h in piv: r^=piv[h]
                    else: piv[h]=r; break
                if 0 in piv and piv[0]==1: return D
        if 0 in piv and piv[0]==1: return D
    return None
CELLS=[(3,3,3,13),(3,3,3,6),(3,3,3,5),(3,3,3,3),(3,3,3,2),
       (4,4,4,13),(4,4,4,7),(4,4,4,6),(4,4,4,4),(4,4,4,3),
       (4,4,4,2),(2,4,4,4),(5,5,5,3)]
if __name__=="__main__":
    only=sys.argv[1:] and [tuple(map(int,s.split(","))) for s in sys.argv[1:]]
    res=[]
    for ci,dims in enumerate(CELLS):
        if only and dims not in only: continue
        for dr in range(4):
            seed=1000*ci+dr
            eqs,N,rng=chain_system(dims,seed)
            nul=random_matched(eqs,N,random.Random(seed+7))
            a=msolve(eqs,N,f"c{seed}"); b=msolve(nul,N,f"n{seed}") if N<=19 else dict(D=None,skipped="pilot: null N=25 exceeded 60s timeout")
            row=dict(dims=dims,N=N,minblock=min(dims),seed=seed,neqs=len(eqs),chain=a,null=b)
            if N<=14 and a.get("unsat"): row["chain_xlD"]=xl_refute(eqs,N); row["null_xlD"]=xl_refute(nul,N) if b.get("unsat") else None
            print(json.dumps(row),flush=True); res.append(row)
