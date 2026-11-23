import sys,random,math,os
random.seed(1)
def load(f):
    r=[]
    for l in open(f):
        p=l.split('\t')
        if len(p)<8: continue
        r.append(dict(kind=p[1],orc=int(p[2]),res=int(p[3]),conf=int(p[4]),prop=int(p[5]),cpu=float(p[7])))
    return r
def boot(a,b,key,B=2000):
    n=len(a);
    def rt(ix):
        sa=sum(a[i][key] for i in ix); sb=sum(b[i][key] for i in ix); return sa/sb if sb>0 else float('nan')
    pt=rt(range(n)); bs=sorted(rt([random.randrange(n) for _ in range(n)]) for _ in range(B))
    return pt,bs[int(.025*B)],bs[int(.975*B)]
def NS(d):
    m=min(len(v) for v in d.values()); L=[n for n in (16,64,256) if n<=m]
    if m not in L: L.append(m)
    return L
out=[]
out.append("cell | N | mode | solved/N | SAT found | censored | conf/target | cpu_s/target | cpu_s/relation | oracle_mismatch")
cells=sys.argv[1:]
for c in cells:
    d={m:load(f"raw/{c}.{m}.tsv") for m in ['inc','fresh','units']}
    for N in NS(d):
        if min(len(v) for v in d.values())<N: out.append(f"{c} | {N} | (incomplete: "+",".join(f"{m}={len(v)}" for m,v in d.items())+")"); continue
        for m,v in d.items():
            s=v[:N]; sat=sum(1 for x in s if x['res']==10); cen=sum(1 for x in s if x['res']==0)
            mism=sum(1 for x in s if x['res']!=0 and ((x['res']==10)!=(x['orc']==1)))
            out.append(f"{c} | {N} | {m} | {N-cen}/{N} | {sat} | {cen} | {sum(x['conf'] for x in s)/N:.0f} | {sum(x['cpu'] for x in s)/N:.3f} | {(sum(x['cpu'] for x in s)/sat) if sat else float('nan'):.3f} | {mism}")
    out.append("")
    out.append("ratio inc/fresh (sum over first N targets, paired bootstrap 95% CI over targets) and inc/units")
    pts=[]
    for N in NS(d):
        if min(len(v) for v in d.values())<N: continue
        for ref in ('fresh','units'):
            for key in ('conf','cpu'):
                r=boot(d['inc'][:N],d[ref][:N],key)
                out.append(f"{c} N={N} inc/{ref} {key}: {r[0]:.3f} [{r[1]:.3f},{r[2]:.3f}]")
                if ref=='fresh' and key=='conf': pts.append((math.log(N),math.log(r[0])))
    if len(pts)>=2:
        n=len(pts);mx=sum(x for x,_ in pts)/n;my=sum(y for _,y in pts)/n
        sl=sum((x-mx)*(y-my) for x,y in pts)/sum((x-mx)**2 for x,_ in pts)
        out.append(f"{c} slope d log2(ratio_conf inc/fresh)/d log2(N) = {sl:.3f}")
    # first-half vs second-half of inc (learning curve)
    M=min(len(v) for v in d.values())
    if M>=64:
        for lo,hi in [(a,min(b,M)) for a,b in ((0,32),(32,64),(64,128),(128,256)) if a<M]:
            a=d['inc'][lo:hi];b=d['fresh'][lo:hi]
            out.append(f"{c} targets[{lo}:{hi}] inc/fresh conf = {sum(x['conf'] for x in a)/max(1,sum(x['conf'] for x in b)):.3f}")
    out.append("")
print("\n".join(out))
