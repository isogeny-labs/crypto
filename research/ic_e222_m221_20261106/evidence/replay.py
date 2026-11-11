#!/usr/bin/env python3
"""Nonce-sampled replay of recorded `ic solve-prime-file` runs (claims C1 to C3).
1. regenerates each instance set from its seed and checks the MANIFEST sha256 in claims.json;
2. draws K rows per raw file with a nonce (printed; pass --nonce to repeat a draw);
3. reruns each drawn row with the recorded arguments and compares trials_total, relations
   collected and the recovered scalar (deterministic given the seed) to the recorded row.
Writes replay-log.json.  Producer-run: it records independent=false.  The OLD arm is sampled from
rungs <= 17 bits only (a 22-bit OLD solve takes about two minutes).  Wall times are not compared."""
import argparse,hashlib,json,os,random,subprocess,sys,tempfile,shutil
HERE=os.path.dirname(os.path.abspath(__file__));ROOT=os.path.dirname(HERE)
ap=argparse.ArgumentParser()
ap.add_argument("--ic",required=True);ap.add_argument("--k",type=int,default=30)
ap.add_argument("--nonce",default=None);ap.add_argument("--robust-rows");ap.add_argument("--out",default=os.path.join(HERE,"replay-log.json"))
a=ap.parse_args()
nonce=a.nonce or os.urandom(8).hex();rng=random.Random(nonce)
C=json.load(open(os.path.join(HERE,"claims.json")))
tmp=tempfile.mkdtemp(prefix="replay-")
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
log={"schema":"replay-log-draft/v1","independent":False,"producer_run":True,"nonce":nonce,"k_per_file":a.k,"runs":[],"instance_sets":{},"mismatches":[]}
# 1. instances
inst={}
for key,s in C["instance_sets"].items():
    d=os.path.join(tmp,key);subprocess.run([sys.executable,"-I",os.path.join(ROOT,"gen_instances.py"),"--out",d]+s["args"].split(),capture_output=True,check=True)
    m=sha(os.path.join(d,"MANIFEST.json"));log["instance_sets"][key]={"manifest_sha256":m,"matches_claims_json":m==s["manifest_sha256"]};inst[key]=d
    if m!=s["manifest_sha256"]: log["mismatches"].append(f"{key}: regenerated manifest differs")
# 2. replay
env0=dict(os.environ)
for k in ("IC_SUM_TABLE","IC_SHAMIR","IC_HOIST_SWEEP","IC_FAST_SWEEP","IC_WALK"): env0.pop(k,None)
def run(row,iset,arm_kind):
    d=json.load(open(os.path.join(inst[iset],row["name"]+".json")))
    t=os.path.join(tmp,"_t.json")
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env=dict(env0);cmd=[a.ic,"--json","solve-prime-file","--input",t,"--seed",str(row["seed"]),"--rho-steps","0","--max-trials-per-relation","2000000","--max-relation-attempts","512"]
    if arm_kind=="OLD": env["IC_SUM_TABLE"]="0";env["IC_SHAMIR"]="0";cmd+=["--fb-size","8","--extra-relations","64"]
    if arm_kind=="on": cmd.append("--project-cofactor")
    o=json.loads(subprocess.run(cmd,capture_output=True,text=True,env=env).stdout)
    st=(o.get("ic") or {}).get("stages",{})
    return {"trials":st.get("trials_total"),"rels":st.get("relations_collected"),"ok":(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],"binary_blake3":o["software"]["binary_blake3"]}
def load(key):
    p=a.robust_rows if key=="robust" and a.robust_rows else os.path.join(ROOT,C["raw_data"][key]["path"])
    if not os.path.exists(p): return None
    if sha(p)!=C["raw_data"][key]["sha256"]: log["mismatches"].append(f"{key}: rows file sha256 differs"); return None
    return [json.loads(l) for l in open(p)]
