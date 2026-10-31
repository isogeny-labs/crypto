"""E1 trend fit: ln(ratio) vs n, three pre-stated conventions.  usage: python3 e1_fit.py e1_res.json"""
import json, sys, math, random, statistics as st
import numpy as np
from scipy import stats
res=json.load(open(sys.argv[1]))
def pick(v, conv, kind):
    M=v["matrix"]; kk="whole" if kind=="whole" else "lower"
    if conv=="same_l":      key="inherited-f4-l4|msolve-x-l4-s4"
    elif conv=="best":
        la=min(v["lab_med"],key=v["lab_med"].get)
        pool=v["pub_med"] if kind=="whole" else v["pub_low"]
        pa=min(pool,key=pool.get); key=la+"|"+pa
    elif conv=="orig":      key={9:"inherited-f4-l4|msolve-x-l4-s4",13:"inherited-f4-l5|msolve-x-l4-s4"}.get(v["n"])
    if key not in M: return None,None
    return key,M[key][kk]
def fit(xs,ys):
    x=np.array(xs,float); y=np.log(np.array(ys,float)); k=len(x)
    sl,ic,r,p,se=stats.linregress(x,y); t=stats.t.ppf(0.975,k-2)
    return sl,se,t,p
rng=random.Random(1)
out={}
for conv in ("same_l","best","orig"):
  for kind in ("whole","lower"):
    pts=[]
    for c,v in res.items():
        key,per=pick(v,conv,kind)
        if per is None: continue
        pts.append((c,v["family"],v["n"],key,st.median(per),per))
    if len(pts)<2: continue
    xs=[p[2] for p in pts]; ys=[p[4] for p in pts]
    sl,se,t,p=fit(xs,ys) if len(pts)>2 else (float('nan'),)*4
    # bootstrap: resample the paired per-target ratios within each cell, refit
    sls=[]
    for _ in range(4000):
        yy=[st.median(rng.choices(p_[5],k=len(p_[5]))) for p_ in pts]
        sls.append(stats.linregress(xs,np.log(yy))[0])
    lo,hi=np.percentile(sls,[2.5,97.5])
    print(f"{conv:7s} {kind:5s} points={[(p_[0],round(p_[4],1)) for p_ in pts]}")
    print(f"        slope d ln(ratio)/dn = {sl:+.3f} +- {t*se:.3f} (OLS 95% CI, k={len(pts)}, p={p:.2f}); bootstrap-over-targets 95% [{lo:+.3f},{hi:+.3f}]; ratio change per +4 bits x{math.exp(4*sl):.2f}")
    out[f"{conv}_{kind}"]=dict(points=[(p_[0],p_[1],p_[2],p_[3],p_[4]) for p_ in pts],slope=sl,ci=t*se,p=p,boot=[lo,hi])
json.dump(out,open("e1_fit.json","w"),indent=1,default=float)
