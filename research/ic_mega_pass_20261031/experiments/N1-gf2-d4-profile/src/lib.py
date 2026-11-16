import sys, random, json, subprocess, os, itertools
HERE=os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, os.path.join(HERE,'..','..','MP-comm_alg-1'))
sys.path.insert(0,'/Volumes/SSD990-2/crypto-autoresearcher/research/verification')
from run import arms, ARMS          # MP-comm_alg-1 generator (unchanged)
import ffd_semaev_core as CORE
BIN=os.path.join(HERE,'gf2close')
def system(n,k,s):
    """All arms for draw s, exactly as MP-comm_alg-1 run.py (full ARMS list -> same rng stream)."""
    rng=random.Random(1000*n+100*k+s)
    sy=arms(n,k,rng,ARMS)
    rrng=random.Random(7_000_000+1000*n+100*k+s)     # independent stream for the null
    sy['RM']=CORE.random_matched(list(sy['A']),2*k,rrng)
    return sy
def enc(mode,N,d,eqs):
    eqs=[e for e in eqs]
    return "%s %d %d\n%d\n"%(mode,N,d,len(eqs))+"".join("%d %s\n"%(len(e)," ".join(map(str,sorted(e)))) for e in eqs)
def call(mode,N,d,eqs,chunk=1024):
    r=subprocess.run([BIN,str(chunk)],input=enc(mode,N,d,eqs),capture_output=True,text=True)
    if r.returncode: raise RuntimeError(r.stderr)
    return json.loads(r.stdout)
def rank_s(sols,N,d):
    """rank of the degree<=d-1 monomial evaluation matrix on the solution points"""
    monos=[sum(1<<i for i in c) for e in range(d) for c in itertools.combinations(range(N),e)]
    piv={}; r=0
    for x in sols:
        row=0
        for i,m in enumerate(monos):
            if x&m==m: row|=1<<i
        while row:
            p=row.bit_length()-1
            if p in piv: row^=piv[p]
            else: piv[p]=row; r+=1; break
    return r
