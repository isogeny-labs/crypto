import sys, random, itertools, json, time
import numpy as np
sys.path.insert(0,'/Volumes/SSD990-2/crypto-autoresearcher/research/verification')
import ffd_semaev_core as C
C.MODS.update({17:(1<<17)|(1<<3)|1, 15:(1<<15)|2|1, 11:(1<<11)|(1<<2)|1, 12:(1<<12)|(1<<6)|(1<<4)|(1<<1)|1, 13:(1<<13)|(1<<4)|(1<<3)|(1<<1)|1})
pc=lambda x: bin(x).count('1')
def table(eqs,N):
    pts=np.arange(1<<N,dtype=np.uint32); ok=np.zeros(1<<N,bool)
    for f in eqs:
        v=np.zeros(1<<N,np.uint8)
        for m in f: v^=((pts&m)==m).astype(np.uint8)
        ok|=v.astype(bool)
    return ok
def run(eqs,N,Dmax,dyn=True):
    monos=sorted([m for d in range(Dmax+1) for m in map(lambda c:sum(1<<i for i in c),itertools.combinations(range(N),d))],key=lambda m:(pc(m),m))
    idx={m:i for i,m in enumerate(monos)}
    start={d:min([i for i,m in enumerate(monos) if pc(m)==d] or [len(monos)]) for d in range(Dmax+2)}
    sm={d:[m for m in monos if pc(m)<=d] for d in range(Dmax+1)}
    res={'NS':None,'F':None}
    for D in range(2,Dmax+1):
        piv={}
        def add(r):
            while r:
                t=r.bit_length()-1
                if t in piv: r^=piv[t]
                else: piv[t]=r; return True
            return False
        for f in eqs:
            for m in sm[D-2]:
                r=0
                for mm in f: r^=1<<idx[mm|m]
                add(r)
        def has1():
            r=1<<idx[0]
            while r:
                t=r.bit_length()-1
                if t in piv: r^=piv[t]
                else: return False
            return True
        if res['NS'] is None and has1(): res['NS']=D
        if dyn and res['F'] is None:
            while True:
                if has1(): res['F']=D; break
                low=[r for t,r in piv.items() if t<start[D]]
                new=False
                for r in low:
                    for v in range(N):
                        out=0; x=r
                        while x:
                            b=x&-x; i=b.bit_length()-1; x^=b
                            out^=1<<idx[monos[i]|(1<<v)]
                        if add(out): new=True
                if not new: break
        if res['NS'] is not None and (not dyn or res['F'] is not None): break
    return res
def inst(n,k,rng,kind):
    while True:
        basis=[];red=[]
        while len(basis)<k:
            x=rng.randrange(1,1<<n); b=x
            for p in red: b=min(b,b^p)
            if b: red.append(b); basis.append(x)
        xR=rng.randrange(1,1<<n)
        eqs=C.build_system(n,k,rng.randrange(1,1<<n),xR,basis)
        N=2*k
        if kind=='null':
            out=[]
            for f in eqs:
                aff={m for m in f if pc(m)<=1}
                bil={(1<<i)|(1<<(k+j)) for i in range(k) for j in range(k) if rng.random()<.5}
                out.append(frozenset(aff^bil))
            eqs=out
        if table(eqs,N).all(): return eqs,N
if __name__=='__main__':
    n=int(sys.argv[1]); reps=int(sys.argv[2]); Dm=int(sys.argv[3]); dyn=int(sys.argv[4])
    k=(n+1)//2; rng=random.Random(n)
    for kind in ('semaev','null'):
        out=[]
        for _ in range(reps):
            t=time.time(); eqs,N=inst(n,k,rng,kind)
            r=run(eqs,N,Dm,bool(dyn)); r['t']=round(time.time()-t,1); out.append(r)
        print(n,k,kind,out,flush=True)
