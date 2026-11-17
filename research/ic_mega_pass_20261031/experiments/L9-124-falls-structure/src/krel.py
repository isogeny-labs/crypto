"""K-relation search. T(window) = F2-span of Tr(theta_t * a^{e} b^{f}) over K-monomials a^e b^f, e=sum 2^i (i in I_a),
f=sum 2^j (j in I_b), I_a,I_b subsets of window, |I_a|+|I_b|<=3 (bit-degree <=3).  E_r = W^(r)_low ∩ T."""
import sys, itertools
from an9 import *
from tensor_closure import pmulmod, modulus
def field(n):
    m=modulus(n); mul=lambda x,y: pmulmod(x,y,m,n)
    def frob(x,i):
        i%=n
        for _ in range(i): x=mul(x,x)
        return x
    trm=0
    for t in range(n):
        x=1<<t; s=0
        for i in range(n): s^=frob(x,i)
        assert s in (0,1); trm|=s<<t
    return mul,frob,trm
def subspaces(n,k,s,name):
    m,V,z,a6=base(n,k,s); V2=V
    if name=='A_V1V2':
        import random; from tensor_closure import rand_subspace
        V2=rand_subspace(n,k,random.Random(9_000_000+1000*n+100*k+s))
    return V,V2
def kmono(Ia,Ib,V,V2,k,mul,frob):
    p={0:1}
    for (blk,i) in [(0,i) for i in Ia]+[(1,j) for j in Ib]:
        VV=V if blk==0 else V2
        lin={1<<(j+k*blk):frob(VV[j],i) for j in range(k)}
        q={}
        for m1,c1 in p.items():
            for m2,c2 in lin.items():
                mm=m1|m2; q[mm]=q.get(mm,0)^mul(c1,c2)
        p={mm:c for mm,c in q.items() if c}
    return p
def trace_rows(p,n,idx,mul,trm):
    rows=[]
    for t in range(n):
        r=0
        for mm,c in p.items():
            if bin(mul(1<<t,c)&trm).count('1')&1: r^=1<<idx[mm]
        rows.append(r)
    return rows
def run(n,name,s,pref,window,rds=(2,3)):
    k=(n+1)//2; mul,frob,trm=field(n); V,V2=subspaces(n,k,s,name)
    monos,_=load(pref,0); idx={m:i for i,m in enumerate(monos)}
    Ws={rd:basis(load(pref,rd)[1]).full() for rd in rds}
    labels=[];T=[]
    W=list(window)
    for da in range(4):
        for db in range(4-da):
            for Ia in itertools.combinations(W,da):
                for Ib in itertools.combinations(W,db):
                    p=kmono(Ia,Ib,V,V2,k,mul,frob)
                    for t,r in enumerate(trace_rows(p,n,idx,mul,trm)): T.append(r); labels.append((Ia,Ib,t))
    out={'window':W,'nT':len(T)}
    TB=basis(T); out['dimT']=len(TB)
    E={}
    for rd in rds:
        WB=Ws[rd]; res=Basis(); comb={}  # residue elimination with tracking
        elems=[]
        for i,r in enumerate(T):
            rr=WB.nf(r); c=1<<i
            while rr:
                lb=rr&-rr; q=res.p.get(lb)
                if q is None: res.p[lb]=rr; comb[lb]=c; break
                rr^=q; c^=comb[lb]
            if not rr:
                e=0; cc=c; j=0
                while cc:
                    lb=cc&-cc; e^=T[lb.bit_length()-1]; cc^=lb
                elems.append(e)
        E[rd]=elems
        out['dim_W%d_cap_T'%rd]=len(basis(elems))
    if 2 in Ws and 3 in Ws:
        B=basis(list(Ws[2].p.values())); before=len(B)
        for e in E[3]: B.add(e)
        out['dimW2']=before; out['dimW3']=len(Ws[3]); out['dim_W2_plus_E3']=len(B)
        out['explained_of_new']=len(B)-before; out['new']=len(Ws[3])-before
    return out,E,labels,T
if __name__=='__main__':
    n,name,s=int(sys.argv[1]),sys.argv[2],int(sys.argv[3]); pref=sys.argv[4]; win=[int(x) for x in sys.argv[5].split(',')]
    out,_,_,_=run(n,name,s,pref,win); out.update(n=n,arm=name,s=s); print(json.dumps(out))
