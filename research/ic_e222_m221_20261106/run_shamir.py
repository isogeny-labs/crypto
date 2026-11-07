#!/usr/bin/env python3
"""Shamir interleaving panel (IC_SHAMIR=1) on fresh prime-order instances.
Arms: table (IC_SUM_TABLE=1, fb 32, extra 2) and retuned sweep (fb 128, extra 2),
each with IC_SHAMIR 0 (default), 1 (interleaved) and 2 (vartime two-ladder control). Serial, interleaved seeded order. Metric: solver
total_ms; equivalence check: trials, relations and recovered log equal per
(instance, seed, collector) between Shamir off and on."""
import json,os,subprocess,random,glob,sys
inst=sys.argv[1];out=sys.argv[2];ic=sys.argv[3] if len(sys.argv)>3 else "../../target/release/ic"
COL={"table":(32,2,1),"sweep":(128,2,0)};SEEDS=[1,2]
jobs=[(f,json.load(open(f)),s,c,sh) for f in sorted(glob.glob(inst+"/*.json")) if not f.endswith("MANIFEST.json") for s in SEEDS for c in COL for sh in (0,1,2)]
random.Random("shamir-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,d,s,c,sh in jobs:
    fb,e,tb=COL[c]
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(os.environ)
    for k in ("IC_SUM_TABLE","IC_SHAMIR","IC_HOIST_SWEEP"): env.pop(k,None)
    if tb: env["IC_SUM_TABLE"]="1"
    if sh: env["IC_SHAMIR"]=str(sh)
    r=subprocess.run([ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size",str(fb),"--extra-relations",str(e),"--rho-steps","0","--max-trials-per-relation","2000000","--max-relation-attempts","512"],capture_output=True,text=True,env=env)
    try:o=json.loads(r.stdout)
    except Exception:o={"status":"crash"}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],r_bits=d["r_bits"],seed=s,collector=c,shamir=sh,status=o.get("status"),ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],
      total_ms=st.get("total_ms"),relations_ms=st.get("relations_ms"),trials=st.get("trials_total"),rels=st.get("relations_collected")))
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
