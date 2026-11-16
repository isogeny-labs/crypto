#!/usr/bin/env python3
"""H1 (--project-cofactor) with the current defaults (table, fb 32, extra 2, joint relation point).
Arms off/on, same binary, no env flags. Serial, seeded interleaved order. Prediction registered
before the run: on/off trials about 1/h (E 0.25, M 0.125), W exactly 1.000. Metric: trials and total_ms."""
import json,os,subprocess,random,glob,sys
inst=sys.argv[1];out=sys.argv[2];ic=sys.argv[3] if len(sys.argv)>3 else "../../target/release/ic"
jobs=[(f,json.load(open(f)),s,a) for f in sorted(glob.glob(inst+"/*.json")) if not f.endswith("MANIFEST.json") for s in (1,2) for a in ("off","on")]
random.Random("h1default-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,d,s,a in jobs:
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(os.environ)
    for k in ("IC_SUM_TABLE","IC_SHAMIR","IC_HOIST_SWEEP","IC_FAST_SWEEP","IC_WALK"): env.pop(k,None)
    cmd=[ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--rho-steps","0","--max-trials-per-relation","2000000","--max-relation-attempts","512"]
    if a=="on": cmd.append("--project-cofactor")
    r=subprocess.run(cmd,capture_output=True,text=True,env=env)
    try:o=json.loads(r.stdout)
    except Exception:o={"status":"crash"}
    st=(o.get("ic") or {}).get("stages",{})
    rows.append(dict(name=d["name"],family=d["family"],r_bits=d["r_bits"],p=d["field"]["modulus"],cofactor=d["cofactor"],seed=s,arm=a,status=o.get("status"),
      ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],total_ms=st.get("total_ms"),trials=st.get("trials_total"),rels=st.get("relations_collected"),fb=st.get("factor_base_size")))
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
