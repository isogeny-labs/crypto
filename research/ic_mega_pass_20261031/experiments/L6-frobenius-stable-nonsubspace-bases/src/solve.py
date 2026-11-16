import sys, os, json, time, subprocess, re, itertools, tempfile
sys.path.insert(0, os.path.dirname(__file__))
from field import *
from concurrent.futures import ThreadPoolExecutor
T=os.environ['TMPDIR']; Y=json.load(open(sys.argv[1])); OUT=sys.argv[2]; TO=120
def mk(S):
    n=S['n']; mul=S['mul']
    def inv(a):
        r=1; e=(1<<n)-2
        while e:
            if e&1: r=mul(r,a)
            a=mul(a,a); e>>=1
        return r
    def tr(a):
        s=a;t=a
        for _ in range(n-1): t=mul(t,t); s^=t
        return s&1
    def htr(c):
        s=c;t=c
        for _ in range((n-1)//2): t=mul(mul(t,t),mul(t,t)) if False else mul(mul(t,t),1); t=mul(t,t); s^=t
        return s
    return inv,tr,htr
def ground(S,base,xr):
    mul=S['mul']; bs=set(base); cnt=0
    for x1 in base:
        for x2 in base:
            pass
    return None
def gt_s3(S,base,xr):
    # brute: ordered pairs with S3=0, via set lookup over quadratic is complex; just double loop with early filter by bits (n small enough)
    mul=S['mul']; c=0; xr2=mul(xr,xr)
    for x1 in base:
        a=mul(x1,x1)^xr2; b=mul(x1,xr); cc=mul(mul(x1,x1),xr2)^1
        for x2 in base:
            if mul(mul(x2,x2),a)^mul(x2,b)^cc==0: c+=1
    return c
def vecs(S,kind,base_meta):
    if kind=='CYC': return [S['g'][a] for a in range(S['p'])]
    if kind=='HAM': return list(S['onb'])
    return base_meta
def system(S,V,xr):
    n=S['n']; mul=S['mul']; xr2=mul(xr,xr); k=len(V)
    M=[[mul(mul(V[a],V[c]),mul(V[a],V[c]))^mul(xr,mul(V[a],V[c])) for c in range(k)] for a in range(k)]
    L=[mul(xr2,mul(V[a],V[a])) for a in range(k)]
    eqs=[]  # each: (quad pairs list, lin1, lin2, const)
    for bit in range(n):
        q=[(a,c) for a in range(k) for c in range(k) if M[a][c]>>bit&1]
        l1=[a for a in range(k) if L[a]>>bit&1]
        eqs.append((q,l1,l1,1 if bit==0 else 0))
    return eqs
def cnf(S,kind,k,eqs,r):
    y=lambda a:1+a; z=lambda a:1+k+a; nv=2*k; aux={}
    cl=[];xs=[]
    def t(a,c):
        nonlocal nv
        if (a,c) not in aux:
            nv+=1; v=nv; aux[(a,c)]=v; cl.extend([[-v,y(a)],[-v,z(c)],[v,-y(a),-z(c)]])
        return aux[(a,c)]
    for q,l1,l2,cst in eqs:
        lits=[t(a,c) for a,c in q]+[y(a) for a in l1]+[z(a) for a in l2]
        if not lits:
            if cst: return None
            continue
        if cst==0: lits[0]=-lits[0]
        xs.append(lits)  # xor of lits = true  <=> sum = cst
    def atmost(vs,R):
        nonlocal nv
        m=len(vs); s=[[0]*(R+1) for _ in range(m)]
        for i in range(m):
            for j in range(1,R+1): nv+=1; s[i][j]=nv
        for i in range(m):
            cl.append([-vs[i],s[i][1]])
            if i>0:
                for j in range(1,R+1):
                    cl.append([-s[i-1][j],s[i][j]])
                    if j>1: cl.append([-vs[i],-s[i-1][j-1],s[i][j]])
                cl.append([-vs[i],-s[i-1][R]])
    for f in (y,z):
        vs=[f(a) for a in range(k)]
        cl.append(vs)
        if kind=='HAM': atmost(vs,r)
        if kind=='CYC':
            atmost(vs,2)
            for a in range(k): cl.append([-vs[a]]+[vs[c] for c in range(k) if c!=a])
    s=f"p cnf {nv} {len(cl)+len(xs)}\n"+"".join(" ".join(map(str,c))+" 0\n" for c in cl)+"".join("x"+" ".join(map(str,c))+" 0\n" for c in xs)
    return s
def msolve_in(S,kind,k,eqs,r):
    names=[f"y{a}" for a in range(k)]+[f"z{a}" for a in range(k)]
    P=[]
    for q,l1,l2,cst in eqs:
        terms=[f"y{a}*z{c}" for a,c in q]+[f"y{a}" for a in l1]+[f"z{a}" for a in l2]+(["1"] if cst else [])
        if terms: P.append("+".join(terms))
    for v in names: P.append(f"{v}^2+{v}")
    for pre in "yz":
        if kind=='HAM':
            for T_ in itertools.combinations(range(k),r+1): P.append("*".join(f"{pre}{a}" for a in T_))
        if kind=='CYC':
            for T_ in itertools.combinations(range(k),3): P.append("*".join(f"{pre}{a}" for a in T_))
            P.append("+".join(f"{pre}{a}*{pre}{c}" for a,c in itertools.combinations(range(k),2))+"+1")
    return ",".join(names)+"\n2\n"+",\n".join(P)+"\n", len(P)
def run_sat(txt):
    fd,fn=tempfile.mkstemp(suffix='.cnf',dir=T); os.write(fd,txt.encode()); os.close(fd)
    t=time.time()
    try:
        o=subprocess.run(['cryptominisat5','--threads','1','--verb','0',fn],capture_output=True,text=True,timeout=TO).stdout
        st='SAT' if 's SATISFIABLE' in o else ('UNSAT' if 's UNSATISFIABLE' in o else 'ERR')
    except subprocess.TimeoutExpired: st='TIMEOUT'
    os.remove(fn); return st, time.time()-t
def run_ms(txt):
    fd,fn=tempfile.mkstemp(suffix='.ms',dir=T); os.write(fd,txt.encode()); os.close(fd); out=fn+'.out'
    t=time.time(); deg=None
    try:
        p=subprocess.run(['msolve','-t','1','-v','2','-g','2','-f',fn,'-o',out],capture_output=True,text=True,timeout=TO)
        steps=[int(m.group(1)) for m in re.finditer(r"^\s*(\d+)\s+\d+\s+\d+\s+\d+ x \d+",p.stdout,re.M)]
        deg=max(steps) if steps else None
        g=open(out).read() if os.path.exists(out) else ''
        gb=re.sub(r"\s","",g)
        st='UNSAT' if re.search(r"\[1\]:?$",gb.rstrip(':')) or gb.endswith('[1]:') else ('CONSISTENT' if p.returncode==0 else 'ERR')
        if p.returncode!=0: st='ERR:rc%d'%p.returncode
    except subprocess.TimeoutExpired: st='TIMEOUT'
    for f in (fn,out):
        if os.path.exists(f): os.remove(f)
    return st, time.time()-t, deg
res={}
plan={11:['CYC2','HAM2','SUB8_s0','SUB6_s0'],23:['CYC2','HAM2','HAM3','SUB8_s0','SUB10_s0','SUB11_s0']}
for n in (11,23):
    S=setup(n); R={}
    for nm in plan[n]:
        base=[int(v) for v in open(f'{T}/base_{n}_{nm}.txt').read().split()]
        kind=nm[:3]; r=int(nm[3]) if kind=='HAM' else 2
        if kind=='SUB':
            kk=int(nm[3:].split('_')[0]); seed=1000*n+int(nm.split('_s')[1]); V,_=subspace(S,kk,seed)
        else: V=vecs(S,kind,None)
        targets=Y[str(n)]['bases'][nm]['targets']
        rows=[]; consec={'sat':0,'ms':0}
        jobs=[]
        for ti,(xr,yr,dec) in enumerate(targets):
            eqs=system(S,V,xr); gt=gt_s3(S,base,xr) if len(base)<=300 else None
            row=dict(t=ti,xr=xr,gt_subgroup_point_decomps=dec,gt_s3_ordered_pairs=gt)
            c=cnf(S,kind,len(V),eqs,r); mtxt,npol=msolve_in(S,kind,len(V),eqs,r); row['msolve_npolys']=npol
            with ThreadPoolExecutor(2) as ex:
                fs=ex.submit(run_sat,c) if consec['sat']<2 else None
                fm=ex.submit(run_ms,mtxt) if consec['ms']<2 else None
                row['sat']=fs.result() if fs else ('TIMEOUT-SKIPPED',None)
                row['ms']=fm.result() if fm else ('TIMEOUT-SKIPPED',None,None)
            consec['sat']=consec['sat']+1 if row['sat'][0]=='TIMEOUT' else (consec['sat'] if row['sat'][0]=='TIMEOUT-SKIPPED' else 0)
            consec['ms']=consec['ms']+1 if row['ms'][0]=='TIMEOUT' else (consec['ms'] if row['ms'][0]=='TIMEOUT-SKIPPED' else 0)
            rows.append(row); print(n,nm,ti,gt,row['sat'],row['ms'],flush=True)
        R[nm]=dict(nvars_per_point=len(V),rows=rows); json.dump(res|{n:R},open(OUT,'w'),indent=0)
    res[n]=R
json.dump(res,open(OUT,'w'),indent=0)
