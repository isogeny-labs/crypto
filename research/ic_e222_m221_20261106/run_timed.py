#!/usr/bin/env python3
"""Serial timed panel: fb x extra grid, interleaved seeded order, one process at a time.
Metric: total_ms from the solver's own stage report (factor base + relations + linear algebra + verify),
plus trials_total.  Correctness: recovered scalar == planted log.  Local, unisolated host."""
import json,os,subprocess,random,glob,sys
inst=sys.argv[1];out=sys.argv[2];ic="../../target/release/ic"
FBS=[8,16,32,64,128];EXTRAS=[64,2];SEEDS=[1,2]
jobs=[]
for f in sorted(glob.glob(inst+"/*.json")):
    d=json.load(open(f))
    for s in SEEDS:
        for fb in FBS:
            for e in EXTRAS: jobs.append((f,d,s,fb,e))
random.Random("timed-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True)
t=os.path.join(out,"_t.json");rows=[]
for f,d,s,fb,e in jobs:
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    r=subprocess.run([ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size",str(fb),"--extra-relations",str(e),"--rho-steps","0","--max-trials-per-relation","200000","--max-relation-attempts","512"],capture_output=True,text=True)
    try:o=json.loads(r.stdout)
    except Exception:o={"status":"crash"}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],r_bits=d["r_bits"],seed=s,fb=fb,extra=e,status=o.get("status"),ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],
      total_ms=st.get("total_ms"),relations_ms=st.get("relations_ms"),la_ms=st.get("linear_algebra_ms"),fb_ms=st.get("factor_base_ms"),verify_ms=st.get("verify_ms"),trials=st.get("trials_total"),rels=st.get("relations_collected")))
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
