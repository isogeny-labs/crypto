import sys,random,json,time; sys.path.insert(0,'.')
from build_h import *; from lattice import *
out={}
for L in map(int,sys.argv[1:]):
    p=int(pari.nextprime(2**L+777)); rng=random.Random(100+L)
    A,B,c=curve(p,rng); xR=rng.randrange(p)
    t=time.time(); coef,info=interpolate(xR,A,B,c,p,rng=rng); tb=time.time()-t
    X=max(2,int(round(2**xstar_bits(L))))
    rows,sc=build(coef,p,bounds(X)); vecs,dt=fplll(rows)
    out[L]={'build_s':round(tb,1),'lll_s':round(dt,1),'X':X,'info':info}; print(L,out[L],flush=True)
json.dump(out,open('raw/timing_%s.json'%'_'.join(sys.argv[1:]),'w'))
