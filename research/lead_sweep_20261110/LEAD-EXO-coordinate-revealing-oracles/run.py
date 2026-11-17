#!/usr/bin/env python3
"""LEAD-EXO-coordinate-revealing-oracles toy: relation-phase cost of an x-coordinate-filtered
two-point-sum index calculus vs BSGS on prime-order prime-field curves. stdlib only."""
import sys, json, random, math

def isprime(n):
    if n < 2: return False
    for q in (2,3,5,7,11,13,17,19,23,29,31,37):
        if n % q == 0: return n == q
    d, s = n-1, 0
    while d % 2 == 0: d//=2; s+=1
    for a in (2,3,5,7,11,13,17,19,23,29,31,37):
        x = pow(a,d,n)
        if x in (1,n-1): continue
        for _ in range(s-1):
            x = x*x % n
            if x == n-1: break
        else: return False
    return True

def add(P,Q,p,a):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if (y1+y2)%p==0: return None
        l=(3*x1*x1+a)*pow(2*y1,-1,p)%p
    else:
        l=(y2-y1)*pow(x2-x1,-1,p)%p
    x3=(l*l-x1-x2)%p
    return (x3,(l*(x1-x3)-y1)%p)

def neg(P,p): return None if P is None else (P[0],(-P[1])%p)

