import json,gzip,sys,collections
S="/tmp/claude-0/-home-user/cba6917f-779a-544b-9ecf-84e512b45b33/scratchpad/lsc-sessions"
CAL=["lookups","row_ops","frobenius_maps","canonicalisations","target_guard_probes","as_solves"]
NONOP={"factor_base_vec_bytes_floor_uncharged","residual_table_bytes_uncharged","native_table_bytes_uncharged"}
arms=sys.argv[1].split(",")
for n in (23,29,37):
    agg=collections.defaultdict(lambda: collections.defaultdict(float)); cnt=collections.Counter()
    for r in (1,2,3):
        for l in gzip.open(f"{S}/n{n}-r{r}/records.jsonl.gz","rt"):
            x=json.loads(l)
            if x["warmup"] or x["arm"] not in arms: continue
            a=x["arm"]; cnt[a]+=1
            unp=[u for u in x["cost"]["unpriced"] if u not in NONOP]
            marks={k[8:] for p in x["phases"] for k in (p.get("work_native") or {}) if k.startswith("counted:")}
            tot={0:0,0.1:0,1:0}
            # top-level unpriced counters
            topU=sum(x["counters"].get(u,0) for u in unp if u in x["counters"])
            for p in x["phases"]:
                nat=p.get("native") or {}; wn=p.get("work_native") or {}
                nC=sum(nat.get(c,0) for c in CAL)
                U=0
                for u in unp:
                    if u in x["counters"]: continue
                    b=u[:-10] if u.endswith("_uncharged") else u
                    U+=nat.get(u,nat.get(b,0))
                T=sum(wn.get(t,0) for t in ("table_insert_attempts","sibling_second_points","sibling_lift_products") if t in marks)
                # C: calibrated gae, approximate as gae - adds - doubles*kd ... compute via residual: gae - group ops
                g=p["gae"] or 0
                go=(p.get("adds") or 0)+(p.get("doubles") or 0)
                for w in (0,0.1,1):
                    v=go+w*(nC+U+T) if nC else g+w*(U+T)
                    # if group-op priced not 1 (doubles ~1.03) keep gae minus calibrated: approximate calibrated = g-go when nC>0
                    agg[a][(p["name"],w)]+=v; tot[w]+=v
            for w in (0,0.1,1):
                agg[a][("TOP",w)]+=topU*w; agg[a][("total",w)]+=tot[w]+topU*w
    for a in arms:
        if not cnt[a]: continue
        keys=sorted({k[0] for k in agg[a]})
        print(f"n{n} {a:12s} "+"  ".join(f"{k}:"+"/".join(f"{agg[a][(k,w)]/cnt[a]:.1f}" for w in (0,0.1,1)) for k in keys))
