#!/usr/bin/env python3
"""Serial timed panel: sweep (default) vs pair-sum table (IC_SUM_TABLE=1), extra=2, fb grid.
Interleaved seeded order, one process at a time; metric total_ms from the solver's stage report
(table construction is inside relations_ms).  Correct = recovered scalar equals planted log."""
import json,os,subprocess,random,glob,sys
inst=sys.argv[1];out=sys.argv[2];ic="../../target/release/ic"
FBS=[int(x) for x in os.environ.get("FBS","16,32,64,128,256").split(",")];SEEDS=[1,2]
jobs=[(f,json.load(open(f)),s,fb,tb) for f in sorted(glob.glob(inst+"/*.json")) for s in SEEDS for fb in FBS for tb in (0,1)]
random.Random("table-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,d,s,fb,tb in jobs:
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(os.environ);env.pop("IC_SUM_TABLE",None)
    if tb: env["IC_SUM_TABLE"]="1"
    r=subprocess.run([ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size",str(fb),"--extra-relations","2","--rho-steps","0","--max-trials-per-relation","200000","--max-relation-attempts","512"],capture_output=True,text=True,env=env)
    try:o=json.loads(r.stdout)
    except Exception:o={"status":"crash"}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],r_bits=d["r_bits"],seed=s,fb=fb,table=tb,status=o.get("status"),ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],
      total_ms=st.get("total_ms"),relations_ms=st.get("relations_ms"),la_ms=st.get("linear_algebra_ms"),trials=st.get("trials_total"),rels=st.get("relations_collected")))
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
