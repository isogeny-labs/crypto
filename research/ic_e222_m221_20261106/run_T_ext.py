#!/usr/bin/env python3
"""Does the IC / batch-rho ratio reach 1 at larger T?  Same rules as before, unchanged: IC fb = 2^round(log2((pT)^(1/4))),
batch rho kappa in {0.5, 1, 2} (best reported).  T in {4096, 16384}; instances_vsrho idx 1..4; seed 1.
VALIDITY RESTRICTION (found after a first attempt hung at 20 bits, T = 16384, kappa = 2): the reference's database must stay
a small fraction of the x-classes (here <= ~5%), because its restarted walks reach only a limited set of distinct points;
that rules out 20/22-bit groups at these T, so only 24 and 25-bit groups are run.  A kappa that exceeds the reachable
database times out and is recorded as such (rho then uses the best kappa that completed)."""
import json,subprocess,math,os
IC="../../target/release/examples/ic_multi_target";BR="../../target/release/examples/ic_batch_rho"
rows=[]
for pb in (24,25):
    for idx in range(1,5):
        f=f"results/instances_vsrho/W{pb}-{idx}.json";d=json.load(open(f));p=int(d["field"]["modulus"])
        for T in (4096,16384):
            m=2**round(math.log2((p*T)**0.25))
            o=json.loads(subprocess.run([IC,f,str(m),str(T),"1"],capture_output=True,text=True,timeout=900).stdout.strip().splitlines()[-1])
            o.update(kind="IC",pbits=pb,idx=idx,T=T,fb=m);rows.append(o)
            for kappa in (0.5,1,2):
                try:
                    b=json.loads(subprocess.run([BR,f,str(T),"1",str(kappa)],capture_output=True,text=True,timeout=120).stdout.strip().splitlines()[-1])
                    b.update(kind="RHO",pbits=pb,idx=idx,T=T,kappa=kappa);rows.append(b)
                except subprocess.TimeoutExpired:
                    rows.append(dict(kind="RHO",pbits=pb,idx=idx,T=T,kappa=kappa,timeout=True,ok=0,targets=T))
            print(pb,idx,T,flush=True)
os.makedirs("results/T_ext",exist_ok=True)
json.dump(rows,open("results/T_ext/rows.json","w"))
print(len(rows),"runs;",sum(r["ok"]==r["targets"] for r in rows),"fully verified;",sum(bool(r.get("timeout")) for r in rows),"timeouts")
