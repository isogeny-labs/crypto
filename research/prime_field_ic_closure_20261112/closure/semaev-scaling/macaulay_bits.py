# O2: low-bit factor base x_i = sum_j 2^j b_ij (b Boolean), one equation S_{m+1}(x_1..x_m, X)=0 over F_p.
# Multilinear Macaulay matrix over F_p in n=m*s Boolean unknowns; D_solve = least D at which the
# row space contains every degree<=1 polynomial vanishing on the zero set (or 1 if no zero).
import numpy as np, math, random, json, sys, time, itertools
import flint
from ec import rand_curve, rand_point, POLYS
def coef_tensor(name, m, p, a, b, X):
    deg = 4 if m==3 else 8
    C = np.zeros((deg+1,)*m, dtype=np.int64)
    for mon,c in POLYS[name].items():
        ea,eb = mon[0],mon[1]; ex = mon[2:2+m]; eX = mon[2+m]
        t = int(c)%p * pow(a,ea,p)*pow(b,eb,p)*pow(X,eX,p) % p
        C[tuple(ex)] = (C[tuple(ex)] + t) % p
    return C
def grid_eval(C, m, s, p):
    xs = np.arange(2**s, dtype=np.int64)
    deg = C.shape[0]-1
    Van = np.ones((2**s, deg+1), dtype=np.int64)
    for e in range(1,deg+1): Van[:,e] = Van[:,e-1]*xs % p
    F = C
    for ax in range(m):
        # contract axis ax of F (exponent) with Van (x,e) -> axis becomes x
        F = np.tensordot(Van, F, axes=([1],[ax])) % p   # new axis 0 = x_ax
        F = np.moveaxis(F, 0, ax)
    return F  # F[x1,...,xm]
def mobius(vec, n, p):
    v = vec.copy()
    for i in range(n):
        step = 1<<i
        v = v.reshape(-1, 2, step)
        v[:,1,:] = (v[:,1,:] - v[:,0,:]) % p
        v = v.reshape(-1)
    return v
def instance(bits, m, seed):
    rng = random.Random(seed)
    p,a,b = rand_curve(bits, rng)
    s = math.ceil(bits/m); n = m*s
    R = rand_point(p,a,b,rng); X = R[0]
    name = 'S4' if m==3 else 'S5'
    C = coef_tensor(name, m, p, a, b, X)
    F = grid_eval(C, m, s, p)            # F[x1..xm], x_i in [0,2^s)
    # flat index with x1 in lowest s bits: idx = sum x_i << (s*i)
    vec = np.transpose(F, axes=list(range(m))[::-1]).reshape(-1) % p
    zeros = np.nonzero(vec==0)[0].tolist()
    coef = mobius(vec, n, p)
    terms = {int(k):int(coef[k]) for k in np.nonzero(coef)[0]}
    d = max(bin(k).count('1') for k in terms)
    # vanishing degree<=1 polys on zero set
    if zeros:
        E = [[1]+[ (z>>j)&1 for j in range(n)] for z in zeros]
        Emat = flint.nmod_mat(E, p)  # |Z| x (n+1)
        # nullspace of Emat: vectors c with E c = 0
        ns, nul = Emat.nullspace()
        L = [[int(ns[i,k]) for i in range(n+1)] for k in range(nul)]
    else:
        L = [[1]+[0]*n]
    res = dict(bits=bits,m=m,seed=seed,p=p,s=s,n=n,X=X,deg_f=d,terms_f=len(terms),cube=2**n,
               zeros=len(zeros), D_solve=None, ranks=[])
    t0=time.time()
    for D in range(d, n+d+1):
        cols = [mk for w in range(0,min(D,n)+1) for mk in masks_w(n,w)]
        cidx = {mk:i for i,mk in enumerate(cols)}
        rows=[]
        for w in range(0, D-d+1):
            if w>n: break
            for T in masks_w(n,w):
                r=[0]*len(cols)
                for S,c in terms.items():
                    j=cidx[S|T]; r[j]=(r[j]+c)%p
                rows.append(r)
        M = flint.nmod_mat(rows, p); rk = M.rank()
        lin=[]
        for l in L:
            r=[0]*len(cols); r[cidx[0]]=l[0]
            for j in range(n): r[cidx[1<<j]]=l[1+j]
            lin.append(r)
        rk2 = flint.nmod_mat(rows+lin, p).rank()
        res['ranks'].append(dict(D=D,rows=len(rows),cols=len(cols),rank=rk,solved=rk2==rk))
        if rk2==rk: res['D_solve']=D; break
        if time.time()-t0>240: res['timeout']=True; break
    res['secs']=round(time.time()-t0,2)
    return res
_mw={}
def masks_w(n,w):
    k=(n,w)
    if k not in _mw: _mw[k]=[sum(1<<i for i in c) for c in itertools.combinations(range(n),w)]
    return _mw[k]
if __name__=='__main__':
    m=int(sys.argv[1]); bitsl=[int(x) for x in sys.argv[2].split(',')]; reps=int(sys.argv[3])
    out=[]
    for bits in bitsl:
        for r in range(reps):
            res=instance(bits,m,7000+100*bits+r); out.append(res)
            print(json.dumps({k:v for k,v in res.items() if k!='ranks'}), flush=True)
    json.dump(out,open(f'macaulay_m{m}_{sys.argv[2].replace(",","-")}.json','w'),indent=1)
