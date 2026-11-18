import json, statistics as st, sys
rows=[json.loads(l) for l in open('runs.jsonl')]
def flat(r):
    o={}
    for k,v in r.items():
        if isinstance(v,dict):
            for kk,vv in v.items(): o[f"{k}[{kk}]"]=vv
        elif isinstance(v,list):
            for f in v: o[f"filter{f['bits_per_key']}_ns_per_probe"]=f['ns_per_probe']; o[f"filter{f['bits_per_key']}_fpr"]=f['false_positive_rate']
        else: o[k]=v
    return o
out={}
for A in (256,1024):
    rs=[flat(r) for r in rows if r['abscissae']==A]
    keys=[k for k in rs[0] if isinstance(rs[0][k],(int,float)) and not isinstance(rs[0][k],bool)]
    d={}
    for k in keys:
        vs=[r[k] for r in rs]
        d[k]={"median":st.median(vs),"min":min(vs),"max":max(vs)}
    d["n_runs"]=len(rs)
    d["best_lanes_seen"]=sorted({r['best_lanes'] for r in rs}); d["best_is_avx512_seen"]=sorted({r['best_is_avx512'] for r in rs})
    out[A]=d
json.dump(out,open('summary.json','w'),indent=1,sort_keys=True)
for A,d in out.items():
    print("A=",A,"runs",d["n_runs"],"best lanes",d["best_lanes_seen"],d["best_is_avx512_seen"])
    for k in sorted(d):
        if isinstance(d[k],dict) and ('ns' in k or 'mismatch' in k or 'hits' in k or 'fpr' in k):
            v=d[k]; print(f"  {k:55s} {v['median']:9.3f}  [{v['min']:.3f}, {v['max']:.3f}]")
