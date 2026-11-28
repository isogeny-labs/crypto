#!/usr/bin/env python3
"""Find and test the first size-matched fresh 13-bit seed-10 instance."""
import argparse, hashlib, json
from pathlib import Path
import f4_height4 as f4

HERE = Path(__file__).resolve().parent
EXPECTED = "06439ae5119b5ed69c413df90e7a65dc67541dd1dfb9f6ac8d77cf2bd4441c59"
REPEATS = 32


def command(binary, seed, fb, oracle, preflight):
    cmd = [str(binary), "bench", "--bits", "13", "--family", "generic", "--seed", str(seed),
           "--repeats", str(REPEATS), "--targets", "walk", "--factor-base", fb,
           "--oracle", "subtract" if preflight else oracle, "--max-trials", "1" if preflight else "1000",
           "--rho-runs", "0" if preflight else "1", "--json"]
    if not preflight: cmd += ["--solver-budget-seconds", "5"]
    return cmd


def size(item):
    rows = (item.get("result") or {}).get("rows") or []
    values = {r.get("factor_base", {}).get("abscissae") for r in rows}
    return values.pop() if len(rows) == REPEATS and len(values) == 1 else None


def load(path):
    out = {}
    if path.exists():
        for line in path.read_text().splitlines():
            item = json.loads(line)
            if item["id"] in out: raise ValueError(f"duplicate {item['id']}")
            out[item["id"]] = item
    return out


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--bin", type=Path, required=True)
    ap.add_argument("--store", type=Path, default=HERE / "scale13_instance_sweep.jsonl")
    args = ap.parse_args(); digest = hashlib.sha256(args.bin.read_bytes()).hexdigest()
    if digest != EXPECTED: raise ValueError("binary differs from protocol")
    records = load(args.store)
    if "header" not in records:
        item = {"id":"header", "schema":"pkm.f4-scale13-instance-sweep/v1", "binary_sha256":digest,
                "protocol":"PROTOCOL_F4_SCALE13_INSTANCE_SWEEP_20261123.md", "instance_seeds":list(range(20261125,20261141))}
        f4.append(args.store,item); records["header"]=item
    if "verdict" in records: print(json.dumps(records["verdict"],sort_keys=True)); return
    selected = None
    for seed in range(20261125, 20261141):
        ck = f"preflight:{seed}:candidate"
        if ck not in records:
            item={"id":ck,"seed":seed,**f4.run(command(args.bin,seed,"prime-isogeny:t=4,seed=10,aux=ordinary","",True),30)}
            f4.append(args.store,item); records[ck]=item
        candidate=records[ck]; cs=size(candidate)
        if candidate["exit_code"] not in (0,1) or cs is None: continue
        instance=candidate["result"]["instance"]
        for below in (8,16,32):
            key=f"preflight:{seed}:interval:{below}"
            if key not in records:
                item={"id":key,"seed":seed,"below":below,**f4.run(command(args.bin,seed,f"prime-abscissa:below={below}","",True),30)}
                f4.append(args.store,item); records[key]=item
            interval=records[key]
            if interval["exit_code"] in (0,1) and interval["result"].get("instance")==instance and size(interval)==cs:
                selected=(seed,instance,below,cs); break
        if selected: break
    if not selected:
        item={"id":"verdict","complete":False,"reason":"no-size-matched-instance"}; f4.append(args.store,item); return
    seed,instance,below,base_size=selected
    order=("baseline:1","candidate:1","candidate:2","baseline:2")
    for arm_rep in order:
        key=f"timed:{arm_rep}"
        if key in records: continue
        arm=arm_rep.split(":")[0]
        fb=f"prime-abscissa:below={below}" if arm=="baseline" else "prime-isogeny:t=4,seed=10,aux=ordinary"
        oracle=f4.INTERVAL_ORACLE.replace("bits=4",f"bits={below.bit_length()-1}") if arm=="baseline" else f4.ISOGENY_ORACLE
        item={"id":key,"arm":arm,"seed":seed,**f4.run(command(args.bin,seed,fb,oracle,False),180)}
        f4.append(args.store,item); records[key]=item
    runs=[records[f"timed:{x}"] for x in order]; rowsets=[x["result"].get("rows",[]) if x.get("result") else [] for x in runs]
    complete=(all(x["exit_code"]==0 and x["result"].get("instance")==instance and size(x)==base_size for x in runs)
              and all(len(rs)==REPEATS for rs in rowsets)
              and all(all(r.get("verified") and r.get("recovered") is not None for r in rs) for rs in rowsets)
              and all(len({rowsets[j][i]["recovered"] for j in range(4)})==1 for i in range(REPEATS)))
    item={"id":"verdict","complete":complete,"seed":seed,"instance":instance,"below":below,"base_size":base_size}
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
