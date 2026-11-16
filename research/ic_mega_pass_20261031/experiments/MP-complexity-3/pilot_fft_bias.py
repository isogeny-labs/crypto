import numpy as np, cypari2, sys
P=cypari2.Pari(); P.allocatemem(10**9) if False else None
rng=np.random.default_rng(1)
def run(bits,ds,npred=200,prime_order=True):
    while True:
        p=int(P.nextprime(int(rng.integers(2**bits,2**(bits+1)))))
        a,b=int(rng.integers(1,p)),int(rng.integers(1,p))
        E=P.ellinit([a,b],p)
        if int(P.ellcard(E))==0: continue
        N=int(P.ellcard(E))
        if (not prime_order) or P.isprime(N): break
    G=P.random(E)
    # enumerate k*G via gp loop
    xs=np.array(P(f"my(E=ellinit([{a},{b}],{p}),G={G},Q=[0],v=vector({N-1}));Q=G;for(i=1,{N-1},if(#Q==1,v[i]=-1,v[i]=Q[1]);Q=elladd(E,Q,G));v"),dtype=np.int64) if False else None
    s=f"E=ellinit([{a},{b}],{p});G={G};Q=G;v=vector({N-1});for(i=1,{N-1},if(#Q==1,v[i]=-1,v[i]=lift(Q[1]));Q=elladd(E,Q,G));"
    P(s); xs=np.array([int(t) for t in P("v")],dtype=np.int64)
    X=np.concatenate([[-1],xs])  # index k=0..N-1, k=0 is infinity
    return p,N,X
def leg_table(p):
    t=np.zeros(p,dtype=np.int8); sq=(np.arange(p,dtype=np.int64)**2)%p; t[:]=-1; t[sq]=1; t[0]=0; return t
def bias(g):
    # g in {+1,-1} over Z/N; joint cond bias via FFT
    N=len(g); u=(g==1).astype(float); m=u.mean()
    F=lambda v:np.fft.fft(v)
    res=[]
    for ia in (0,1):
      for ib in (0,1):
        A=u if ia else 1-u; B=u if ib else 1-u
        # count of (a,b) with A[a]B[b]u[a+b]
        # sum_a,b A[a]B[b]u[a+b] = (1/N) sum_xi conj(Â) conj(B̂)?? use direct: conv(A,B)[c]*u[c]
        c=np.fft.ifft(F(A)*F(B)).real
        num=(c*u).sum(); den=A.sum()*B.sum()
        res.append(num/den-m)
    return m-0.5,max(abs(r) for r in res)
if __name__=="__main__":
    for bits in (14,16,18):
        for po in (True,False):
            p,N,X=run(bits,None,prime_order=po)
            L=leg_table(p); rng2=np.random.default_rng(bits)
            out=[]
            for d in (1,2,4,8,16):
                best=0;bm=0
                for _ in range(60):
                    co=rng2.integers(0,p,d+1)
                    xv=np.where(X<0,0,X)
                    val=np.zeros(len(X),dtype=np.int64)
                    for c in co: val=(val*xv+c)%p
                    g=L[val].astype(float); g[X<0]=1; g[g==0]=1
                    m,bb=bias(g); best=max(best,bb); bm=max(bm,abs(m))
                out.append((d,round(best*np.sqrt(p)/d,4),round(bm*np.sqrt(p)/d,3)))
            print(bits,p,N,"prime" if po else "any",out,flush=True)
    # structured: 2-torsion Kummer predicate
