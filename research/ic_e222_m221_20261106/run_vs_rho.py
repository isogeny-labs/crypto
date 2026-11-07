#!/usr/bin/env python3
"""Matched IC-vs-rho panel on fresh instances.  fb per size frozen in vsrho_frozen.json (from the exploratory
scale_walk panel, committed before this run).  Arms per instance and solver seed: IW = table + restarted walk
(IC_SUM_TABLE=1 IC_WALK=1, extra 2, --rho-steps 0), RH = same-target Pollard rho of the same binary
(`solve-prime-file --rho-steps 5000000`; its IC part uses fb 8 and is NOT timed here, only the `rho.wall_ms`
field is read).  Serial, interleaved seeded order, 300 s timeout."""
import json,os,subprocess,random,glob,sys,time
inst,out=sys.argv[1:3];ic="../../target/release/ic"
fz=json.load(open("results/vsrho_frozen.json"))
files=[f for f in sorted(glob.glob(inst+"/W*.json"))]
jobs=[(f,json.load(open(f)),s,a) for f in files for s in (1,2) for a in ("IW","RH")]
random.Random("vsrho-v1").shuffle(jobs)
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json");rows=[]
for f,d,s,a in jobs:
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    pbits=int(d["field"]["modulus"]).bit_length();env=dict(os.environ)
    for k in ("IC_SUM_TABLE","IC_WALK","IC_HOIST_SWEEP"): env.pop(k,None)
    if a=="IW":
        env.update({"IC_SUM_TABLE":"1","IC_WALK":"1"})
        cmd=[ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size",str(fz[str(pbits)]),"--extra-relations","2","--rho-steps","0","--max-trials-per-relation","2000000","--max-relation-attempts","512"]
    else:
        cmd=[ic,"--json","solve-prime-file","--input",t,"--seed",str(s),"--fb-size","8","--extra-relations","2","--rho-steps","5000000","--max-trials-per-relation","2000000","--max-relation-attempts","512"]
    try:
        r=subprocess.run(cmd,capture_output=True,text=True,env=env,timeout=300);o=json.loads(r.stdout)
    except Exception:
        o={}
    if a=="IW":
        st=(o.get("ic") or {}).get("stages",{})
        rows.append(dict(name=d["name"],pbits=pbits,n=int(d["subgroup_order"]),seed=s,arm=a,fb=fz[str(pbits)],ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],ms=st.get("total_ms")))
    else:
        rh=o.get("rho") or {}
        rows.append(dict(name=d["name"],pbits=pbits,n=int(d["subgroup_order"]),seed=s,arm=a,ok=bool(rh.get("verified")) and str(rh.get("scalar"))==str(d["fixture"]["known_log"]),ms=rh.get("wall_ms")))
    print(rows[-1]["name"],a,rows[-1]["ok"],rows[-1]["ms"],flush=True)
with open(os.path.join(out,"rows.jsonl"),"w") as fh:
    for r in rows: fh.write(json.dumps(r)+"\n")
print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
