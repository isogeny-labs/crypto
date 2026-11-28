# EXP11c: GB-only (msolve -g 2) of sigma system + field equations; read F_11 points off the reduced GB by
# exhaustive evaluation over F_11^4; count points whose quartic X^4 - s1 X^3 + s2 X^2 - s3 X + s4 splits over F_11.
import subprocess, time, os, re, itertools, statistics
p=11; env=dict(os.environ, DYLD_LIBRARY_PATH="/opt/homebrew/Cellar/flint/3.6.0/lib", OMP_NUM_THREADS="1"); T=os.environ.get("TMPDIR","/tmp")
def parse(gbtxt):
    body=gbtxt[gbtxt.index('['):gbtxt.rindex(']')]
    polys=[]
    for ps in body.strip('[]:').split(','):
        ps=ps.strip()
        if not ps: continue
        terms=[]
        for t in re.findall(r'([+-]?\d+)((?:\*s\d\^\d+)*)', ps.replace(' ','')):
            c=int(t[0]); e=[0]*4
            for v,k in re.findall(r's(\d)\^(\d+)', t[1]): e[int(v)-1]=int(k)
            terms.append((c,e))
        polys.append(terms)
    return polys
def ev(P,x): return sum(c*pow(x[0],e[0],p)*pow(x[1],e[1],p)*pow(x[2],e[2],p)*pow(x[3],e[3],p) for c,e in P)%p
def splits(s):
    r=[a for a in range(p) for _ in [0] if (a**4-s[0]*a**3+s[1]*a**2-s[2]*a+s[3])%p==0]
    # count with multiplicity via repeated division
    co=[1,(-s[0])%p,s[1]%p,(-s[2])%p,s[3]%p]; n=0
    for a in range(p):
        while len(co)>1:
            q=[co[0]]
            for c in co[1:]: q.append((q[-1]*a+c)%p)
            if q[-1]==0: co=q[:-1]; n+=1
            else: break
    return n==4
rows=[]
for line in open("msolve_runs/manifest.txt"):
    f,typ,cur,bf=line.split()[:4]
    if not f.endswith(".ms"): continue
    txt=open(f).read().rstrip("\n"); ff=T+"/fe.ms"
    open(ff,"w").write(txt+",\n"+",\n".join(f"1*s{i}^{p}+{p-1}*s{i}" for i in range(1,5))+"\n")
    t0=time.time(); subprocess.run(["msolve","-f",ff,"-o",T+"/fe.gb","-g","2","-t","1"],env=env,capture_output=True,timeout=300); dt=time.time()-t0
    G=parse(open(T+"/fe.gb").read())
    pts=[x for x in itertools.product(range(p),repeat=4) if all(ev(P,x)==0 for P in G)]
    ns=sum(splits(x) for x in pts)
    rows.append((typ,int(bf),dt,len(pts),ns)); print(f,typ,"bf",bf,f"gb {dt:.2f}s","Fq-pts",len(pts),"split",ns,flush=True)
for t in "PR":
    R=[r for r in rows if r[0]==t]
    print(f"SUMMARY {t}: n={len(R)} GB+fieldeq time median {statistics.median(r[2] for r in R):.2f}s max {max(r[2] for r in R):.2f}s | split {sum(r[4] for r in R)} vs brute force {sum(r[1] for r in R)} | exact match {sum(r[4]==r[1] for r in R)}/{len(R)}")
