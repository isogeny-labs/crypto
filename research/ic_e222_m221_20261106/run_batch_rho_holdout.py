#!/usr/bin/env python3
"""Counted anchored batch rho on exactly the multi-target holdout cells (instances_vsrho idx 1..11, seeds 1 and 2,
T in {128, 1024}, 20/22/24-bit groups).  Rule fixed before the run: kappa in {0.5, 1, 2}; report the BEST kappa per
configuration (favours rho) and kappa = 1 (fixed).  Memory-unbounded, x-only lookups, no distinguished-point overhead."""
import json,subprocess,os
EX="../../target/release/examples/ic_batch_rho"
rows=[]
for pb in (20,22,24):
    for idx in range(1,12):
        f=f"results/instances_vsrho/W{pb}-{idx}.json"
        for T in (128,1024):
            for seed in (1,2):
                for kappa in (0.5,1,2):
                    r=subprocess.run([EX,f,str(T),str(seed),str(kappa)],capture_output=True,text=True,timeout=280)
                    o=json.loads(r.stdout.strip().splitlines()[-1]);o.update(pbits=pb,idx=idx,T=T,seed=seed)
                    rows.append(o)
        print(pb,idx,flush=True)
os.makedirs("results/batch_rho",exist_ok=True)
json.dump(rows,open("results/batch_rho/rows.json","w"))
print(len(rows),"runs;",sum(r["ok"]==r["targets"] for r in rows),"fully verified")
