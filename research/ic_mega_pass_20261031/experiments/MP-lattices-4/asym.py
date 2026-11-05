import numpy as np, json
rng=np.random.default_rng(7); N=4_000_000
def simplex(coef):  # uniform in {sum coef*u<=1}
    E=rng.exponential(size=(N,5)); s=E/E.sum(1,keepdims=True); return s[:,:4]/np.array(coef)
out={}
u=rng.random((N,4)); out['xbox']=float(np.min(u,1).mean()/(8*u.sum(1).mean()))
for name,coef,w,leads in [('S4_e_x',(1,1,1,1),(1,2,3,4),[(1,0,0,0),(0,0,0,1)]),('D4_eY2_pi',(2,2,2,1),(2,4,6,4),[(.5,0,0,0),(0,0,.5,0),(0,0,0,1)])]:
    x=simplex(coef); best=0
    for ld in leads:
        k=np.min(np.stack([x[:,i]/ld[i] for i in range(4) if ld[i]>0]),0); k=np.minimum(k,1)
        best=max(best,float(k.mean()/(8*(x@np.array(w)).mean())))
    out[name]=best
out['ratio_D4_S4e']=out['D4_eY2_pi']/out['S4_e_x']; print(json.dumps(out)); json.dump(out,open('asym_out.json','w'))
