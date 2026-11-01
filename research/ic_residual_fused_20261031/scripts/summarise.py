#!/usr/bin/env python3
"""Reads the saved comparisons only; writes analysis/summary.json and prints a table."""
import json, glob, os
os.chdir(os.path.join(os.path.dirname(__file__), ".."))
out = {}
def load(path): return json.load(open(path))
for c in ["p16", "p18", "p36", "p37"]:
    d = f"sessions/F-{c}/comparisons/"
    rows = {}
    for b in (2, 4, 8):
        j = load(d + f"res{b}__res{b}f.json")["ops"]
        rows[f"res{b}"] = {"pairs": j["pairs"], "w0": j["ratio_b_over_a"], "ci0": j["ci95"],
                           "band": dict(zip(["w0", "w0.1", "w1"], j["band"]["ratio_b_over_a"])), "order": j["band"]["order"]}
        for w in ("0.1", "1"):
            f = [p for p in glob.glob(d + f"res{b}__res{b}f__*.json")
                 if abs(load(p)["ops"].get("band", {}).get("weights", [0, 0, 0])[0] - 0) < 1e9]
        # per-weight CIs from the weight-specific saved files
        for p in glob.glob(d + f"res{b}__res{b}f__*.json"):
            o = load(p)["ops"]
            wt = o.get("weight", None)
            rows[f"res{b}"].setdefault("by_file", []).append({"file": os.path.basename(p), "ratio": o["ratio_b_over_a"], "ci": o["ci95"], "weight": wt})
    aa = load(d + "res4__res4aa.json")["ops"]
    rows["aa"] = {"ratio": aa["ratio_b_over_a"], "ci": aa["ci95"]}
    rho = {}
    for a in ("res2", "res4", "res8", "res2f", "res4f", "res8f"):
        j = load(d + f"rho__{a}.json")["ops"]
        rho[a] = {"w0": j["ratio_b_over_a"], "ci0": j["ci95"], "band": j["band"]["ratio_b_over_a"]}
    out[c] = {"fused_vs_base": rows, "vs_rho": rho}
json.dump(out, open("analysis/summary.json", "w"), indent=1)
for c, v in out.items():
    for b in ("res2", "res4", "res8"):
        r = v["fused_vs_base"][b]
        print(c, b, "fused/base w=0 %.3f [%.3f,%.3f]  w=0.1 %.3f  w=1 %.3f | %s" % (r["w0"], *r["ci0"], r["band"]["w0.1"], r["band"]["w1"], r["order"]))
    print(c, "A/A", "%.4f [%.3f,%.3f]" % (v["fused_vs_base"]["aa"]["ratio"], *v["fused_vs_base"]["aa"]["ci"]))
    for a in ("res4", "res4f"):
        r = v["vs_rho"][a]; print(c, "rho->", a, "w0 %.2f  band(0,1) %s" % (r["w0"], [round(x, 2) for x in r["band"]]))
