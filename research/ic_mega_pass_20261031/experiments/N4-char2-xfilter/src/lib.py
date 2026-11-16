"""N4-char2-xfilter library: enumeration wrapper, F_2-linear filters, exact single-bucket gain with XOR combiner."""
import os, json, subprocess, hashlib, numpy as np, scipy.fft as sfft
HERE=os.path.dirname(os.path.abspath(__file__)); TMP=os.environ.get('TMPDIR','/tmp')
SENT=np.uint32(0xFFFFFFFF)
def load_curves():
    return [json.loads(l) for l in open(os.path.join(HERE,'..','raw','curves.jsonl'))]
def enumerate_x(c, mode='P', seed=1):
    fn=os.path.join(TMP,f"x_{c['n']}_{c['a']}_{c['b']}_{mode}_{seed}.bin")
    out=subprocess.run([os.path.join(HERE,'enum2'),str(c['n']),str(c['poly']),str(c['a']),str(c['b']),str(c['h']),str(c['l']),mode,str(seed),fn],capture_output=True,text=True,check=True).stdout
    X=np.fromfile(fn,dtype=np.uint32); os.remove(fn); return X, json.loads(out)
def parity(v):
    return (np.bitwise_count(v) & 1).astype(np.int64)
def labels(X, masks):
    """h(k) = (parity(x_k & m_i))_i as int in [0,2^j); x(O) -> label 0."""
    h=np.zeros(len(X),dtype=np.int64)
    for i,m in enumerate(masks): h |= parity(X & np.uint32(m)) << i
    h[X==SENT]=0
    return h
def _wht(a):  # Walsh-Hadamard along axis 0 (len power of 2), unnormalised
    a=a.copy(); M=a.shape[0]; s=1
    while s<M:
        for i in range(0,M,2*s):
            x=a[i:i+s].copy(); y=a[i+s:i+2*s].copy(); a[i:i+s]=x+y; a[i+s:i+2*s]=x-y
        s*=2
    return a
def gain(h, j, want_pi=False):
    """Exact G = M * max_{c,d} pi_c(d), pi_c(d)=Pr[h(P+Q)=c | h(P)^h(Q)^d=c], over all L^2 ordered pairs
    of the cyclic group Z/L (index = dlog). Combiner XOR on F_2^j. Also returns bucket-averaged delta (best d).
    Requires every bucket non-empty (den>0); else returns None (caller must reduce to M_eff)."""
    M=1<<j; L=len(h); nf=sfft.next_fast_len(2*L-1, real=True)
    n=np.bincount(h,minlength=M).astype(np.float64)
    D=np.zeros((M,M))  # D[t,c] = sum_{k:h(k)=c} C_t[k]
    D[0]=n*float(L)    # t=0: C_0[k]=L for all k
    for t in range(1,M):
        g=1.0-2.0*(np.bitwise_count(h & t).astype(np.int64)&1)
        fg=sfft.rfft(g,nf); c=sfft.irfft(fg*fg,nf)
        C=c[:L].copy(); C[:L-1]+=c[L:2*L-1]   # fold linear conv mod L
        D[t]=np.bincount(h,weights=C,minlength=M)
    num=_wht(D)/M          # num[s,c] = #{(P,Q): h(P)^h(Q)=s, h(P+Q)=c}
    den=_wht(_wht(n)**2)/M # den[s] = sum_a n_a n_{a^s}
    if den.min()<=0.5: return None
    pi=num/den[:,None]
    G=M*pi.max(); s,c=np.unravel_index(pi.argmax(),pi.shape)
    delta=max(sum(num[c2^d,c2] for c2 in range(M)) for d in range(M))/float(L)**2
    r={'G':float(G),'M':M,'delta_gain':float(M*delta),'arg_s':int(s),'arg_c':int(c)}
    if want_pi: r['pi']=pi; r['num']=num
    return r
def gain_brute(h,j):
    M=1<<j; L=len(h); num=np.zeros((M,M)); 
    for i in range(L):
        k=(i+np.arange(L))%L; np.add.at(num,(h[i]^h,h[k]),1)
    den=num.sum(1); pi=num/den[:,None]; return float(M*pi.max())
def reduce_masks(X, masks):
    """Drop functionals constant on the domain (M_eff rule, XOR-compatible): returns basis of span(masks) modulo the
    subspace of span elements that are constant on X (excluding x(O)). Returns (reduced_masks, n_const_dims)."""
    Xv=X[X!=SENT]; j=len(masks); const=[]
    for t in range(1,1<<j):
        m=0
        for i in range(j):
            if t>>i&1: m^=masks[i]
        p=parity(Xv & np.uint32(m))
        if p.min()==p.max(): const.append(t)
    if not const: return list(masks),0
    K={0}|set(const); kdim=int(round(np.log2(len(K)))); basis=[]; span=set(K)
    for i in range(j):
        e=1<<i
        if e not in span:
            basis.append(i); span=span|{s^e for s in span}
    return [masks[i] for i in basis], kdim
def sym_perm(X, rng):
    """NULL-1sym: permute x-values over pairs {k,-k}, keeping x(kG)=x(-kG); fixes k=0 and k=L/2."""
    L=len(X); hh=(L-1)//2; perm=rng.permutation(hh)+1
    Y=X.copy(); idx=np.arange(1,hh+1); Y[idx]=X[perm]; Y[L-idx]=X[perm]; return Y
