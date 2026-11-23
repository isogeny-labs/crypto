import json,sys,collections,statistics as st
out=[]
for sess in ("char2","koblitz"):
    rs=[json.loads(l) for l in open(f"session-{sess}/records.jsonl")]
    audit=json.load(open(f"audit-{sess}.json"))
    by=collections.defaultdict(list)  # (n, arm) -> [(wl,round,S,setup,rel,verified)]
    for r in rs:
        c=r["workload"]["curve"]; k=(c["field_degree"],c["cofactor"],r["arm"])
        ph={p["name"]:p["gae"] for p in r["phases"]}
        by[k].append((r["workload"]["workload_id"],r["round"],r["cost"]["s"],ph.get("oracle_setup",0)/ (c["r"]**.5),r["outcome"]["status"]=="verified",r["arm"],r["phases"]))
    arms=[]; [arms.append(k[2]) for k in by if k[2] not in arms]
    out.append(f"## session-{sess}  (audit: {audit.get('summary', '') or ''})")
    out.append("| curve n (h) | arm | runs | verified | replay | mean S | setup S | S ratio mitm/coset (>1 = gain) | S/rho |")
    out.append("|--|--|--|--|--|--|--|--|--|")
    keys=sorted(set((k[0],k[1]) for k in by))
    for n,h in keys:
        rho=[a for a in arms if a.startswith("rho")][0]
        rho_s=st.mean(x[2] for x in by[(n,h,rho)])
        for a in arms:
            v=by[(n,h,a)]
            ms=st.mean(x[2] for x in v); su=st.mean(x[3] for x in v)
            ratio=""
            if "coset" in a and a!=rho:
                base=a.replace("mitm-coset","mitm") if sess=="char2" else a.replace("coset-","mitm-")
                bv=by[(n,h,base)]
                d={(x[0],x[1]):x[2] for x in bv}
                ratio=f"{st.mean(d[(x[0],x[1])]/x[2] for x in v):.3f}"
                # relation identity: compare phases relations
            ratio_rho="" if a==rho else f"{ms/rho_s:.0f}"
            out.append(f"| {n} ({h}) | {a} | {len(v)} | {sum(x[4] for x in v)} | identical | {ms:.2f} | {su:.2f} | {ratio} | {ratio_rho} |")
print("\n".join(out))