bins=set()
for key,iset in (("paired","def_inst"),("robust","rob_inst"),("h1_defaults","h1d_inst")):
    rows=load(key)
    if rows is None: log["runs"].append({"file":key,"skipped":"rows not available or hash differs"});continue
    pool=[r for r in rows if not (r.get("arm")=="OLD" and r["r_bits"]>17)]
    for r in rng.sample(pool,min(a.k,len(pool))):
        kind=r.get("arm","NEW");res=run(r,iset,kind);bins.add(res["binary_blake3"])
        same=res["trials"]==r["trials"] and res["rels"]==r["rels"] and res["ok"]==r["ok"]
        log["runs"].append({"file":key,"name":r["name"],"seed":r["seed"],"arm":kind,"recorded":{"trials":r["trials"],"rels":r["rels"],"ok":r["ok"]},"replayed":{k:res[k] for k in ("trials","rels","ok")},"identical":same})
        if not same: log["mismatches"].append(f"{key} {r['name']} seed {r['seed']} arm {kind}: recorded {r['trials']}/{r['rels']} replayed {res['trials']}/{res['rels']}")
# band (claim C5): rerun a drawn sample of the per-instance outputs; every IC counter and the matched rho must repeat exactly
bm=os.path.join(ROOT,C["raw_data"]["band"]["path"])
if os.path.exists(bm) and sha(bm)==C["raw_data"]["band"]["sha256"]:
    man=json.load(open(bm));bdir=os.path.dirname(bm)
    files=sorted(man["files"]);pick=rng.sample(files,min(a.k,len(files)))
    for rel in pick:
        p=os.path.join(bdir,os.path.relpath(rel,"band"))
        if sha(p)!=man["files"][rel]: log["mismatches"].append(f"band {rel}: file sha256 differs from the manifest");continue
        rec=json.load(open(p));meta=rec["_meta"]
        d=json.load(open(os.path.join(inst["h1d_inst"],meta["instance"]+".json")))
        t=os.path.join(tmp,"_t.json")
        json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
        cmd=[a.ic,"--json","solve-prime-file","--input",t,"--seed","1","--rho-steps","0","--matched-rho-runs","16","--max-trials-per-relation","2000000","--max-relation-attempts","512"]
        if meta["config"]=="h1": cmd.append("--project-cofactor")
        o=json.loads(subprocess.run(cmd,capture_output=True,text=True,env=env0).stdout);bins.add(o["software"]["binary_blake3"])
        same=(o["ic"]["stages"]["counters"]==rec["ic"]["stages"]["counters"] and o["matched_rho"]["mean_gae"]==rec["matched_rho"]["mean_gae"]
              and o["matched_rho"]["counters_summed"]==rec["matched_rho"]["counters_summed"] and o["ic"]["scalar"]==rec["ic"]["scalar"])
        log["runs"].append({"file":"band","name":meta["instance"],"config":meta["config"],"recorded":{"group_additions":rec["ic"]["stages"]["counters"]["group_additions"],"rho_mean_gae":rec["matched_rho"]["mean_gae"]},
                            "replayed":{"group_additions":o["ic"]["stages"]["counters"]["group_additions"],"rho_mean_gae":o["matched_rho"]["mean_gae"],"ok":o["ic"]["scalar"]==rec["ic"]["scalar"]},"identical":same})
        if not same: log["mismatches"].append(f"band {meta['instance']} {meta['config']}: counters or rho differ on replay")
log["binary_blake3"]=sorted(bins);log["git_head"]=subprocess.run(["git","-C",ROOT,"rev-parse","HEAD"],capture_output=True,text=True).stdout.strip()
n=[r for r in log["runs"] if "identical" in r];log["replayed"]=len(n);log["identical"]=sum(r["identical"] for r in n)
log["status"]="pass" if n and not log["mismatches"] else "fail"
log["scalars_verified"]=bool(n) and all(r["replayed"]["ok"] for r in n)
shutil.rmtree(tmp,ignore_errors=True)
open(a.out,"w").write(json.dumps(log,indent=1))
print(f"nonce {nonce}: replayed {log['replayed']} runs, identical {log['identical']}, mismatches {len(log['mismatches'])}, status {log['status']}, binary {log['binary_blake3']}, head {log['git_head'][:10]}")
sys.exit(0 if log["status"]=="pass" else 1)
