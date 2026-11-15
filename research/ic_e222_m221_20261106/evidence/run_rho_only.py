#!/usr/bin/env python3
"""Matched-rho only: many runs of the counted negation-map walk on each generated instance, without the index-calculus stage.
`ic solve-prime-file` computes the matched rho after the IC stage; here the IC stage is given a one-trial budget (it ends bounded_unknown at once)
and only the `matched_rho` block of the output is kept. The walk seeds are the command's own (seed ^ (i+1)*0x9E3779B97F4A7C15), so the first K0
runs equal the runs of an earlier K0-run invocation with the same --seed. One JSON per instance under OUT/<seed>/.
usage: run_rho_only.py --ic target/release/ic --out DIR --seeds e222m221-mid-v1,e222m221-mid-v2 --bits 34,38,40,44 [--families W,E,M] [--runs 200]"""
import argparse,json,os,subprocess,sys,tempfile
HERE=os.path.dirname(os.path.abspath(__file__));ROOT=os.path.dirname(HERE)
ap=argparse.ArgumentParser();ap.add_argument("--ic",required=True);ap.add_argument("--out",required=True)
ap.add_argument("--seeds",required=True);ap.add_argument("--bits",required=True);ap.add_argument("--families",default="W,E,M")
ap.add_argument("--runs",type=int,default=200);ap.add_argument("--per",type=int,default=1);a=ap.parse_args()
n=0
for seed in a.seeds.split(","):
    tmp=tempfile.mkdtemp(prefix="rho-");inst=os.path.join(tmp,"inst")
    subprocess.run([sys.executable,"-I",os.path.join(ROOT,"gen_instances.py"),"--out",inst,"--families",a.families,"--bits",a.bits,"--per",str(a.per),"--seed",seed],capture_output=True,check=True)
    for f in sorted(os.listdir(inst)):
        if f=="MANIFEST.json" or not f.endswith(".json"): continue
        d=json.load(open(os.path.join(inst,f)));t=os.path.join(tmp,"_t.json")
        json.dump({"p":int(d["field"]["modulus"]),"a":int(d["a"]),"b":int(d["b"]),"n":int(d["subgroup_order"]),"gx":int(d["generator"]["x"]),"gy":int(d["generator"]["y"]),"qx":int(d["point"]["x"]),"qy":int(d["point"]["y"]),"group_order":int(d["group_order"])},open(t,"w"))
        r=subprocess.run([a.ic,"--json","solve-prime-file","--input",t,"--seed","1","--rho-steps","0","--matched-rho-runs",str(a.runs),"--fb-size","2","--extra-relations","0","--max-trials-per-relation","1","--max-relation-attempts","1"],capture_output=True,text=True)
        try: o=json.loads(r.stdout)
        except Exception: print("crash",seed,d["name"],r.stderr[-200:],file=sys.stderr);continue
        mr=o.get("matched_rho")
        if not mr: print("no matched_rho",seed,d["name"],file=sys.stderr);continue
        out={"_meta":{"instance":d["name"],"family":d["family"],"seed":seed,"r_bits":d["r_bits"],"n":d["subgroup_order"],"p":d["field"]["modulus"],"ic_stage":"not run (one-trial budget)","runs":a.runs,"software":o.get("software")},"matched_rho":mr}
        os.makedirs(os.path.join(a.out,seed),exist_ok=True)
        json.dump(out,open(os.path.join(a.out,seed,f"{d['name']}.json"),"w"),separators=(",",":"),sort_keys=True);n+=1
        g=mr["gae_per_run"];print(seed,d["name"],"n",d["subgroup_order"],"runs",len(g),"mean %.3g median %.3g"%(mr["mean_gae"],mr["median_gae"]),"verified",mr["all_verified"],flush=True)
print(n,"instances")
