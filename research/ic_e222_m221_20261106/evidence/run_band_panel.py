#!/usr/bin/env python3
"""Regenerate the band panel: 72 instances (E, M, W; generator rungs 12, 16, 20; seed e222m221-h1default-v1),
each solved by `ic solve-prime-file --matched-rho-runs 16` with the default configuration and, for the
cofactor curves, with --project-cofactor. Writes one JSON per instance and configuration under OUT/{default,h1}/.
The software block (binary identity) is dropped from each file and recorded once in OUT/binary.json.
usage: run_band_panel.py --ic target/release/ic --out DIR"""
import argparse,glob,json,os,subprocess,sys,tempfile
HERE=os.path.dirname(os.path.abspath(__file__));ROOT=os.path.dirname(HERE)
ap=argparse.ArgumentParser();ap.add_argument("--ic",required=True);ap.add_argument("--out",required=True);a=ap.parse_args()
tmp=tempfile.mkdtemp(prefix="band-");inst=os.path.join(tmp,"inst")
subprocess.run([sys.executable,"-I",os.path.join(ROOT,"gen_instances.py"),"--out",inst,"--families","E,M,W","--bits","12,16,20","--per","8","--seed","e222m221-h1default-v1"],capture_output=True,check=True)
bins=set();n=0
for f in sorted(glob.glob(inst+"/*.json")):
    if f.endswith("MANIFEST.json"): continue
    d=json.load(open(f));t=os.path.join(tmp,"_t.json")
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    for cfg,extra in (("default",[]),("h1",["--project-cofactor"])):
        if cfg=="h1" and d["family"]=="W": continue
        r=subprocess.run([a.ic,"--json","solve-prime-file","--input",t,"--seed","1","--rho-steps","0","--matched-rho-runs","16","--max-trials-per-relation","2000000","--max-relation-attempts","512"]+extra,capture_output=True,text=True)
        o=json.loads(r.stdout);bins.add(o["software"]["binary_blake3"]);o.pop("software",None)
        o["_meta"]={"instance":d["name"],"family":d["family"],"config":cfg,"r_bits":d["r_bits"]}
        os.makedirs(os.path.join(a.out,cfg),exist_ok=True)
        json.dump(o,open(os.path.join(a.out,cfg,d["name"]+".json"),"w"),separators=(",",":"),sort_keys=True);n+=1
json.dump({"runs":n,"binary_blake3":sorted(bins),"git_head":subprocess.run(["git","-C",ROOT,"rev-parse","HEAD"],capture_output=True,text=True).stdout.strip()},open(os.path.join(a.out,"binary.json"),"w"),indent=1)
print(n,"runs; binary",sorted(bins))
