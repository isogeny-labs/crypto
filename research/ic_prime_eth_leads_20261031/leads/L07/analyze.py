import json, sys, math, glob, collections
import numpy as np
rng=np.random.default_rng(1)
rows=[]
for f in sorted(glob.glob('raw*.jsonl')):
    for l in open(f):
        try: rows.append(json.loads(l))
        except: pass
seen=set(); LIM={14:60,18:6,22:1}
cells=collections.defaultdict(list)
for d in rows:
    k=(d['bits'],d['curve'],d['rep'],d['m'],d['Enom'],d['mode'],d['arm'])
    if k in seen or d['rep']>=LIM[d['bits']]: continue
    seen.add(k); cells[(d['bits'],d['m'],d['Enom'],d['mode'],d['arm'])].append(d)
def boot(vals_a, vals_b, fn, B=400):
    a=np.array(vals_a); b=np.array(vals_b); n=len(a)
    est=fn(a,b); bs=[]
    for _ in range(B):
        i=rng.integers(0,n,n); bs.append(fn(a[i],b[i]))
    return est, float(np.std(bs))
res={}
out=[]
def get(d,k): return d['arm_stats'][k], d['ref_stats'][k]
for key,L in sorted(cells.items()):
    bits,m,E,mode,arm=key
    A={k:[get(d,k)[0] for d in L] for k in ('E','Var','P1','P2')}
    B={k:[get(d,k)[1] for d in L] for k in ('E','Var','P1','P2')}
    rP1,sP1=boot(A['P1'],B['P1'],lambda a,b:a.mean()/b.mean())
    rP2,sP2=boot(A['P2'],B['P2'],lambda a,b:a.mean()/max(b.mean(),1e-12))
    rF,sF=boot([v/max(e,1e-12) for v,e in zip(A['Var'],A['E'])],[v/max(e,1e-12) for v,e in zip(B['Var'],B['E'])],lambda a,b:a.mean()/b.mean())
    rE,_=boot(A['E'],B['E'],lambda a,b:a.mean()/b.mean())
    res[key]=dict(n=len(L),E=np.mean(A['E']),Er=np.mean(B['E']),Var=np.mean(A['Var']),P1=np.mean(A['P1']),P1r=np.mean(B['P1']),
        P2=np.mean(A['P2']),inv=1/np.mean(A['P1']),rP1=rP1,sP1=sP1,rP2=rP2,sP2=sP2,rF=rF,sF=sF,rE=rE)
json.dump({'|'.join(map(str,k)):v for k,v in res.items()},open('results.json','w'),indent=1,default=float)
# table
print('bits m E_nom mode arm n | E_arm E_ref | Var | P1 | P1/ref (se) | P2/ref (se) | Fano/ref (se) | 1/P1 (m!)')
for key,v in sorted(res.items()):
    bits,m,E,mode,arm=key
    print(f"{bits} {m} {E} {mode:6s} {arm:8s} n={v['n']:3d} | {v['E']:.3f} {v['Er']:.3f} | {v['Var']:.3g} | {v['P1']:.4f} | {v['rP1']:.3f} ({v['sP1']:.3f}) | {v['rP2']:.2f} ({v['sP2']:.2f}) | {v['rF']:.3f} ({v['sF']:.3f}) | {v['inv']:.2f} ({math.factorial(m)})")
# slope vs b of log2 ratio P1
print('\nSLOPE of ln(P1/ref) per bit of b, OLS over available bit sizes (se from per-point se)')
for mode in ('indep','shared'):
  for m in (2,3,4):
    for E in (1.0,0.25):
      for arm in ('window','affine','sparse5','dense','interval','random'):
        pts=[(b,res[(b,m,E,mode,arm)]) for b in (14,18,22) if (b,m,E,mode,arm) in res]
        if len(pts)<2: continue
        x=np.array([p[0] for p in pts],float); y=np.log([p[1]['rP1'] for p in pts]); s=np.array([p[1]['sP1']/p[1]['rP1'] for p in pts])
        w=1/s**2; xm=(w*x).sum()/w.sum(); sl=(w*(x-xm)*(y-(w*y).sum()/w.sum())).sum()/(w*(x-xm)**2).sum(); se=1/math.sqrt((w*(x-xm)**2).sum())
        print(f"{mode:6s} m={m} E={E} {arm:8s} slope={sl:+.4f} +- {se:.4f} per bit (z={sl/se:+.1f})  pts={[round(p[1]['rP1'],3) for p in pts]}")
