#!/usr/bin/env python3
"""Baseline panel: `ic solve-prime-file` on E/M/W analog instances, seeds 1..S.
Metric: trials_total (relation-sampler trials, deterministic given seed).
Verification: recovered scalar == planted known_log.  Interleaved order, seeded.
Local development run (not isolated): wall time is NOT used."""
import json, os, subprocess, sys, random, glob, statistics, argparse
ap=argparse.ArgumentParser()
ap.add_argument("--inst");ap.add_argument("--out");ap.add_argument("--ic",default="target/release/ic")
ap.add_argument("--seeds",type=int,default=3);ap.add_argument("--fb",type=int,default=8)
ap.add_argument("--rho-steps",type=int,default=0);ap.add_argument("--extra",default="")
a=ap.parse_args()
os.makedirs(a.out,exist_ok=True)
jobs=[]
for f in sorted(glob.glob(a.inst+"/*.json")):
    if f.endswith("MANIFEST.json"): continue
    d=json.load(open(f))
    for s in range(1,a.seeds+1): jobs.append((f,d,s))
random.Random("order-v1").shuffle(jobs)   # interleaved arm order
rows=[]
for f,d,s in jobs:
    t=os.path.join(a.out,"_t.json")
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    r=subprocess.run([a.ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size",str(a.fb),"--rho-steps",str(a.rho_steps),
                      "--max-trials-per-relation","200000","--max-relation-attempts","512"]+a.extra.split(),capture_output=True,text=True)
    try: o=json.loads(r.stdout)
    except Exception: o={"status":"crash","err":r.stderr[-200:]}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],family=d["family"],r_bits=d["r_bits"],p=d["field"]["modulus"],seed=s,status=o.get("status"),
      ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],trials_total=st.get("trials_total"),
      relations=st.get("relations_collected"),exhausted=st.get("relation_attempts_exhausted"),fb=st.get("factor_base_size")))
with open(os.path.join(a.out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
