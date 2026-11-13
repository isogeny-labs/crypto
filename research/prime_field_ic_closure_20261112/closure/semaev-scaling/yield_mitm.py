# O1: success probability and MITM/table oracle cost for interval factor base x<2^s, m=3,4.
import random, math, json, sys
from ec import rand_curve, add, neg, lift, rand_point
from sympy import isprime
def run(bits, m, seed, ntarg):
    rng=random.Random(seed)
    p,a,b=rand_curve(bits,rng)
    s=math.ceil(bits/m)
    V=[]
    for x in range(min(2**s,p)): V+=lift(x,p,a,b)
    Vx=len({P[0] for P in V})
    T2={}
    ops=0
    for i,P in enumerate(V):
        for Q in V:
            S=add(P,Q,p,a); ops+=1
            if S is not None: T2.setdefault(S[0],1)
    setup=ops
    succ=0; qops=0
    for _ in range(ntarg):
        R=rand_point(p,a,b,rng); hit=False
        if m==3:
            for P in V:
                Z=add(R,neg(P,p),p,a); qops+=1
                if Z is not None and Z[0] in T2: hit=True;break
        else:
            for P in V:
                RP=add(R,neg(P,p),p,a); qops+=1
                for Q in V:
                    Z=add(RP,neg(Q,p),p,a); qops+=1
                    if Z is not None and Z[0] in T2: hit=True;break
                if hit: break
        succ+=hit
    # full (non-early-exit) per-query cost = |V| (m=3) or |V|+|V|^2 (m=4)
    Qfull = len(V) if m==3 else len(V)+len(V)**2
    P_s = succ/ntarg
    C = setup + (Vx/ P_s)*Qfull if P_s>0 else float('nan')
    pred = 2**(m*s)/(math.factorial(m)*p) * (1/2)**0  # heuristic, abscissa-level
    return dict(bits=bits,m=m,seed=seed,p=p,s=s,V_points=len(V),V_abscissas=Vx,P_succ=P_s,succ=succ,ntarg=ntarg,
                pred_naive=min(1,2**(m*s)/2**m/ (math.factorial(m))*2**(m-1)/ (p/2) ),setup_ops=setup,query_ops_full=Qfull,total_ops=C,
                rho_neg=math.sqrt(math.pi*p/4))
if __name__=='__main__':
    m=int(sys.argv[1]); out=[]
    for bits in range(10,21,2):
        for seed in range(3):
            r=run(bits,m,1000*bits+seed, 400 if m==3 else 200)
            out.append(r); print(json.dumps(r),flush=True)
    json.dump(out,open(f'yield_m{m}.json','w'),indent=1)
