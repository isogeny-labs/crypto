#!/usr/bin/env python3
# SWB-RECOMPUTE-12, row 90 (20261014-greedy-factor-base-yield).
# Reads committed raw records only (data/rows.json, data/summary.json, scaling/*/summary.json).
import json
import os
import statistics

REPO = "/home/user/crypto"
STUDY = "research/greedy_factor_base_yield_20261014"
OUT = os.path.join(REPO, "research/validation_sweep_20261019/B-benchmarks/recompute/rows_swb12/90.json")
SCRIPT = "research/validation_sweep_20261019/B-benchmarks/scripts/recompute/90.py"


def load(rel):
    with open(os.path.join(REPO, STUDY, rel)) as fh:
        return json.load(fh)


def rel_ok(claimed, computed, decimals):
    if round(computed, decimals) == claimed:
        return True
    return abs(computed - claimed) <= 0.01 * abs(claimed)


def make_claim(claimed_str, pairs, definition, files):
    ok = all(rel_ok(c, v, d) for c, v, d in pairs)
    computed = ", ".join(f"{v:.{d}f}" for _, v, d in pairs)
    return {"claimed": claimed_str, "computed": computed, "definition": definition,
            "files": files, "status": "match" if ok else "mismatch"}


rows = load("data/rows.json")
summ = load("data/summary.json")
ROWS_F = STUDY + "/data/rows.json"
SUMM_F = STUDY + "/data/summary.json"

CURVES = [("K0/2^13", 13, 0), ("K0/2^15", 15, 0), ("K1/2^17", 17, 1)]
claims = []
greedy_all = {}
for label, n, a in CURVES:
    g = [x for x in rows if x["arm"] == "greedy" and x["n"] == n and x["a"] == a]
    r = [x for x in rows if x["arm"] == "random" and x["n"] == n and x["a"] == a]
    gu = {x["columns"]: x for x in g}
    best_u = min(gu, key=lambda u: gu[u]["expected_trials"])
    gE = gu[best_u]["expected_trials"]
    rE = [x["expected_trials"] for x in r if x["columns"] == best_u]
    rmean = statistics.mean(rE)
    greedy_all[label] = {"U*": best_u, "gE": gE, "rE_mean": rmean, "n_random_seeds": len(rE),
                         "greedy_rows": len(g), "random_rows": len(r)}
    print(label, "U*=", best_u, "greedy E=", gE, "random mean E=", rmean, "seeds=", len(rE),
          "ratio=", rmean / gE)

claims.append(make_claim(
    "expected trials at best U: 4.46 greedy (K0/2^13)",
    [(4.46, greedy_all["K0/2^13"]["gE"], 2)],
    "greedy-arm expected_trials at U* = argmin_U greedy expected_trials, n=13 a=0",
    [ROWS_F]))
claims.append(make_claim(
    "5.29 random mean (K0/2^13)",
    [(5.29, greedy_all["K0/2^13"]["rE_mean"], 2)],
    "mean over 20 random seeds of random-arm expected_trials at the same U*",
    [ROWS_F]))
claims.append(make_claim(
    "1.18x E ratio random/greedy (K0/2^13)",
    [(1.18, greedy_all["K0/2^13"]["rE_mean"] / greedy_all["K0/2^13"]["gE"], 2)],
    "random mean E / greedy E at U*",
    [ROWS_F]))
claims.append(make_claim(
    "3.00 vs 3.25 (1.08x) K0/2^15",
    [(3.00, greedy_all["K0/2^15"]["gE"], 2), (3.25, greedy_all["K0/2^15"]["rE_mean"], 2),
     (1.08, greedy_all["K0/2^15"]["rE_mean"] / greedy_all["K0/2^15"]["gE"], 2)],
    "greedy E at U*, random mean E at U*, ratio (n=15 a=0)",
    [ROWS_F]))
