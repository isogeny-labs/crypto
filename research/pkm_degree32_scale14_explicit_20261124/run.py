#!/usr/bin/env python3
"""Resume the frozen explicit-curve 14-bit degree-32 panel."""
import argparse, datetime, hashlib, json, os, statistics, subprocess, sys
from pathlib import Path

HERE=Path(__file__).resolve().parent
sys.path.insert(0,str(HERE.parent/"pkm_adaptive_sweep_20261123"))
import f4_height4 as f4  # noqa: E402

EXPECTED="06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
REPEATS=16
BASE="prime-abscissa:below=32"
CAND="prime-isogeny:t=5,seed=5,aux=ordinary"
BOR="pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"
COR="pkm-semaev:m=2,engine=f4,degree=8,mode=chain"
ORDER=("baseline:1","candidate:1","candidate:2","baseline:2")
# i,p,a,b,r,x,y,z,target-seed,baseline-size,candidate-size
DATA=[
(1,16381,1,17,16381,2,384,1,20262001,13,17),(2,16369,1,14,16319,1,4,1,20262002,19,13),
(3,16363,1,35,16453,0,181,1,20262003,16,16),(4,16361,1,3,16249,1,1464,1,20262004,13,18),
(5,16349,1,66,16249,0,7709,1,20262005,16,14),(6,16339,1,4,16451,0,2,1,20262006,17,7),
(7,16333,1,15,16453,1,7903,1,20262007,15,23),(8,16319,1,3,16547,0,1411,1,20262008,18,22),
(9,16301,1,14,16087,0,6768,1,20262009,20,17),(10,16273,1,7,16427,1,3,1,20262010,15,20),
(11,16267,1,53,16417,0,1980,1,20262011,20,17),(12,16253,1,26,16061,2,6,1,20262012,13,14),
(13,16249,1,16,16069,0,4,1,20262013,16,16),(14,16231,1,31,16369,2,2434,1,20262014,17,18),
(15,16229,1,22,16061,1,5817,1,20262015,18,10),(16,16223,1,1,16333,0,1,1,20262016,16,16)]
CURVES=[dict(zip(("index","p","a","b","r","x","y","z","seed","bs","cs"),x)) for x in DATA]

def cmd(binary,c,fb,oracle,pre):
 g=f"{c['x']},{c['y']},{c['z']}"
 x=[str(binary),"bench","--efd-model","shortw","--efd-chart","jacobian","--efd-p",str(c["p"]),
    "--efd-param",f"a={c['a']}","--efd-param",f"b={c['b']}","--efd-order",str(c["r"]),
    "--efd-cofactor","1","--efd-generator",g,"--seed",str(c["seed"]),"--repeats",str(REPEATS),
    "--targets","walk","--factor-base",fb,"--oracle","subtract" if pre else oracle,
    "--max-trials","1" if pre else "1000","--rho-runs","0" if pre else "1","--json"]
 if not pre:x += ["--solver-budget-seconds","5"]
 return x

def load(path):
 out={}
 if path.exists():
  for line in path.read_text().splitlines():
   x=json.loads(line)
   if x["id"] in out:raise ValueError(f"duplicate {x['id']}")
   out[x["id"]]=x
 return out

def size(x):
 rows=(x.get("result") or {}).get("rows") or [];v={r.get("factor_base",{}).get("abscissae") for r in rows}
 return v.pop() if len(rows)==REPEATS and len(v)==1 else None

def total(rows,*keys):
 z=0
 for r in rows:
  v=r
  for k in keys:v=v.get(k,{}) if isinstance(v,dict) else {}
  z += v if isinstance(v,(int,float)) else 0
 return z

