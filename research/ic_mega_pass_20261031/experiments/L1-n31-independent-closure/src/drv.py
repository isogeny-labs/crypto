import sys, os, json, subprocess, itertools
HERE=os.path.dirname(os.path.abspath(__file__))
N1=os.path.join(HERE,'..','..','N1-gf2-d4-profile','src'); sys.path.insert(0,N1)
import lib                                  # N1 generator + gf2close wrapper (read-only use)
BIN=os.path.join(HERE,'macclose')
def mac(eqs,N,d):
    s="%d %d %d\n"%(N,d,len(eqs))+"".join(str(len(e))+"".join(" %d"%m for m in e)+"\n" for e in eqs)
    r=subprocess.run([BIN],input=s,capture_output=True,text=True)
    if r.returncode: raise RuntimeError(r.stderr)
    out=json.loads(r.stdout); out['log']=r.stderr.strip().split('\n'); return out
def pyref(eqs,N,d):
    """tiny brute reference: dict-of-sets, fixpoint on full span recomputation"""
    monos=[sum(1<<i for i in c) for e in range(d+1) for c in itertools.combinations(range(N),e)]
    def vec(p): return sum(1<<monos.index(m) for m in p)
    deg=lambda m: bin(m).count('1')
    def rk(vs):
        piv={}
        for v in vs:
            while v:
                h=v.bit_length()-1
                if h in piv: v^=piv[h]
                else: piv[h]=v; break
        return piv
    gens=[vec(e) for e in eqs]
    while True:
        piv=rk(gens); low=[v for v in piv.values() if all(deg(monos[i])<d for i in range(len(monos)) if v>>i&1)]
        # low space = span of basis elements of degree<d is not enough in general: use echelon by degree order (monos sorted by degree ascending, pivot=highest bit) -> rows with top bit in low block span W∩P<d
        new=[]
        for g in low:
            for v in range(N):
                p=set()
                for i in range(len(monos)):
                    if g>>i&1: p^={monos[i]|(1<<v)}
                new.append(vec(p))
        piv2=rk(gens+new)
        if len(piv2)==len(piv):
            one=1 if rk(list(piv.values())+[1]) and len(rk(list(piv.values())+[1]))==len(piv) else 0
            return dict(dim=len(piv),low=len(low),one=one)
        gens=list(piv2.values())
