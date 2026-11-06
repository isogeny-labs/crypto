# POST-HOC supplement (not pre-registered): NULL-1s = permutation respecting x(kG)=x(-kG) symmetry.
import json,sys,numpy as np; sys.argv=['x','/dev/null']
src=open('run.py').read().split("out={'per_p'")[0]; exec(src)
res=[]
for k in [14,16,18]:
    p=int(pari.nextprime(2**k)); seed=20261031+k*100
    a,b,N,gx,gy=curve(p,seed); X=enum(p,a,N,gx,gy)
    rng=np.random.default_rng(seed+7); h=(N-1)//2; perm=rng.permutation(h)+1
    Xs=X.copy(); Xs[1:h+1]=X[perm]; Xs[N-np.arange(1,h+1)]=X[perm]
    assert all(Xs[i]==Xs[N-i] for i in range(1,50))
    for d in [1,2,4,6]:
        r=np.random.default_rng(seed+d); bud=40
        lr=climb(X,p,d,r,bud)[1]; r=np.random.default_rng(seed+d); ls=climb(Xs,p,d,r,bud)[1]
        res.append({'k':k,'d':d,'lam_real':lr,'lam_null1sym':ls,'ratio':lr/ls})
        print(res[-1],flush=True)
json.dump(res,open('RESULTS_supp_symnull.json','w'),indent=1)
