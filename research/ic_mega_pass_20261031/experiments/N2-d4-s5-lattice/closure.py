"""Min-cut closure reach (MP-lattices-3/lp.py, unchanged) on the true D4 invariant H; validation vs lit2026 S4 row."""
import sys,json,time,random,itertools
sys.path.insert(0,'.'); sys.path.insert(0,'../MP-lattices-3')
from lp import solve, pmul
import sympy as sp
from build_h import *
out={}
mode=sys.argv[1]
if mode=='validate':
    a,b,c,u,v=sp.symbols('a b c u v'); A2,A4,A6=7,11,13
    S3=lambda x,y,z:(x-y)**2*z**2-2*((x+y)*(x*y+A4)+2*A6)*z+(x*y-A4)**2
    s4=sp.Poly(sp.resultant(S3(a,b,v),S3(c,5,v),v),a,b,c)
    P={e:int(co) for e,co in s4.terms()}
    for l,L in ((1,14),(2,20)):
        best=max((solve([P],lambda d:[d]*3,[L]*3,l,o) for o in ([1,1,1],[-1,-1,-1],[1,-1,-1],[1,1,-1])),key=lambda r:r['delta'])
        out[f'S4_l{l}_L{L}']=best; print(l,L,best,flush=True)
else:
    p=int(pari.nextprime(2**256+12345)); rng=random.Random(1); A,B,cc=curve(p,rng)
    coef,info=interpolate(rng.randrange(p),A,B,cc,p,rng=rng)
    H={m:co for m,co in zip(MONOS,coef) if co}
    l=int(sys.argv[2])
    for box in json.loads(sys.argv[3]):
        best=None
        for o in ([1,2,3,2],[2,4,6,4],[1,1,1,1],[0,0,0,1],[1,0,0,0]):
            t=time.time(); r=solve([H],lambda d:[2*d,4*d,6*d,4*d],box,l,o); r['order']=o; r['sec']=round(time.time()-t,1)
            if best is None or r['delta']>best['delta']: best=r
        out[str(box)]=best; print(l,box,best,flush=True)
json.dump(out,open(f'raw/closure_{"_".join(sys.argv[1:3])}.json','w'),indent=1,default=str)
