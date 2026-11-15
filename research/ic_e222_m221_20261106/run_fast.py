#!/usr/bin/env python3
"""Word-sweep protocol (v7): same as run_merged.py with the word-sized sweep (IC_FAST_SWEEP=1) as the baseline arm Ef.  `tune` (instances idx 0-9, seed 1): per arm, pick the fb with the lowest median solver total_ms over a
grid that is wide on both sides; write frozen.json.  `eval` (instances idx 10-29, seeds 1, 2): run the frozen settings, interleaved seeded order,
serial.  Arms: A shipped default (sweep, extra 64, fb 8); Eh hoisted sweep extra 2; D pair-sum table extra 2; Dw table + restarted walk extra 2.
Every row records the commit, the binary sha256, the exact argv and the IC_* environment actually used (all IC_* variables are scrubbed before each run)."""
import json,os,subprocess,random,glob,sys,time,hashlib,statistics as S
phase,inst,out=sys.argv[1:4];ic="../../target/release/ic"
BIN=hashlib.sha256(open(ic,"rb").read()).hexdigest()
COMMIT=subprocess.run(["git","rev-parse","HEAD"],capture_output=True,text=True).stdout.strip()
ENV={"A":{},"Eh":{"IC_HOIST_SWEEP":"1"},"Ef":{"IC_FAST_SWEEP":"1"},"D":{"IC_SUM_TABLE":"1"},"Dw":{"IC_SUM_TABLE":"1","IC_WALK":"1"}}
EXTRA={"A":64,"Eh":2,"Ef":2,"D":2,"Dw":2}
GRID={"Eh":[64,128,192,256,384,512],"Ef":[64,96,128,192,256,384,512],"D":[8,12,16,24,32,48,64],"Dw":[6,8,12,16,24,32,48]}
def idx(f): return int(os.path.basename(f).split("-")[1].split(".")[0])
def run(f,d,seed,arm,fb,t):
    json.dump({k:int(v) for k,v in dict(p=d["field"]["modulus"],a=d["a"],b=d["b"],n=d["subgroup_order"],gx=d["generator"]["x"],gy=d["generator"]["y"],qx=d["point"]["x"],qy=d["point"]["y"]).items()},open(t,"w"))
    env={k:v for k,v in os.environ.items() if not k.startswith("IC_")};env.update(ENV[arm])
    argv=[ic,"--json","solve-prime-file","--input",t,"--seed",str(seed),"--fb-size",str(fb),"--extra-relations",str(EXTRA[arm]),"--rho-steps","0","--max-trials-per-relation","200000","--max-relation-attempts","512"]
    t0=time.perf_counter()
    try: r=subprocess.run(argv,capture_output=True,text=True,env=env,timeout=120)
    except subprocess.TimeoutExpired: r=subprocess.CompletedProcess([],1,"","timeout")
    wall=(time.perf_counter()-t0)*1e3
    try:o=json.loads(r.stdout)
    except Exception:o={}
    st=(o.get("ic") or {}).get("stages",{})
    return dict(name=d["name"],r_bits=d["r_bits"],seed=seed,arm=arm,fb=fb,ok=(o.get("ic") or {}).get("scalar")==d["fixture"]["known_log"],scalar=(o.get("ic") or {}).get("scalar"),
      total_ms=st.get("total_ms"),wall_ms=wall,trials=st.get("trials_total"),rels=st.get("relations_collected"),commit=COMMIT,binary_sha256=BIN,ic_env={k:v for k,v in env.items() if k.startswith("IC_")},argv=argv[1:])
os.makedirs(out,exist_ok=True);t=os.path.join(out,"_t.json")
if phase=="tune":
    files=[f for f in sorted(glob.glob(inst+"/W*.json")) if idx(f)<10]
    jobs=[(f,json.load(open(f)),1,a,fb) for f in files for a in GRID for fb in GRID[a]]
    random.Random("tune-fast-v1").shuffle(jobs)
    rows=[run(f,d,s,a,fb,t) for f,d,s,a,fb in jobs]
    frozen={"A":8}
    for a in GRID:
        m={fb:S.median(r["total_ms"] for r in rows if r["arm"]==a and r["fb"]==fb and r["ok"]) for fb in GRID[a]}
        frozen[a]=min(m,key=m.get);print(a,{k:round(v,2) for k,v in m.items()},"->",frozen[a],"(edge)" if frozen[a] in (GRID[a][0],GRID[a][-1]) else "")
    json.dump(frozen,open(os.path.join(out,"frozen.json"),"w"));json.dump(rows,open(os.path.join(out,"tune_rows.json"),"w"))
else:
    frozen=json.load(open(os.path.join(out,"frozen.json")))
    files=[f for f in sorted(glob.glob(inst+"/W*.json")) if 10<=idx(f)<30]
    jobs=[(f,json.load(open(f)),s,a,frozen[a]) for f in files for s in (1,2) for a in frozen]
    random.Random("eval-fast-v1").shuffle(jobs)
    rows=[run(f,d,s,a,fb,t) for f,d,s,a,fb in jobs]
    with open(os.path.join(out,"rows.jsonl"),"w") as fh:
        for r in rows: fh.write(json.dumps(r)+"\n")
    print(len(rows),"rows;",sum(r["ok"] for r in rows),"verified")
