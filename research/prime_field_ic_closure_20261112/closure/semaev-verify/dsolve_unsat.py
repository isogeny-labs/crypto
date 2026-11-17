# O2 at scale: for UNSAT queries (no Boolean zero) the multilinear Macaulay/Nullstellensatz certificate
# 1 = g*f is unique on the cube (g = 1/f), so D_solve = deg f + deg_ml(1/f) exactly in this model.
import numpy as np, math, random, json, sys, time
from math import comb
from ec import rand_curve, rand_point
from macaulay_bits import coef_tensor, grid_eval, mobius
def popcnt_arr(n):
    idx=np.arange(2**n,dtype=np.int64); c=np.zeros_like(idx)
    for i in range(n): c+= (idx>>i)&1
    return c
def powmod_vec(v,e,p):
    r=np.ones_like(v); b=v.copy()
    while e:
        if e&1: r=r*b%p
        b=b*b%p; e>>=1
    return r
def run(bits,m,seed):
    rng=random.Random(seed); p,a,b=rand_curve(bits,rng); s=math.ceil(bits/m); n=m*s
    X=rand_point(p,a,b,rng)[0]
    C=coef_tensor('S4' if m==3 else 'S5',m,p,a,b,X); F=grid_eval(C,m,s,p)
    vec=np.transpose(F,axes=list(range(m))[::-1]).reshape(-1)%p
    pc=popcnt_arr(n)
    cf=mobius(vec,n,p); nz=cf!=0
    degf=int(pc[nz].max()); terms=int(nz.sum())
    z=int((vec==0).sum())
    r=dict(bits=bits,m=m,seed=seed,p=p,s=s,n=n,deg_f=degf,terms_f=terms,zeros=z)
    if z==0:
        inv=powmod_vec(vec,p-2,p); g=mobius(inv,n,p); gnz=g!=0
        dg=int(pc[gnz].max()); D=degf+dg
        cols=sum(comb(n,i) for i in range(0,min(D,n)+1))
        r.update(deg_inv=dg,terms_inv=int(gnz.sum()),D_solve=D,cols_at_D=cols,log2cols_over_log2p=math.log2(cols)/math.log2(p))
    return r
if __name__=='__main__':
    m=int(sys.argv[1]); lo,hi=map(int,sys.argv[2].split('-')); reps=int(sys.argv[3]); out=[]
    for bits in range(lo,hi+1):
        k=0; seed=0
        while k<reps and seed<reps*20:
            r=run(bits,m,90000+1000*bits+seed); seed+=1
            print(json.dumps(r),flush=True); out.append(r)
            if r['zeros']==0: k+=1
    json.dump(out,open(f'dsolve_unsat_m{m}_{lo}-{hi}.json','w'),indent=1)
