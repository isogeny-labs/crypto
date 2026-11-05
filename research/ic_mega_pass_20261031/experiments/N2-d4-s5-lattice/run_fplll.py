"""Arm Y planted below/above/null + Arm X (small-x) checks. usage: run_fplll.py L seed..."""
import sys,random,json,time,math; sys.path.insert(0,'.')
import build_h; from build_h import *; from lattice import *
L=int(sys.argv[1]); build_h.DISTINCT_SQ=(L>4096); ARMX=(L<=4096); p=int(pari.nextprime(2**L+777))
while p%4!=3: p=int(pari.nextprime(p+1))
xs=xstar_bits(L)
for seed in map(int,sys.argv[2:]):
    rng=random.Random(seed)
    while True:  # need >=1 on-curve small Y at the smallest X (pre-check), else new curve
        A,B,c=curve(p,rng); X0=int(round(2**(0.85*xs)))
        if sum(bool(on_curve(c*(1+y)*pow(1-y,-1,p)%p,A,B,p)) for y in range(-X0,X0+1) if y not in (0,1,-1))>=4 and (not build_h.DISTINCT_SQ or len({abs(y) for y in range(-X0,X0+1) if y not in (0,1,-1) and on_curve(c*(1+y)*pow(1-y,-1,p)%p,A,B,p)})>=4): break
    rec={'L':L,'seed':seed,'xstar_bits':xs}
    for arm,f in (('below',0.85),('above',1.15)):
        X=int(round(2**(f*xs))); Ys,xR=plant(p,A,B,c,X,rng,'Y')
        coef,info=interpolate(xR,A,B,c,p,rng=rng)
        iv=inv_vals(Ys); assert sum(a*b for a,b in zip(coef,mono_eval(inv_vals(Ys,p),p)))%p==0
        rows,sc=build(coef,p,bounds(X)); vecs,dt=fplll(rows)
        rec[arm]={'X':X,'delta':f*xs/L,'Ys':Ys,'info':info,'lll_s':round(dt,1),'vecs':check(vecs,sc,iv)}
        if arm=='below':
            # null: same Ys, random xR (no planted root), same X
            xRn=rng.randrange(p); cn,_=interpolate(xRn,A,B,c,p,rng=rng)
            Hn=sum(a*b for a,b in zip(cn,mono_eval(inv_vals(Ys,p),p)))%p
            rows,sc=build(cn,p,bounds(X)); vecs,dt=fplll(rows)
            rec['null']={'X':X,'H_at_Ys_modp_zero':Hn==0,'lll_s':round(dt,1),'vecs':check(vecs,sc,iv)}
    if not ARMX:
        print(json.dumps(rec),flush=True); json.dump(rec,open(f'raw/fplll_L{L}_s{seed}.json','w')); continue
    # Arm X: small Weierstrass x (|x|<=2^16), fiat-model bounds at X=below value
    X=rec['below']['X']; Ys,xR=plant(p,A,B,c,2**16,rng,'X')
    coef,_=interpolate(xR,A,B,c,p,rng=rng); ivp=inv_vals(Ys,p)
    lift=[v if v<p//2 else v-p for v in ivp]
    logs=[round(math.log(abs(v))/math.log(p),4) if v else 0 for v in lift]
    rows,sc=build(coef,p,bounds(X)); vecs,dt=fplll(rows)
    rec['armX']={'xbox_bits':16,'log_p_abs_invariants':logs,'fiat_X':X,'lll_s':round(dt,1),'vecs':check(vecs,sc,lift)}
    print(json.dumps(rec),flush=True)
    json.dump(rec,open(f'raw/fplll_L{L}_s{seed}.json','w'))
