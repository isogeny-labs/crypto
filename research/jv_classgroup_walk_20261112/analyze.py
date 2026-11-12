#!/usr/bin/env python3
"""Coverage of the JV weak class by the class-group walk (CG) vs the rational-root walk (RR).
Reads runs/CG_p*.jsonl (and runs/RR_p*.jsonl for the paired large-prime arm). Writes analysis.json."""
import json, math, glob, statistics, os
def wilson(k, n, z=1.96):
    if n == 0: return [0.0, 1.0]
    ph=k/n; d=1+z*z/n; c=(ph+z*z/(2*n))/d; h=z*math.sqrt(ph*(1-ph)/n+z*z/(4*n*n))/d
    return [max(0,c-h), min(1,c+h)]
# published rational-root W (jv_weak_coverage_20261111, cap 10000, 60 walks)
RR_PUB={"37":[24,60],"53":[25,60],"1009":[3,60],"1511":[0,60],"1823":[0,60]}
def load(tag,p):
    try: return [json.loads(l) for l in open(f"runs/{tag}_p{p}.jsonl") if l.strip()]
    except FileNotFoundError: return None
out={"small_cap":10000,"small_walklimit":60,"big_cap":3000,"big_walklimit":30,"primes":{}}
for p in ("37","53","1009","1511","1823"):
    row={}
    cg=load("CG",p)
    if cg:
        w=[r for r in cg if r["walked"]]; fnd=[r for r in w if r["found"]]
        odd=[r["path_odd"] for r in fnd]
        row["cg"]={"walked":len(w),"found":len(fnd),"frac":len(fnd)/max(1,len(w)),
                   "ci":wilson(len(fnd),len(w)),"odd_used":sum(1 for o in odd if o>0),
                   "total_odd_steps":sum(odd),
                   "median_muls_found":statistics.median([r["muls_walk"] for r in fnd]) if fnd else None,
                   "admitted":sum(1 for r in cg if r["admitted"]),"n":len(cg)}
    rr=load("RR",p)
    if rr:
        w=[r for r in rr if r["walked"]]; fnd=[r for r in w if r["found"]]
        row["rr_paired"]={"walked":len(w),"found":len(fnd),"frac":len(fnd)/max(1,len(w)),"ci":wilson(len(fnd),len(w))}
    if p in RR_PUB:
        k,n=RR_PUB[p]; row["rr_published"]={"found":k,"walked":n,"frac":k/n,"ci":wilson(k,n)}
    out["primes"][p]=row
json.dump(out,open("analysis.json","w"),indent=1)
for p,r in out["primes"].items():
    cg=r.get("cg"); rrp=r.get("rr_published"); rrpair=r.get("rr_paired")
    s=f"p={p}: "
    if cg: s+=f"CG {cg['found']}/{cg['walked']}={cg['frac']:.2f} (odd-used {cg['odd_used']}/{cg['found']}) "
    if rrpair: s+=f"| RR-paired {rrpair['found']}/{rrpair['walked']}={rrpair['frac']:.2f} "
    if rrp: s+=f"| RR-pub {rrp['found']}/{rrp['walked']}={rrp['frac']:.2f}"
    print(s)
