#!/usr/bin/env python3
"""Counted multi-target BSGS on exactly the large-T holdout cells (instances_vsrho idx 1..11, seeds 1 and 2, T in {4096, 16384},
24 and 25-bit groups).  Rule fixed before the run: mfactor in {0.7, 1.0, 1.4}; the best per configuration is reported."""
import json,subprocess,os
EX="../../target/release/examples/ic_multi_bsgs"
rows=[]
for pb in (24,25):
    for idx in range(1,12):
        f=f"results/instances_vsrho/W{pb}-{idx}.json"
        for T in (4096,16384):
            for seed in (1,2):
                for mf in (0.7,1.0,1.4):
                    o=json.loads(subprocess.run([EX,f,str(T),str(seed),str(mf)],capture_output=True,text=True,timeout=600).stdout.strip().splitlines()[-1])
                    o.update(kind="BSGS",pbits=pb,idx=idx,T=T,seed=seed);rows.append(o)
        print(pb,idx,flush=True)
os.makedirs("results/bsgs_large_T",exist_ok=True)
json.dump(rows,open("results/bsgs_large_T/rows.json","w"))
print(len(rows),"runs;",sum(r["ok"]==r["targets"] for r in rows),"fully verified")
