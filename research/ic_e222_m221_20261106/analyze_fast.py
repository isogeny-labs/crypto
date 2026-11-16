#!/usr/bin/env python3
"""Reproduces the merged-baseline evaluation tables from results/merged/rows.jsonl (frozen settings in results/merged/frozen.json)."""
import json,random,statistics as S,collections,sys
d=sys.argv[1] if len(sys.argv)>1 else "results/merged"
R=[json.loads(l) for l in open(f"{d}/rows.jsonl")];fz=json.load(open(f"{d}/frozen.json"))
print("frozen",fz,"| rows",len(R),"recovered",sum(r["ok"] for r in R),"| missing timings",sum(r["total_ms"] is None for r in R))
print("binary",R[0]["binary_sha256"][:12],"commit",R[0]["commit"][:10],"| IC_ env seen:",{json.dumps(r["ic_env"],sort_keys=True) for r in R})
g=collections.defaultdict(dict)
for r in R: g[(r["name"],r["seed"])][r["arm"]]=r
print("pairs",len(g),"instances",len({k[0] for k in g}))
nm={"A":"shipped default (sweep, extra 64)","Eh":"hoisted BigUint sweep, extra 2","Ef":"word-sized sweep, extra 2","D":"pair-sum table, extra 2","Dw":"table + restarted walk, extra 2"}
for a in ("A","Eh","Ef","D","Dw"):
    print(a,nm[a].ljust(34),"fb",fz[a],"| solver ms median %.2f | process ms median %.1f | trials %d"%(S.median(x[a]["total_ms"] for x in g.values()),S.median(x[a]["wall_ms"] for x in g.values()),S.median(x[a]["trials"] for x in g.values())))
def boot(rat,inst_of,B=2000,seed=19):
    rng=random.Random(seed);by=collections.defaultdict(list)
    for r,i in zip(rat,inst_of): by[i].append(r)
    ks=list(by);meds=[]
    for _ in range(B):
        pick=[x for k in rng.choices(ks,k=len(ks)) for x in by[k]];meds.append(S.median(pick))
    meds.sort();return meds[int(0.025*B)],meds[int(0.975*B)]
keys=list(g)
print("paired ratios, median [bootstrap 95% by instance]  solver | process | wins(solver)")
for n,dn in (("Ef","Eh"),("D","Ef"),("Dw","Ef"),("Dw","D"),("Ef","A"),("D","A"),("Dw","A")):
    out=[]
    for key in ("total_ms","wall_ms"):
        rat=[g[k][n][key]/g[k][dn][key] for k in keys];lo,hi=boot(rat,[k[0] for k in keys]);out.append("%.3f [%.3f,%.3f]"%(S.median(rat),lo,hi))
    w=sum(g[k][n]["total_ms"]<g[k][dn]["total_ms"] for k in keys)
    print("  %s/%s: %s | %s | %d/%d"%(n,dn,out[0],out[1],w,len(keys)))
print("by size (Dw/Ef, D/Ef solver):",{rb:(round(S.median(g[k]["Dw"]["total_ms"]/g[k]["Ef"]["total_ms"] for k in keys if g[k]["Eh"]["r_bits"]==rb),3),round(S.median(g[k]["D"]["total_ms"]/g[k]["Ef"]["total_ms"] for k in keys if g[k]["Eh"]["r_bits"]==rb),3)) for rb in sorted({g[k]["Eh"]["r_bits"] for k in keys})})
