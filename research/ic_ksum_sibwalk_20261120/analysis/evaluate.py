#!/usr/bin/env python3
"""Registered evaluation (PROTOCOL.md section 4). usage (cwd = study dir): python3 analysis/evaluate.py <ecbench> <sessions dir>
Writes results.json. Every ratio is `ecbench compare ... --weight w --resamples 4000 --seed 20261120` (ops ratio of totals,
paired by target and round); this script only reads the comparisons and applies the registered thresholds."""
import json, subprocess, sys
B, D = sys.argv[1], sys.argv[2]
ROWS = ["P20", "P22", "P24", "B15", "B18", "B21", "B24", "B27"]; WS = ["0", "0.1", "1"]
PREFIX_TEST_ROWS = ["P22", "P24", "B27"]
TESTS = {"T1": ("inc", "cand"), "T3": ("inc", "inc-prefix")}


def cmp(dirs, a, b, w):
    args = [B, "compare"] + sum([["--dir", d] for d in dirs], []) + ["--a", a, "--b", b, "--weight", w, "--resamples", "4000", "--seed", "20261120", "--json"]
    j = json.loads(subprocess.run(args, capture_output=True, text=True, check=True).stdout)
    if "ops" in j:  # one session: ecbench.comparison/v2
        o = j["ops"]; c = j["curves"][0]
        ratio, ci, boot, pairs, ok = o["ratio_b_over_a"], o["ci95"], c.get("ci95_bootstrap"), o["pairs"], o.get("status") == "ok"
    else:  # several sessions: ecbench.pooled_comparison/v1
        ratio, ci, boot, pairs, ok = j["ratio_b_over_a"], j["ci95"], j.get("ci95_bootstrap"), j["pairs"], bool(j.get("admissible"))
    return {"ratio": ratio, "ci95": ci, "ci95_bootstrap": boot, "pairs": pairs,
            "upper": max(ci[1], (boot or [0, 0])[1]), "lower": min(ci[0], (boot or [9, 9])[0]), "admissible": ok}


def test(dirs, a, b, aa_dev):
    L = {w: {"pooled": cmp(dirs, a, b, w), "per_replicate": [cmp([d], a, b, w) for d in dirs]} for w in WS}
    ups = {w: max([L[w]["pooled"]["upper"]] + [p["upper"] for p in L[w]["per_replicate"]]) for w in WS}
    L["max_upper_by_w"] = ups
    L["holds_by_w"] = {w: ups[w] < 1 and (1 - ups[w]) > aa_dev for w in WS}
    L["max_upper"] = max(ups.values())
    L["holds"] = all(L["holds_by_w"].values())
    L["significantly_worse"] = any(L[w]["pooled"]["lower"] > 1 for w in WS)
    return L


res = {"rows": {}, "tests": {}}
for row in ROWS:
    dirs = [f"{D}/{row}-r{i}" for i in (1, 2, 3)]
    R = {"aa": {w: cmp(dirs, "rho-neg", "rho-neg-aa", w) for w in WS},
         "off": {w: cmp(dirs, "inc", "off", w) for w in WS}}
    arms = ["inc", "cand"] + (["inc-prefix"] if row in PREFIX_TEST_ROWS else [])
    R["gap"] = {arm: {w: cmp(dirs, "rho-neg", arm, w) for w in WS} for arm in arms}
    aa_dev = max(abs(R["aa"][w]["ratio"] - 1) for w in WS)
    R["aa_max_dev"] = aa_dev
    R["T1"] = test(dirs, "inc", "cand", aa_dev)
    if row in PREFIX_TEST_ROWS:
        R["T3"] = test(dirs, "inc", "inc-prefix", aa_dev)
    res["rows"][row] = R
for t in TESTS:
    rows = [r for r in ROWS if t in res["rows"][r]]
    res["tests"][t] = {"rows_holding": [r for r in rows if res["rows"][r][t]["holds"]],
                       "rows_holding_by_w": {w: [r for r in rows if res["rows"][r][t]["holds_by_w"][w]] for w in WS},
                       "rows_significantly_worse": [r for r in rows if res["rows"][r][t]["significantly_worse"]]}
    res["tests"][t]["improves"] = bool(res["tests"][t]["rows_holding"]) and not res["tests"][t]["rows_significantly_worse"]
json.dump(res, open("results.json", "w"), indent=1)
for row in ROWS:
    v = res["rows"][row]
    for t in ("T1", "T3"):
        if t not in v:
            continue
        L = v[t]
        print(row, t, "holds" if L["holds"] else "fails", f"max_upper={L['max_upper']:.4f}",
              *(f"w{w}: {L[w]['pooled']['ratio']:.4f} [{L[w]['pooled']['ci95'][0]:.4f}, {L[w]['pooled']['ci95'][1]:.4f}] up {L['max_upper_by_w'][w]:.4f}" for w in WS))
    print(row, "off/inc", *(f"w{w}: {v['off'][w]['ratio']:.6f}" for w in WS), "aa", f"{v['aa_max_dev']:.6f}",
          "gap inc->cand", *(f"w{w}: {v['gap']['inc'][w]['ratio']:.3f}->{v['gap']['cand'][w]['ratio']:.3f}" for w in WS))
print(json.dumps(res["tests"]))
