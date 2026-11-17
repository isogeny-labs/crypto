import json, glob, math, statistics, collections, os
ROOT = "/home/user/crypto/research/ic_frontier_campaign_20261015"
recs = []
for p in sorted(glob.glob(os.path.join(ROOT, "phase1/sessions/*/records.jsonl"))):
    for l in open(p):
        if l.strip(): recs.append(json.loads(l))
print("records", len(recs))
print("unique curve slugs", len(set(r["workload"]["curve"]["slug"] for r in recs)))
# A. S = total_gae / sqrt(r), check against cost.s
dev = []
for r in recs:
    c = r["cost"]; rr = r["workload"]["curve"]["r"]
    if c.get("s") is not None:
        dev.append(abs(c["total_gae"] / math.sqrt(rr) - c["s"]) / max(c["s"], 1e-12))
print("S=total_gae/sqrt(r) vs cost.s max rel dev", max(dev), "n", len(dev))
# B. band rows: group non-warmup verified by (curve slug, arm)
grp = collections.defaultdict(list)
for r in recs:
    if r["warmup"] or r["outcome"]["status"] != "verified": continue
    grp[(r["workload"]["curve"]["slug"], r["arm"])].append(r)
def priced(r, w):
    c = r["cost"]; rr = r["workload"]["curve"]["r"]
    unp = sum(r["counters"].get(k, 0) for k in c.get("unpriced", []))
    return (c["total_gae"] + w * unp) / math.sqrt(rr)
for band in ["analysis/band-koblitz.json", "analysis/band-prime.json"]:
    b = json.load(open(os.path.join(ROOT, band)))
    maxdev = {0.0: 0, 0.1: 0, 1.0: 0}; n = 0; missing = 0; none_band = {}
    for row in b["rows"]:
        recs_k = grp.get((row["curve"], row["arm"]), [])
        if len(recs_k) != row["runs"]:
            missing += 1
        for i, w in enumerate(b["weights"]):
            vals = [priced(r, w) for r in recs_k]
            if not vals: continue
            if row["mean_s"][i] is None:
                none_band[w] = none_band.get(w, 0) + 1; continue
            m = statistics.fmean(vals)
            dev = abs(m - row["mean_s"][i]) / row["mean_s"][i]
            maxdev[w] = max(maxdev[w], dev)
        n += 1
    print(band, "band mean_s None count by weight", none_band, "rows", n, "rows whose run count differs from band", missing, "max rel dev per weight", {k: round(v, 6) for k, v in maxdev.items()})
# C. ratios to rho at w=0, 0.1, 1 using rho-frob (Koblitz) and rho-neg (prime)
def ratios(band):
    b = json.load(open(os.path.join(ROOT, band)))
    out = {}
    byc = collections.defaultdict(dict)
    for row in b["rows"]:
        byc[row["curve"]][row["arm"]] = row
    return byc
out = []
for slug, arms in []: pass
for cls_name, denom in [("koblitz", "rho-frob"), ("prime", "rho-neg")]:
    cur = set(k[0] for k in grp if (k[1] == denom))
    for slug in sorted(cur):
        den = [priced(r, 0) for r in grp.get((slug, denom), [])]
        if not den: continue
        dm = statistics.fmean(den)
        for arm in ["ic-compact", "ic-pair", "ic-negfold", "ic-f4", "ic-pkm", "ic-shared"]:
            ks = grp.get((slug, arm), [])
            if not ks: continue
            row_r = []
            for w in [0.0, 0.1, 1.0]:
                row_r.append(statistics.fmean([priced(r, w) for r in ks]) / statistics.fmean([priced(r, w) for r in grp[(slug, denom)]]))
            rr = ks[0]["workload"]["curve"]["r"]
            out.append((cls_name, slug, round(math.log2(rr), 3), arm, len(ks), [round(x, 4) for x in row_r]))
for o in out: print(o)
# D. replay-all audits
aud = [json.load(open(p)) for p in sorted(glob.glob(os.path.join(ROOT, "phase1/audits/*.json")))]
print("audits", len(aud), "ok True", sum(1 for a in aud if a.get("ok")), "records sum", sum(a["records"] for a in aud), "verified_records sum", sum(a["verified_records"] for a in aud))
rep = [x for a in aud for x in a.get("replays", [])]
print("replays", len(rep), "reproduced True", sum(1 for x in rep if x.get("reproduced")), "detail identical", sum(1 for x in rep if x.get("detail") == "identical"), "regraded True", sum(1 for a in aud if a.get("regraded")))
print("replay-all? replays per audit", sorted(set(len(a.get("replays", [])) for a in aud)), "records per audit", sorted(set(a["records"] for a in aud))[:5])
