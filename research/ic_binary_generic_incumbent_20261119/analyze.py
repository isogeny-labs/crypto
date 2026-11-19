#!/usr/bin/env python3
"""Paired analysis: pruned k-sum incumbent vs free=1 candidates, three random-binary curves. Inputs: raw/B<n>-r<rep>.jsonl (example),
band_B<n>-r<rep>.json (ecbench band of the matched rho sessions). Output: results.json, figure.svg. Everything [pcc:unverified]."""
import json, math, random, statistics as st, collections, sys
W = (0.0, 0.1, 1.0)
CURVES = {21: "icv1-f2m21-t2591-de54d72e", 24: "icv1-f2m24-tm3047-2d0c0e8c", 27: "icv1-f2m27-t12543-569dca8b"}
N0 = {21: {4: 10, 5: 9}, 24: {4: 12, 5: 9}, 27: {4: 16, 5: 12}}
COMMITTED = {  # research/ic_ksum_prune_20261115/band/B*.json, S(w) of ksum4-p / ksum5-p
    21: {"k4-p-p0": (6.554, 7.397, 14.984), "k5-p-p0": (6.337, 7.063, 13.596)},
    24: {"k4-p-p0": None, "k5-p-p0": None}, 27: {"k4-p-p0": None, "k5-p-p0": None}}
PRICED = ("lookups", "row_ops", "target_guard_probes", "table_insert_attempts")  # unpriced natives of these arms + counted table work
rng = random.Random(20261119)

def load(b):
    runs = collections.defaultdict(dict)   # arm -> {(rep,t,round): (S0,U/sqrt r, extra)}
    for rep in (1, 2, 3):
        for l in open(f"raw/B{b}-r{rep}.jsonl"):
            d = json.loads(l)
            if not d["ok"]:
                runs["__fail__"][(rep, d["arm"], d["target"], d["round"])] = d["error"]; continue
            sq = math.sqrt(d["r"])
            U = sum(v["native"].get(k, 0) + v["work_native"].get(k, 0) for v in d["phases"].values() for k in PRICED)
            rel = d["phases"]["relations"]["native"]
            runs[d["arm"]][(rep, d["target"], d["round"])] = (d["total_gae"] / sq, U / sq, rel.get("free_rows_emitted", 0),
                rel.get("free_collisions_checked", 0), d["targets_tried"], d["verified"], d["matrix_rank"], d["matrix_rows"],
                d["phases"]["oracle_setup"]["gae"] / sq)
    return runs

def sw(v, w): return v[0] + w * v[1]
def mean_s(arm, w, keys=None): return st.mean(sw(v, w) for k, v in arm.items() if keys is None or k[0] in keys)

def ratio_ci(a, c, w, reps=(1, 2, 3), n=4000):
    """cost(c)/cost(a), paired by run, cluster bootstrap over (rep, target) clusters."""
    cl = collections.defaultdict(list)
    for k in a:
        if k[0] in reps and k in c: cl[(k[0], k[1])].append((sw(a[k], w), sw(c[k], w)))
    cls = list(cl.values())
    sa = [sum(x for x, _ in g) for g in cls]; sc = [sum(y for _, y in g) for g in cls]
    pt = sum(sc) / sum(sa)
    bs = []
    for _ in range(n):
        idx = [rng.randrange(len(cls)) for _ in cls]
        bs.append(sum(sc[i] for i in idx) / sum(sa[i] for i in idx))
    bs.sort()
    return pt, bs[int(0.025 * n)], bs[int(0.975 * n)]

def rho_s(b):
    ss = []
    for rep in (1, 2, 3):
        d = json.load(open(f"band_B{b}-r{rep}.json"))
        ss.append(next(r for r in d["rows"] if r["arm"] == "rho-neg")["mean_s"])
    return [st.mean(s[i] for s in ss) for i in range(3)]