claims.append(make_claim(
    "16.35 vs 18.18 (1.11x) K1/2^17",
    [(16.35, greedy_all["K1/2^17"]["gE"], 2), (18.18, greedy_all["K1/2^17"]["rE_mean"], 2),
     (1.11, greedy_all["K1/2^17"]["rE_mean"] / greedy_all["K1/2^17"]["gE"], 2)],
    "greedy E at U*, random mean E at U*, ratio (n=17 a=1)",
    [ROWS_F]))

# coverage ratio: max over all summary entries (all curves, all U) with its bootstrap interval
best = max(summ, key=lambda s: s["coverage_ratio"])
claims.append(make_claim(
    "coverage ratio up to 1.181 [1.138, 1.230]",
    [(1.181, best["coverage_ratio"], 3), (1.138, best["coverage_ratio_ci95"][0], 3),
     (1.230, best["coverage_ratio_ci95"][1], 3)],
    "max coverage_ratio over data/summary.json entries (n=%d a=%d U=%d); ci95 from the same entry"
    % (best["n"], best["a"], best["columns"]),
    [SUMM_F]))

# construction time on K1/2^17 (greedy arm build_ms)
g17 = {x["columns"]: x for x in rows if x["arm"] == "greedy" and x["n"] == 17 and x["a"] == 1}
claims.append(make_claim(
    "full-scan construction K1/2^17 0.23 s at 4 columns",
    [(0.23, g17[4]["build_ms"] / 1000.0, 2)],
    "greedy-arm build_ms / 1000 at U=4, n=17 a=1",
    [ROWS_F]))
claims.append(make_claim(
    "1.5 s at 12 columns",
    [(1.5, g17[12]["build_ms"] / 1000.0, 2)],
    "greedy-arm build_ms / 1000 at U=12, n=17 a=1",
    [ROWS_F]))

# scaling: best coverage ratio per curve from scaling/*/summary.json
scal = {}
for name in ["main", "icv1-f2m37-tm534059-32aad96b-large"]:
    s = load("scaling/%s/summary.json" % name)
    for e in s:
        key = (name, e["n"], e["a"])
        cur = scal.get(key)
        if cur is None or e["ratio"] > cur["ratio"]:
            scal[key] = {"ratio": e["ratio"], "subgroup_bits": e["subgroup_bits"], "columns": e["columns"]}
for k, v in sorted(scal.items(), key=lambda kv: kv[1]["subgroup_bits"]):
    print("scaling best", k, v)


PINNED = [
    # (claimed string, scaling file, n, a, subgroup_bits target, claimed ratio, decimals)
    ("best ratio 1.090 at r=2^16", "main", 17, 1, 16.0, 1.090, 3),
    ("best ratio 1.046 at r=2^21", "main", 23, 0, 21.0, 1.046, 3),
    ("best ratio 1.012 at r=2^27.8", "icv1-f2m37-tm534059-32aad96b-large", 37, 0, 27.78, 1.012, 3),
    ("best ratio 1.0000 at r=2^39", "main", 41, 0, 39.0, 1.0000, 4),
]
for claimed_str, name, n, a, tbits, claimed, dec in PINNED:
    e = [x for x in load("scaling/%s/summary.json" % name) if x["n"] == n and x["a"] == a]
    v = max(e, key=lambda x: x["ratio"])
    claims.append(make_claim(
        claimed_str,
        [(claimed, v["ratio"], dec)],
        "max ratio over columns in scaling/%s/summary.json for n=%d a=%d (subgroup_bits %.3f, columns %d)"
        % (name, n, a, v["subgroup_bits"], v["columns"]),
        [STUDY + "/scaling/%s/summary.json" % name]))

def merge_claims(group, label):
    files = []
    for c in group:
        for f in c["files"]:
            if f not in files:
                files.append(f)
    statuses = [c["status"] for c in group]
    status = "mismatch" if "mismatch" in statuses else ("cannot_define" if "cannot_define" in statuses else "match")
    return {"claimed": label, "computed": " | ".join(c["claimed"] + " -> " + c["computed"] for c in group),
            "definition": " ; ".join(c["definition"] for c in group), "files": files, "status": status}


