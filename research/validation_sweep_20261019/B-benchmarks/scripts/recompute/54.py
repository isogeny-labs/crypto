import json, glob, math, os, collections, statistics
RUN = "/home/user/crypto/research/ic_improve3_20261015/01-genus3-relations/results/run"
def load(bits):
    out = collections.defaultdict(dict)   # (curve) -> arm label -> result
    for p in sorted(glob.glob(os.path.join(RUN, "b%d_c*_*.jsonl" % bits))):
        for l in open(p):
            if not l.strip(): continue
            r = json.loads(l)
            if r.get("bits") != bits: continue
            key = (r["curve"], r["c"], os.path.basename(p).split("_")[-1])
            for x in r["results"]:
                out[r["curve"]][(x["arm"], os.path.basename(p))] = x
    return out
def arms_of(d):
    labs = set(a for c in d for (a, f) in d[c])
    return sorted(labs)
d = load(22)
print("bits 22 curves", sorted(d.keys()), "arm labels", arms_of(d))
def get(d, curve, label, field):
    for (a, f), x in d[curve].items():
        if a == label:
            return x[field], x.get("correct"), f
    return None, None, None
def ratio(d, cand, base, field):
    per = []
    for c in sorted(d):
        cv, ok1, f1 = get(d, c, cand, field)
        bv, ok2, f2 = get(d, c, base, field)
        if cv is None or bv is None: continue
        per.append((c, cv / bv, ok1, ok2))
    gm = math.exp(statistics.fmean(math.log(x[1]) for x in per)) if per else None
    return gm, per
INC = "sched:lpsgebw2@8"; R1 = "sched:lpsgebw2@8+R1"; R2 = "r2:lpsgebw2@8+R1"; R2K = "r2:lpsgebw2@8"; AA = None
for name, cand in [("R1 whole", R1), ("R1 relation", R1), ("R2 whole", R2), ("R2 kappa-only whole", R2K)]:
    field = "rel_cost" if "relation" in name else "cost"
    gm, per = ratio(d, cand, INC, field)
    print(name, "field", field, "geomean over curves", round(gm, 4) if gm else None, "per curve", [(c, round(v, 4), ok1, ok2) for c, v, ok1, ok2 in per])
print("correct flags all true", all(x.get("correct") for c in d for (a, f), x in d[c].items() if a in (INC, R1, R2, R2K)))
