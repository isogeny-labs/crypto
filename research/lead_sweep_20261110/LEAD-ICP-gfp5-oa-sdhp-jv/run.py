#!/usr/bin/env python3
"""LEAD-ICP-gfp5-oa-sdhp-jv toy: is the Joux-Vitse (n-1)=4 point decomposition
on E(F_{p^5}) with factor base {x in F_p} generic (same image size as a random
negation-closed base of equal size)?  stdlib only, deterministic."""
import sys, json, random, itertools, time, math
SMOKE = "--smoke" in sys.argv
N5 = 5

def mk_field(p, rnd):
    while True:
        a, b = rnd.randrange(p), rnd.randrange(1, p)
        # t^5 = -a t - b ; irreducible iff no monic factor of degree 1 or 2
        def pm(c):  # poly mod monic divisor d via lists (low->high)
            pass
        poly = [b, a, 0, 0, 0, 1]
        ok = True
        for d in (1, 2):
            for cs in itertools.product(range(p), repeat=d):
                div = list(cs) + [1]
                r = poly[:]
                for i in range(5, d - 1, -1):
                    c = r[i]
                    if c:
                        for j in range(d + 1):
                            r[i - d + j] = (r[i - d + j] - c * div[j]) % p
                if all(v == 0 for v in r[:d]):
                    ok = False; break
            if not ok: break
        if ok: return a, b

class F:
    def __init__(s, p, a, b): s.p, s.a, s.b = p, a, b; s.zero=(0,)*5; s.one=(1,0,0,0,0)
    def add(s, x, y): p=s.p; return tuple((u+v)%p for u,v in zip(x,y))
    def sub(s, x, y): p=s.p; return tuple((u-v)%p for u,v in zip(x,y))
    def neg(s, x): p=s.p; return tuple((-u)%p for u in x)
    def mul(s, x, y):
        p=s.p; r=[0]*9
        for i,u in enumerate(x):
            if u:
                for j,v in enumerate(y): r[i+j]+=u*v
        for k in range(8,4,-1):
            c=r[k]
            if c: r[k-5+1]-=c*s.a; r[k-5]-=c*s.b
        return tuple(v%p for v in r[:5])
    def pw(s, x, e):
        r=s.one
        while e:
            if e&1: r=s.mul(r,x)
            x=s.mul(x,x); e>>=1
        return r
    def inv(s, x): return s.pw(x, s.p**5-2)
    def sc(s, c): return (c%s.p,0,0,0,0)

def elems(p):
    return itertools.product(range(p), repeat=5)

def curve_setup(p, rnd, F_):
    A = tuple(rnd.randrange(p) for _ in range(5)); B = tuple(rnd.randrange(p) for _ in range(5))
    sq = {}
    for y in elems(p):
        y2 = F_.mul(y, y); sq.setdefault(y2, []).append(y)
    def f(x): return F_.add(F_.add(F_.mul(F_.mul(x,x),x), F_.mul(A,x)), B)
    N = 1
    for x in elems(p):
        N += len(sq.get(f(x), ()))
    return A, B, sq, f, N

def add_pt(F_, A, P, Q):
    if P is None: return Q
    if Q is None: return P
    x1,y1=P; x2,y2=Q
    if x1==x2:
        if F_.add(y1,y2)==F_.zero: return None
        num=F_.add(F_.mul(F_.sc(3),F_.mul(x1,x1)),A); den=F_.mul(F_.sc(2),y1)
    else:
        num=F_.sub(y2,y1); den=F_.sub(x2,x1)
    l=F_.mul(num,F_.inv(den))
    x3=F_.sub(F_.sub(F_.mul(l,l),x1),x2)
    y3=F_.sub(F_.mul(l,F_.sub(x1,x3)),y1)
    return (x3,y3)

