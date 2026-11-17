"""Validation: (a) reduction of the lifted short model mod p carries the lifted points to
points whose discrete logs (relative to point 1) equal c_i/c_1 mod q; (b) height
truncation check (n vs n+1 doublings) on a few trials.  [pcc:unverified]"""
import sys, json, random, math, time
sys.path.insert(0, '.')
import xedni_toy as X
from heights import height_data

def check_reduction(bits, r, ntr, seed=7):
    rng = random.Random(seed*100+bits)
    tg = X.make_target(bits, rng)
    p,A,B,q,S = tg['p'],tg['A'],tg['B'],tg['q'],tg['S']
    ok=0; tot=0
    for it in range(ntr):
        k = rng.randrange(1,q); T = X.ec_mul(k,S,A,p)
        s=[rng.randrange(q) for _ in range(r)]; t=[rng.randrange(q) for _ in range(r)]
        cs=[(si-k*ti)%q for si,ti in zip(s,t)]
        if any(c==0 for c in cs): continue
        P=[X.ec_mul(c,S,A,p) for c in cs]
        pts=[(X.balanced(Q[0],p),X.balanced(Q[1],p)) for Q in P]
        if len(set(pts))<r: continue
        u,why=X.fit_curve(pts,p,A,B)
        if u is None: continue
        sh=X.to_short(u,pts)
        a=sh['As']%p; b=sh['Bs']%p
        assert X.count_points(a,b,p)==q, 'reduced curve order mismatch'
        P1=(sh['spts'][0][0]%p, sh['spts'][0][1]%p)
        tab=X.bsgs_table(P1,q,a,p)
        inv1=pow(cs[0],-1,q)
        good=all(X.bsgs(tab,(x%p,y%p),q,a,p)==cs[i]*inv1%q for i,(x,y) in enumerate(sh['spts']))
        ok+=good; tot+=1
    return ok,tot

for bits in (8,12,16):
    for r in (2,4,6):
        print('reduction check', bits, r, check_reduction(bits, r, 15))

# height truncation check
rng = random.Random(99)
for bits in (8,16):
    tg = X.make_target(bits, rng)
    for r in (3,6):
        rec = None
        seed = 12345+bits+r
        rr = random.Random(seed)
        p,A,B,q,S = tg['p'],tg['A'],tg['B'],tg['q'],tg['S']
        k=rr.randrange(1,q)
        cs=[rr.randrange(1,q) for _ in range(r)]
        P=[X.ec_mul(c,S,A,p) for c in cs]
        pts=[(X.balanced(Q[0],p),X.balanced(Q[1],p)) for Q in P]
        u,why=X.fit_curve(pts,p,A,B)
        sh=X.to_short(u,pts)
        for nd in (3,4,5):
            t0=time.time(); hd=height_data(sh,n_dbl=nd); dt=time.time()-t0
            print(f'bits={bits} r={r} n_dbl={nd} t={dt:.2f}s eps={hd["eps"]:.4f} H={hd["H"]:.4f} hmin={hd["hmin"]:.4f} lamG={hd["lambda_G"]:.4f} rhoG={hd["rho_G"]:.3f} logDW={hd["log_absDW"]:.1f} lang_hmin={hd["lang_hmin"]:.4f} lang_lamG={hd["lang_lambdaG"]:.4f} hhat={[round(x,3) for x in hd["hhat"]]}')
