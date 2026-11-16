"""L13 driver. jobs: gen:<n>:<k>:<arm>:<s>:<engine>  (engine gf2close|macclose)  or  syn:<n>:<N>:<seed> (synthetic
rank-n triple system g_r = lam_r mu_r + ell_r + c_r, generator-independent, macclose). <=3 concurrent processes."""
import sys, os, json, time, random, subprocess
from concurrent.futures import ThreadPoolExecutor
HERE=os.path.dirname(os.path.abspath(__file__)); EXP=os.path.join(HERE,'..','..')
sys.path.insert(0,os.path.join(EXP,'N1-gf2-d4-profile','src')); import lib
MAC=os.path.join(EXP,'L8-n33-control','src','macclose')
def mac(eqs,N,d):
    s="%d %d %d\n"%(N,d,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in e)+"\n" for e in eqs)
    r=subprocess.run([MAC],input=s,capture_output=True,text=True)
    if r.returncode: raise RuntimeError(r.stderr)
    o=json.loads(r.stdout); o['log']=r.stderr.strip().split('\n'); return o
def bmul(p,q):
    out=set()
    for a in p:
        for b in q: out^={a|b}
    return out
def syn(n,N,seed):
    rng=random.Random(seed); eqs=[]
    lin=lambda: {1<<i for i in range(N) if rng.getrandbits(1)}|({0} if rng.getrandbits(1) else set())
    for _ in range(n):
        e=bmul(lin(),lin())^lin()
        if rng.getrandbits(1): e^={0}
        eqs.append(sorted(e))
    return eqs
def job(spec):
    p=spec.split(':'); t=time.time()
    if p[0]=='syn':
        n,N,seed=map(int,p[1:]); r=mac(syn(n,N,seed),N,4); rec=dict(kind='syn',n=n,N=N,seed=seed,engine='macclose')
    else:
        n,k,arm,s,eng=int(p[1]),int(p[2]),p[3],int(p[4]),p[5]; N=2*k
        eqs=lib.system(n,k,s)[arm]
        r=mac(eqs,N,4) if eng=='macclose' else lib.call('closure',N,4,eqs)
        rec=dict(kind='gen',n=n,k=k,N=N,arm=arm,s=s,engine=eng)
    rec.update({x:r[x] for x in r}); rec['wall']=round(time.time()-t,1); return rec
if __name__=='__main__':
    out=sys.argv[1]; specs=sys.argv[2:]
    with ThreadPoolExecutor(3) as ex, open(out,'a') as f:
        for rec in ex.map(job,specs):
            f.write(json.dumps(rec)+'\n'); f.flush()
            print({x:rec[x] for x in rec if x!='log'},flush=True)