def four_sum_count(F_, A, base):
    pts = list(base)
    s2 = set()
    for i in range(len(pts)):
        for j in range(i, len(pts)):
            s2.add(add_pt(F_, A, pts[i], pts[j]))
    s2 = list(s2); out = set()
    for i in range(len(s2)):
        for j in range(i, len(s2)):
            out.add(add_pt(F_, A, s2[i], s2[j]))
    return len(out)

def neg_close(F_, pts):
    s = []
    for (x,y) in pts: s.append((x,y)); s.append((x,F_.neg(y)))
    return list(dict.fromkeys(s))

def fb(F_, f, sq, p):
    pts=[]
    for c in range(p):
        x=F_.sc(c); r=sq.get(f(x))
        if r and r[0]!=F_.zero: pts.append((x,r[0]))
    return neg_close(F_, pts)

def rand_base(F_, f, sq, p, m, rnd):
    pts=set()
    while len(pts)*2 < m:
        x=tuple(rnd.randrange(p) for _ in range(5)); r=sq.get(f(x))
        if r and r[0]!=F_.zero and any(x[1:]): pts.add((x,r[0]))
    return neg_close(F_, sorted(pts))[:m]

def struct_base(F_, A, f, sq, p, m, seed_pt):
    pts=[]; Q=None
    for k in range(1, m//2+1):
        Q=add_pt(F_, A, Q, seed_pt)
        if Q is None: break
        pts.append(Q)
    return neg_close(F_, pts)

def run_one(p, cseed, nnull):
    rnd=random.Random(cseed)
    a,b=mk_field(p,rnd); F_=F(p,a,b)
    A,B,sq,f,N=curve_setup(p,rnd,F_)
    base=fb(F_,f,sq,p); m=len(base)
    d_fb=four_sum_count(F_,A,base)
    nulls=[four_sum_count(F_,A,rand_base(F_,f,sq,p,m,random.Random(cseed*1000+k))) for k in range(nnull)]
    ctrl=four_sum_count(F_,A,struct_base(F_,A,f,sq,p,m,base[0]))
    T=math.comb(m+3,4)
    return dict(p=p,curve_seed=cseed,N=N,m=m,fb=d_fb,nulls=nulls,pos_ctrl=ctrl,
                poisson=round(N*(1-math.exp(-T/N)),1),decomp_prob_times_p=round(d_fb/N*p,4))

def main():
    t0=time.time()
    plan=[(5,[1])] if SMOKE else [(7,[11,12,13,14]),(11,[21,22])]
    nnull=2 if SMOKE else 3
    rows=[run_one(p,s,nnull) for p,seeds in plan for s in seeds]
    ratios=[];aa=[];pc=[]
    for r in rows:
        n=r["nulls"]; mean=sum(n)/len(n)
        r["ratio_fb_null"]=round(r["fb"]/mean,4); ratios.append(r["ratio_fb_null"])
        aa.append(abs(n[0]-n[1])/mean); pc.append(r["pos_ctrl"]/mean)
    ctrl_ok = max(pc)<0.5 and max(aa)<0.05
    in_band = all(0.9<=x<=1.1 for x in ratios)
    if not ctrl_ok: verdict="inconclusive"
    elif in_band: verdict="consistent_with_prediction"
    else: verdict="falsified"
    print(json.dumps(dict(lead="LEAD-ICP-gfp5-oa-sdhp-jv",smoke=SMOKE,statistic="distinct 4-sums of FB / mean distinct 4-sums of equal-size random negation-closed base",
        threshold="all ratios in [0.9,1.1]; controls: pos_ctrl/null<0.5 and A/A rel diff<0.05",
        ratios=ratios,max_AA_rel=round(max(aa),4),max_poscontrol_over_null=round(max(pc),4),
        controls_ok=ctrl_ok,rows=rows,seeds="curve seeds in rows; null seed=curve_seed*1000+k",
        params=dict(n=5,decomp_points=4,primes=[r["p"] for r in rows]),
        seconds=round(time.time()-t0,2),verdict=verdict)))
main()