def make_curve(bits, rng):
    while True:
        p = rng.randrange(1<<(bits-1), 1<<bits) | 3
        if p%4!=3 or not isprime(p): continue
        qr = bytearray(p)
        for y in range(1,p): qr[y*y%p]=1
        for _ in range(60):
            a,b = rng.randrange(p), rng.randrange(p)
            if (4*a**3+27*b*b)%p==0: continue
            s=0
            for x in range(p):
                v=(x*x*x+a*x+b)%p
                s += 0 if v==0 else (1 if qr[v] else -1)
            N=p+1+s
            if isprime(N) and N>100:
                while True:
                    x=rng.randrange(p); v=(x**3+a*x+b)%p
                    if v and qr[v]:
                        return p,a,b,N,(x,pow(v,(p+1)//4,p))

def bsgs_cost(p,a,G,N,rng,reps=3):
    m=math.isqrt(N)+1; tot=0
    for _ in range(reps):
        k=rng.randrange(1,N)
        # build target by double-and-add (not counted)
        Q=None; B=G; kk=k
        while kk:
            if kk&1: Q=add(Q,B,p,a)
            B=add(B,B,p,a); kk>>=1
        tab={}; R=None; c=0
        for j in range(m):
            tab.setdefault(R,j) if R is None else tab.setdefault(R[0],(j,R[1])); R=add(R,G,p,a); c+=1
        mG=neg(R,p) if R else None; c+=0
        T=Q; found=False
        for i in range(m+1):
            if T is None: found=True; break
            if T[0] in tab: found=True; break
            T=add(T,mG,p,a); c+=1
        tot+=c
    return tot/reps

def logtable(p,a,G,N):
    t={}; R=G
    for k in range(1,N):
        t[R[0]]=(k,R[1]); R=add(R,G,p,a)
    return t

def ic_cost(p,a,N,G,B,fbmode,rng,lt):
    """returns (adds, relations, fbsize, valid, capped)"""
    cnt=0
    if fbmode=="smallx":
        # points with smallest x (y chosen as a root, sign irrelevant), found by scan (free)
        fb=[]; x=0
        while len(fb)<B and x<p:
            v=(x**3+a*x+rng.randrange(1)+0)%p  # placeholder replaced below
            x+=1
        raise RuntimeError
    return None

def ic_run(curve,B,fbmode,seed,lt,b):
    p,a,N,G=curve
    rng=random.Random(seed)
    fb=[]
    if fbmode=="smallx":
        x=0
        while len(fb)<B and x<p:
            v=(x**3+a*x+b)%p
            if v and pow(v,(p-1)//2,p)==1: fb.append((x,pow(v,(p+1)//4,p)))
            x+=1
    else:
        ks=set()
        while len(ks)<B: ks.add(rng.randrange(1,N))
        for k in ks:
            Q=None; Bp=G; kk=k
            while kk:
                if kk&1: Q=add(Q,Bp,p,a)
                Bp=add(Bp,Bp,p,a); kk>>=1
            fb.append(Q)
    B=len(fb); cnt=0; tab={}
    for i in range(B):
        for j in range(i+1,B):
            S=add(fb[i],fb[j],p,a); cnt+=1
            if S: tab.setdefault(S[0],(i,j,1))
            S=add(fb[i],neg(fb[j],p),p,a); cnt+=1
            if S: tab.setdefault(S[0],(i,j,-1))
    # random start and random stride point
    s=rng.randrange(1,N); r0=rng.randrange(1,N)
    def mul(k):
        Q=None; Bp=G; kk=k
        while kk:
            if kk&1: Q=add(Q,Bp,p,a)
            Bp=add(Bp,Bp,p,a); kk>>=1
        return Q
    R=mul(r0); S=mul(s); cap=4*p; rel=0; valid=0; checked=0; t=0
    while rel<B+1 and cnt<cap:
        R=add(R,S,p,a); cnt+=1; t+=1
        if R is None: continue
        h=tab.get(R[0])
        if h is not None:
            rel+=1
            if lt is not None and checked<20:
                checked+=1
                i,j,sg=h
                lg=lambda P:(lt[P[0]][0] if lt[P[0]][1]==P[1] else (-lt[P[0]][0])%N)
                lhs=(r0+t*s)%N
                ok=False
                for e in (1,-1):
                    for e2 in (1,-1):
                        if (e*lhs-(lg(fb[i])+sg*e2*0))%N==0: pass
                # check lhs = +-(Pi + sg Pj) up to sign of R
                li,lj=lg(fb[i]),lg(fb[j])
                tgt=(li+sg*lj)%N
                if lhs==tgt or lhs==(-tgt)%N: ok=True
                valid+=ok
    return cnt,rel,B,valid,checked,cnt>=cap

def slope(xs,ys):
    n=len(xs); mx=sum(xs)/n; my=sum(ys)/n
    return sum((x-mx)*(y-my) for x,y in zip(xs,ys))/sum((x-mx)**2 for x in xs)

def main():
    smoke="--smoke" in sys.argv
    sizes=[10,12] if smoke else [12,14,16,18,20]
    mults=[1.0] if smoke else [0.5,1.0,2.0]
    SEEDS=(20261110,20261111)
    res={"lead":"LEAD-EXO-coordinate-revealing-oracles","seeds":list(SEEDS),"sizes":sizes,"mults":mults,"rows":[]}
    arms={"coord":[], "randfb":[], "coord_AA":[], "bsgs":[]}
    for bits in sizes:
        rng=random.Random(SEEDS[0]*1000+bits)
        p,a,b,N,G=make_curve(bits,rng)
        lt=logtable(p,a,G,N) if bits<=18 else None
        row={"bits":bits,"p":p,"N":N,"a":a,"b":b}
        row["bsgs"]=bsgs_cost(p,a,G,N,random.Random(5+bits))
        arms["bsgs"].append(row["bsgs"])
        for arm,mode,seed in (("coord","smallx",SEEDS[0]),("randfb","random",SEEDS[0]),("coord_AA","smallx",SEEDS[1])):
            best=None
            for c in mults:
                B=max(4,int(round(c*p**(1/3))))
                r=ic_run((p,a,N,G),B,mode,seed+bits,lt,b)
                if best is None or r[0]<best[0]: best=r+(c,)
            row[arm]={"adds":best[0],"rel":best[1],"fb":best[2],"valid":best[3],"checked":best[4],"capped":best[5],"mult":best[6]}
            arms[arm].append(best[0])
        res["rows"].append(row)
    xs=[math.log2(r["N"]) for r in res["rows"]]
    sl={k:slope(xs,[math.log2(v) for v in vs]) for k,vs in arms.items()}
    res["slopes_log2cost_vs_log2N"]=sl
    last=res["rows"][-1]
    ratio=last["coord"]["adds"]/last["bsgs"]
    null_ratio=last["randfb"]["adds"]/last["coord"]["adds"]
    aa=last["coord_AA"]["adds"]/last["coord"]["adds"]
    allvalid=all(r["coord"]["valid"]==r["coord"]["checked"] and r["randfb"]["valid"]==r["randfb"]["checked"] for r in res["rows"])
    res["stats"]={"coord_over_bsgs_at_largest":ratio,"randfb_over_coord":null_ratio,"AA_ratio":aa,"relations_valid":allvalid,
                  "any_capped":any(r["coord"]["capped"] for r in res["rows"])}
    pos_ok = abs(sl["bsgs"]-0.5)<0.1
    if smoke: verdict="inconclusive"
    elif not (pos_ok and allvalid): verdict="inconclusive"
    elif sl["coord"]<0.55 and ratio<1: verdict="falsified"
    elif 0.58<=sl["coord"]<=0.85 and ratio>1 and 0.5<null_ratio<2: verdict="consistent_with_prediction"
    else: verdict="inconclusive"
    res["controls"]={"positive_bsgs_slope_near_0.5":pos_ok,"negative_randomFB":null_ratio,"AA":aa}
    res["verdict"]=verdict
    print(json.dumps(res))
main()
