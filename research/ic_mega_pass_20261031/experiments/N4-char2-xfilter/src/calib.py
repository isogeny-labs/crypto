import sys, json, time, numpy as np
sys.path.insert(0,'.'); from lib import *
part=sys.argv[1]; C=load_curves(); t0=time.time(); res=[]
R1={c['n']:c for c in C if c['kind']=='random1' and c['a']==1}
DOSES=[0.005,0.01,0.02,0.05,0.1,0.2]
def jth(l,th): return int(np.floor(th*np.log2(l)))
if part=='K1':
    for n in [13,15,17,19,21,23]:
        c=R1[n]; X,_=enumerate_x(c,'P',1); L=len(X)
        js=sorted(set([1,jth(c['l'],0.25),jth(c['l'],1/3)]))
        if n==23: js=[1,jth(c['l'],0.25)]  # j_{1/3}=7 over budget at n=23 (declared)
        for j in js:
            nulls=[gain(prf_labels(X,j,key),j)['G']-1 for key in range(8)]
            base=prf_labels(X,j,999)
            pl=[{'rho':r,'E':gain(plant_labels(base,r),j)['G']-1} for r in DOSES]
            for p in pl: p['detected']=bool(p['E']>max(nulls))
            det=[p['E'] for p in pl if p['detected']]
            rec={'n':n,'L':L,'j':j,'null_E':nulls,'null_max':max(nulls),'planted':pl,'E_star':min(det) if det else None,'secs':time.time()-t0}
            res.append(rec); print(json.dumps(rec),flush=True)
elif part=='K2':
    for n in [13,15]:
        c=R1[n]; X,_=enumerate_x(c,'P',1); rng=np.random.default_rng(20261101+n)
        Enull=scan_j1(sym_perm(X,np.random.default_rng(5)),n); nullmax=float(np.nanmax(Enull))
        mstar=int(rng.integers(1,1<<n))
        u=pick_u(rng,n,mstar,trmask(c))
        for rho in DOSES:
            E=scan_j1(plant_x(X,mstar,u,rho),n); am=int(np.nanargmax(E))
            rec={'n':n,'rho':rho,'mstar':mstar,'argmax':am,'E_mstar':float(E[mstar]),'E_max':float(np.nanmax(E)),'null_family_max':nullmax,
                 'detected':bool(am==mstar and E[mstar]>nullmax),'secs':time.time()-t0}
            res.append(rec); print(json.dumps(rec),flush=True)
elif part=='K3':
    k1={(r['n'],r['j']):r for r in json.load(open('../raw/calib_K1.json'))}
    for n in [13,17]:
        c=R1[n]; X,_=enumerate_x(c,'P',1); j=jth(c['l'],0.25); rng=np.random.default_rng(20261101+n+7)
        det=[p['rho'] for p in k1[(n,j)]['planted'] if p['detected']]; rho=min(det) if det else max(DOSES)
        mstar=int(rng.integers(1,1<<n))
        u=pick_u(rng,n,mstar,trmask(c))
        Y=plant_x(X,mstar,u,rho)
        cl=climb(Y,n,j,20261101+1000*n+10*j); cn=climb(sym_perm(Y,np.random.default_rng(5)),n,j,20261101+1000*n+10*j)
        oracle=eval_masks(Y,[mstar]+[int(v) for v in rng.integers(1,1<<n,j-1)])
        orn=eval_masks(sym_perm(Y,np.random.default_rng(5)),oracle['masks'])
        for d in (cl,cn,oracle,orn): d.pop('pi',None); d.pop('num',None)
        rec={'n':n,'j':j,'rho':rho,'climber_planted_E':cl['E'],'climber_null1sym_E':cn['E'],'oracle_E':oracle['E'],'oracle_on_null1sym_E':orn['E'],
             'K1_null_max':k1[(n,j)]['null_max'],'climber_found':bool(cl['E']>max(cn['E'],k1[(n,j)]['null_max'])),'oracle_detected':bool(oracle['E']>max(orn['E'],k1[(n,j)]['null_max'])),'secs':time.time()-t0}
        res.append(rec); print(json.dumps(rec),flush=True)
json.dump(res,open(f'../raw/calib_{part}'+('_amended' if part in ('K2','K3') else '')+'.json','w'),indent=1)
