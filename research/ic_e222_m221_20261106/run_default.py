#!/usr/bin/env python3
"""Default-flip panel. Arms: OLD = the former shipped defaults (fb 8, extra 64, per-trial
sweep, two scalar_mul ladders: IC_SUM_TABLE=0 IC_SHAMIR=0); NEW = no flags (current
defaults). `new-only` mode runs NEW alone (robustness sample). Serial, seeded interleaved
order. Metric: solver total_ms; every run checked against the planted log."""
import json,os,subprocess,random,glob,sys
inst=sys.argv[1];out=sys.argv[2];mode=sys.argv[3];ic=sys.argv[4] if len(sys.argv)>4 else "../../target/release/ic"
SEEDS=[1,2] if mode=="paired" else [1,2,3]
ARMS=["OLD","NEW"] if mode=="paired" else ["NEW"]
jobs=[(f,json.load(open(f)),s,a) for f in sorted(glob.glob(inst+"/*.json")) if not f.endswith("MANIFEST.json") for s in SEEDS for a in ARMS]
random.Random("default-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,d,s,a in jobs:
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(os.environ)
    for k in ("IC_SUM_TABLE","IC_SHAMIR","IC_HOIST_SWEEP","IC_FAST_SWEEP","IC_WALK"): env.pop(k,None)
    cmd=[ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--rho-steps","0","--max-trials-per-relation","2000000","--max-relation-attempts","512"]
    if a=="OLD":
        env["IC_SUM_TABLE"]="0";env["IC_SHAMIR"]="0";cmd+=["--fb-size","8","--extra-relations","64"]
    r=subprocess.run(cmd,capture_output=True,text=True,env=env)
    try:o=json.loads(r.stdout)
    except Exception:o={"status":"crash"}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],r_bits=d["r_bits"],seed=s,arm=a,status=o.get("status"),ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],
      total_ms=st.get("total_ms"),relations_ms=st.get("relations_ms"),trials=st.get("trials_total"),rels=st.get("relations_collected"),fb=st.get("factor_base_size")))
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
