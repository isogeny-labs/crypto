#!/usr/bin/env python3
"""v3: same holdout and frozen rule; pure-G arms with b = 1 (no per-target arms) on top of the v2 table build. Multi-target holdout with a FROZEN fb rule (committed before the run):
   fb = 2^round(log2((p*T)^(1/4)))   (fits the best fb of every cell of the exploratory sweep results/multi/analysis.txt).
Instances: results/instances_vsrho idx 1..11 (the exploratory sweep used idx 0 only), seeds 1, 2, T in {128, 1024}.
Counts exact group additions in-code; unpriced work (sparse LA mod ops, one hash per trial, factor-base construction) is
reported separately so the 0 / 0.1 / 1 GAE band can be applied.  Verification: every recovered log is checked by one scalar
multiplication (priced) and equals the planted one."""
import json,subprocess,math,glob,os,sys
EX="../../target/release/examples/ic_multi_target"
rows=[]
for pb in (20,22,24):
    for idx in range(1,12):
        f=f"results/instances_vsrho/W{pb}-{idx}.json"
        d=json.load(open(f));p=int(d["field"]["modulus"])
        for T in (128,1024):
            m=2**round(math.log2((p*T)**0.25))
            for seed in (1,2):
                r=subprocess.run([EX,f,str(m),str(T),str(seed)],capture_output=True,text=True,timeout=280)
                o=json.loads(r.stdout.strip().splitlines()[-1]);o.update(pbits=pb,idx=idx,T=T,seed=seed,p=p,fb_rule=m)
                rows.append(o);print(pb,idx,T,seed,m,o["ok"]==T,round(o["adds_per_target_amortized"]),flush=True)
os.makedirs("results/multi_holdout_v3",exist_ok=True)
json.dump(rows,open("results/multi_holdout_v3/rows.json","w"))
print(len(rows),"configs;",sum(r["ok"]==r["T"] for r in rows),"fully verified")
