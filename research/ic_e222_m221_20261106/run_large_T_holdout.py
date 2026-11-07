#!/usr/bin/env python3
"""Large-T holdout, IC v3 vs counted anchored batch rho.  Rules fixed before the run:
IC: fb = 2^round(log2((p T)^(1/4))) (unchanged), v3 code (pure-G arms, b = 1, table with free negation).
Rho: kappa in {0.5, 0.7, 1.0}, best kappa per configuration at each price (0.7 added after a kappa scan showed rho's
minimum at 0.5-0.7; kappa = 2 never wins and times out at T = 16384).
Cells: 24 and 25-bit groups (valid regime: database <= ~5% of the x-classes), T in {4096, 16384}, instances_vsrho idx 1..11,
seeds 1 and 2 -> 22 paired configurations per cell.  Both arms' unpriced work is priced symmetrically in the analysis."""
import json,subprocess,math,os
IC="../../target/release/examples/ic_multi_target";BR="../../target/release/examples/ic_batch_rho"
rows=[]
for pb in (24,25):
    for idx in range(1,12):
        f=f"results/instances_vsrho/W{pb}-{idx}.json";d=json.load(open(f));p=int(d["field"]["modulus"])
        for T in (4096,16384):
            m=2**round(math.log2((p*T)**0.25))
            for seed in (1,2):
                o=json.loads(subprocess.run([IC,f,str(m),str(T),str(seed)],capture_output=True,text=True,timeout=900).stdout.strip().splitlines()[-1])
                o.update(kind="IC",pbits=pb,idx=idx,T=T,seed=seed,fb=m);rows.append(o)
                for kappa in (0.5,0.7,1.0):
                    b=json.loads(subprocess.run([BR,f,str(T),str(seed),str(kappa)],capture_output=True,text=True,timeout=900).stdout.strip().splitlines()[-1])
                    b.update(kind="RHO",pbits=pb,idx=idx,T=T,seed=seed,kappa=kappa);rows.append(b)
        print(pb,idx,flush=True)
os.makedirs("results/large_T_holdout",exist_ok=True)
json.dump(rows,open("results/large_T_holdout/rows.json","w"))
print(len(rows),"runs;",sum(r["ok"]==r["targets"] for r in rows),"fully verified")
