#!/usr/bin/env python3
"""Index-calculus cost against the matched rho with many rho runs, priced across the section 5a band.
For each IC output JSON, the rho reference is the mean over `runs` matched-rho walks on the same instance (rho200/<seed>/<name>.json
when present, else the output's own matched_rho block). Both arms are priced at w = 0, 0.1, 1 GAE per unpriced operation: cost(w) = gae + w * sum of
the arm's `*_uncharged` counters (the rho arm: its per-run counters). Prints one row per run.
usage: analyze_rho200.py --rho rho200 --json out.json  DIR:SEED [DIR:SEED ...]   (DIR holds {default,h1}/*.json)"""
import argparse,glob,json,math,os,statistics,subprocess,sys,tempfile
HERE=os.path.dirname(os.path.abspath(__file__));ROOT=os.path.dirname(HERE)
sys.path.insert(0,os.path.join(ROOT,"native_band"))
from native_band_model import model
HCOF={"E":4,"M":8,"W":1}
_ncache={}
def subgroup_order(seed,name):
    """n of instance `name` (e.g. E44-0) for generator seed `seed`; gen_instances.py is deterministic."""
    fam,bits=name[0],int(name[1:].split("-")[0])
    if (seed,fam,bits) not in _ncache:
        tmp=tempfile.mkdtemp(prefix="an-")
        subprocess.run([sys.executable,"-I",os.path.join(ROOT,"gen_instances.py"),"--out",tmp,"--families",fam,"--bits",str(bits),"--per","1","--seed",seed],capture_output=True,check=True)
        _ncache[(seed,fam,bits)]=int(json.load(open(os.path.join(tmp,name+".json")))["subgroup_order"])
    return _ncache[(seed,fam,bits)]
ap=argparse.ArgumentParser();ap.add_argument("--rho",default="rho200");ap.add_argument("--json");ap.add_argument("dirs",nargs="+");a=ap.parse_args()
W=(0,0.1,1)
def ic_cost(c,w): return c["group_additions"]+c["group_doublings"]+w*sum(v for k,v in c.items() if k.endswith("_uncharged"))
def rho_cost(mr,w): return mr["mean_gae"]+w*sum(v for k,v in mr["counters_per_run"].items() if k.endswith("_uncharged"))
rows=[]
for spec in a.dirs:
    d,seed=spec.split(":")
    for f in sorted(glob.glob(os.path.join(d,"*","*.json"))):
        if os.path.basename(f) in ("MANIFEST.json","binary.json"): continue
        o=json.load(open(f))
        if "ic" not in o or "_meta" not in o: continue
        m=o["_meta"];c=o["ic"]["stages"]["counters"]
        rf=os.path.join(a.rho,seed,m["instance"]+".json")
        if os.path.exists(rf): mr=json.load(open(rf))["matched_rho"];src="rho200"
        else: mr=o["matched_rho"];src="own"
        g=mr["gae_per_run"];K=len(g)
        n=subgroup_order(seed,m["instance"]);pred=model(n,HCOF[m["instance"][0]],m["F"],m["config"]=="h1")[1]
        row={"dir":d,"seed":seed,"instance":m["instance"],"config":m["config"],"F":m["F"],"relations":o["ic"]["stages"]["relations_collected"],
             "n":n,"trials":o["ic"]["stages"]["trials_total"],"trials_pred":pred,"obs_over_pred":o["ic"]["stages"]["trials_total"]/pred,"rho_runs":K,"rho_source":src,"rho_mean":mr["mean_gae"],"rho_median":mr["median_gae"],
             "rho_cv":(statistics.pstdev(g)/mr["mean_gae"]) if K>1 else None,"rho_all_verified":mr["all_verified"],"recovered":m["known_log_recovered"],
             "ic":{str(w):ic_cost(c,w) for w in W},"rho":{str(w):rho_cost(mr,w) for w in W}}
        row["distance"]={str(w):row["ic"][str(w)]/row["rho"][str(w)] for w in W}
        rows.append(row)
for r in rows:
    print("%-8s %-12s %-7s F=%-5d rel=%-5d trials=%.3g obs/pred=%.3f rho(%s,%d runs) mean=%.3g cv=%.2f | IC/rho w0 %.3g w0.1 %.3g w1 %.3g | rec=%s ver=%s"%(
        r["instance"],r["seed"][-2:],r["config"],r["F"],r["relations"],r["trials"],r["obs_over_pred"],r["rho_source"],r["rho_runs"],r["rho_mean"],r["rho_cv"] or 0,
        r["distance"]["0"],r["distance"]["0.1"],r["distance"]["1"],r["recovered"],r["rho_all_verified"]))
if a.json: json.dump(rows,open(a.json,"w"),indent=1)