def summarize(x):
 rows=x["result"]["rows"] if x.get("result") else []
 ns=[r["decomposition"]["solver"].get("ns_per_op") for r in rows if r.get("decomposition",{}).get("solver")]
 rho=[r["s_over_rho"] for r in rows if isinstance(r.get("s_over_rho"),(int,float))]
 systems=sorted({json.dumps(r.get("decomposition",{}).get("system",{}),sort_keys=True) for r in rows})
 return {"exit_code":x.get("exit_code"),"rows":len(rows),"verified":sum(bool(r.get("verified")) for r in rows),
  "distinct":len({r.get("recovered") for r in rows if r.get("recovered") is not None}),"wall_s":x.get("wall_s"),
  "cpu_s":x.get("child_cpu_s"),"gae":total(rows,"total_gae"),"relations":total(rows,"decomposition","relations_found"),
  "trials":total(rows,"decomposition","targets_tried"),"ops":total(rows,"decomposition","solver","ops"),
  "solver_wall_s":total(rows,"decomposition","solver","wall_ns")/1e9,
  "median_ns_op":statistics.median(ns) if ns else None,"median_s_rho":statistics.median(rho) if rho else None,
  "systems":[json.loads(v) for v in systems]}

def main():
 ap=argparse.ArgumentParser();ap.add_argument("--bin",type=Path,required=True);ap.add_argument("--store",type=Path,default=HERE/"explicit14.jsonl")
 ap.add_argument("--run-timed",action="store_true");args=ap.parse_args();digest=hashlib.sha256(args.bin.read_bytes()).hexdigest()
 if digest!=EXPECTED:raise ValueError("binary hash differs")
 rec=load(args.store)
 if "header" not in rec:
  x={"id":"header","schema":"pkm.explicit14/v1","binary_sha256":digest,"protocol":"PROTOCOL.md","curves":CURVES,"order":list(ORDER)}
  f4.append(args.store,x);rec["header"]=x
 if "verdict" in rec:print(json.dumps(rec["verdict"],sort_keys=True));return
 selected=[]
 for c in CURVES:
  for arm,fb in (("baseline",BASE),("candidate",CAND)):
   k=f"preflight:{c['index']}:{arm}"
   if k not in rec:
    x={"id":k,"curve_index":c["index"],"arm":arm,**f4.run(cmd(args.bin,c,fb,"",True),30)};f4.append(args.store,x);rec[k]=x
  a,b=rec[f"preflight:{c['index']}:baseline"],rec[f"preflight:{c['index']}:candidate"]
  expected=f"EFD shortw/jacobian over F_{c['p']}";observed=[size(a),size(b)]
  valid=(a["exit_code"] in (0,1) and b["exit_code"] in (0,1) and a["result"]["instance"]==b["result"]["instance"]==expected
         and observed==[c["bs"],c["cs"]] and all(len(q["result"]["rows"])==REPEATS for q in (a,b))
         and all(all(r["r"]==c["r"] for r in q["result"]["rows"]) for q in (a,b)))
  if not valid:raise RuntimeError(f"curve {c['index']} preflight mismatch: {observed}")
  if observed[0]==observed[1]:selected.append({"curve":c,"instance":expected,"base_size":observed[0]})
 selected=selected[:3]
 if [x["curve"]["index"] for x in selected] != [3,13,16]:raise RuntimeError("selection mismatch")
 if "selection" not in rec:
  x={"id":"selection","instances":selected};f4.append(args.store,x);rec["selection"]=x
 if not args.run_timed:
  print(json.dumps(rec["selection"],sort_keys=True));return
 if "load-check" not in rec:
  ps=subprocess.run(["ps","-axo","pid,pcpu,comm,args"],capture_output=True,text=True,check=True).stdout.splitlines()
  heavy=[line for line in ps if any(word in line for word in ("cargo","rustc","ic bench","pkm_degree32_fullcost"))]
  receipt={"time":datetime.datetime.now().astimezone().isoformat(),"loadavg":list(os.getloadavg()),
           "logical_cpus":os.cpu_count(),"matching_processes":heavy}
  if heavy:
   n=sum(k.startswith("load-blocked:") for k in rec)+1;receipt["id"]=f"load-blocked:{n}"
   f4.append(args.store,receipt);print(json.dumps(receipt,sort_keys=True));return
  receipt["id"]="load-check";f4.append(args.store,receipt);rec["load-check"]=receipt
 for choice in selected:
  c=choice["curve"]
  for ar in ORDER:
   k=f"timed:{c['index']}:{ar}"
   if k in rec:continue
   arm=ar.split(":")[0];fb=BASE if arm=="baseline" else CAND;oracle=BOR if arm=="baseline" else COR
   x={"id":k,"curve_index":c["index"],"arm":arm,**f4.run(cmd(args.bin,c,fb,oracle,False),600)}
   f4.append(args.store,x);rec[k]=x
 verdicts=[];agg={k:0 for k in ("bg","cg","bw","cw","bc","cc","br","cr","bt","ct")}
 for choice in selected:
  c=choice["curve"];runs=[rec[f"timed:{c['index']}:{x}"] for x in ORDER];rowsets=[(x.get("result") or {}).get("rows") or [] for x in runs]
  complete=(all(x.get("exit_code")==0 and x.get("result",{}).get("instance")==choice["instance"] and size(x)==choice["base_size"] for x in runs)
   and all(len(rs)==REPEATS and len({r.get("recovered") for r in rs})==REPEATS and all(r.get("verified") and r.get("r")==c["r"] for r in rs) for rs in rowsets)
   and all(len({rowsets[j][i]["recovered"] for j in range(4)})==1 for i in range(REPEATS)))
  ss=[summarize(x) for x in runs];entry={"curve_index":c["index"],"curve":c,"instance":choice["instance"],"base_size":choice["base_size"],"complete":complete,"runs":dict(zip(ORDER,ss))}
  if complete:
   pairs=[]
   for ai,ci in ((0,1),(3,2)):
    pairs.append({"gae_ratio":ss[ci]["gae"]/ss[ai]["gae"],"wall_ratio":ss[ci]["wall_s"]/ss[ai]["wall_s"],"cpu_ratio":ss[ci]["cpu_s"]/ss[ai]["cpu_s"]})
   stable={}
   for arm,ii in (("baseline",(0,3)),("candidate",(1,2))):
    g=[ss[i]["gae"] for i in ii];w=[ss[i]["wall_s"] for i in ii];stable[arm]={"gae_max_over_min":max(g)/min(g),"wall_max_over_min":max(w)/min(w)}
   stability_pass=all(x["gae_max_over_min"]<=1.5 and x["wall_max_over_min"]<=1.5 for x in stable.values())
   bg=sum(ss[i]["gae"] for i in (0,3));cg=sum(ss[i]["gae"] for i in (1,2));passed=all(p["gae_ratio"]<.9 and p["wall_ratio"]<1 and p["cpu_ratio"]<1 for p in pairs) and cg/bg<.9 and stability_pass
   entry.update({"pairs":pairs,"combined_gae_ratio":cg/bg,"stability":stable,"stable":stability_pass,"component_gate_pass":passed})
   for i in (0,3):agg["bg"]+=ss[i]["gae"];agg["bw"]+=ss[i]["wall_s"];agg["bc"]+=ss[i]["cpu_s"];agg["br"]+=ss[i]["relations"];agg["bt"]+=ss[i]["trials"]
   for i in (1,2):agg["cg"]+=ss[i]["gae"];agg["cw"]+=ss[i]["wall_s"];agg["cc"]+=ss[i]["cpu_s"];agg["cr"]+=ss[i]["relations"];agg["ct"]+=ss[i]["trials"]
  else:entry.update({"stable":False,"component_gate_pass":False})
  verdicts.append(entry)
 complete=all(x["complete"] for x in verdicts);overall={"complete":complete}
 if complete:
  overall.update({"gae_ratio":agg["cg"]/agg["bg"],"wall_ratio":agg["cw"]/agg["bw"],"cpu_ratio":agg["cc"]/agg["bc"],
   "relation_probability_ratio":(agg["cr"]/agg["ct"])/(agg["br"]/agg["bt"]),"useful_relations_per_gae_ratio":(agg["cr"]/agg["cg"])/(agg["br"]/agg["bg"]),"totals":agg})
 passed=complete and all(x["component_gate_pass"] for x in verdicts) and overall["gae_ratio"]<1 and overall["wall_ratio"]<1 and overall["cpu_ratio"]<1
 x={"id":"verdict","complete":complete,"instances":verdicts,"overall":overall,"explicit14_transfer_gate_pass":passed}
 f4.append(args.store,x);print(json.dumps(x,sort_keys=True))

if __name__=="__main__":main()
