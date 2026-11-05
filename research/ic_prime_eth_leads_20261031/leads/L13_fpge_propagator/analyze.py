import json,sys,glob,math,random,collections
rows=[json.loads(l) for f in glob.glob('raw/*.jsonl') for l in open(f) if l.startswith('{')]
def work(r,k=1.0): return r['props']+k*r['ops']
def cens(r): return r['res']=='UNK'
arms=sorted(set(r['arm'] for r in rows))
tq={1:12.71,2:4.303,3:3.182,4:2.776,5:2.571,6:2.447,7:2.365,8:2.306,9:2.262,10:2.228,11:2.201,12:2.179,13:2.160,14:2.145,15:2.131,19:2.093,29:2.045}
def tcrit(df): return tq.get(df,2.0)
def ols(xs,ys):
    n=len(xs);mx=sum(xs)/n;my=sum(ys)/n
    return sum((x-mx)*(y-my) for x,y in zip(xs,ys))/sum((x-mx)**2 for x in xs)
def slopes(arm,metric):
    by=collections.defaultdict(lambda:collections.defaultdict(list))
    for r in rows:
        if r['arm']!=arm: continue
        by[r['seed']][r['s']].append(math.log2(max(metric(r),1)))
    out=[]
    for sd,d in by.items():
        ss=sorted(d)
        if len(ss)<3: continue
        out.append(ols(ss,[sum(d[s])/len(d[s]) for s in ss]))
    return out
def ci(v):
    n=len(v);m=sum(v)/n
    if n<2: return m,float('nan'),float('nan'),n
    sd=math.sqrt(sum((x-m)**2 for x in v)/(n-1));h=tcrit(n-1)*sd/math.sqrt(n)
    return m,m-h,m+h,n
def boot(v,B=4000):
    random.seed(1);ms=sorted(sum(random.choice(v) for _ in v)/len(v) for _ in range(B));return ms[int(.025*B)],ms[int(.975*B)]
print("# slope e = d log2(work)/d s, per-curve OLS, t-CI over curves")
mets={'PE(1x)':lambda r:work(r,1),'PE(0.1x ops)':lambda r:work(r,0.1),'PE(10x ops)':lambda r:work(r,10),'conflicts':lambda r:r['conflicts'],'cpu_ms':lambda r:r['cpu']*1000}
for name,m in mets.items():
    print("##",name)
    for a in arms:
        v=slopes(a,m)
        if not v: continue
        mu,lo,hi,n=ci(v);bl,bh=boot(v) if n>1 else (nan:=float('nan'),)*2
        print(f"{a:14s} e={mu:6.3f} t95=[{lo:6.3f},{hi:6.3f}] boot95=[{bl:6.3f},{bh:6.3f}] curves={n}")
print("## cells: arm,b,s,n,censored%,median conflicts,median PE")
cell=collections.defaultdict(list)
for r in rows: cell[(r['arm'],r['b'],r['s'])].append(r)
for k in sorted(cell):
    c=cell[k];ws=sorted(work(r) for r in c);cf=sorted(r['conflicts'] for r in c)
    print(k,len(c),f"{100*sum(map(cens,c))/len(c):.0f}%",cf[len(cf)//2],int(ws[len(ws)//2]))
