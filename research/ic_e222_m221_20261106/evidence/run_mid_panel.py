#!/usr/bin/env python3
"""Mid-size panel: subgroup orders of about 2^26 to 2^34, in the regime n >> 2 F^2 where the native sizes live.
For each (family, bits) it generates instances (gen_instances.py, which supplies the group order), then runs
`ic solve-prime-file` (table forced with IC_SUM_TABLE=1 so F above 64 works) with --matched-rho-runs 8:
default configuration and, for cofactor curves, --project-cofactor. One JSON per run under OUT/{default,h1}/.
usage: run_mid_panel.py --ic target/release/ic --out DIR [--bits 26,30,34] [--fb 128,512] [--per 3] [--seed S]"""
import argparse,glob,json,os,subprocess,sys,tempfile
HERE=os.path.dirname(os.path.abspath(__file__));ROOT=os.path.dirname(HERE)
ap=argparse.ArgumentParser();ap.add_argument("--ic",required=True);ap.add_argument("--out",required=True)
ap.add_argument("--bits",default="26,30,34");ap.add_argument("--fb",default="128,512");ap.add_argument("--per",type=int,default=3)
ap.add_argument("--seed",default="e222m221-mid-v1");ap.add_argument("--families",default="W,E,M",help="subset of W,E,M (run families in parallel processes)");ap.add_argument("--skip-default-cofactor",action="store_true",help="run only the H1 configuration on E/M (the default arm costs hours above 36 bits)");ap.add_argument("--rho-runs",type=int,default=8);a=ap.parse_args()
tmp=tempfile.mkdtemp(prefix="mid-");inst=os.path.join(tmp,"inst")
subprocess.run([sys.executable,"-I",os.path.join(ROOT,"gen_instances.py"),"--out",inst,"--families",a.families,"--bits",a.bits,"--per",str(a.per),"--seed",a.seed],capture_output=True,check=True)
bins=set();n=0
for f in sorted(glob.glob(inst+"/*.json")):
    if f.endswith("MANIFEST.json"): continue
    d=json.load(open(f));t=os.path.join(tmp,"_t.json")
    json.dump({**{k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},"group_order":int(d["group_order"])},open(t,"w"))
    for F in map(int,a.fb.split(",")):
        for cfg,extra in (("default",[]),("h1",["--project-cofactor"])):
            if cfg=="h1" and d["family"]=="W": continue
            if cfg=="default" and d["family"]!="W" and a.skip_default_cofactor: continue
            env=dict(os.environ);env["IC_SUM_TABLE"]="1"
            r=subprocess.run([a.ic,"--json","solve-prime-file","--input",t,"--seed","1","--rho-steps","0","--matched-rho-runs",str(a.rho_runs),"--fb-size",str(F),"--extra-relations","2","--max-trials-per-relation","200000000","--max-relation-attempts","64"]+extra,capture_output=True,text=True,env=env)
            try: o=json.loads(r.stdout)
            except Exception: print("crash",d["name"],F,cfg,r.stderr[-200:],file=sys.stderr);continue
            if o.get("status")!="complete": print("not complete",d["name"],F,cfg,o.get("message") or o.get("status"),file=sys.stderr);continue
            bins.add(o["software"]["binary_blake3"]);o.pop("software",None)
            o["_meta"]={"instance":d["name"],"family":d["family"],"config":cfg,"r_bits":d["r_bits"],"F":F,"known_log_recovered":o["ic"]["scalar"]==d["fixture"]["known_log"]}
            os.makedirs(os.path.join(a.out,cfg),exist_ok=True)
            json.dump(o,open(os.path.join(a.out,cfg,f"{d['name']}-F{F}.json"),"w"),separators=(",",":"),sort_keys=True);n+=1
            print(d["name"],F,cfg,"trials",o["ic"]["stages"]["trials_total"],"ms",round(o["ic"]["stages"]["total_ms"]),flush=True)
json.dump({"runs":n,"binary_blake3":sorted(bins),"git_head":subprocess.run(["git","-C",ROOT,"rev-parse","HEAD"],capture_output=True,text=True).stdout.strip(),"args":vars(a)},open(os.path.join(a.out,"binary.json"),"w"),indent=1)
print(n,"runs; binary",sorted(bins))
