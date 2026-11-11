#!/usr/bin/env python3
"""Recompute the headline numbers of claims.json from the raw data (stdlib only).
Reads raw rows from this repository (or --robust-rows PATH for the artifact-store file),
checks each file's sha256 against claims.json, prints a validator-report-shaped JSON.
This script is the PRODUCER's recompute procedure: the report it writes says
independent=false. An independent validator must run their own code."""
import argparse,hashlib,json,os,re,statistics,sys
HERE=os.path.dirname(os.path.abspath(__file__));ROOT=os.path.dirname(HERE)
ap=argparse.ArgumentParser();ap.add_argument("--robust-rows");ap.add_argument("--out");a=ap.parse_args()
C=json.load(open(os.path.join(HERE,"claims.json")))
def sha(p): return hashlib.sha256(open(p,'rb').read()).hexdigest()
def path(k):
    r=C["raw_data"][k]
    if k=="robust" and a.robust_rows: return a.robust_rows
    return os.path.join(ROOT,r["path"])
raw=[];problems=[]
for k,r in C["raw_data"].items():
    p=path(k)
    if not os.path.exists(p): problems.append(f"{k}: {p} not found"); continue
    ok=sha(p)==r["sha256"]
    # paths are relative to this evidence directory (what the gate resolves); the artifact-store file is
    # recorded under its logical repository path, with a note when it was read from a fetched copy
    ent={"path":os.path.relpath(os.path.join(ROOT,r["path"]),HERE),"sha256":sha(p),"pinned_sha256_matches":ok}
    if k=="robust" and a.robust_rows: ent["read_from_fetched_copy"]=True; ent["in_repository"]=False
    raw.append(ent)
    if not ok: problems.append(f"{k}: sha256 differs from claims.json")
def rows(k): return [json.loads(l) for l in open(path(k))]
val={}
# C1 paired
if os.path.exists(path("paired")):
    P=rows("paired");old={(r["name"],r["seed"]):r for r in P if r["arm"]=="OLD"};new={(r["name"],r["seed"]):r for r in P if r["arm"]=="NEW"}
    ks=sorted(set(old)&set(new))
    val["paired_median_total_ms_ratio_new_over_old"]=statistics.median([new[k]["total_ms"]/old[k]["total_ms"] for k in ks])
    val["paired_n"]=len(ks)
    val["rung22_median_trials_old"]=statistics.median([old[k]["trials"] for k in ks if old[k]["r_bits"]==22])
    val["rung22_median_trials_new"]=statistics.median([new[k]["trials"] for k in ks if new[k]["r_bits"]==22])
    val["paired_all_recovered"]=int(all(r["ok"] for r in P))
# C2 robust
if os.path.exists(path("robust")):
    Rb=rows("robust");val["robust_runs"]=len(Rb);val["robust_failures"]=sum(not r["ok"] for r in Rb)
# C3 h1
if os.path.exists(path("h1_defaults")):
    H=rows("h1_defaults");off={(r["name"],r["seed"]):r for r in H if r["arm"]=="off"};on={(r["name"],r["seed"]):r for r in H if r["arm"]=="on"}
    for fam in "EMW":
        ks=[k for k in off if off[k]["family"]==fam];rat=[on[k]["trials"]/off[k]["trials"] for k in ks]
        val[f"{fam}_trials_on_over_off_median"]=statistics.median(rat)
        if fam=="W": val["W_pairs_exactly_1"]=sum(x==1.0 for x in rat)
    val["h1_all_recovered"]=int(all(r["ok"] for r in H))
