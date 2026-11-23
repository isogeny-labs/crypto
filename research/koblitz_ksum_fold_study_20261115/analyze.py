#!/usr/bin/env python3
"""Registered analysis (PROTOCOL.md section 7). Reads sessions/, writes results.json, summary.json, tables/.
Intervals are the cluster-bootstrap 95% (ci95_bootstrap, 4000 resamples, seed 20261002)."""
import json, os, subprocess, sys
HERE = os.path.dirname(os.path.abspath(__file__)); ROOT = subprocess.check_output(["git", "-C", HERE, "rev-parse", "--show-toplevel"], text=True).strip()
EB = os.path.join(ROOT, "target/release/ecbench"); SD = os.path.join(HERE, "sessions"); W = [0.0, 0.1, 1.0]
CURVES = {23: dict(slug="icv1-f2m23-tm5197-1f85e9e1", lazy=("lazy-c3", "lazy-c4"), k1="k1-l9", full="lazy-c4-full"),
          29: dict(slug="icv1-f2m29-tm40309-30c52b96", lazy=("lazy-c1", "lazy-c2"), k1="k1-l10", full="lazy-c1-full")}
CAND, CANDW, CANDF = "kf3-c1g-sr", "kf3-c1g-w", "kf3-c1g-sr-full"
cache = {}
def cmp(dirs, a, b, w=None):
    key = (tuple(dirs), a, b, w)
    if key in cache: return cache[key]
    cmd = [EB, "compare"] + sum([["--dir", os.path.join(SD, d)] for d in dirs], []) + ["--a", a, "--b", b, "--resamples", "4000", "--json"]
    if w is not None: cmd += ["--weight", str(w)]
    d = json.loads(subprocess.check_output(cmd, text=True)); c = d["curves"][0] if "curves" in d else d
    ver = bool(d.get("admissible", True))
    r = dict(ratio=c["ratio_b_over_a"], lo=c["ci95_bootstrap"][0], hi=c["ci95_bootstrap"][1], pairs=c["pairs"], admissible=ver,
             ci_bootstrap_method=d.get("ci_bootstrap_method") or c.get("ci_bootstrap_method", "cluster"))
    cache[key] = r; return r
res = {"schema": "crypto.koblitz_ksum_fold_results/v1", "tag": "[pcc:unverified]", "interval": "cluster bootstrap 95%, 4000 resamples, seed 20261002", "curves": {}}
for n, C in CURVES.items():
    reps = [f"n{n}-r{i}" for i in (1, 2, 3)]; out = {"slug": C["slug"], "weights": {}}
    for w in W:
        o = {}
        # validity: A/A
        o["aa_pooled"] = cmp(reps, "rho-strong", "rho-strong-aa", w)
        o["aa_per_rep"] = [cmp([r], "rho-strong", "rho-strong-aa", w) for r in reps]
        # strictest incumbent at this w (smaller pooled cost)
        l1, l2 = C["lazy"]; cl = cmp(reps, l1, l2, w)  # ratio = cost(l2)/cost(l1)
        strict = l1 if cl["ratio"] > 1 else l2; o["strictest_incumbent"] = strict; o["incumbent_pair_ratio_%s_over_%s" % (l2, l1)] = cl
        o["h1_pooled"] = cmp(reps, strict, CAND, w); o["h1_per_rep"] = [cmp([r], strict, CAND, w) for r in reps]
        o["h1_vs_each_incumbent"] = {l: cmp(reps, l, CAND, w) for l in C["lazy"]}
        aa_lo = o["aa_pooled"]["lo"]; p = o["h1_pooled"]
        a_ = p["hi"] < 1; b_ = all(x["hi"] < 1 for x in o["h1_per_rep"]); c_ = p["ratio"] < aa_lo
        o["h1_holds"] = bool(a_ and b_ and c_); o["h1_parts"] = dict(pooled_upper_lt_1=a_, every_replicate_upper_lt_1=b_, point_below_AA_lower=c_)
        o["h1_regression"] = bool(p["lo"] > 1)
        o["aa_contains_1"] = bool(o["aa_pooled"]["lo"] <= 1 <= o["aa_pooled"]["hi"])
        o["h1_walk_candidate_vs_strictest"] = cmp(reps, strict, CANDW, w)
        o["h2_vs_k1"] = cmp(reps, C["k1"], CAND, w)
        o["h3_gap_to_rho_strong"] = {a: cmp(reps, "rho-strong", a, w) for a in (CAND, CANDW, l1, l2, C["k1"], "rho-neg")}
        out["weights"][str(w)] = o
    # secondary pricing: recorded S of *_full arms
    sec = {}
    if n == 23:
        sec["note"] = "not measured: src/cryptanalysis/ecbench/full_prices.json has no row for icv1-f2m23-tm5197-1f85e9e1 (a = 1), every *_full record errored"
    else:
        sec["full_cand_vs_lazy_full"] = {"recorded_S": cmp(reps, C["full"], CANDF), "per_rep": [cmp([r], C["full"], CANDF) for r in reps]}
        sec["full_cand_vs_rho_strong_full"] = cmp(reps, "rho-strong-full", CANDF); sec["full_lazy_vs_rho_strong_full"] = cmp(reps, "rho-strong-full", C["full"])
    out["secondary_full"] = sec
    # mean S per arm per w, from band
    bj = os.path.join(HERE, "tables", f"band-n{n}.json"); os.makedirs(os.path.dirname(bj), exist_ok=True)
    txt = subprocess.check_output([EB, "band", "--ranks", "--weights", "0,0.1,1", "--json", bj] + sum([["--dir", os.path.join(SD, r)] for r in reps], []), text=True)
    open(os.path.join(HERE, "tables", f"band-n{n}.txt"), "w").write(txt)
    b = json.load(open(bj)); out["mean_S"] = {r["arm"]: dict(zip(["0", "0.1", "1"], r["mean_s"])) | {"runs": r["runs"], "verified": r["verified"]} for r in b["rows"]}
    out["band_ranks"] = [r for r in b["ranks"] if "full" not in r["a"] and "full" not in r["b"]]
    res["curves"][str(n)] = out
json.dump(res, open(os.path.join(HERE, "results.json"), "w"), indent=1)
rows = {}
for n in CURVES:
    for w in W:
        o = res["curves"][str(n)]["weights"][str(w)]; p = o["h1_pooled"]
        rows[f"n{n}_w{w}"] = dict(h1_holds=o["h1_holds"], regression=o["h1_regression"], R=p["ratio"], ci=[p["lo"], p["hi"]], strictest=o["strictest_incumbent"], aa_contains_1=o["aa_contains_1"])
json.dump({"schema": "crypto.koblitz_ksum_fold_summary/v1", "tag": "[pcc:unverified]", "candidate": CAND, "rows": rows}, open(os.path.join(HERE, "summary.json"), "w"), indent=1)
print(json.dumps(rows, indent=1))
