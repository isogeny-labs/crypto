#!/usr/bin/env python3
"""Confirmatory panel with settings FROZEN before the run (chosen on earlier panels):
A shipped default (sweep, extra 64, fb 8); B tuned baseline (sweep, extra 64, fb 256);
C retuned sweep (extra 2, fb 128); D pair-sum table (extra 2, fb 32).
Serial, interleaved seeded order; metric = solver total_ms (table build inside relations_ms)."""
import json,os,subprocess,random,glob,sys
inst=sys.argv[1];out=sys.argv[2];ic="../../target/release/ic"
ARMS={"A":(8,64,0),"B":(256,64,0),"C":(128,2,0),"D":(32,2,1)};SEEDS=[1,2]
jobs=[(f,json.load(open(f)),s,a) for f in sorted(glob.glob(inst+"/*.json")) for s in SEEDS for a in ARMS]
random.Random("confirm-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,d,s,a in jobs:
    fb,e,tb=ARMS[a]
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(os.environ);env.pop("IC_SUM_TABLE",None)
    if tb: env["IC_SUM_TABLE"]="1"
    r=subprocess.run([ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size",str(fb),"--extra-relations",str(e),"--rho-steps","0","--max-trials-per-relation","200000","--max-relation-attempts","512"],capture_output=True,text=True,env=env)
    try:o=json.loads(r.stdout)
    except Exception:o={"status":"crash"}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],r_bits=d["r_bits"],seed=s,arm=a,status=o.get("status"),ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],
      total_ms=st.get("total_ms"),trials=st.get("trials_total"),rels=st.get("relations_collected")))
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
