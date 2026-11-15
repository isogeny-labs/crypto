"""P1: compare Z-model new K-relations (window {0..3}) with L9 analysis/newrel_n<n>_A_s<s>.json (same K-RREF recipe as L9 newrel.py)."""
import sys,json; sys.path.insert(0,'.')
from zmodel import *
sys.path.insert(0,'../../L9-124-falls-structure/src')
from sys9 import base
def lab(key,lo=0):
    e,f=key
    Ia=tuple(i+lo for i in range(64) if e>>i&1); Ib=tuple(i+lo for i in range(64) if f>>i&1); return Ia,Ib
def fmt(Ia,Ib):
    s='*'.join(['a%d'%i for i in Ia]+['b%d'%j for j in Ib]); return s or '1'
def newrels(K,R2,R3,lo=0):
    keys=sorted(set(lab(k,lo) for P in R2+R3 for k in P),key=lambda m:(-(len(m[0])+len(m[1])),m))
    pos={m:i for i,m in enumerate(keys)}
    vec=lambda P: [0]*0 or [dict((pos[lab(k,lo)],v) for k,v in P.items()).get(i,0) for i in range(len(keys))]
    def rref(vs):
        out=[];piv=[]
        for v in vs:
            v=v[:]
            for pc,rv in zip(piv,out):
                if v[pc]: f=v[pc]; v=[a^K.mul(f,b) for a,b in zip(v,rv)]
            nz=[i for i in range(len(v)) if v[i]]
            if not nz: continue
            pc=nz[0]; iv=K.inv(v[pc]); v=[K.mul(iv,x) for x in v]
            for j in range(len(out)):
                if out[j][pc]: f=out[j][pc]; out[j]=[a^K.mul(f,b) for a,b in zip(out[j],v)]
            out.append(v); piv.append(pc)
        return out,piv
    B2,p2=rref([vec(P) for P in R2]); B3,p3=rref([vec(P) for P in R2]+[vec(P) for P in R3])
    return [{fmt(*keys[i]):hex(x) for i,x in enumerate(v) if x} for v,p in zip(B3,p3) if p not in p2], len(B2), len(B3)
if __name__=='__main__':
    n,s=int(sys.argv[1]),int(sys.argv[2]); k=(n+1)//2; m,V,z,a6=base(n,k,s); K=Field(n,m)
    st,R2,R3,_=closure(K,z,a6,0,4,range(0,4))
    new,d2,d3=newrels(K,R2,R3)
    L9=json.load(open('../../L9-124-falls-structure/analysis/newrel_n%d_A_s%d.json'%(n,s)))
    same=[a==b for a,b in zip(new,L9['new_relations'])]
    print(json.dumps(dict(n=n,s=s,dimK2=d2,dimK3=d3,L9=(L9['dimK_R2'],L9['dimK_R3']),new=len(new),identical=same,
        terms=[len(r) for r in new],pivots=[next(iter(r)) for r in new])))
