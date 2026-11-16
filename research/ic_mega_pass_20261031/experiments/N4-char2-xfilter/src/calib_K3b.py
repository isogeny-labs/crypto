"""K3b (AMENDMENT 2): pipeline dose-response for j>=2, full j-dim plant through x-data. Oracle + climber, n=13,17, j_{1/4}."""
import sys, json, time, numpy as np
sys.path.insert(0,'.'); from lib import *
C=load_curves(); R1={c['n']:c for c in C if c['kind']=='random1' and c['a']==1}; res=[]; t0=time.time()
k1={(r['n'],r['j']):r for r in json.load(open('../raw/calib_K1.json'))}
for n in [13,17]:
    c=R1[n]; X,_=enumerate_x(c,'P',1); j=int(np.floor(0.25*np.log2(c['l']))); rng=np.random.default_rng(20261101+n+77); tm=trmask(c)
    def rk(v):
        b=[]
        for x in v:
            for y in b: x=min(x,x^y)
            if x: b.append(x)
        return len(b)
    while True:
        ms=[int(v) for v in rng.integers(1,1<<n,j)]
        if rk(ms+[tm])==j+1: break
    for rho in [0.005,0.01,0.02,0.05,0.1,0.2]:
        Y=plant_x_multi(X,ms,tm,rho,n); Ys=sym_perm(Y,np.random.default_rng(5))
        o=eval_masks(Y,ms); on=eval_masks(Ys,ms)
        seed=20261101+1000*n+10*j; cl=climb(Y,n,j,seed); cn=climb(Ys,n,j,seed)
        rec={'n':n,'j':j,'rho':rho,'oracle_E':o['E'],'oracle_null1sym_E':on['E'],'K1_null_max':k1[(n,j)]['null_max'],
             'oracle_detected':bool(o['E']>max(on['E'],k1[(n,j)]['null_max'])),'climber_E':cl['E'],'climber_null1sym_E':cn['E'],
             'climber_detected':bool(cl['E']>max(cn['E'],k1[(n,j)]['null_max'])),'secs':time.time()-t0}
        res.append(rec); print(json.dumps(rec),flush=True)
json.dump(res,open('../raw/calib_K3b.json','w'),indent=1)
