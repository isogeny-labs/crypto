#!/usr/bin/env python3
"""Reads saved comparisons only; writes analysis/summary.json and prints the C1/C2/C3 tables (PROTOCOL.md)."""
import json, glob, os
os.chdir(os.path.join(os.path.dirname(__file__), ".."))
W = ["0", "0.1", "1"]
def cell(d, a, b):
    main = json.load(open(d + f"{a}__{b}.json"))["ops"]
    pts = main["band"]["ratio_b_over_a"]
    out = {"w": {"0": {"ratio": main["ratio_b_over_a"], "ci": main["ci95"]}}, "order": main["band"]["order"], "pairs": main["pairs"]}
    for p in glob.glob(d + f"{a}__{b}__*.json"):
        o = json.load(open(p))["ops"]
        for k, w in ((1, "0.1"), (2, "1")):
            if abs(o["ratio_b_over_a"] - pts[k]) < 1e-9:
                out["w"][w] = {"ratio": o["ratio_b_over_a"], "ci": o["ci95"]}
    out["below_1"] = all(w in out["w"] and out["w"][w]["ci"][1] < 1 for w in W)
    return out
S = {}
for c in ("p36", "p40", "p42"):
    d = f"sessions/S-{c}/comparisons/"
    r = {"vs_base": {}, "dp_vs_eq": {}, "rho": {}}
    for b in (1, 4):
        for x in ("dp", "eq"):
            r["vs_base"][f"res{b}{x}/res{b}"] = cell(d, f"res{b}", f"res{b}{x}")
    for a in ("res1", "res4", "res1dp", "res4dp", "res1eq", "res4eq"):
        j = json.load(open(d + f"rho__{a}.json"))["ops"]
        r["rho"][a] = j["band"]["ratio_b_over_a"]
    aa = json.load(open(d + "res1__res1aa.json"))["ops"]
    r["aa"] = [aa["ratio_b_over_a"], aa["ci95"]]
    S[c] = r
json.dump(S, open("analysis/summary.json", "w"), indent=1)
def fmt(x): return "  ".join("w=%s %.3f [%.3f,%.3f]" % (w, x["w"][w]["ratio"], *x["w"][w]["ci"]) if w in x["w"] else "w=%s ?" % w for w in W)
for c, r in S.items():
    for k, x in r["vs_base"].items(): print(c, k, fmt(x), "| <1:", x["below_1"])
    print(c, "A/A", r["aa"])
    for a, v in r["rho"].items(): print(c, "rho->", a, [round(t, 2) for t in v])
print("C1 met (p40,p42 cells):", all(x["below_1"] for c, r in S.items() if c != "p36" for x in r["vs_base"].values()))
