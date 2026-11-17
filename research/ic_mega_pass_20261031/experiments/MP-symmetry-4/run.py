import numpy as np, json, time, sys, math
T0=time.time()
POLY={7:(1<<7)|3,11:(1<<11)|5,13:(1<<13)|0b11011,17:(1<<17)|9,19:(1<<19)|0b100111}
class F:
    def __init__(s,n):
        s.n=n; s.q=1<<n; p=POLY[n]; N=s.q-1
        for g in range(2,s.q):
            exp=np.zeros(2*N,dtype=np.int64); v=1; ok=True
            for i in range(N):
                exp[i]=v
                v2=0;a=v;b=g
                while b:
                    if b&1: v2^=a
                    b>>=1;a<<=1
                    if a>>n: a^=p
                v=v2
                if v==1 and i<N-1: ok=False;break
            if ok and v==1: break
        exp[N:]=exp[:N]; s.exp=exp; s.log=np.zeros(s.q,dtype=np.int64); s.log[exp[:N]]=np.arange(N); s.N=N
        sq=exp[(2*np.arange(N))%N]; # trace via sum of conjugates on basis
        tr=np.zeros(s.q,dtype=np.int64)
        tb=[]
        for i in range(n):
            x=1<<i; acc=0; y=x
            for _ in range(n): acc^=y; y=s.mul(y,y)
            assert acc in (0,1); tb.append(acc)
        s.tmask=sum(1<<i for i in range(n) if tb[i])
        idx=np.arange(s.q); s.TR=np.array([bin(int(v)&s.tmask).count('1')&1 for v in idx],dtype=np.int8)
    def mul(s,a,b):
        if a==0 or b==0: return 0
        return int(s.exp[s.log[a]+s.log[b]])
    def inv(s,a): return int(s.exp[(s.N-s.log[a])%s.N])
    def tr(s,a): return int(s.TR[a])
    def sqrt(s,b): return int(s.exp[(s.log[b]*(1<<(s.n-1)))%s.N])
def Zset(f,a,b):
    beta=f.sqrt(b); x=np.arange(1,f.q); lx=f.log[x]
    binv=f.exp[(f.log[beta]-lx)%f.N]
    z=np.zeros(f.q,dtype=bool); z[1:]=(f.TR[x]^f.TR[binv])==f.tr(a); return z,beta,binv
def direct(f,a,b):
    img=np.zeros(f.q,dtype=bool); zz=np.arange(f.q)
    sq=np.where(zz>0,f.exp[(2*f.log[zz])%f.N],0); img[sq^zz]=True
    x=np.arange(1,f.q); lx=f.log[x]
    x3=f.exp[(3*lx)%f.N]; ax2=f.exp[(f.log[a]+2*lx)%f.N] if a else 0
    rhs=x3^ax2^b; c=np.where(rhs>0,f.exp[(f.log[np.maximum(rhs,1)]-2*lx)%f.N],0)
    d=np.zeros(f.q,dtype=bool); d[1:]=img[c]; return d
def count_exact(Z,l,collect=False):
    q=len(Z); idx=np.arange(q); res=[0]; out=[]
    def rec(M,span,piv,last,depth):
        cand=M & ((idx & piv)==0) & (idx>last)
        if depth==l-1:
            res[0]+=int(cand.sum())
            if collect:
                for v in np.nonzero(cand)[0]: out.append(span+[x^int(v) for x in span]+[int(v)] if False else sorted(set(span)|{int(v)^s for s in span}|{int(v)}))
            return
        for v in np.nonzero(cand)[0]:
            v=int(v); M2=M & M[idx^v]
            ns=span+[v^s for s in span]+[v]
            rec(M2,ns,piv|(1<<(v.bit_length()-1)),v,depth+1)
    rec(Z.copy(),[],0,0,0); return res[0],out
def count_sis(Z,l,chains,rng):
    q=len(Z); idx=np.arange(q); w=np.zeros(chains)
    for c in range(chains):
        M=Z.copy(); piv=0; last=0; wt=1.0
        for d in range(l):
            cand=np.nonzero(M & ((idx&piv)==0) & (idx>last))[0]
            if len(cand)==0: wt=0;break
            wt*=len(cand)
            if d==l-1: break
            v=int(cand[rng.integers(len(cand))]); M=M&M[idx^v]; piv|=1<<(v.bit_length()-1); last=v
        w[c]=wt
    return w.mean(), w.std()/math.sqrt(chains)
def gauss(n,l):
    num=1;den=1
    for i in range(l): num*=(2**(n-i)-1); den*=(2**(i+1)-1)
    return num//den
def null_exact(n,l,k):
    p=1.0
    for i in range(2**l-1): p*=(k-i)/(2**n-1-i)
    return gauss(n,l)*p
