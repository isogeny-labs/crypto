import sys,random,json; sys.path.insert(0,'.')
import build_h; from build_h import *; from lattice import *
L=6144; build_h.DISTINCT_SQ=True; p=int(pari.nextprime(2**L+777))
while p%4!=3: p=int(pari.nextprime(p+1))
X=128
for seed in map(int,sys.argv[1:]):
    rng=random.Random(1000+seed); A,B,c=curve(p,rng)
    Ys,xR=plant(p,A,B,c,X,rng,'Y'); coef,info=interpolate(xR,A,B,c,p,rng=rng)
    rows,sc=build(coef,p,bounds(X)); vecs,dt=fplll(rows)
    rec={'L':L,'seed':seed,'X':X,'Ys':Ys,'lll_s':round(dt,1),'vecs':check(vecs,sc,inv_vals(Ys))}
    print(json.dumps(rec),flush=True); json.dump(rec,open(f'raw/far_L{L}_s{seed}.json','w'))
