"""Arm A: exhaustive j=1 scan on <P>. usage: arm_A.py n kind_a(e.g. random1:1) n_null"""
import sys, json, time, numpy as np, cypari2
sys.path.insert(0,'.'); from lib import *
n=int(sys.argv[1]); kind,a=sys.argv[2].split(':'); a=int(a); nnull=int(sys.argv[3]); t0=time.time()
c=[x for x in load_curves() if x['n']==n and x['kind']==kind and x['a']==a][0]
X,g=enumerate_x(c,'P',1); E=scan_j1(X,n)
deg=[int(m) for m in np.nonzero(np.isnan(E[1:]))[0]+1]
rec={'curve':c,'L':len(X),'n_masks':(1<<n)-1,'degenerate_masks':deg,'E_max':float(np.nanmax(E)),'argmax':int(np.nanargmax(E)),
     'E_q999':float(np.nanquantile(E,0.999)),'E_median':float(np.nanmedian(E)),'E_mean':float(np.nanmean(E))}
pari=cypari2.Pari(); pari(f"T=Pol(binary({c['poly']}),'w)"); pari("w=ffgen(T*Mod(1,2),'w)")
pari(f"mk(m)=sum(i=0,{n}-1,bittest(m,i)*w^i)"); pari(f"msk(be)=sum(i=0,{n}-1,lift(trace(be*w^i))*2^i)")
rec['trace_mask']=int(pari("msk(1)"))
if kind=='koblitz':
    rng=np.random.default_rng(3); fr=[]
    for _ in range(200):
        bm=int(rng.integers(1,1<<n)); m1=int(pari(f"msk(mk({bm}))")); m2=int(pari(f"msk(mk({bm})^2)"))
        fr.append(abs(E[m1]-E[m2]) if not (np.isnan(E[m1]) and np.isnan(E[m2])) else 0.0)
    rec['frobenius_max_absdiff']=float(max(fr))
nulls=[]
for r in range(nnull):
    En=scan_j1(sym_perm(X,np.random.default_rng(100+r)),n)
    nulls.append({'E_max':float(np.nanmax(En)),'E_q999':float(np.nanquantile(En,0.999)),'E_median':float(np.nanmedian(En)),'n_degenerate':int(np.isnan(En[1:]).sum())})
rec['null1sym']=nulls; rec['secs']=time.time()-t0
print(json.dumps(rec)); json.dump(rec,open(f'../raw/armA_n{n}_{kind}_a{a}.json','w'),indent=1)
