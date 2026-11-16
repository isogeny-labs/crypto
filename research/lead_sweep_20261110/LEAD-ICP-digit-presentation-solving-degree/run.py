import sys, json, random, itertools, time
# Poly = dict {exps tuple: coeff mod p}. Variables: a1,b1,a2,b2,a3,b3 ; x_i = a_i + B*b_i.
NV = 6
def padd(f,g,p):
    r=dict(f)
    for k,v in g.items():
        r[k]=(r.get(k,0)+v)%p
        if r[k]==0: del r[k]
    return r
def pmul(f,g,p):
    r={}
    for k1,v1 in f.items():
        for k2,v2 in g.items():
            k=tuple(a+b for a,b in zip(k1,k2)); r[k]=(r.get(k,0)+v1*v2)%p
    return {k:v for k,v in r.items() if v}
def const(c,p): return {(0,)*NV:c%p} if c%p else {}
def scal(f,c,p): return {k:v*c%p for k,v in f.items() if v*c%p}
def var(i):
    e=[0]*NV; e[i]=1; return {tuple(e):1}
def deg(f): return max(sum(k) for k in f) if f else 0
def S3(a,b,p,B):
    x=[padd(var(2*i),scal(var(2*i+1),B,p),p) for i in range(3)]
    x1,x2,x3=x
    neg=lambda f:scal(f,-1,p)
    s=padd(x1,x2,p); pr=pmul(x1,x2,p)
    d=padd(x1,neg(x2),p)
    t1=pmul(pmul(d,d,p),pmul(x3,x3,p),p)
    inner=padd(pmul(s,padd(pr,const(a,p),p),p),const(2*b,p),p)
    t2=scal(pmul(inner,x3,p),-2,p)
    q=padd(pr,const(-a,p),p)
    t3=padd(pmul(q,q,p),scal(s,-4*b,p),p)
    return padd(padd(t1,t2,p),t3,p)
def rangepoly(i,p,B):
    f=const(1,p)
    for j in range(B): f=pmul(f,padd(var(i),const(-j,p),p),p)
    return f
def randpoly_like(f,p,rng):  # support-matched random polynomial
    return {k:rng.randrange(1,p) for k in f}
def mons(d):
    out=[]
    def rec(i,left,cur):
        if i==NV: out.append(tuple(cur)); return
        for e in range(left+1): rec(i+1,left-e,cur+[e])
    rec(0,d,[]); return out
def rank_at(gens,p,d):
    """rank of Macaulay matrix (all monomial multiples up to total degree d)."""
    piv={}  # lead monomial -> row (dict)
    rk=0
    for g in gens:
        dg=deg(g)
        if dg>d: continue
        for m in mons(d-dg):
            row={tuple(a+b for a,b in zip(k,m)):v for k,v in g.items()}
            while row:
                lead=max(row)
                if lead in piv:
                    pr=piv[lead]; c=row[lead]*pow(pr[lead],-1,p)%p
                    for k,v in pr.items():
                        nv=(row.get(k,0)-c*v)%p
                        if nv: row[k]=nv
                        else: row.pop(k,None)
                else:
                    piv[lead]=row; rk+=1; break
    return rk
def system(kind,a,b,p,B,rng):
    base=S3(a,b,p,B)
    s = base if kind in("real","planted") else randpoly_like(base,p,rng)
    gens=[s]+[rangepoly(i,p,B) for i in range(NV)]
    if kind=="planted":  # positive control: S3 replaced by a member of the range ideal (forced early fall)
        gens[0]=pmul(var(1),rangepoly(0,p,B),p)
    return gens
def main():
    smoke="--smoke" in sys.argv
    B=3; p=11 if smoke else 13; D=4 if smoke else 5
    ncurves=1 if smoke else 4
    rng=random.Random(20261110)
    curves=[]
    while len(curves)<ncurves:
        a,b=rng.randrange(p),rng.randrange(p)
        if (4*a**3+27*b*b)%p: curves.append((a,b))
    res=[]; t0=time.time()
    for (a,b) in curves:
        row={"curve":[a,b],"rank":{}}
        for kind in ("real","null1","null2","planted"):
            row["rank"][kind]=[rank_at(system(kind,a,b,p,B,rng),p,d) for d in range(3,D+1)]
        res.append(row)
    # statistic: max over curves,d of |rank_real - rank_null1|; A/A spread: |null1-null2|
    dev=max(abs(r["rank"]["real"][i]-r["rank"]["null1"][i]) for r in res for i in range(D-2))
    aa=max(abs(r["rank"]["null2"][i]-r["rank"]["null1"][i]) for r in res for i in range(D-2))
    pos=max(r["rank"]["null1"][i]-r["rank"]["planted"][i] for r in res for i in range(D-2))
    pos=max(pos, max(abs(r["rank"]["planted"][i]-r["rank"]["null1"][i]) for r in res for i in range(D-2)))
    if pos==0: verdict="inconclusive"
    elif dev<=aa: verdict="consistent_with_prediction"
    else: verdict="falsified"
    print(json.dumps({"lead":"LEAD-ICP-digit-presentation-solving-degree","smoke":smoke,"p":p,"B":B,"m":2,"seed":20261110,
      "degrees":list(range(3,D+1)),"curves":res,"stat_max_real_minus_null":dev,"aa_spread":aa,
      "positive_control_detect":pos,"verdict":verdict,"seconds":round(time.time()-t0,2)}))
main()