def shuffle(f,Z,beta,binv,rng):
    # orbits {x,beta/x}
    x=np.arange(1,f.q); partner=np.zeros(f.q,dtype=np.int64); partner[1:]=binv
    seen=np.zeros(f.q,bool); pairs=[]; fixed=[]
    for v in range(1,f.q):
        if seen[v]: continue
        p=int(partner[v]); seen[v]=seen[p]=True
        (fixed if p==v else pairs).append((v,p))
    labs=[Z[a] for a,_ in pairs]; perm=rng.permutation(len(pairs))
    Z2=np.zeros(f.q,bool)
    for (a,b_),j in zip(pairs,perm):
        if labs[j]: Z2[a]=Z2[b_]=True
    for (a,_) in fixed: Z2[a]=Z[a]
    return Z2
def lstar(Z,budget,rng,lmax=10):
    q=len(Z); idx=np.arange(q); best=[1 if Z.any() else 0]; nodes=[0]
    def rec(M,piv,last,d):
        if d>best[0]: best[0]=d
        nodes[0]+=1
        if nodes[0]>budget: return
        cand=np.nonzero(M & ((idx&piv)==0) & (idx>last))[0]
        if len(cand)==0: return
        # need at least 2^(target-d)-1 candidates heuristic: skip
        rng.shuffle(cand)
        for v in cand[:64]:
            v=int(v); rec(M&M[idx^v],piv|(1<<(v.bit_length()-1)),0,d+1)
            if nodes[0]>budget: return
    rec(Z.copy(),0,0,0); return best[0]
R={"seeds":{"curves":"20261031+n","sis":7,"shuffle":99,"lstar":5},"cells":{}}
def curves(f,rng):
    cs=[(int(rng.integers(1,f.q)),1)]
    while len(cs)<9: cs.append((int(rng.integers(0,f.q)),int(rng.integers(1,f.q))))
    return cs
sis_rng=np.random.default_rng(7); sh_rng=np.random.default_rng(99); ls_rng=np.random.default_rng(5)
for n in [7,11,13,17,19]:
    f=F(n); rng=np.random.default_rng(20261031+n); cs=curves(f,rng); cell=R["cells"][str(n)]={"curves":[]}
    for ci,(a,b) in enumerate(cs):
        Z,beta,binv=Zset(f,a,b); k=int(Z.sum()); c={"a":a,"b":b,"Tr_a":f.tr(a),"Z":k}
        c["identity_mismatch"]=int((direct(f,a,b)[1:]!=Z[1:]).sum())
        if n==7:
            # brute-force validation of counter on l=2,3
            pts=np.nonzero(Z)[0]; bf=0
            for i in pts:
                for j in pts:
                    if j>i and Z[i^j]: bf+=1
            c["l2_bruteforce"]=bf//3; c["l2_dfs"]=count_exact(Z,2)[0]
        if n in (11,13):
            for l in ([2,3,4] if n==11 else ([2,3] if ci<2 else [2])):
                t=time.time(); cnt,Vs=count_exact(Z,l,collect=(n==11 and l in(2,3)))
                c[f"l{l}"]={"obs":cnt,"null":null_exact(n,l,k),"sec":round(time.time()-t,1)}
                if n==11 and l in (2,3):
                    sig_ok=0;sig0=0;closed=0
                    for V in Vs:
                        s=0
                        for x in V: s^=f.inv(x)
                        sig0+= s==0
                        sig_ok+= (f.tr(f.mul(beta,s))==f.tr(a))
                        img=set(int(binv[x-1]) for x in V)
                        closed+= all((u^w) in img or (u^w)==0 for u in img for w in img)
                    c[f"l{l}"].update(sigma_ok=sig_ok,sigma_zero=int(sig0),betaV_closed=closed,nV=len(Vs))
            if n==11 and ci<3:
                for l in (3,4):
                    c[f"shuffle_l{l}"]=[count_exact(shuffle(f,Z,beta,binv,sh_rng),l)[0] for _ in range(2)]
                if ci<3: m,se=count_sis(Z,4,20000,sis_rng); c["sis_l4"]=[m,se]
                if ci==0:
                    Zs=shuffle(f,Z,beta,binv,sh_rng); b0=count_exact(Zs,4)[0]
                    basis=[int(v) for v in sh_rng.integers(1,f.q,4)]
                    V={0}
                    for v in basis: V|={v^u for u in V}
                    Zp=Zs.copy()
                    for v in V:
                        if v: Zp[v]=True
                    c["planted"]={"dimV":int(math.log2(len(V))),"before":b0,"after":count_exact(Zp,4)[0]}
            if n==13:
                m,se=count_sis(Z,3,10000,sis_rng); c["l3_sis"]={"est":m,"se":se,"null":null_exact(n,3,k)}
                m,se=count_sis(Z,4,20000,sis_rng); c["l4_sis"]={"est":m,"se":se,"null":null_exact(n,4,k)}
        if ci<3:
            c["lstar"]=lstar(Z,20000,ls_rng); c["lstar_shuffled"]=lstar(shuffle(f,Z,beta,binv,sh_rng),20000,ls_rng)
            c["lstar_formula"]=max(l for l in range(1,12) if l<=n and null_exact(n,l,k)>=1)
        cell["curves"].append(c); print(n,ci,json.dumps(c),round(time.time()-T0),flush=True)
R["runtime_s"]=round(time.time()-T0,1); R["numpy"]=np.__version__; R["python"]=sys.version.split()[0]
json.dump(R,open("RESULTS.json","w"),indent=1)