# group the 13 per-number checks into 6 claims (the card allows up to 10)
claims = [
    merge_claims(claims[0:3], "K0/2^13 greedy E at best U 4.46, random mean E 5.29, ratio 1.18"),
    merge_claims(claims[3:4], "K0/2^15 greedy E 3.00 vs random 3.25 (1.08x)"),
    merge_claims(claims[4:5], "K1/2^17 greedy E 16.35 vs random 18.18 (1.11x)"),
    merge_claims(claims[5:6], "coverage ratio up to 1.181 [1.138, 1.230]"),
    merge_claims(claims[6:8], "K1/2^17 full-scan construction 0.23 s at 4 columns, 1.5 s at 12"),
    merge_claims(claims[8:], "scaling best ratios 1.090 (r=2^16), 1.046 (2^21), 1.012 (2^27.8), 1.0000 (2^39)"),
]

# identical-output claim: greedy vs random, equal columns and equal points, matched on (n, a, U)
n_compared = n_equal = n_differ = 0
for label, n, a in CURVES:
    gu = {x["columns"]: x for x in rows if x["arm"] == "greedy" and x["n"] == n and x["a"] == a}
    for x in rows:
        if x["arm"] != "random" or x["n"] != n or x["a"] != a:
            continue
        g = gu.get(x["columns"])
        if g is None:
            continue
        n_compared += 1
        if g["columns"] == x["columns"] and g["points"] == x["points"]:
            n_equal += 1
        else:
            n_differ += 1
print("identical:", n_compared, n_equal, n_differ)
for c in claims:
    print(c["status"], "|", c["claimed"], "|", c["computed"])

row = {
    "row_no": 90,
    "slug": "20261014-greedy-factor-base-yield",
    "claims": claims,
    "identical": {
        "claimed": True,
        "n_compared": n_compared,
        "n_equal": n_equal,
        "n_differ": n_differ,
        "files": [ROWS_F],
        "status": "match" if n_differ == 0 and n_compared > 0 else "mismatch",
    },
    "offrepo_refs": [],
    "aa_lines": [
        "research/greedy_factor_base_yield_20261014/REPORT.typ:40: \"*random orbits*: U orbits drawn uniformly from the same orbit list, by partial Fisher\u2013Yates. Same granularity, same count, no coverage information. This is the control.\"",
        "research/greedy_factor_base_yield_20261014/REPORT.typ:46: \"== Data and controls\" (section heading; no A/A arm in the study)",
    ],
    "isolation_lines": [
        "research/greedy_factor_base_yield_20261014/REPORT.typ:140: caption \"...Host: 4 vCPU Intel Xeon 2.10 GHz, shared container, one run, no isolation.\"",
        "research/greedy_factor_base_yield_20261014/README.md:61: \"fields will differ between hosts and runs.\"",
    ],
    "protocol_git_date": "none: no PROTOCOL file in study (ls of research/greedy_factor_base_yield_20261014 shows README.md, REPORT.typ, data, e2e, graphs, scaling)",
    "first_raw_git_date": "2026-10-17T10:41:58-07:00 (commit 62c2e7d5b6d38c59d3693276d5c46ccecbf429c2, adds data/rows.json)",
    "script": SCRIPT,
    "notes": "identical check covers the greedy arm (one base per U) against each of the 20 random seeds at the same (n, a, U): 840 pairs. "
             "REPORT.typ:194 states 2240 greedy-or-pooled vs random pairs; that count is not recomputed here. "
             "Off-repo grep (drive.google, /Volumes, SSD, /Users, s3://, gs://, store:, modal, not committed, scratch) on README.md and REPORT.typ: no matches. "
             "Coverage ratio maximum is taken over all 56 entries of data/summary.json. "
             "Scaling claims: ratio at r=2^27.8 is taken from the icv1 run (main run gives a different max, see its summary). "
             "Dense-cap claim (r <= 2^22) is in code, not in the records; not recomputed.",
}
os.makedirs(os.path.dirname(OUT), exist_ok=True)
with open(OUT, "w") as fh:
    fh.write(json.dumps(row, ensure_ascii=False) + "\n")
print("wrote", OUT)
