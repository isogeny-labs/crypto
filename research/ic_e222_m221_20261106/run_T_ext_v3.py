#!/usr/bin/env python3
"""v3 IC (pure-G arms, b = 1, cheap table build) at large T on the valid cells of run_T_ext.py (24 and 25-bit groups,
instances_vsrho idx 1..4, seed 1, same frozen fb rule).  The batch-rho costs for the same instances/seed/T are the ones
already recorded in results/T_ext/rows.json (kappa rule unchanged)."""
import json,subprocess,math,os
IC="../../target/release/examples/ic_multi_target"
rows=[]
for pb in (24,25):
    for idx in range(1,5):
        f=f"results/instances_vsrho/W{pb}-{idx}.json";d=json.load(open(f));p=int(d["field"]["modulus"])
        for T in (4096,16384):
            m=2**round(math.log2((p*T)**0.25))
            o=json.loads(subprocess.run([IC,f,str(m),str(T),"1"],capture_output=True,text=True,timeout=900).stdout.strip().splitlines()[-1])
            o.update(kind="IC",pbits=pb,idx=idx,T=T,fb=m);rows.append(o);print(pb,idx,T,o["ok"]==T,flush=True)
os.makedirs("results/T_ext_v3",exist_ok=True)
json.dump(rows,open("results/T_ext_v3/rows.json","w"))
print(len(rows),"runs;",sum(r["ok"]==r["targets"] for r in rows),"fully verified")