res = {"schema": "ic_binary_generic_incumbent_20261119/results/v1", "note": "[pcc:unverified]; counted cost (L0); unit GAE per sqrt(r); example-priced, see PROTOCOL section 6", "curves": {}}
for b in CURVES:
    runs = load(b)
    fails = runs.pop("__fail__", {})
    arms = {a: v for a, v in runs.items()}
    out = {"slug": CURVES[b], "failures": len(fails), "runs_per_arm": {a: len(v) for a, v in arms.items()},
           "all_verified": all(x[5] for v in arms.values() for x in v.values()),
           "unverified_runs": sum(1 for v in arms.values() for x in v.values() if not x[5]),
           "rho_neg_S": rho_s(b), "arms": {}}
    for a, v in arms.items():
        out["arms"][a] = {"S": [mean_s(v, w) for w in W], "free_rows_emitted_mean": st.mean(x[2] for x in v.values()),
                          "free_collisions_checked_mean": st.mean(x[3] for x in v.values()),
                          "targets_tried_mean": st.mean(x[4] for x in v.values()), "setup_S0_mean": st.mean(x[8] for x in v.values()),
                          "matrix_rows_mean": st.mean(x[7] for x in v.values()), "N": None}
    inc = [a for a in arms if "-pf-" not in a and a != "k5-p-p0-aa"]
    cand = [a for a in arms if "-pf-" in a]
    # A/A
    out["aa_k5-p-p0_vs_dup"] = [ratio_ci(arms["k5-p-p0"], arms["k5-p-p0-aa"], w)[0] for w in W]
    # T1 fixed config
    out["T1"] = {}
    for m in (4, 5):
        for d in ("m1", "p0", "p1", "p2"):
            a, c = f"k{m}-p-{d}", f"k{m}-pf-{d}"
            out["T1"][f"k{m}-{d}"] = {f"w{w}": ratio_ci(arms[a], arms[c], w) for w in W}
    # T2 strictest vs strictest, chosen on pooled data per w; also per replicate
    out["T2"] = {}
    for w in W:
        ia = min(inc, key=lambda a: mean_s(arms[a], w)); ca = min(cand, key=lambda a: mean_s(arms[a], w))
        pooled = ratio_ci(arms[ia], arms[ca], w)
        per = [ratio_ci(arms[ia], arms[ca], w, reps=(r,)) for r in (1, 2, 3)]
        rho = out["rho_neg_S"][W.index(w)]
        out["T2"][f"w{w}"] = {"incumbent": ia, "candidate": ca, "R_pooled": pooled, "R_per_replicate": per,
                              "S_incumbent": mean_s(arms[ia], w), "S_candidate": mean_s(arms[ca], w), "S_rho": rho,
                              "dist_incumbent": mean_s(arms[ia], w) / rho, "dist_candidate": mean_s(arms[ca], w) / rho}
    res["curves"][str(b)] = out
json.dump(res, open("results.json", "w"), indent=1)

# compact print
for b, o in res["curves"].items():
    print(f"== B{b}  failures={o['failures']} unverified={o['unverified_runs']} rho S={['%.3f'%x for x in o['rho_neg_S']]} A/A={['%.4f'%x for x in o['aa_k5-p-p0_vs_dup']]}")
    for a in sorted(o["arms"]):
        x = o["arms"][a]; print(f"  {a:12s} S={['%.3f'%s for s in x['S']]} free={x['free_rows_emitted_mean']:.2f} coll={x['free_collisions_checked_mean']:.2f} tried={x['targets_tried_mean']:.0f} setupS0={x['setup_S0_mean']:.2f}")
    for k, t in o["T1"].items(): print("  T1", k, ["%.3f[%.3f,%.3f]" % tuple(t[f'w{w}']) for w in W])
    for k, t in o["T2"].items(): print("  T2", k, t["incumbent"], t["candidate"], "R=%.3f[%.3f,%.3f]" % tuple(t["R_pooled"]), "per-rep", ["%.3f" % p[0] for p in t["R_per_replicate"]], "dist inc/cand %.2f/%.2f" % (t["dist_incumbent"], t["dist_candidate"]))
