#!/usr/bin/env python3
"""Fresh-instance mirrored replication for the degree-32 seed-5 map."""
import argparse, hashlib, json
from pathlib import Path
import f4_height4 as f4
import f4_degree32 as d32

HERE=Path(__file__).resolve().parent


def command(binary,seed,fb,oracle,preflight):
    cmd=[str(binary),"bench","--bits","13","--family","generic","--seed",str(seed),"--repeats",str(d32.REPEATS),
         "--targets","walk","--factor-base",fb,"--oracle","subtract" if preflight else oracle,
         "--max-trials","1" if preflight else "1000","--rho-runs","0" if preflight else "1","--json"]
    if not preflight: cmd += ["--solver-budget-seconds","5"]
    return cmd


def main():
    ap=argparse.ArgumentParser(description=__doc__); ap.add_argument("--bin",type=Path,required=True)
    ap.add_argument("--store",type=Path,default=HERE/"degree32_replication.jsonl")
    ap.add_argument("--seed-start",type=int,default=20261129)
    ap.add_argument("--seed-end",type=int,default=20261140)
    ap.add_argument("--protocol",default="PROTOCOL_F4_DEGREE32_REPLICATION_20261123.md")
    args=ap.parse_args()
    digest=hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest!=d32.EXPECTED: raise ValueError("binary differs from protocol")
    records={}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item=json.loads(line)
            if item["id"] in records: raise ValueError(f"duplicate {item['id']}")
            records[item["id"]]=item
    if "header" not in records:
        item={"id":"header","schema":"pkm.f4-degree32-replication/v1","binary_sha256":digest,
              "protocol":args.protocol,"instance_seeds":list(range(args.seed_start,args.seed_end+1))}
        f4.append(args.store,item); records["header"]=item
    if "verdict" in records: print(json.dumps(records["verdict"],sort_keys=True)); return
    selected=None
    for seed in range(args.seed_start,args.seed_end+1):
        for arm,fb in (("baseline",d32.INTERVAL),("candidate","prime-isogeny:t=5,seed=5,aux=ordinary")):
            key=f"preflight:{seed}:{arm}"
            if key not in records:
                item={"id":key,"seed":seed,"arm":arm,**f4.run(command(args.bin,seed,fb,"",True),30)}
                f4.append(args.store,item); records[key]=item
        a,b=records[f"preflight:{seed}:baseline"],records[f"preflight:{seed}:candidate"]
        if (a["exit_code"] in (0,1) and b["exit_code"] in (0,1) and a["result"].get("instance")==b["result"].get("instance")
                and d32.size(a) is not None and d32.size(a)==d32.size(b)):
            selected=(seed,a["result"]["instance"],d32.size(a)); break
    if selected is None:
        item={"id":"verdict","complete":False,"reason":"no-size-matched-instance"}; f4.append(args.store,item); return
    seed,instance,base_size=selected; order=("baseline:1","candidate:1","candidate:2","baseline:2")
    for arm_rep in order:
        key=f"timed:{arm_rep}"
        if key in records: continue
        arm=arm_rep.split(":")[0]; fb=d32.INTERVAL if arm=="baseline" else "prime-isogeny:t=5,seed=5,aux=ordinary"
        oracle=d32.INTERVAL_ORACLE if arm=="baseline" else f4.ISOGENY_ORACLE
        item={"id":key,"arm":arm,"seed":seed,**f4.run(command(args.bin,seed,fb,oracle,False),240)}
        f4.append(args.store,item); records[key]=item
    runs=[records[f"timed:{x}"] for x in order]; rowsets=[(x.get("result") or {}).get("rows") or [] for x in runs]
    complete=(all(x["exit_code"]==0 and x["result"].get("instance")==instance and d32.size(x)==base_size for x in runs)
              and all(len(rs)==d32.REPEATS for rs in rowsets)
              and all(all(r.get("verified") and r.get("recovered") is not None for r in rs) for rs in rowsets)
              and all(len({rowsets[j][i]["recovered"] for j in range(4)})==1 for i in range(d32.REPEATS)))
    item={"id":"verdict","complete":complete,"seed":seed,"instance":instance,"base_size":base_size}
    if complete:
        pairs=[]
        for ai,bi in ((0,1),(3,2)):
            a,b=runs[ai],runs[bi]; ag=sum(r["total_gae"] for r in rowsets[ai]); bg=sum(r["total_gae"] for r in rowsets[bi])
            pairs.append({"gae_ratio":bg/ag,"wall_ratio":b["wall_s"]/a["wall_s"],"cpu_ratio":b["child_cpu_s"]/a["child_cpu_s"],
                          "totals_gae":[ag,bg],"wall_s":[a["wall_s"],b["wall_s"]],"cpu_s":[a["child_cpu_s"],b["child_cpu_s"]]})
        item["pairs"]=pairs; item["combined_gae_ratio"]=sum(p["totals_gae"][1] for p in pairs)/sum(p["totals_gae"][0] for p in pairs)
        item["raw_relations"]=[sum(r["decomposition"]["relations_found"] for rs in (rowsets[0],rowsets[3]) for r in rs),sum(r["decomposition"]["relations_found"] for rs in (rowsets[1],rowsets[2]) for r in rs)]
        item["target_trials"]=[sum(r["decomposition"]["targets_tried"] for rs in (rowsets[0],rowsets[3]) for r in rs),sum(r["decomposition"]["targets_tried"] for rs in (rowsets[1],rowsets[2]) for r in rs)]
        item["robust_component_lead"]=item["combined_gae_ratio"]<.9 and all(p["gae_ratio"]<.9 and p["wall_ratio"]<1 and p["cpu_ratio"]<1 for p in pairs)
    else: item["robust_component_lead"]=False
    f4.append(args.store,item); print(json.dumps(item,sort_keys=True))

if __name__=="__main__": main()
