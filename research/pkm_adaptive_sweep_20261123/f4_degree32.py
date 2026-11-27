#!/usr/bin/env python3
"""Resume the fresh 13-bit degree-32 PKM component screen."""
import argparse, hashlib, json
from pathlib import Path
import f4_height4 as f4

HERE=Path(__file__).resolve().parent
EXPECTED="06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
SEED=20261128
REPEATS=16
INTERVAL="prime-abscissa:below=32"
INTERVAL_ORACLE="pkm-semaev:m=2,engine=f4,degree=8,bits=5,lo=0,mode=chain"


def command(binary,fb,oracle,preflight):
    cmd=[str(binary),"bench","--bits","13","--family","generic","--seed",str(SEED),"--repeats",str(REPEATS),
         "--targets","walk","--factor-base",fb,"--oracle","subtract" if preflight else oracle,
         "--max-trials","1" if preflight else "1000","--rho-runs","0" if preflight else "1","--json"]
    if not preflight: cmd += ["--solver-budget-seconds","5"]
    return cmd


def size(item):
    rows=(item.get("result") or {}).get("rows") or []
    values={r.get("factor_base",{}).get("abscissae") for r in rows}
    return values.pop() if len(rows)==REPEATS and len(values)==1 else None


def main():
    ap=argparse.ArgumentParser(description=__doc__); ap.add_argument("--bin",type=Path,required=True)
    ap.add_argument("--store",type=Path,default=HERE/"degree32.jsonl"); args=ap.parse_args()
    digest=hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest!=EXPECTED: raise ValueError("binary differs from protocol")
    records={}
    if args.store.exists():
        for line in args.store.read_text().splitlines():
            item=json.loads(line)
            if item["id"] in records: raise ValueError(f"duplicate {item['id']}")
            records[item["id"]]=item
    if "header" not in records:
        item={"id":"header","schema":"pkm.f4-degree32/v1","binary_sha256":digest,
              "protocol":"PROTOCOL_F4_DEGREE32_20261123.md","seed":SEED,"repeats":REPEATS}
        f4.append(args.store,item); records["header"]=item
    if "verdict" in records: print(json.dumps(records["verdict"],sort_keys=True)); return
    if "preflight:baseline" not in records:
        item={"id":"preflight:baseline",**f4.run(command(args.bin,INTERVAL,"",True),30)}
        f4.append(args.store,item); records[item["id"]]=item
    baseline=records["preflight:baseline"]; base_size=size(baseline)
    if baseline["exit_code"] not in (0,1) or base_size is None: raise RuntimeError("baseline preflight incomplete")
    instance=baseline["result"]["instance"]; selected=None
    for aux in range(32):
        key=f"preflight:{aux}"
        if key not in records:
            item={"id":key,"aux_seed":aux,**f4.run(command(args.bin,f"prime-isogeny:t=5,seed={aux},aux=ordinary","",True),30)}
            f4.append(args.store,item); records[key]=item
        item=records[key]
        if item["exit_code"] in (0,1) and item["result"].get("instance")==instance and size(item)==base_size:
            selected=aux; break
    if selected is None:
        item={"id":"verdict","complete":False,"reason":"no-size-match","instance":instance,"base_size":base_size}
        f4.append(args.store,item); print(json.dumps(item,sort_keys=True)); return
    for arm in ("baseline","candidate"):
        key=f"timed:{arm}"
        if key in records: continue
        fb=INTERVAL if arm=="baseline" else f"prime-isogeny:t=5,seed={selected},aux=ordinary"
        oracle=INTERVAL_ORACLE if arm=="baseline" else f4.ISOGENY_ORACLE
        item={"id":key,"arm":arm,"aux_seed":selected if arm=="candidate" else None,
              **f4.run(command(args.bin,fb,oracle,False),180)}
        f4.append(args.store,item); records[key]=item
    a,b=records["timed:baseline"],records["timed:candidate"]
    ar=(a.get("result") or {}).get("rows") or []; br=(b.get("result") or {}).get("rows") or []
    complete=(a["exit_code"]==b["exit_code"]==0 and a["result"].get("instance")==b["result"].get("instance")==instance
              and size(a)==size(b)==base_size and len(ar)==len(br)==REPEATS
              and all(x.get("verified") and y.get("verified") and x.get("recovered") is not None and x.get("recovered")==y.get("recovered") for x,y in zip(ar,br)))
    item={"id":"verdict","complete":complete,"instance":instance,"base_size":base_size,"aux_seed":selected}
    if complete:
        ag=sum(r["total_gae"] for r in ar); bg=sum(r["total_gae"] for r in br)
        item.update({"gae_ratio":bg/ag,"wall_ratio":b["wall_s"]/a["wall_s"],"cpu_ratio":b["child_cpu_s"]/a["child_cpu_s"],
                     "totals_gae":[ag,bg],"raw_relations":[sum(r["decomposition"]["relations_found"] for r in z) for z in (ar,br)],
                     "target_trials":[sum(r["decomposition"]["targets_tried"] for r in z) for z in (ar,br)]})
        item["adaptive_screen_pass"]=item["gae_ratio"]<.9 and item["wall_ratio"]<1 and item["cpu_ratio"]<1
    else: item["adaptive_screen_pass"]=False
    f4.append(args.store,item); print(json.dumps(item,sort_keys=True))

if __name__=="__main__": main()