# C5 band (section 5a): re-derived from the raw per-instance outputs listed in the manifest
if os.path.exists(path("band")):
    man=json.load(open(path("band")));bdir=os.path.dirname(path("band"))
    NON_OP={"residual_table_bytes_uncharged","factor_base_vec_bytes_floor_uncharged","native_table_bytes_uncharged"}
    def unp(c): return sum(v for k,v in c.items() if k.endswith("_uncharged") and k not in NON_OP)
    W=(0.0,0.1,1.0);band={}
    for rel,h in man["files"].items():
        p=os.path.join(bdir,os.path.relpath(rel,"band"))
        if sha(p)!=h: problems.append(f"band: {rel} sha256 differs from the manifest");continue
        o=json.load(open(p));cfg=o["_meta"]["config"];c=o["ic"]["stages"]["counters"];m=o["matched_rho"]
        icc=c["group_additions"]+c["group_doublings"];icu=unp(c);rc=m["mean_gae"];ru=unp(m["counters_per_run"])
        r=[(icc+w*icu)/(rc+w*ru) for w in W]
        band.setdefault(cfg,[]).append((r,all(x>1 for x in r) or all(x<1 for x in r),o["ic"]["stages"].get("counters_unemitted",[])))
    for cfg in ("default","h1"):
        rows=band.get(cfg,[])
        val[f"band_{cfg}_instances"]=len(rows)
        val[f"band_{cfg}_order_holds_instances"]=sum(1 for r in rows if r[1] and all(x>1 for x in r[0]))
    if band.get("default"):
        for i,w in enumerate(("w0","w0.1","w1")):
            val[f"band_default_median_ic_over_rho_{w}"]=statistics.median(r[0][i] for r in band["default"])
    if any(r[2] for rows in band.values() for r in rows): problems.append("band: some IC outputs list unemitted counters")
# C6 counter model (checked against the raw band files) and its projection
if os.path.exists(path("native_band_model")) and os.path.exists(path("band")):
    import importlib.util
    sp=importlib.util.spec_from_file_location("nbm",path("native_band_model"));nbm=importlib.util.module_from_spec(sp);sp.loader.exec_module(nbm)
    vv,_=nbm.validate(os.path.dirname(path("band")));pr=nbm.project()
    val["model_default_trials_obs_over_pred"]=vv["default"]["trials_total"]["median_obs_over_pred"]
    val["model_h1_trials_obs_over_pred"]=vv["h1"]["trials_total"]["median_obs_over_pred"]
    val["model_default_inserts_obs_over_pred"]=vv["default"]["table_insert_attempts_uncharged"]["median_obs_over_pred"]
    val["model_h1_negations_obs_over_pred"]=vv["h1"]["point_negations_uncharged"]["median_obs_over_pred"]
    g=lambda cv:next(x for x in pr if x["curve"]==cv and x["H1"] and x["log2_F"]==20)["log2_ic"]["0"]
    val["projection_E222_H1_F2e20_log2_ic_w0"]=g("E-222");val["projection_M221_H1_F2e20_log2_ic_w0"]=g("M-221")
    val["projection_max_abs_log2_ic_w0_minus_w1"]=max(abs(x["log2_ic"]["0"]-x["log2_ic"]["1"]) for x in pr)
# C4 native
t=open(path("native_timing")).read()
for name,key in (("E-222","E222"),("M-221","M221")):
    m=re.search(name+r".*?joint/former=([0-9.]+)",t)
    if m: val[f"{key}_joint_over_former"]=float(m.group(1))
headline=[];fail=[]
for c in C["claims"]:
    for h in c["headline"]:
        if h["name"] not in val: problems.append(f"{c['id']}:{h['name']} not recomputed (raw data missing)"); continue
        rv=val[h["name"]];ok=abs(rv-h["claimed"])<=h["rel_tol"]*abs(h["claimed"])+(0 if h["rel_tol"] else 0)
        if h["rel_tol"]==0.0: ok=rv==h["claimed"]
        headline.append({"name":h["name"],"claimed":h["claimed"],"recomputed":rv,"rel_tol":h["rel_tol"],"within_tolerance":ok})
        if not ok: fail.append(h["name"])
rep={"schema":"validator-report-draft/v1","status":"pass" if not fail and not problems else "fail","independent":False,
     "producer_run":True,"note":"written by the producer session's own recompute script; not an independent validation",
     "raw_data":raw,"headline":headline,"problems":problems,"failed":fail}
s=json.dumps(rep,indent=1)
if a.out: open(a.out,"w").write(s)
print(s);sys.exit(0 if rep["status"]=="pass" else 1)
