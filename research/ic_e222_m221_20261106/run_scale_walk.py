#!/usr/bin/env python3
"""Exploratory scaling panel (not frozen): best-of-grid solver time vs size for the hoisted sweep (extra 2)
and the pair-sum table (extra 2), W instances 18..24 bits, instances idx 0-2, seed 1.  Serial, interleaved."""
import json,os,subprocess,random,glob,sys,time
inst,out=sys.argv[1:3]; ic="../../target/release/ic"
GRID={"Dw":[8,16,32,64,128,256]}
ENV={"Dw":{"IC_SUM_TABLE":"1","IC_WALK":"1"}}
files=[f for f in sorted(glob.glob(inst+"/W*.json")) if int(os.path.basename(f).split("-")[1].split(".")[0])<3]
jobs=[(f,a,fb) for f in files for a in GRID for fb in GRID[a]]
random.Random("scale-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,a,fb in jobs:
    d=json.load(open(f))
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(os.environ)
    for k in ("IC_SUM_TABLE","IC_HOIST_SWEEP"): env.pop(k,None)
    env.update(ENV[a]);t0=time.perf_counter()
    r=subprocess.run([ic,"--json","solve-prime-file","--input",t,"--seed","1","--fb-size",str(fb),"--extra-relations","2","--rho-steps","0","--max-trials-per-relation","2000000","--max-relation-attempts","512"],capture_output=True,text=True,env=env,timeout=300)
    try:o=json.loads(r.stdout)
    except Exception:o={}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],r_bits=d["r_bits"],p=int(d["field"]["modulus"]),n=int(d["subgroup_order"]),arm=a,fb=fb,ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],
        total_ms=st.get("total_ms"),la_ms=st.get("linear_algebra_ms"),rel_ms=st.get("relations_ms"),wall_ms=(time.perf_counter()-t0)*1e3,trials=st.get("trials_total")))
    print(rows[-1]["name"],a,fb,rows[-1]["ok"],rows[-1]["total_ms"],flush=True)
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
