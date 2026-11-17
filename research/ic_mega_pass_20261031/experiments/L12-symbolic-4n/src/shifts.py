"""P3 explicit: in Z-model window {0..L-1}, the shifts tau^j R_i (j=0..L-4) lie in W3∩Win, are K-independent mod W2∩Win,
and together with W2∩Win span all of W3∩Win."""
import sys,json; sys.path.insert(0,'.'); sys.path.insert(0,'../../L9-124-falls-structure/src')
from zmodel import *; from sys9 import base; from evalsym import evalrels
S=json.load(open('../analysis/sym_rref.json'))
def parse(l):
    if l=='1': return (),()
    return tuple(int(t[1:]) for t in l.split('*') if t[0]=='a'),tuple(int(t[1:]) for t in l.split('*') if t[0]=='b')
out={}
for n,s in [(31,2),(33,0),(35,0)]:
    k=(n+1)//2; m,V,z,a6=base(n,k,s); K=Field(n,m); R=evalrels(S['new_raw'],n,m,z,a6)
    for L in [4,5,6,7]:
        st,W2w,W3w,_=closure(K,z,a6,0,L,range(L))
        sh=[]
        for j in range(L-3):
            for r in R:
                P={}
                for l,v in r.items():
                    Ia,Ib=parse(l); key=(sum(1<<(i+j) for i in Ia),sum(1<<(i+j) for i in Ib)); P[key]=K.frob(v,j)
                sh.append(P)
        def rk(rows):
            keys=sorted(set(kk for P in rows for kk in P)); return kech(K,rows,keys,len(keys))[0] if rows else 0
        r2=rk(W2w); r3=rk(W3w); r2s=rk(W2w+sh); r3s=rk(W3w+sh)
        out['n%d_L%d'%(n,L)]=dict(dimW2=r2,dimW3=r3,dim_W2_plus_shifts=r2s,dim_W3_plus_shifts=r3s,nshifts=len(sh),
            shifts_in_W3=(r3s==r3),indep_mod_W2=(r2s-r2==len(sh)),complete=(r2s==r3))
        print(n,L,out['n%d_L%d'%(n,L)],flush=True)
json.dump(out,open('../analysis/p3_shifts.json','w'),indent=0)
