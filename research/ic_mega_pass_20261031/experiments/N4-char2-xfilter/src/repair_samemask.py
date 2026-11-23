"""Amendment 4 repair: same-mask z of REAL best tuple vs K NULL-1sym permutations. usage: repair_samemask.py rawfile.json"""
import sys, json, time, numpy as np
sys.path.insert(0,'.'); from lib import *
f=sys.argv[1]; K=10; recs=json.load(open(f)); out=[]
c=recs[0]['curve']; mode=recs[0]['mode']; X,_=enumerate_x(c,mode,1)
nulls=[sym_perm(X,np.random.default_rng(9000+k)) for k in range(K)]
for r in recs:
    t0=time.time(); m=r['real']['masks']; er=r['real']['E']
    chk=eval_masks(X,m)['E']
    rm,_=reduce_masks(X,m); h=labels(X,rm); cnt=np.bincount(h,minlength=1<<len(rm)); Emarg=(1<<len(rm))*cnt.max()/len(X)-1
    En=[eval_masks(Y,m)['E'] for Y in nulls]; En=[e for e in En if e==e]
    mu,sd=float(np.mean(En)),float(np.std(En,ddof=1))
    o={'n':c['n'],'kind':c['kind'],'a':c['a'],'h':c['h'],'mode':mode,'j':r['j'],'masks':m,'E_real':er,'E_real_recheck':chk,
       'E_marg':float(Emarg),'null_E':En,'null_mean':mu,'null_sd':sd,'z':(er-mu)/sd if sd>0 else None,'secs':time.time()-t0}
    out.append(o); print(json.dumps({k:v for k,v in o.items() if k not in('null_E','masks')}),flush=True)
json.dump(out,open(f.replace('armB_','repair_samemask_'),'w'),indent=1)