def prf_labels(X, j, key):
    """Keyed PRF null (splitmix64 finaliser of x^key); depends on x only, hence even like the real filter.
    Stands in for the card's SHA null (vectorised; SHA-256 per point is too slow at L=4M in python)."""
    z=(X.astype(np.uint64) ^ np.uint64((key*0x9E3779B97F4A7C15) & 0xFFFFFFFFFFFFFFFF))
    with np.errstate(over='ignore'):
        z=z+np.uint64(0x9E3779B97F4A7C15); z=(z^(z>>np.uint64(30)))*np.uint64(0xBF58476D1CE4E5B9)
        z=(z^(z>>np.uint64(27)))*np.uint64(0x94D049BB133111EB); z=z^(z>>np.uint64(31))
    h=(z>>np.uint64(40)).astype(np.int64) & ((1<<j)-1); h[X==SENT]=0; return h

# ---------------- search / evaluation helpers
def eval_masks(X, masks):
    rm,kd=reduce_masks(X, masks)
    if not rm: return {'G':float('nan'),'M':1,'E':float('nan'),'degenerate':True,'masks':[int(m) for m in masks]}
    r=gain(labels(X,rm),len(rm))
    if r is None: return {'G':float('nan'),'M':1<<len(rm),'E':float('nan'),'empty_bucket':True,'masks':[int(m) for m in masks]}
    r['E']=r['G']-1; r['M_eff']=r['M']; r['const_dims']=kd; r['masks']=[int(m) for m in masks]; return r
def climb(X, n, j, seed, R=6, S=4):
    """B=R*S evaluations; proposal draws independent of acceptance."""
    rng=np.random.default_rng(seed); best=None; trace=[]
    for r in range(R):
        cur=[int(v) for v in rng.integers(1,1<<n,j)]; ev=eval_masks(X,cur); trace.append(ev['E'])
        curE=ev['E'] if ev['E']==ev['E'] else -1
        if best is None or curE>best['E']: best=dict(ev)
        for s in range(S-1):
            i=int(rng.integers(0,j)); b=int(rng.integers(0,n)); cand=list(cur); cand[i]^=(1<<b)
            if cand[i]==0: cand[i]=1<<b
            ev=eval_masks(X,cand); trace.append(ev['E']); e=ev['E'] if ev['E']==ev['E'] else -1
            if e>curE: cur,curE=cand,e
            if e>best['E']: best=dict(ev)
    best.pop('pi',None); best.pop('num',None); best['evals']=len(trace); return best
def scan_j1(X, n):
    """Exhaustive over all 2^n-1 masks, j=1. Returns array E[m] (nan = degenerate)."""
    E=np.full(1<<n,np.nan)
    for m in range(1,1<<n):
        h=labels(X,[m])
        if h.min()==h.max() or (h[1:].min()==h[1:].max()): continue
        r=gain(h,1); E[m]=r['G']-1 if r else np.nan
    return E
def plant_x(X, mstar, u, rho):
    L=len(X); w=int(round(rho*L/2)); Y=X.copy()
    idx=np.concatenate([np.arange(1,w+1), L-np.arange(1,w+1)])
    sel=idx[parity(Y[idx]&np.uint32(mstar))==1]; Y[sel]^=np.uint32(u); return Y
def plant_labels(h, rho):
    L=len(h); w=int(round(rho*L/2)); h=h.copy(); h[np.arange(-w,w+1)%L]=0; return h
def trmask(c):
    import cypari2; pari=cypari2.Pari(); n=c['n']
    pari(f"T=Pol(binary({c['poly']}),'w)"); pari("w=ffgen(T*Mod(1,2),'w)")
    return int(pari(f"sum(i=0,{n}-1,lift(trace(w^i))*2^i)"))
def pick_u(rng, n, mstar, tm):
    """plant offset u: parity(u&m*)=1 (flips the planted functional) and Tr(u)=0 (keeps Tr(x) constant on <P>,
    i.e. the plant does not break the curve's own known invariant) -- AMENDMENT 1."""
    while True:
        u=int(rng.integers(1,1<<n))
        if bin(u&mstar).count('1')%2==1 and bin(u&tm).count('1')%2==0: return u
def plant_x_multi(X, mstars, tm, rho, n):
    """Force all planted functionals m*_i to 0 on dlog interval T, keeping Tr(x) (AMENDMENT 2).
    v_i: parity(v_i&m*_l)=delta_il, parity(v_i&tm)=0; x_k ^= XOR_{i: parity(x_k&m*_i)=1} v_i."""
    import itertools
    rows=list(mstars)+[tm]; J=len(mstars); V=[]
    # brute small search over random u for each target pattern
    rng=np.random.default_rng(11)
    for i in range(J):
        while True:
            u=int(rng.integers(1,1<<n))
            if all(bin(u&rows[l]).count('1')%2==(1 if l==i else 0) for l in range(J+1)): V.append(u); break
    L=len(X); w=int(round(rho*L/2)); Y=X.copy(); idx=np.concatenate([np.arange(1,w+1), L-np.arange(1,w+1)])
    for i in range(J):
        sel=idx[parity(Y[idx]&np.uint32(mstars[i]))==1]; Y[sel]^=np.uint32(V[i])
    return Y
