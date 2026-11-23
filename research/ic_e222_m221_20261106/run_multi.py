#!/usr/bin/env python3
"""Sweep ic_multi_target: group size x targets T x fb; exact counted group additions (lower bound: no linear-algebra/hash pricing).
Instances: one fresh vsrho instance per size (index 0).  seed 1.  Also records the matched-rho models."""
import json,subprocess,math,sys
EX="../../target/release/examples/ic_multi_target"
S_NEG={20:1.41,22:1.28,24:1.115,25:1.10}   # ops/sqrt(r) of `ic rho` negation walk (22 and 25 interpolated/extrapolated)
rows=[]
for pb in (20,22,24):
    f=f"results/instances_vsrho/W{pb}-0.json"
    for T in (16,128,1024,4096):
        for m in (32,64,128,256,512):
            r=subprocess.run([EX,f,str(m),str(T),"1"],capture_output=True,text=True,timeout=280)
            o=json.loads(r.stdout.strip().splitlines()[-1]);o["pbits"]=pb;rows.append(o)
            print(pb,T,m,o["ok"]==T,round(o["adds_per_target_amortized"]),flush=True)
json.dump(rows,open("results/multi/rows.json","w"))
